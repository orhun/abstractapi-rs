#![allow(missing_docs)]

use crate::ApiType;
use lazy_regex::*;

/// Regex for matching the API key from an error response.
static API_KEY_REGEX: Lazy<Regex> = lazy_regex!("api_key=[a-zA-Z0-9]{32,}");

thiserror_lite::err_enum! {
    /// Custom errors.
    #[derive(Debug)]
    pub enum Error {
        // Error that may occur when an API key is already set.
        #[error("This API key is already set")]
        ApiKeySetError,
        // Error that may occur when an API key is not present.
        #[error("API key is not present: `{0}`")]
        ApiKeyNotPresent(ApiType),
        // Error that may occur when handling a request.
        #[error("Request error: `{0}`")]
        RequestError(String),
        // Error that may occur while handling IO operations.
        #[error("IO error: `{0}`")]
        IoError(#[from] std::io::Error)
    }
}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        Self::RequestError(
            API_KEY_REGEX
                .replace(&format!("{:?}", error), "api_key=***")
                .to_string(),
        )
    }
}

/// Alias for the standard [`Result`] type.
pub(crate) type Result<T> = std::result::Result<T, Error>;
