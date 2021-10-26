#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

pub type Holidays = Vec<Holiday>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Holiday {
    pub name: String,
    #[serde(rename = "name_local")]
    pub name_local: String,
    pub language: String,
    pub description: String,
    pub country: String,
    pub location: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub date: String,
    #[serde(rename = "date_year")]
    pub date_year: String,
    #[serde(rename = "date_month")]
    pub date_month: String,
    #[serde(rename = "date_day")]
    pub date_day: String,
    #[serde(rename = "week_day")]
    pub week_day: String,
}
