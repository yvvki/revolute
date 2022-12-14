/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ChannelOneOf2 : Group channel between 1 or more participants



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelOneOf2 {
    #[serde(rename = "channel_type")]
    pub channel_type: ChannelType,
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Display name of the channel
    #[serde(rename = "name")]
    pub name: String,
    /// User id of the owner of the group
    #[serde(rename = "owner")]
    pub owner: String,
    /// Channel description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Array of user ids participating in channel
    #[serde(rename = "recipients")]
    pub recipients: Vec<String>,
    /// Custom icon attachment
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Box<crate::models::File>>,
    /// Id of the last message sent in this channel
    #[serde(rename = "last_message_id", skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,
    /// Permissions assigned to members of this group (does not apply to the owner of the group)
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<i64>,
    /// Whether this group is marked as not safe for work
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

impl ChannelOneOf2 {
    /// Group channel between 1 or more participants
    pub fn new(channel_type: ChannelType, _id: String, name: String, owner: String, recipients: Vec<String>) -> ChannelOneOf2 {
        ChannelOneOf2 {
            channel_type,
            _id,
            name,
            owner,
            description: None,
            recipients,
            icon: None,
            last_message_id: None,
            permissions: None,
            nsfw: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    #[serde(rename = "Group")]
    Group,
}

impl Default for ChannelType {
    fn default() -> ChannelType {
        Self::Group
    }
}

