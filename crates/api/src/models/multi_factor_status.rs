/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MultiFactorStatus {
    #[serde(rename = "email_otp")]
    pub email_otp: bool,
    #[serde(rename = "trusted_handover")]
    pub trusted_handover: bool,
    #[serde(rename = "email_mfa")]
    pub email_mfa: bool,
    #[serde(rename = "totp_mfa")]
    pub totp_mfa: bool,
    #[serde(rename = "security_key_mfa")]
    pub security_key_mfa: bool,
    #[serde(rename = "recovery_active")]
    pub recovery_active: bool,
}

impl MultiFactorStatus {
    pub fn new(email_otp: bool, trusted_handover: bool, email_mfa: bool, totp_mfa: bool, security_key_mfa: bool, recovery_active: bool) -> MultiFactorStatus {
        MultiFactorStatus {
            email_otp,
            trusted_handover,
            email_mfa,
            totp_mfa,
            security_key_mfa,
            recovery_active,
        }
    }
}


