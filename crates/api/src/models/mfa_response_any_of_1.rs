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
pub struct MfaResponseAnyOf1 {
    #[serde(rename = "recovery_code")]
    pub recovery_code: String,
}

impl MfaResponseAnyOf1 {
    pub fn new(recovery_code: String) -> MfaResponseAnyOf1 {
        MfaResponseAnyOf1 {
            recovery_code,
        }
    }
}


