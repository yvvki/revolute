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
pub struct ErrorOneOf38 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl ErrorOneOf38 {
    pub fn new(r#type: RHashType) -> ErrorOneOf38 {
        ErrorOneOf38 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "InvalidOperation")]
    InvalidOperation,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::InvalidOperation
    }
}

