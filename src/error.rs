#![allow(missing_docs)]

use crate::api::ApiType;

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
        RequestError(#[from] ureq::Error),
        // Error that may occur while handling IO operations.
        #[error("IO error: `{0}`")]
        IoError(#[from] std::io::Error)
    }
}

/// Alias for the standard [`Result`] type.
pub(crate) type Result<T> = std::result::Result<T, Error>;
