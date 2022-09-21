/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// MetadataOneOf : File is just a generic uncategorised file



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetadataOneOf {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl MetadataOneOf {
    /// File is just a generic uncategorised file
    pub fn new(r#type: RHashType) -> MetadataOneOf {
        MetadataOneOf {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "File")]
    File,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::File
    }
}

