//! Rust API bindings for the Abstract HTTP API.

#![warn(missing_docs)]

/// API bindings.
pub mod api;
/// Error implementation.
pub mod error;
/// Common types that can be glob-imported for convenience.
pub mod prelude;

use api::*;
use dashmap::DashMap;
use error::{Error, Result};
use std::fmt;
use std::time::Duration;
use ureq::{Agent as HttpClient, AgentBuilder, Request};

/// Base domain for Abstract API.
const ABSTRACTAPI_DOMAIN: &str = "abstractapi.com";

/// A supported API which is in free/paid plan.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ApiType {
    /// Geolocation API.
    Geolocation,
    /// Holidays API.
    Holidays,
    /// Exchange rates API.
    ExchangeRates,
    /// Company details API.
    CompanyEnrichment,
    /// Timezone API.
    Timezone,
    /// Email validation API.
    EmailValidation,
    /// Phone validation API.
    PhoneValidation,
    /// VAT API.
    Vat,
}

impl fmt::Display for ApiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Geolocation => "ipgeolocation",
                Self::Holidays => "holidays",
                Self::ExchangeRates => "exchange-rates",
                Self::CompanyEnrichment => "companyenrichment",
                Self::Timezone => "timezone",
                Self::EmailValidation => "emailvalidation",
                Self::PhoneValidation => "phonevalidation",
                Self::Vat => "vat",
            }
        )
    }
}

/// Client for Abstract API.
pub struct AbstractApi {
    http_client: HttpClient,
    api_keys: DashMap<ApiType, String>,
}

impl Default for AbstractApi {
    fn default() -> Self {
        Self::new_with_http_client(AgentBuilder::new().timeout(Duration::from_secs(15)).build())
    }
}

impl AbstractApi {
    /// Creates a new Abstract API client with the default HTTP client.
    pub fn new<S: Into<String>>(
        http_client: HttpClient,
        api_keys: Vec<(ApiType, S)>,
    ) -> Result<Self> {
        let mut abstractapi = Self::new_with_http_client(http_client);
        abstractapi.set_api_keys(api_keys)?;
        Ok(abstractapi)
    }

    /// Creates a new Abstract API client that uses the given HTTP client.
    pub fn new_with_http_client(http_client: HttpClient) -> Self {
        Self {
            http_client,
            api_keys: DashMap::new(),
        }
    }

    /// Creates a new Abstract API client with the given API key set.
    pub fn new_with_api_key<S: Into<String>>(api_type: ApiType, api_key: S) -> Result<Self> {
        let mut abstractapi = Self::default();
        abstractapi.set_api_key(api_type, api_key)?;
        Ok(abstractapi)
    }

    /// Creates a new Abstract API client with the given API keys set.
    pub fn new_with_api_keys<S: Into<String>>(api_keys: Vec<(ApiType, S)>) -> Result<Self> {
        let mut abstractapi = Self::default();
        abstractapi.set_api_keys(api_keys)?;
        Ok(abstractapi)
    }

    /// Sets an API key for an API.
    pub fn set_api_key<S: Into<String>>(&mut self, api_type: ApiType, api_key: S) -> Result<()> {
        match self.api_keys.insert(api_type, api_key.into()) {
            Some(_) => Err(Error::ApiKeySetError),
            _ => Ok(()),
        }
    }

    /// Sets the API keys for specified APIs.
    pub fn set_api_keys<S: Into<String>>(&mut self, api_keys: Vec<(ApiType, S)>) -> Result<()> {
        for (api_type, api_key) in api_keys {
            self.set_api_key(api_type, api_key)?;
        }
        Ok(())
    }

    /// Constructs and returns an HTTP request for an API.
    fn get_api_request(&self, api_type: ApiType, path: &str) -> Result<Request> {
        Ok(self
            .http_client
            .get(&format!("https://{api_type}.{ABSTRACTAPI_DOMAIN}/{path}/"))
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
            .call()
            .map_err(Error::from)?
            .into_json()?)
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
            .call()
            .map_err(Error::from)?
            .into_json()?)
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
        Ok(request.call().map_err(Error::from)?.into_json()?)
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
        Ok(request.call().map_err(Error::from)?.into_json()?)
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
        Ok(request.call().map_err(Error::from)?.into_json()?)
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
        Ok(request.call().map_err(Error::from)?.into_json()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/timezone/documentation>
    pub fn get_current_time<S: AsRef<str>>(&self, location: S) -> Result<LocationTime> {
        Ok(self
            .get_api_request(ApiType::Timezone, "v1/current_time")?
            .query("location", location.as_ref())
            .call()
            .map_err(Error::from)?
            .into_json()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/timezone/documentation>
    pub fn convert_time<S: AsRef<str>>(
        &self,
        base_location: S,
        base_datetime: S,
        target_location: S,
    ) -> Result<ConvertedTime> {
        Ok(self
            .get_api_request(ApiType::Timezone, "v1/convert_time")?
            .query("base_location", base_location.as_ref())
            .query("base_datetime", base_datetime.as_ref())
            .query("target_location", target_location.as_ref())
            .call()
            .map_err(Error::from)?
            .into_json()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/email-validation/documentation>
    pub fn validate_email<S: AsRef<str>>(
        &self,
        email: S,
        auto_correct: bool,
    ) -> Result<EmailDetails> {
        Ok(self
            .get_api_request(ApiType::EmailValidation, "v1")?
            .query("email", email.as_ref())
            .query("auto_correct", &auto_correct.to_string())
            .call()
            .map_err(Error::from)?
            .into_json()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/phone-validation/documentation>
    pub fn validate_phone<S: AsRef<str>>(&self, phone: S) -> Result<PhoneDetails> {
        Ok(self
            .get_api_request(ApiType::PhoneValidation, "v1")?
            .query("phone", phone.as_ref())
            .call()
            .map_err(Error::from)?
            .into_json()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/vat/documentation>
    pub fn validate_vat<S: AsRef<str>>(&self, vat_number: S) -> Result<VatDetails> {
        Ok(self
            .get_api_request(ApiType::Vat, "v1/validate")?
            .query("vat_number", vat_number.as_ref())
            .call()
            .map_err(Error::from)?
            .into_json()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/vat/documentation>
    pub fn calculate_vat<S: AsRef<str>>(
        &self,
        amount: f64,
        country_code: S,
        is_vat_incl: bool,
        vat_category: Option<S>,
    ) -> Result<Vat> {
        let mut request = self
            .get_api_request(ApiType::Vat, "v1/calculate")?
            .query("amount", &amount.to_string())
            .query("country_code", country_code.as_ref())
            .query("is_vat_incl", &is_vat_incl.to_string());
        if let Some(vat_category) = vat_category {
            request = request.query("vat_category", vat_category.as_ref())
        }
        Ok(request.call().map_err(Error::from)?.into_json()?)
    }

    /// Upstream documentation: <https://app.abstractapi.com/api/vat/documentation>
    pub fn get_vat_rates<S: AsRef<str>>(&self, country_code: S) -> Result<VatRates> {
        Ok(self
            .get_api_request(ApiType::Vat, "v1/categories")?
            .query("country_code", country_code.as_ref())
            .call()
            .map_err(Error::from)?
            .into_json()?)
    }
}
