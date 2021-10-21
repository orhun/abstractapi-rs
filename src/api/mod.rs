/// Geolocation API.
pub mod geolocation;

use std::fmt;

/// Type of an API.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ApiType {
    /// Geolocation API.
    Geolocation,
}

impl fmt::Display for ApiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Geolocation => "ipgeolocation",
            }
        )
    }
}
