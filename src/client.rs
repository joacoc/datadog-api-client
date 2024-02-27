use crate::{
    error::{Error, ErrorVec},
    models::client::ErrorResponse,
};
use reqwest::{header, Method, RequestBuilder, StatusCode, Url};
use serde::de::DeserializeOwned;

const DATADOG_API_URL: &str = "https://api.datadoghq.com/";

/// Datadog's API client, designed to perform asynchronous calls.
pub struct Client {
    inner: reqwest::Client,
    api_url: Url,
}

struct Config {
    api_key: String,
    // TODO: Rename to site
    // Check if wiremock supports it.
    api_url: Url,
    application_key: String,
}

/// Client builder for the [Client].
pub struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    /// Constructs a new [ClientBuilder].
    pub fn new(api_key: &str, application_key: &str) -> Self {
        ClientBuilder {
            config: Config {
                api_key: api_key.to_string(),
                // TODO: Remove unwrap.
                api_url: Url::parse(DATADOG_API_URL).unwrap(),
                application_key: application_key.to_string(),
            },
        }
    }

    /// Set's Datadog's API url. Used for internal test.
    pub fn set_api_url(mut self, url: Url) -> ClientBuilder {
        self.config.api_url = url;
        self
    }

    /// Set [Datadog application key](https://docs.datadoghq.com/account_management/api-app-keys/#application-keys)
    pub fn set_application_key(mut self, application_key: &str) -> ClientBuilder {
        self.config.application_key = application_key.to_string();
        self
    }

    /// Set [Datadog api key](https://docs.datadoghq.com/account_management/api-app-keys/#api-keys)
    pub fn set_api_key(mut self, api_key: &str) -> ClientBuilder {
        self.config.api_key = api_key.to_string();
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
        let mut application_key_value =
            header::HeaderValue::from_str(&self.config.application_key).unwrap();
        application_key_value.set_sensitive(true);
        headers.insert("DD-APPLICATION-KEY", application_key_value);
        let mut api_key_value = header::HeaderValue::from_str(&self.config.api_key).unwrap();
        api_key_value.set_sensitive(true);
        headers.insert("DD-API-KEY", api_key_value);

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        Ok(Client {
            inner: client,
            api_url: self.config.api_url,
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
    pub fn new(api_key: &str, application_key: &str) -> Self {
        ClientBuilder::new(api_key, application_key)
            .build()
            .expect("Client::new()")
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
            || status == StatusCode::NO_CONTENT
        {
            Ok(response.json().await?)
        } else {
            let response: ErrorResponse = response.json().await?;
            Err(Error::InvalidRequest(ErrorVec(response.errors)))
        }
    }
}
