/// Company details API.
pub mod company_enrichment;
/// Exchange rates API.
pub mod exchange_rates;
/// Geolocation API.
pub mod geolocation;
/// Holidays API.
pub mod holidays;
/// Common types that can be glob-imported for convenience.
pub mod prelude;

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
            }
        )
    }
}
