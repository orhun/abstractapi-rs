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
    fn get_api_request(&self, api_type: ApiType, path: &str) -> Result<Request> {
        Ok(self
            .http_client
            .get(&format!(
                "https://{}.{}/{}/",
                api_type, ABSTRACTAPI_DOMAIN, path
            ))
            .query(
                "api_key",
                self.api_keys
                    .get(&api_type)
                    .ok_or(Error::ApiKeyNotPresent(api_type))?
                    .value(),
            ))
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/ip-geolocation/documentation>
    pub fn get_geolocation<S: AsRef<str>>(&self, ip_address: S) -> Result<Geolocation> {
        Ok(self
            .get_api_request(ApiType::Geolocation, "v1")?
            .query("ip_address", ip_address.as_ref())
            .call()?
            .into_json::<Geolocation>()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/holidays/documentation>
    pub fn get_holidays<S: AsRef<str>>(
        &self,
        country: S,
        year: S,
        month: S,
        day: S,
    ) -> Result<Holidays> {
        Ok(self
            .get_api_request(ApiType::Holidays, "v1")?
            .query("country", country.as_ref())
            .query("year", year.as_ref())
            .query("month", month.as_ref())
            .query("day", day.as_ref())
            .call()?
            .into_json::<Holidays>()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/exchange-rates/documentation>
    pub fn get_latest_exchange_rates<S: AsRef<str>>(
        &self,
        base: S,
        target: Option<S>,
    ) -> Result<ExchangeRatesResult> {
        let mut request = self
            .get_api_request(ApiType::ExchangeRates, "v1/live")?
            .query("base", base.as_ref());
        if let Some(target) = target {
            request = request.query("target", target.as_ref());
        }
        Ok(request.call()?.into_json::<ExchangeRatesResult>()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/exchange-rates/documentation>
    pub fn get_historical_exchange_rates<S: AsRef<str>>(
        &self,
        base: S,
        target: Option<S>,
        date: S,
    ) -> Result<ExchangeRatesResult> {
        let mut request = self
            .get_api_request(ApiType::ExchangeRates, "v1/historical")?
            .query("base", base.as_ref())
            .query("date", date.as_ref());
        if let Some(target) = target {
            request = request.query("target", target.as_ref());
        }
        Ok(request.call()?.into_json::<ExchangeRatesResult>()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/exchange-rates/documentation>
    pub fn convert_currency<S: AsRef<str>>(
        &self,
        base: S,
        target: S,
        date: Option<S>,
        base_amount: Option<u64>,
    ) -> Result<ConvertedExchangeRate> {
        let mut request = self
            .get_api_request(ApiType::ExchangeRates, "v1/convert")?
            .query("base", base.as_ref())
            .query("target", target.as_ref());
        if let Some(date) = date {
            request = request.query("date", date.as_ref());
        }
        if let Some(base_amount) = base_amount {
            request = request.query("base_amount", &base_amount.to_string());
        }
        Ok(request.call()?.into_json::<ConvertedExchangeRate>()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/company-enrichment/documentation>
    pub fn get_company_details<S: AsRef<str>>(
        &self,
        domain: Option<S>,
        email: Option<S>,
    ) -> Result<CompanyDetails> {
        let mut request = self.get_api_request(ApiType::CompanyEnrichment, "v1")?;
        if let Some(domain) = domain {
            request = request.query("domain", domain.as_ref());
        }
        if let Some(email) = email {
            request = request.query("email", email.as_ref());
        }
        Ok(request.call()?.into_json::<CompanyDetails>()?)
    }
}
