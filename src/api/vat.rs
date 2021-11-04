#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VatDetails {
    #[serde(rename = "vat_number")]
    pub vat_number: String,
    pub valid: bool,
    pub company: Company,
    pub country: Country,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub name: String,
    pub address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vat {
    #[serde(rename = "amount_excluding_vat")]
    pub amount_excluding_vat: String,
    #[serde(rename = "amount_including_vat")]
    pub amount_including_vat: String,
    #[serde(rename = "vat_amount")]
    pub vat_amount: String,
    #[serde(rename = "vat_category")]
    pub vat_category: String,
    #[serde(rename = "vat_rate")]
    pub vat_rate: String,
    pub country: Country,
}

pub type VatRates = Vec<VatRate>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VatRate {
    #[serde(rename = "country_code")]
    pub country_code: String,
    pub rate: String,
    pub category: String,
    pub description: String,
}
