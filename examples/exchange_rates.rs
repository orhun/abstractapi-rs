fn main() {
    // Create a new client.
    let mut abstractapi = abstractapi::AbstractApi::default();
    abstractapi
        .set_api_key(
            abstractapi::api::ApiType::ExchangeRates,
            std::env::var("EXCHANGE_RATES_API_KEY").unwrap(),
        )
        .unwrap();

    // Get the latest exchange rate for BTC and USD.
    let latest_rate = abstractapi
        .get_latest_exchange_rates("BTC", Some("USD"))
        .unwrap();

    // Print the exchange rate.
    println!(
        "1 {} = {} USD",
        latest_rate.base,
        latest_rate.exchange_rates.usd.unwrap()
    );

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Get the exchange rate in a date.
    let historical_rate = abstractapi
        .get_historical_exchange_rates("BTC", Some("USD"), "2021-01-31")
        .unwrap();

    // Print the exchange rate.
    println!(
        "1 {} was {} USD on {}",
        historical_rate.base,
        historical_rate.exchange_rates.usd.unwrap(),
        historical_rate.date.unwrap()
    );

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Get the converted amount of 10 USD in TRY, 10+ years ago.
    let converted_rate = abstractapi
        .convert_currency("USD", "TRY", Some("2010-01-31"), Some(10))
        .unwrap();

    // Print the exchange rate.
    println!(
        "{:?} {} was {:?} {} on {}",
        converted_rate.base_amount,
        converted_rate.base,
        converted_rate.converted_amount,
        converted_rate.target,
        converted_rate.date.unwrap(),
    );
}
