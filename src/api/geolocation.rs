#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geolocation {
    #[serde(rename = "ip_address")]
    pub ip_address: String,
    pub city: Option<String>,
    #[serde(rename = "city_geoname_id")]
    pub city_geoname_id: Option<i64>,
    pub region: Option<String>,
    #[serde(rename = "region_iso_code")]
    pub region_iso_code: Option<String>,
    #[serde(rename = "region_geoname_id")]
    pub region_geoname_id: Option<i64>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    pub country: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    #[serde(rename = "country_geoname_id")]
    pub country_geoname_id: i64,
    #[serde(rename = "country_is_eu")]
    pub country_is_eu: bool,
    pub continent: String,
    #[serde(rename = "continent_code")]
    pub continent_code: String,
    #[serde(rename = "continent_geoname_id")]
    pub continent_geoname_id: i64,
    pub longitude: f64,
    pub latitude: f64,
    pub security: Security,
    pub timezone: Timezone,
    pub flag: Option<Flag>,
    pub currency: Currency,
    pub connection: Connection,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Security {
    #[serde(rename = "is_vpn")]
    pub is_vpn: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timezone {
    pub name: String,
    pub abbreviation: String,
    #[serde(rename = "gmt_offset")]
    pub gmt_offset: i64,
    #[serde(rename = "current_time")]
    pub current_time: String,
    #[serde(rename = "is_dst")]
    pub is_dst: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    pub emoji: String,
    pub unicode: String,
    pub png: String,
    pub svg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    #[serde(rename = "currency_name")]
    pub currency_name: String,
    #[serde(rename = "currency_code")]
    pub currency_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    #[serde(rename = "autonomous_system_number")]
    pub autonomous_system_number: i64,
    #[serde(rename = "autonomous_system_organization")]
    pub autonomous_system_organization: String,
    #[serde(rename = "connection_type")]
    pub connection_type: String,
    #[serde(rename = "isp_name")]
    pub isp_name: String,
    #[serde(rename = "organization_name")]
    pub organization_name: Option<String>,
}
