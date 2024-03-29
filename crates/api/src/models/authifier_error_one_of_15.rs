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
pub struct AuthifierErrorOneOf15 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl AuthifierErrorOneOf15 {
    pub fn new(r#type: RHashType) -> AuthifierErrorOneOf15 {
        AuthifierErrorOneOf15 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "TotpAlreadyEnabled")]
    TotpAlreadyEnabled,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::TotpAlreadyEnabled
    }
}

