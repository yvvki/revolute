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
pub struct ErrorOneOf24 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl ErrorOneOf24 {
    pub fn new(r#type: RHashType) -> ErrorOneOf24 {
        ErrorOneOf24 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "InvalidRole")]
    InvalidRole,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::InvalidRole
    }
}

