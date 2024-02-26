use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::client::{EmptyResponse, EmptyStruct, Request, Response};

#[derive(Debug, Serialize, Deserialize)]
/// Data necessary to create an OpenAPI.
pub struct CreateOpenApiRequestData {
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
pub struct CreateOpenApiResponseData {
    /// Attributes for `CreateOpenAPI`.
    pub attributes: CreateOpenApiResponseAttributes,
    /// API identifier.
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
/// Attributes for `UpdateOpenAPI`.
pub struct UpdateOpenApiResponseAttributes {
    /// List of endpoints which couldn't be parsed.
    pub failed_endpoints: Vec<FailedEndpoints>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenApiParams {
    /// ID of the API to retrieve.
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOpenApiRequestData {
    /// ID of the API to retrieve.
    pub id: String,
    /// Binary OpenAPI spec file.
    pub openapi_spec_file: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Data envelope for UpdateOpenAPIResponse.
pub struct UpdateOpenApiResponseData {
    /// Attributes for UpdateOpenAPI.
    pub attributes: UpdateOpenApiResponseAttributes,
    /// API identifier.
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOpenApiRequestData {
    /// ID of the API to retrieve.
    pub id: String,
}

/// Responses and requests
pub type CreateOpenApiResponse = Response<CreateOpenApiResponseData>;
pub type CreateOpenApiRequest = CreateOpenApiRequestData;

pub type GetOpenApiResponse = EmptyStruct;

pub type UpdateOpenApiRequest = Request<UpdateOpenApiRequestData>;
pub type UpdateOpenApiResponse = Request<UpdateOpenApiResponseData>;

pub type DeleteOpenApiRequest = Request<DeleteOpenApiRequestData>;
pub type DeleteOpenApiResponse = EmptyResponse;
