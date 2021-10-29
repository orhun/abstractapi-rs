#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationTime {
    pub datetime: String,
    #[serde(rename = "timezone_name")]
    pub timezone_name: String,
    #[serde(rename = "timezone_location")]
    pub timezone_location: String,
    #[serde(rename = "timezone_abbreviation")]
    pub timezone_abbreviation: String,
    #[serde(rename = "gmt_offset")]
    pub gmt_offset: f64,
    #[serde(rename = "is_dst")]
    pub is_dst: bool,
    #[serde(rename = "requested_location")]
    pub requested_location: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertedTime {
    #[serde(rename = "base_location")]
    pub base_location: LocationTime,
    #[serde(rename = "target_location")]
    pub target_location: LocationTime,
}
