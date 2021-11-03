#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneResult {
    pub phone: String,
    pub valid: bool,
    pub format: Format,
    pub country: Country,
    pub location: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub carrier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub international: String,
    pub local: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub code: String,
    pub name: String,
    pub prefix: String,
}
