use crate::{
    error::{Error, ErrorVec},
    models::client::ErrorResponse,
};
use reqwest::{header, Method, RequestBuilder, StatusCode, Url};
use serde::de::DeserializeOwned;

/// Datadog's API client, designed to perform asynchronous calls.
pub struct Client {
    inner: reqwest::Client,
    api_url: Url,
}

/// Datadog's API client config.
pub struct Config {
    /// Datadog's API key.
    ////
    /// For more information about API keys, [visit this link](https://docs.datadoghq.com/account_management/api-app-keys/#api-keys).
    pub api_key: Option<String>,
    /// Datadog's API site.
    ///
    /// For more information about sites, [visit this link](https://docs.datadoghq.com/getting_started/site/).
    ///
    /// Default: [Site::Us1]
    pub site: Option<Site>,
    /// Datadog's application key.
    ////
    /// For more information about application keys, [visit this link](https://docs.datadoghq.com/account_management/api-app-keys/#application-keys).
    pub application_key: Option<String>,
}

/// Datadog's sites.
pub enum Site {
    /// US1 Location
    Us1,
    /// US3 Location
    Us3,
    /// US5 Location
    Us5,
    /// EU1 Location
    Eu1,
    /// US1-FED Location
    Us1Fed,
    /// AP1 Location
    Ap1,
    /// Custom location used for testing purposes.
    Custom(String),
}

impl Site {
    /// Returns the site API url.
    ///
    /// For custom sites will return the internal string.
    pub fn to_api_url(&self) -> Result<Url, Error> {
        match self {
            Site::Us1 => Ok(Url::parse("https://api.datadoghq.com")?),
            Site::Us3 => Ok(Url::parse("https://api.us3.datadoghq.com")?),
            Site::Us5 => Ok(Url::parse("https://api.us5.datadoghq.com")?),
            Site::Eu1 => Ok(Url::parse("https://api.datadoghq.eu")?),
            Site::Us1Fed => Ok(Url::parse("https://api.ddog-gov.com")?),
            Site::Ap1 => Ok(Url::parse("https://api.ap1.datadoghq.com")?),
            Site::Custom(s) => Ok(Url::parse(s)?),
        }
    }
}

/// Client builder for the [Client].
pub struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    /// Constructs a new [ClientBuilder].
    pub fn new(config: Config) -> Self {
        ClientBuilder { config }
    }

    /// Set's Datadog's site.
    pub fn set_site(mut self, site: Option<Site>) -> ClientBuilder {
        self.config.site = site;
        self
    }

    /// Set [Datadog application key](https://docs.datadoghq.com/account_management/api-app-keys/#application-keys)
    pub fn set_application_key(mut self, application_key: Option<String>) -> ClientBuilder {
        self.config.application_key = application_key;
        self
    }

    /// Set [Datadog api key](https://docs.datadoghq.com/account_management/api-app-keys/#api-keys)
    pub fn set_api_key(mut self, api_key: Option<String>) -> ClientBuilder {
        self.config.api_key = api_key;
        self
    }

    /// Returns a [Client] that uses this [ClientBuilder] configuration.
    pub fn build(self) -> Result<Client, Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Accept",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );

        if let Some(application_key) = self.config.application_key {
            let mut application_key_value =
                header::HeaderValue::from_str(&application_key).unwrap();
            application_key_value.set_sensitive(true);
            headers.insert("DD-APPLICATION-KEY", application_key_value);
        }

        if let Some(api_key) = self.config.api_key {
            let mut api_key_value = header::HeaderValue::from_str(&api_key).unwrap();
            api_key_value.set_sensitive(true);
            headers.insert("DD-API-KEY", api_key_value);
        }

        let api_url = self.config.site.unwrap_or(Site::Us1).to_api_url()?;
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        Ok(Client {
            inner: client,
            api_url,
        })
    }
}

impl Client {
    /// Constructs a new `Client`.
    ///
    /// # Panics
    ///
    /// This method panics if the api key or application key is invalid.
    ///
    /// Use `Client::builder()` if you wish to handle the failure as an `Error`
    /// instead of panicking.
    pub fn new(config: Config) -> Self {
        ClientBuilder::new(config).build().expect("Client::new()")
    }

    /// Builds a new request
    pub(crate) fn build_request(
        &self,
        method: Method,
        path: &str,
    ) -> Result<RequestBuilder, Error> {
        let extended_url = self.api_url.join(path)?;
        let req = self.inner.request(method, extended_url);

        Ok(req)
    }

    /// Sends a request and parses the response.
    pub(crate) async fn send_request<T>(&self, request: RequestBuilder) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let response = request.send().await?;
        let status = response.status();

        // TODO: Handle the [400, 403, 404, 429] in particular ways
        // parsing the errors returned by Datadog.
        // For other errors, display in a different way.
        if status == StatusCode::OK
            || status == StatusCode::CREATED
            || status == StatusCode::ACCEPTED
            || status == StatusCode::NO_CONTENT
        {
            Ok(response.json().await?)
        } else {
            let response: ErrorResponse = response.json().await?;
            Err(Error::InvalidRequest(ErrorVec(response.errors)))
        }
    }
}
