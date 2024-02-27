use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::models::api_management::{
    OpenApiCreateRequest, OpenApiCreateResponse, OpenApiDeleteResponse, OpenApiGetResponse,
    OpenApiUpdateRequest, OpenApiUpdateResponse,
};

pub(crate) static BASE_PATH: &str = "api/v2/apicatalog";

impl Client {
    /// Create a new API from the [OpenAPI](https://spec.openapis.org/oas/latest.html) specification given. It supports version 2.0, 3.0 and 3.1 of the specification.
    /// A specific extension section, x-datadog, let you specify the teamHandle for your team responsible for the API in Datadog.
    /// It returns the created API ID.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/api-management/#create-a-new-api)
    pub async fn create_openapi(
        &self,
        request: OpenApiCreateRequest,
    ) -> Result<OpenApiCreateResponse, Error> {
        let req = self.build_request(Method::POST, &format!("{}/openapi", BASE_PATH))?;
        let req = req.json(&request);

        self.send_request::<OpenApiCreateResponse>(req).await
    }

    /// Retrieve information about a specific API in [OpenAPI](https://spec.openapis.org/oas/latest.html) format file.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/api-management/#get-an-api)
    pub async fn get_openapi(&self, id: &str) -> Result<OpenApiGetResponse, Error> {
        let req = self.build_request(Method::GET, &format!("{}/api/{}/openapi", BASE_PATH, id))?;

        self.send_request(req).await
    }

    /// Update information about a specific API.
    /// The given content will replace all API content of the given ID.
    /// The ID is returned by the create API, or can be found in the URL in the API catalog UI.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/api-management/#update-an-api)
    pub async fn update_openapi(
        &self,
        id: &str,
        request: OpenApiUpdateRequest,
    ) -> Result<OpenApiUpdateResponse, Error> {
        let req = self.build_request(Method::PUT, &format!("{}/api/{}/openapi", BASE_PATH, id))?;
        let req = req.json(&request);

        self.send_request(req).await
    }

    /// Delete a specific API by ID.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/api-management/#delete-an-api)
    pub async fn delete_openapi(&self, id: &str) -> Result<OpenApiDeleteResponse, Error> {
        let req = self.build_request(Method::DELETE, &format!("{}/api/{}", BASE_PATH, id))?;

        self.send_request(req).await
    }
}
