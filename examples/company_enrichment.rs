fn main() {
    // Create a new client.
    let mut abstractapi = abstractapi::AbstractApi::default();
    abstractapi
        .set_api_key(
            abstractapi::api::ApiType::CompanyEnrichment,
            std::env::var("COMPANY_ENRICHMENT_API_KEY").unwrap(),
        )
        .unwrap();

    // Define a closure for printing the results in pretty format.
    let print_pretty = |company_details: abstractapi::api::company_enrichment::CompanyDetails| {
        println!(
            "{}, founded in {} ({}), has {} employees (LinkedIn: {})",
            company_details.name.unwrap(),
            company_details.year_founded.unwrap(),
            company_details.country.unwrap(),
            company_details.employees_count.unwrap(),
            company_details.linkedin_url.unwrap_or("?".to_string()),
        );
    };

    // Get details about google.com domain.
    let company_details = abstractapi
        .get_company_details(Some("google.com"), None)
        .unwrap();
    print_pretty(company_details);

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Get details about a facebook.com email.
    let company_details = abstractapi
        .get_company_details(None, Some("mark@facebook.com"))
        .unwrap();
    print_pretty(company_details);
}
