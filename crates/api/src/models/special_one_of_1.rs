/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// SpecialOneOf1 : Content hint that this contains a GIF  Use metadata to find video or image to play



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpecialOneOf1 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl SpecialOneOf1 {
    /// Content hint that this contains a GIF  Use metadata to find video or image to play
    pub fn new(r#type: RHashType) -> SpecialOneOf1 {
        SpecialOneOf1 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "GIF")]
    Gif,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Gif
    }
}

