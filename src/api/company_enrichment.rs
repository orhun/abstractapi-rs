#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyDetails {
    pub name: Option<String>,
    pub domain: String,
    #[serde(rename = "year_founded")]
    pub year_founded: Option<i64>,
    pub industry: Option<String>,
    #[serde(rename = "employees_count")]
    pub employees_count: Option<i64>,
    pub locality: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "linkedin_url")]
    pub linkedin_url: Option<String>,
}
