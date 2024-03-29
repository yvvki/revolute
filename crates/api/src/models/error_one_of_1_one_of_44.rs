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
pub struct ErrorOneOf1OneOf44 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl ErrorOneOf1OneOf44 {
    pub fn new(r#type: RHashType) -> ErrorOneOf1OneOf44 {
        ErrorOneOf1OneOf44 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "InvalidSession")]
    InvalidSession,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::InvalidSession
    }
}

