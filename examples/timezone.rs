fn main() {
    // Create a new client.
    let mut abstractapi = abstractapi::AbstractApi::default();
    abstractapi
        .set_api_key(
            abstractapi::ApiType::Timezone,
            std::env::var("TIMEZONE_API_KEY").unwrap(),
        )
        .unwrap();

    // Define a closure for printing the results in pretty format.
    let print_pretty = |description: &str, time: abstractapi::api::timezone::LocationTime| {
        println!(
            "{}: {}, Timezone: {}, Location: {} (long: {} lat: {})",
            description,
            time.datetime,
            time.timezone_name,
            time.requested_location,
            time.longitude,
            time.latitude,
        );
    };

    // Get the current time in Ankara, Turkey.
    let current_time = abstractapi.get_current_time("Ankara").unwrap();
    print_pretty("Current time", current_time);

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Convert time using base and target locations.
    let converted_time = abstractapi
        .convert_time(
            "Los Angeles,CA",
            "2020-05-01 07:00:00",
            "Oxford,United Kingdom",
        )
        .unwrap();
    print_pretty("Base location time", converted_time.base_location);
    print_pretty("Target location time", converted_time.target_location);
}
