/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// SystemMessage : Representation of a system event message



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SystemMessage {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "by")]
    pub by: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
}

impl SystemMessage {
    /// Representation of a system event message
    pub fn new(r#type: RHashType, content: String, id: String, by: String, name: String, from: String, to: String) -> SystemMessage {
        SystemMessage {
            r#type,
            content,
            id,
            by,
            name,
            from,
            to,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "channel_ownership_changed")]
    ChannelOwnershipChanged,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::ChannelOwnershipChanged
    }
}

