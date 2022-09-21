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
pub struct ErrorOneOf14 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl ErrorOneOf14 {
    pub fn new(r#type: RHashType) -> ErrorOneOf14 {
        ErrorOneOf14 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "CannotJoinCall")]
    CannotJoinCall,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::CannotJoinCall
    }
}

