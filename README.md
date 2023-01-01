# abstractapi-rs

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/orhun/abstractapi-rs/ci.yml)](https://github.com/orhun/abstractapi-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/abstractapi)](https://crates.io/crates/abstractapi)
[![docs.rs](https://img.shields.io/docsrs/abstractapi)](https://docs.rs/abstractapi/latest)
[![Codecov](https://img.shields.io/codecov/c/gh/orhun/abstractapi-rs)](https://app.codecov.io/gh/orhun/abstractapi-rs)

Rust API bindings for the [**Abstract**](https://www.abstractapi.com/) HTTP API.

## APIs

`abstractapi-rs` is compatible with `v1` versions of the following API's that Abstract provides:

- [x] Verify
  - [x] [Email validation](https://app.abstractapi.com/api/email-validation)
  - [x] [Phone validation](https://app.abstractapi.com/api/phone-validation)
  - [x] [VAT](https://app.abstractapi.com/api/vat)
- [x] Lookup
  - [x] [IP geolocation](https://app.abstractapi.com/api/ip-geolocation)
  - [x] [Holidays](https://app.abstractapi.com/api/holidays)
  - [x] [Exchange rates](https://app.abstractapi.com/api/exchange-rates)
  - [x] [Company enrichment](https://app.abstractapi.com/api/company-enrichment)
  - [x] [Timezone](https://app.abstractapi.com/api/timezone)
- [ ] Create
  - [ ] [Avatars](https://app.abstractapi.com/api/avatars)
  - [ ] [Screenshot](https://app.abstractapi.com/api/screenshot)
  - [ ] [Scrape](https://app.abstractapi.com/api/scrape)
  - [ ] [Images](https://app.abstractapi.com/api/images)

## Usage

Add `abstractapi` to dependencies in your `Cargo.toml`:

```toml
[dependencies]
abstractapi = "0.1.*"
```

## Getting Started

In order to interact with the APIs, you need to create a client ([`AbstractApi`](https://docs.rs/abstractapi/latest/abstractapi/struct.AbstractApi.html)) first:

```rs
let mut abstractapi = abstractapi::AbstractApi::default();
```

Then you should set an API key specific for the API you would like to use. Here is an example for [Geolocation API](https://app.abstractapi.com/api/ip-geolocation):

```rs
abstractapi.set_api_key(abstractapi::ApiType::Geolocation, "<api_key>").unwrap();
```

See [`ApiType`](https://docs.rs/abstractapi/latest/abstractapi/enum.ApiType.html) enum for currently supported APIs.

The next step would be calling the function related to the API you want to use:

```rs
let geolocation: abstractapi::api::Geolocation = abstractapi.get_geolocation("172.217.19.142").unwrap();
```

Function parameters and return values (`Struct`s) are directly mapped from the [official API documentation](#apis) so you may frequently need to refer to it for the meaning of these fields.

#### Tips

- You can use the [`prelude`](https://docs.rs/abstractapi/latest/abstractapi/prelude/index.html) module for glob-importing the common types.
- There are alternative constructor methods available for creating a client with API keys. (e.g. [`new_with_api_keys`](https://docs.rs/abstractapi/latest/abstractapi/struct.AbstractApi.html#method.new_with_api_keys))

Here is a full example that shows the basic usage of phone validation API:

```rs
use abstractapi::prelude::*;

fn main() -> Result<(), AbstractApiError> {
    // Create a new Abstract API client for phone validation.
    let abstractapi = AbstractApi::new_with_api_key(
        ApiType::PhoneValidation,
        std::env::var("PHONE_VALIDATION_API_KEY").unwrap(),
    )?;

    // Get the phone number details.
    let phone_details: PhoneDetails = abstractapi.validate_phone("14152007986")?;

    // Print the result.
    println!("{:#?}", phone_details);

    Ok(())
}
```

## Examples

Look through the [examples folder](./examples/) to see how the library can be used for integrating different [APIs](#apis).

## Contributing

Pull requests are welcome!

## License

All code is dual-licensed under [The MIT License](./LICENSE-MIT) and [Apache 2.0 License](./LICENSE-APACHE).
