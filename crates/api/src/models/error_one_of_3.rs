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
pub struct ErrorOneOf3 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "max")]
    pub max: i32,
}

impl ErrorOneOf3 {
    pub fn new(r#type: RHashType, max: i32) -> ErrorOneOf3 {
        ErrorOneOf3 {
            r#type,
            max,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "TooManyEmbeds")]
    TooManyEmbeds,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::TooManyEmbeds
    }
}

