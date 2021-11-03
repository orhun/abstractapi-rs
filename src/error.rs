#![allow(missing_docs)]

use crate::api::ApiType;
use std::error::Error as StdError;

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
        if cfg!(debug_assertions) {
            Self::RequestError(format!("{:?}", error))
        } else {
            Self::RequestError(match error {
                ureq::Error::Status(code, _) => {
                    format!("Request error, code: {}", code.to_string())
                }
                ureq::Error::Transport(e) => format!("Transport error, source: {:?}", e.source()),
            })
        }
    }
}

/// Alias for the standard [`Result`] type.
pub(crate) type Result<T> = std::result::Result<T, Error>;
