fn main() {
    // Create a new client.
    let mut abstractapi = abstractapi::AbstractApi::default();
    abstractapi
        .set_api_key(
            abstractapi::ApiType::Vat,
            std::env::var("VAT_API_KEY").unwrap(),
        )
        .unwrap();

    // Get the VAT details from the given VAT number.
    let vat_details = abstractapi.validate_vat("SE556656688001").unwrap();

    // Print the result in a pretty format.
    println!(
        "VAT: {}, Company: {}, Country: {}, valid: {}",
        vat_details.vat_number,
        vat_details.company.name,
        vat_details.country.name,
        vat_details.valid
    );

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Calculate a VAT compliant price from the given country and price.
    let vat = abstractapi.calculate_vat(200., "DE", false, None).unwrap();

    // Print the result in a pretty format.
    println!(
        "VAT amount for {} EUR: {}, including VAT: {}, VAT rate: {}, Country: {}",
        vat.amount_excluding_vat,
        vat.vat_amount,
        vat.amount_including_vat,
        vat.vat_rate,
        vat.country.name,
    );

    // Get the latest VAT rates for the given country.
    let vat_rates = abstractapi.get_vat_rates("DE").unwrap();

    // Print the results.
    for vat_rate in vat_rates {
        println!(
            "VAT rate: {}, Country: {}, Category: {}",
            vat_rate.rate, vat_rate.country_code, vat_rate.category,
        )
    }
}
