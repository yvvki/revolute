/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataEditMessage {
    /// New message content
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Embeds to include in the message
    #[serde(rename = "embeds", skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<crate::models::SendableEmbed>>,
}

impl DataEditMessage {
    pub fn new() -> DataEditMessage {
        DataEditMessage {
            content: None,
            embeds: None,
        }
    }
}


