use serde::{Deserialize, Deserializer, Serialize};

/// A helpful struct to parse API responses with an empty json object `{}`
#[derive(Deserialize)]
pub struct EmptyStruct {}

/// A helpful struct to parse API responses with an empty error array.
/// This means that the request was correct and no errors are present.
#[derive(Deserialize)]
pub struct EmptyErrorsResponse {
    /// Empty array of errors.
    pub errors: Vec<String>,
}

/// A helpful struct to parse API responses with no data (204/NO_CONTENT).
pub struct EmptyResponse {}

impl<'de> Deserialize<'de> for EmptyResponse {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(EmptyResponse {})
    }
}

/// Links attributes.
#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    /// Link for the next set of results. Note that the request can also be made using the POST endpoint.
    next: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// A list of warnings (non-fatal errors) encountered. Partial results may return if warnings are present in the response.
pub struct Warnings {
    /// Unique code for this type of warning.
    code: String,
    /// Detailed explanation of this specific warning.
    detail: String,
    /// Short human-readable summary of the warning.
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Paging attributes.
pub struct Page {
    /// The cursor to use to get the next results, if any.
    /// To make the next request, use the same parameters with the addition of page[cursor].
    after: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// The status of the response. Allowed enum values: done,timeout
pub enum Status {
    #[serde(rename = "done")]
    /// Reponse was ok.
    Done,
    #[serde(rename = "timeout")]
    /// Response timedout.
    Timeout,
}

/// The metadata associated with a request.
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    /// Time elapsed in milliseconds.
    elapsed: i64,
    /// Paging attributes.
    page: Page,
    /// The identifier of the request.
    request_id: String,
    /// The status of the response. Allowed enum values: `done`,`timeout`
    status: Status,
    /// A list of warnings (non-fatal errors) encountered.
    /// Partial results may return if warnings are present in the response.
    warnings: Vec<Warnings>,
}

#[derive(Debug, Serialize, Deserialize)]
/// A response object matching the request.
pub struct Response<T> {
    /// Response data envelope.
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
/// Paged response object matching the request and pagination information.
pub struct PagedResponse<T> {
    /// Array of events matching the request.
    pub data: Vec<T>,
    /// Links attributes.
    pub links: Option<Link>,
    /// The metadata associated with a request.
    pub meta: Option<Meta>,
}

#[derive(Debug, Serialize, Deserialize)]
/// A request object.
pub struct Request<T> {
    /// Request data envelope.
    pub data: T,
}

#[derive(Debug, Deserialize)]
/// An error response object.
pub struct ErrorResponse {
    /// A list of errors.
    pub errors: Vec<String>,
}
