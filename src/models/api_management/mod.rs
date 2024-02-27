use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::client::{EmptyResponse, EmptyStruct, Response};

#[derive(Debug, Serialize, Deserialize)]
/// Data necessary to create an OpenAPI.
pub struct OpenApiCreateRequest {
    /// Binary OpenAPI spec file.
    pub openapi_spec_file: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Endpoint which couldn't be parsed.
pub struct FailedEndpoints {
    /// The endpoint method.
    pub method: String,
    /// The endpoint path.
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Attributes for `CreateOpenAPI`.
pub struct CreateOpenApiResponseAttributes {
    /// List of endpoints which couldn't be parsed.
    pub failed_endpoints: Vec<FailedEndpoints>,
}

/// Data envelope for [CreateOpenAPIResponse].
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiCreateResponseData {
    /// Attributes for `CreateOpenAPI`.
    pub attributes: CreateOpenApiResponseAttributes,
    /// API identifier.
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
/// Attributes for [OpenApiUpdateResponseData].
pub struct OpenApiUpdateResponseDataAttributes {
    /// List of endpoints which couldn't be parsed.
    pub failed_endpoints: Vec<FailedEndpoints>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Request fields to update OpenAPI.
pub struct OpenApiUpdateRequest {
    /// Binary OpenAPI spec file.
    pub openapi_spec_file: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Data envelope for [OpenApiUpdateResponse].
pub struct OpenApiUpdateResponseData {
    /// Attributes for [OpenApiUpdateResponseData]
    pub attributes: OpenApiUpdateResponseDataAttributes,
    /// API identifier.
    pub id: String,
}

/// Requests and responses
/// Create OpenAPI response.
pub type OpenApiCreateResponse = Response<OpenApiCreateResponseData>;

/// Get OpenAPI response.
pub type OpenApiGetResponse = EmptyStruct;

/// Update OpenAPI response.
pub type OpenApiUpdateResponse = Response<OpenApiUpdateResponseData>;

/// Delete OpenAPI response.
pub type OpenApiDeleteResponse = EmptyResponse;
