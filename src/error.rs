use core::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
/// Contains the different error types in the client.
pub enum Error {
    /// Error occurred in the underlying transport mechanism (e.g., network error).
    #[error("Transport error")]
    RequestTransport(#[from] reqwest::Error),

    /// Error occurred while parsing a URL.
    #[error("URL parse error")]
    InvalidUrl(#[from] url::ParseError),

    /// The request to Datadog's API was invalid.
    /// This variant holds the specific reasons why the request was considered invalid.
    #[error("Invalid request: {0}")]
    InvalidRequest(ErrorVec),

    /// Error occurred when serializing or deserializing query parameters or headers.
    #[error("Invalid query")]
    InvalidRequestQueryHeaders(#[from] serde_qs::Error),
}

/// The thiserror crate lacks support for directly formatting collections like Vec.
/// Given that Vec itself does not implement fmt::Display out of the box,
/// the struct [ErrorVec] is a workaround.
#[derive(Debug)]
pub struct ErrorVec(pub Vec<String>);

impl fmt::Display for ErrorVec {
    // Method to format the Vec<String> for the InvalidRequest variant
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}
