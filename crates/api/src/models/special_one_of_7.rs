/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// SpecialOneOf7 : Bandcamp track



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpecialOneOf7 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "content_type")]
    pub content_type: crate::models::BandcampType,
    #[serde(rename = "id")]
    pub id: String,
}

impl SpecialOneOf7 {
    /// Bandcamp track
    pub fn new(r#type: RHashType, content_type: crate::models::BandcampType, id: String) -> SpecialOneOf7 {
        SpecialOneOf7 {
            r#type,
            content_type,
            id,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Bandcamp")]
    Bandcamp,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Bandcamp
    }
}

