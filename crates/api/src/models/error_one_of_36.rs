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
pub struct ErrorOneOf36 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "with")]
    pub with: String,
}

impl ErrorOneOf36 {
    pub fn new(r#type: RHashType, operation: String, with: String) -> ErrorOneOf36 {
        ErrorOneOf36 {
            r#type,
            operation,
            with,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "DatabaseError")]
    DatabaseError,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::DatabaseError
    }
}

