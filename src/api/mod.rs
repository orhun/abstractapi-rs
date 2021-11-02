/// Company details API.
pub mod company_enrichment;
/// Email validation API.
pub mod email_validation;
/// Exchange rates API.
pub mod exchange_rates;
/// Geolocation API.
pub mod geolocation;
/// Holidays API.
pub mod holidays;
/// Phone validation API.
pub mod phone_validation;
/// Common types that can be glob-imported for convenience.
pub mod prelude;
/// Timezone API.
pub mod timezone;

use std::fmt;

/// Type of an API.
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
            }
        )
    }
}
