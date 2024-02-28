use serde::Deserialize;

/// Represent validation endpoint responses.
#[derive(Deserialize)]
pub struct ValidateApiKeyResponse {
    /// Return true if the authentication response is valid.
    pub valid: bool,
}
