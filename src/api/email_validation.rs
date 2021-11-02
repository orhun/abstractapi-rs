#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailResult {
    pub email: String,
    pub autocorrect: String,
    pub deliverability: String,
    #[serde(rename = "quality_score")]
    pub quality_score: String,
    #[serde(rename = "is_valid_format")]
    pub is_valid_format: IsValidFormat,
    #[serde(rename = "is_free_email")]
    pub is_free_email: IsFreeEmail,
    #[serde(rename = "is_disposable_email")]
    pub is_disposable_email: IsDisposableEmail,
    #[serde(rename = "is_role_email")]
    pub is_role_email: IsRoleEmail,
    #[serde(rename = "is_catchall_email")]
    pub is_catchall_email: IsCatchallEmail,
    #[serde(rename = "is_mx_found")]
    pub is_mx_found: IsMxFound,
    #[serde(rename = "is_smtp_valid")]
    pub is_smtp_valid: IsSmtpValid,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsValidFormat {
    pub value: bool,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsFreeEmail {
    pub value: bool,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsDisposableEmail {
    pub value: bool,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsRoleEmail {
    pub value: bool,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsCatchallEmail {
    pub value: bool,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsMxFound {
    pub value: bool,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsSmtpValid {
    pub value: bool,
    pub text: String,
}
