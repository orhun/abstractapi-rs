fn main() {
    // Create a new client.
    let mut abstractapi = abstractapi::AbstractApi::default();
    abstractapi
        .set_api_key(
            abstractapi::api::ApiType::PhoneValidation,
            std::env::var("PHONE_VALIDATION_API_KEY").unwrap(),
        )
        .unwrap();

    // Get the phone number details.
    let phone_details = abstractapi.validate_phone("14152007986").unwrap();

    // Print the result in a pretty format.
    println!(
        "Phone: {}, Country: {}, Location: {}, valid: {}, type: {}",
        phone_details.phone,
        phone_details.country.name,
        phone_details.location,
        phone_details.valid,
        phone_details.type_,
    );
}
