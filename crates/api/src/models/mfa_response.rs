/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// MfaResponse : MFA response



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MfaResponse {
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "recovery_code")]
    pub recovery_code: String,
    #[serde(rename = "totp_code")]
    pub totp_code: String,
}

impl MfaResponse {
    /// MFA response
    pub fn new(password: String, recovery_code: String, totp_code: String) -> MfaResponse {
        MfaResponse {
            password,
            recovery_code,
            totp_code,
        }
    }
}

