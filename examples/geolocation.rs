fn main() {
    // Create a new client.
    let mut abstractapi = abstractapi::AbstractApi::default();
    abstractapi
        .set_api_key(
            abstractapi::api::ApiType::Geolocation,
            std::env::var("GEOLOCATION_API_KEY").unwrap(),
        )
        .unwrap();

    // Get geolocation from IP address.
    let geolocation = abstractapi.get_geolocation("172.217.19.142").unwrap();

    // Print the result in a pretty format.
    println!(
        "Location: {:?}/{:?}, Timezone: {:?}, ISP: {:?}",
        geolocation.city,
        geolocation.country,
        geolocation.timezone.name,
        geolocation.connection.isp_name
    );
}
