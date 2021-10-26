//! Rust API bindings for the Abstract HTTP API.

#![warn(missing_docs)]

/// API bindings.
pub mod api;
/// Error implementation.
pub mod error;
/// Common types that can be glob-imported for convenience.
pub mod prelude;

use api::prelude::*;
use dashmap::DashMap;
use error::{Error, Result};
use std::time::Duration;
use ureq::{Agent as HttpClient, AgentBuilder, Request};

/// Base domain for Abstract API.
const ABSTRACTAPI_DOMAIN: &str = "abstractapi.com";

/// Client for Abstract API.
pub struct AbstractApi {
    http_client: HttpClient,
    api_keys: DashMap<ApiType, String>,
}

impl Default for AbstractApi {
    fn default() -> Self {
        Self::new()
    }
}

impl AbstractApi {
    /// Creates a new Abstract API client with the default HTTP client.
    pub fn new() -> Self {
        Self::with_http_client(AgentBuilder::new().timeout(Duration::from_secs(15)).build())
    }

    /// Creates a new Abstract API client that uses the given HTTP client.
    pub fn with_http_client(http_client: HttpClient) -> Self {
        Self {
            http_client,
            api_keys: DashMap::new(),
        }
    }

    /// Sets an API key for an API.
    pub fn set_api_key<S: Into<String>>(&mut self, api_type: ApiType, api_key: S) -> Result<()> {
        match self.api_keys.insert(api_type, api_key.into()) {
            Some(_) => Err(Error::ApiKeySetError),
            _ => Ok(()),
        }
    }

    /// Constructs and returns an HTTP request for an API.
    fn get_api_request(&self, api_type: ApiType) -> Result<Request> {
        Ok(self
            .http_client
            .get(&format!("https://{}.{}/v1/", api_type, ABSTRACTAPI_DOMAIN))
            .query(
                "api_key",
                self.api_keys
                    .get(&api_type)
                    .ok_or(Error::ApiKeyNotPresent(api_type))?
                    .value(),
            ))
    }

    /// Upstream documentation: <https://www.abstractapi.com/ip-geolocation-api>
    pub fn get_geolocation<S: AsRef<str>>(&self, ip_address: S) -> Result<Geolocation> {
        Ok(self
            .get_api_request(ApiType::Geolocation)?
            .query("ip_address", ip_address.as_ref())
            .call()?
            .into_json::<Geolocation>()?)
    }

    /// Upstream documentation: <https://www.abstractapi.com/holidays-api>
    pub fn get_holidays<S: AsRef<str>>(
        &self,
        country: S,
        year: S,
        month: S,
        day: S,
    ) -> Result<Holidays> {
        Ok(self
            .get_api_request(ApiType::Holidays)?
            .query("country", country.as_ref())
            .query("year", year.as_ref())
            .query("month", month.as_ref())
            .query("day", day.as_ref())
            .call()?
            .into_json::<Holidays>()?)
    }
}
