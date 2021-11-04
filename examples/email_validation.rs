fn main() {
    // Create a new client.
    let mut abstractapi = abstractapi::AbstractApi::default();
    abstractapi
        .set_api_key(
            abstractapi::api::ApiType::EmailValidation,
            std::env::var("EMAIL_VALIDATION_API_KEY").unwrap(),
        )
        .unwrap();

    // Get the email details.
    let email_details = abstractapi.validate_email("test@gmial.com", true).unwrap();

    // Print the result in a pretty format.
    println!(
        "Email: {:?} (corrected: {:?}), status: {}, valid: {}, free: {}, disposable: {}, score: {}",
        email_details.email,
        email_details.autocorrect,
        email_details.deliverability,
        email_details.is_valid_format.value,
        email_details.is_free_email.value,
        email_details.is_disposable_email.value,
        email_details.quality_score
    );
}
