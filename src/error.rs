use core::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Transport error")]
    RequestTransportError(#[from] reqwest::Error),
    #[error("URL parse error")]
    UrlParseError(#[from] url::ParseError),
    #[error("Invalid request: {0}")]
    InvalidRequest(ErrorVec),
    #[error("Invalid query")]
    RequestQueryHeadersError(#[from] serde_qs::Error),
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
