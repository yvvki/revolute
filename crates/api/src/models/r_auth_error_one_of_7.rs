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
pub struct RAuthErrorOneOf7 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl RAuthErrorOneOf7 {
    pub fn new(r#type: RHashType) -> RAuthErrorOneOf7 {
        RAuthErrorOneOf7 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "InvalidToken")]
    InvalidToken,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::InvalidToken
    }
}

