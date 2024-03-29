/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// SpecialOneOf3 : Lightspeed.tv stream



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpecialOneOf3 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "content_type")]
    pub content_type: crate::models::LightspeedType,
    #[serde(rename = "id")]
    pub id: String,
}

impl SpecialOneOf3 {
    /// Lightspeed.tv stream
    pub fn new(r#type: RHashType, content_type: crate::models::LightspeedType, id: String) -> SpecialOneOf3 {
        SpecialOneOf3 {
            r#type,
            content_type,
            id,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Lightspeed")]
    Lightspeed,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Lightspeed
    }
}

