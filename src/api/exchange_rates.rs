#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRatesResult {
    pub base: String,
    pub date: Option<String>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    #[serde(rename = "exchange_rates")]
    pub exchange_rates: ExchangeRates,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRates {
    #[serde(rename = "USD")]
    pub usd: Option<f64>,
    #[serde(rename = "EUR")]
    pub eur: Option<f64>,
    #[serde(rename = "JPY")]
    pub jpy: Option<f64>,
    #[serde(rename = "BGN")]
    pub bgn: Option<f64>,
    #[serde(rename = "CZK")]
    pub czk: Option<f64>,
    #[serde(rename = "DKK")]
    pub dkk: Option<f64>,
    #[serde(rename = "GBP")]
    pub gbp: Option<f64>,
    #[serde(rename = "HUF")]
    pub huf: Option<f64>,
    #[serde(rename = "PLN")]
    pub pln: Option<f64>,
    #[serde(rename = "RON")]
    pub ron: Option<f64>,
    #[serde(rename = "SEK")]
    pub sek: Option<f64>,
    #[serde(rename = "CHF")]
    pub chf: Option<f64>,
    #[serde(rename = "ISK")]
    pub isk: Option<f64>,
    #[serde(rename = "NOK")]
    pub nok: Option<f64>,
    #[serde(rename = "HRK")]
    pub hrk: Option<f64>,
    #[serde(rename = "RUB")]
    pub rub: Option<f64>,
    #[serde(rename = "TRY")]
    pub try_field: Option<f64>,
    #[serde(rename = "AUD")]
    pub aud: Option<f64>,
    #[serde(rename = "BRL")]
    pub brl: Option<f64>,
    #[serde(rename = "CAD")]
    pub cad: Option<f64>,
    #[serde(rename = "CNY")]
    pub cny: Option<f64>,
    #[serde(rename = "HKD")]
    pub hkd: Option<f64>,
    #[serde(rename = "IDR")]
    pub idr: Option<f64>,
    #[serde(rename = "ILS")]
    pub ils: Option<f64>,
    #[serde(rename = "INR")]
    pub inr: Option<f64>,
    #[serde(rename = "KRW")]
    pub krw: Option<f64>,
    #[serde(rename = "MXN")]
    pub mxn: Option<f64>,
    #[serde(rename = "MYR")]
    pub myr: Option<f64>,
    #[serde(rename = "NZD")]
    pub nzd: Option<f64>,
    #[serde(rename = "PHP")]
    pub php: Option<f64>,
    #[serde(rename = "SGD")]
    pub sgd: Option<f64>,
    #[serde(rename = "THB")]
    pub thb: Option<f64>,
    #[serde(rename = "ZAR")]
    pub zar: Option<f64>,
    #[serde(rename = "ARS")]
    pub ars: Option<f64>,
    #[serde(rename = "DZD")]
    pub dzd: Option<f64>,
    #[serde(rename = "MAD")]
    pub mad: Option<f64>,
    #[serde(rename = "TWD")]
    pub twd: Option<f64>,
    #[serde(rename = "BTC")]
    pub btc: Option<f64>,
    #[serde(rename = "ETH")]
    pub eth: Option<f64>,
    #[serde(rename = "BNB")]
    pub bnb: Option<f64>,
    #[serde(rename = "DOGE")]
    pub doge: Option<f64>,
    #[serde(rename = "XRP")]
    pub xrp: Option<f64>,
    #[serde(rename = "BCH")]
    pub bch: Option<f64>,
    #[serde(rename = "LTC")]
    pub ltc: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertedExchangeRate {
    pub base: String,
    pub target: String,
    #[serde(rename = "base_amount")]
    pub base_amount: i64,
    #[serde(rename = "converted_amount")]
    pub converted_amount: f64,
    #[serde(rename = "exchange_rate")]
    pub exchange_rate: f64,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    pub date: Option<String>,
}
