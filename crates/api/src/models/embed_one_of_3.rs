/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// EmbedOneOf3 : Text Embed



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EmbedOneOf3 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// URL to icon
    #[serde(rename = "icon_url", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// URL for title
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Title of text embed
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Description of text embed
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID of uploaded autumn file
    #[serde(rename = "media", skip_serializing_if = "Option::is_none")]
    pub media: Option<Box<crate::models::File>>,
    /// CSS Colour
    #[serde(rename = "colour", skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
}

impl EmbedOneOf3 {
    /// Text Embed
    pub fn new(r#type: RHashType) -> EmbedOneOf3 {
        EmbedOneOf3 {
            r#type,
            icon_url: None,
            url: None,
            title: None,
            description: None,
            media: None,
            colour: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Text")]
    Text,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Text
    }
}

