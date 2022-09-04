/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataSendPasswordReset {
    /// Email associated with the account
    #[serde(rename = "email")]
    pub email: String,
    /// Captcha verification code
    #[serde(rename = "captcha", skip_serializing_if = "Option::is_none")]
    pub captcha: Option<String>,
}

impl DataSendPasswordReset {
    pub fn new(email: String) -> DataSendPasswordReset {
        DataSendPasswordReset {
            email,
            captcha: None,
        }
    }
}

