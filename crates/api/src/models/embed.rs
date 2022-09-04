/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Embed : Embed



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Embed {
    #[serde(rename = "type")]
    pub _type: Type,
    /// URL for title
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Original direct URL
    #[serde(rename = "original_url", skip_serializing_if = "Option::is_none")]
    pub original_url: Option<String>,
    /// Remote content
    #[serde(rename = "special", skip_serializing_if = "Option::is_none")]
    pub special: Option<Box<crate::models::Special>>,
    /// Title of text embed
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Description of text embed
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Embedded image
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<crate::models::Image>>,
    /// Embedded video
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<crate::models::Video>>,
    /// Site name
    #[serde(rename = "site_name", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// URL to icon
    #[serde(rename = "icon_url", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// CSS Colour
    #[serde(rename = "colour", skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
    /// Width of the video
    #[serde(rename = "width")]
    pub width: i32,
    /// Height of the video
    #[serde(rename = "height")]
    pub height: i32,
    /// Positioning and size
    #[serde(rename = "size")]
    pub size: Option<Box<crate::models::ImageSize>>,
    /// ID of uploaded autumn file
    #[serde(rename = "media", skip_serializing_if = "Option::is_none")]
    pub media: Option<Box<crate::models::File>>,
}

impl Embed {
    /// Embed
    pub fn new(_type: Type, url: Option<String>, width: i32, height: i32, size: Option<crate::models::ImageSize>) -> Embed {
        Embed {
            _type,
            url,
            original_url: None,
            special: None,
            title: None,
            description: None,
            image: None,
            video: None,
            site_name: None,
            icon_url: None,
            colour: None,
            width,
            height,
            size: super::box_option(size),
            media: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "None")]
    None,
}

impl Default for Type {
    fn default() -> Type {
        Self::None
    }
}

