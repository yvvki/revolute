/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// MetadataOneOf4 : File is audio



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetadataOneOf4 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl MetadataOneOf4 {
    /// File is audio
    pub fn new(r#type: RHashType) -> MetadataOneOf4 {
        MetadataOneOf4 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Audio")]
    Audio,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Audio
    }
}

