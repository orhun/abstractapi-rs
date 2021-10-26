use abstractapi::prelude::*;
use pretty_assertions::assert_eq;
use std::env;
use std::thread;
use std::time::Duration;

#[test]
fn test_geolocation_api() -> Result<(), AbstractApiError> {
    let mut abstractapi = AbstractApi::new();
    abstractapi.set_api_key(
        ApiType::Geolocation,
        env::var("GEOLOCATION_API_KEY").expect("GEOLOCATION_API_KEY is not set"),
    )?;

    let geolocation: Geolocation = abstractapi.get_geolocation("172.217.19.142")?;
    assert_eq!(
        Some("Google LLC"),
        geolocation.connection.organization_name.as_deref()
    );
    Ok(())
}

#[test]
fn test_holidays_api() -> Result<(), AbstractApiError> {
    let mut abstractapi = AbstractApi::new();
    abstractapi.set_api_key(
        ApiType::Holidays,
        env::var("HOLIDAYS_API_KEY").expect("HOLIDAYS_API_KEY is not set"),
    )?;

    let holidays: Holidays = abstractapi.get_holidays("TR", "2021", "10", "29")?;
    assert_eq!("Republic Day", holidays[0].name);
    Ok(())
}

#[test]
fn test_exchange_rates_api() -> Result<(), AbstractApiError> {
    let mut abstractapi = AbstractApi::new();
    abstractapi.set_api_key(
        ApiType::ExchangeRates,
        env::var("EXCHANGE_RATES_API_KEY").expect("EXCHANGE_RATES_API_KEY is not set"),
    )?;

    let _ = abstractapi.get_latest_exchange_rates("BTC", Some("USD"))?;
    thread::sleep(Duration::from_secs(1));

    let rate = abstractapi.get_historical_exchange_rates("BTC", Some("USD"), "2021-01-31")?;
    assert_eq!(Some(33021.639792), rate.exchange_rates.usd);
    thread::sleep(Duration::from_secs(1));

    let converted_rate =
        abstractapi.convert_currency("USD", "TRY", Some("2021-01-31"), Some(10))?;
    assert_eq!(7.314766, converted_rate.exchange_rate);
    assert_eq!(73.14766, converted_rate.converted_amount);
    Ok(())
}

#[test]
fn test_company_enrichment_api() -> Result<(), AbstractApiError> {
    let mut abstractapi = AbstractApi::new();
    abstractapi.set_api_key(
        ApiType::CompanyEnrichment,
        env::var("COMPANY_ENRICHMENT_API_KEY").expect("COMPANY_ENRICHMENT_API_KEY is not set"),
    )?;

    let company_details: CompanyDetails =
        abstractapi.get_company_details(Some("google.com"), None)?;
    assert_eq!(Some(1998), company_details.year_founded);
    assert_eq!(Some("United States"), company_details.country.as_deref());
    Ok(())
}
