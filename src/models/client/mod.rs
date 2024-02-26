use serde::{Deserialize, Deserializer, Serialize};

/// For when the API returns an empty json object `{}`
#[derive(Deserialize)]
pub struct EmptyStruct {}

/// For when the API returns no data as a response (204/NO_CONTENT).
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
pub struct Warnings {
    /// Unique code for this type of warning.
    code: String,
    /// Detailed explanation of this specific warning.
    detail: String,
    /// Short human-readable summary of the warning.
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    /// The cursor to use to get the next results, if any.
    /// To make the next request, use the same parameters with the addition of page[cursor].
    after: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "timeout")]
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
pub struct Response<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub links: Option<Link>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
}
