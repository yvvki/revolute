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
    pub _type: Type,
}

impl ErrorOneOf14 {
    pub fn new(_type: Type) -> ErrorOneOf14 {
        ErrorOneOf14 {
            _type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "CannotJoinCall")]
    CannotJoinCall,
}

impl Default for Type {
    fn default() -> Type {
        Self::CannotJoinCall
    }
}
