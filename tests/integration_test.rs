use abstractapi::prelude::*;
use pretty_assertions::assert_eq;
use std::env;

#[test]
fn test_geolocation_api() -> Result<(), AbstractApiError> {
    let mut abstractapi = AbstractApi::new();
    abstractapi.set_api_key(
        ApiType::Geolocation,
        env::var("GEOLOCATION_API_KEY").expect("GEOLOCATION_API_KEY is not set"),
    )?;
    let geolocation: Geolocation = abstractapi.get_geolocation("172.217.19.142")?;
    assert_eq!(
        "Google LLC",
        geolocation.connection.organization_name.unwrap()
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
    assert_eq!("Republic Day", holidays.first().unwrap().name);
    Ok(())
}
