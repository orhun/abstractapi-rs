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
                .replace(&format!("{error:?}"), "api_key=***")
                .to_string(),
        )
    }
}

/// Alias for the standard [`Result`] type.
pub(crate) type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_hide_api_key() {
        let mock_url = "https://emailvalidation.abstractapi.com/v1/?api_key=ef0482afc956e2ede15ed2d4b7c9c01e&email=test%40gmial.com&auto_correct=false";
        let ureq_error = ureq::get(mock_url).call().unwrap_err();
        let error = Error::from(ureq_error);
        assert_eq!("RequestError(\"Status(401, Response[status: 401, status_text: Unauthorized, \
            url: https://emailvalidation.abstractapi.com/v1/?api_key=***&email=test%40gmial.com&auto_correct=false])\")", format!("{error:?}"));
    }
}
