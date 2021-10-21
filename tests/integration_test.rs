use abstractapi::prelude::*;
use std::env;

#[test]
fn test_geolocation() -> Result<(), AbstractApiError> {
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
