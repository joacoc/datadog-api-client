use reqwest::Method;

use crate::{client::Client, error::Error, models::authentication::ValidateApiKeyResponse};

const BASE_PATH: &str = "api/v1/validate";

impl Client {
    /// Check if the API key (not the APP key) is valid. If invalid, a 403 is returned.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/authentication/#validate-api-key)
    pub async fn validate_api_key(&self) -> Result<ValidateApiKeyResponse, Error> {
        let req: reqwest::RequestBuilder =
            self.build_request(Method::GET, &format!("{}", BASE_PATH))?;

        self.send_request::<ValidateApiKeyResponse>(req).await
    }
}
