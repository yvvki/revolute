/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// UserProfile : User's profile page



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserProfile {
    /// Text content on user's profile
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "background", skip_serializing_if = "Option::is_none")]
    pub background: Option<Box<crate::models::UserProfileBackground>>,
}

impl UserProfile {
    /// User's profile page
    pub fn new() -> UserProfile {
        UserProfile {
            content: None,
            background: None,
        }
    }
}


