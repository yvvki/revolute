/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Masquerade : Name and / or avatar override information



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Masquerade {
    /// Replace the display name shown on this message
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Replace the avatar shown on this message (URL to image file)
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Replace the display role colour shown on this message  Must have `ManageRole` permission to use
    #[serde(rename = "colour", skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
}

impl Masquerade {
    /// Name and / or avatar override information
    pub fn new() -> Masquerade {
        Masquerade {
            name: None,
            avatar: None,
            colour: None,
        }
    }
}


