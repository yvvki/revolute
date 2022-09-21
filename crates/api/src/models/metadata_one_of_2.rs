/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// MetadataOneOf2 : File is an image with specific dimensions



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetadataOneOf2 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "width")]
    pub width: i32,
    #[serde(rename = "height")]
    pub height: i32,
}

impl MetadataOneOf2 {
    /// File is an image with specific dimensions
    pub fn new(r#type: RHashType, width: i32, height: i32) -> MetadataOneOf2 {
        MetadataOneOf2 {
            r#type,
            width,
            height,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Image")]
    Image,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Image
    }
}

