fn main() {
    // Create a new client.
    let mut abstractapi = abstractapi::AbstractApi::default();
    abstractapi
        .set_api_key(
            abstractapi::ApiType::Holidays,
            std::env::var("HOLIDAYS_API_KEY").unwrap(),
        )
        .unwrap();

    // Get holiday information on 29-10-2021 in Turkey.
    let holidays = abstractapi.get_holidays("TR", "2021", "10", "29").unwrap();

    // Print the results in a pretty format
    for holiday in holidays {
        println!(
            "{:?} - Holiday: {:?}, Type: {:?}, Location: {:?}",
            holiday.date, holiday.name, holiday.type_, holiday.location
        );
    }
}
