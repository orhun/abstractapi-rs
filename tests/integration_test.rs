use abstractapi::prelude::*;
use pretty_assertions::assert_eq;
use std::env;
use std::thread;
use std::time::Duration;

type TestResult = Result<(), AbstractApiError>;
const SLEEP_DURATION: Option<&'static str> = option_env!("SLEEP_DURATION");

fn sleep() {
    thread::sleep(Duration::from_millis(
        SLEEP_DURATION
            .map(|v| v.parse().ok())
            .flatten()
            .unwrap_or(1000),
    ));
}

#[test]
fn test_geolocation_api() -> TestResult {
    let abstractapi = AbstractApi::with_api_keys(vec![(
        ApiType::Geolocation,
        env::var("GEOLOCATION_API_KEY").expect("GEOLOCATION_API_KEY is not set"),
    )])?;

    sleep();
    let geolocation: Geolocation = abstractapi.get_geolocation("172.217.19.142")?;
    assert_eq!(
        Some("Google LLC"),
        geolocation.connection.organization_name.as_deref()
    );
    Ok(())
}

#[test]
fn test_holidays_api() -> TestResult {
    let abstractapi = AbstractApi::with_api_keys(vec![(
        ApiType::Holidays,
        env::var("HOLIDAYS_API_KEY").expect("HOLIDAYS_API_KEY is not set"),
    )])?;

    sleep();
    let holidays: Holidays = abstractapi.get_holidays("TR", "2021", "10", "29")?;
    assert_eq!("Republic Day", holidays[0].name);
    Ok(())
}

#[test]
fn test_exchange_rates_api() -> TestResult {
    let abstractapi = AbstractApi::with_api_keys(vec![(
        ApiType::ExchangeRates,
        env::var("EXCHANGE_RATES_API_KEY").expect("EXCHANGE_RATES_API_KEY is not set"),
    )])?;

    sleep();
    let _ = abstractapi.get_latest_exchange_rates("BTC", Some("USD"))?;

    sleep();
    let rate = abstractapi.get_historical_exchange_rates("BTC", Some("USD"), "2021-01-31")?;
    assert_eq!(Some(33021.639792), rate.exchange_rates.usd);

    sleep();
    let converted_rate =
        abstractapi.convert_currency("USD", "TRY", Some("2021-01-31"), Some(10))?;
    assert_eq!(7.314766, converted_rate.exchange_rate);
    assert_eq!(73.14766, converted_rate.converted_amount);
    Ok(())
}

#[test]
fn test_company_enrichment_api() -> TestResult {
    let abstractapi = AbstractApi::with_api_keys(vec![(
        ApiType::CompanyEnrichment,
        env::var("COMPANY_ENRICHMENT_API_KEY").expect("COMPANY_ENRICHMENT_API_KEY is not set"),
    )])?;

    sleep();
    let company_details: CompanyDetails =
        abstractapi.get_company_details(Some("google.com"), None)?;
    assert_eq!(Some(1998), company_details.year_founded);
    assert_eq!(Some("United States"), company_details.country.as_deref());
    Ok(())
}

#[test]
fn test_timezone_api() -> TestResult {
    let abstractapi = AbstractApi::with_api_keys(vec![(
        ApiType::Timezone,
        env::var("TIMEZONE_API_KEY").expect("TIMEZONE_API_KEY is not set"),
    )])?;

    sleep();
    let current_time: LocationTime = abstractapi.get_current_time("Ankara")?;
    assert_eq!("Europe/Istanbul", current_time.timezone_location);
    assert_eq!(3., current_time.gmt_offset);

    sleep();
    let converted_time: ConvertedTime = abstractapi.convert_time(
        "Los Angeles,CA",
        "2020-05-01 07:00:00",
        "Oxford,United Kingdom",
    )?;
    assert_eq!("PDT", converted_time.base_location.timezone_name);
    assert_eq!(-7., converted_time.base_location.gmt_offset);
    assert_eq!("BST", converted_time.target_location.timezone_name);
    assert_eq!(1., converted_time.target_location.gmt_offset);
    Ok(())
}

#[test]
fn test_email_validation_api() -> TestResult {
    let abstractapi = AbstractApi::with_api_keys(vec![(
        ApiType::EmailValidation,
        env::var("EMAIL_VALIDATION_API_KEY").expect("EMAIL_VALIDATION_API_KEY is not set"),
    )])?;

    sleep();
    let email_result: EmailResult = abstractapi.validate_email("test@gmial.com", true)?;
    assert_eq!("test@gmail.com", email_result.autocorrect);
    assert_eq!("UNDELIVERABLE", email_result.deliverability);
    assert!(email_result.is_valid_format.value);

    sleep();
    let email_result: EmailResult = abstractapi.validate_email("test@yopmail.com", true)?;
    assert!(email_result.is_disposable_email.value);

    Ok(())
}

#[test]
fn test_phone_validation_api() -> TestResult {
    let abstractapi = AbstractApi::with_api_keys(vec![(
        ApiType::PhoneValidation,
        env::var("PHONE_VALIDATION_API_KEY").expect("PHONE_VALIDATION_API_KEY is not set"),
    )])?;

    sleep();
    let phone_result: PhoneResult = abstractapi.validate_phone("14152007986")?;
    assert!(phone_result.valid);
    assert_eq!("US", phone_result.country.code);
    assert_eq!("California", phone_result.location);
    assert_eq!("mobile", phone_result.type_);

    Ok(())
}

#[test]
fn test_vat_api() -> TestResult {
    let abstractapi = AbstractApi::with_api_keys(vec![(
        ApiType::Vat,
        env::var("VAT_API_KEY").expect("VAT_API_KEY is not set"),
    )])?;

    sleep();
    let vat_result = abstractapi.validate_vat("SE556656688001")?;
    assert!(vat_result.valid);
    assert_eq!("GOOGLE SWEDEN AB", vat_result.company.name);
    assert_eq!("SE", vat_result.country.code);

    sleep();
    let vat = abstractapi.calculate_vat(200., "DE", false, None)?;
    assert_eq!("standard", vat.vat_category);
    assert_eq!("DE", vat.country.code);

    sleep();
    let vat_rates = abstractapi.get_vat_rates("DE")?;
    assert!(vat_rates
        .iter()
        .any(|vat_rate| vat_rate.category == "books"));

    Ok(())
}
