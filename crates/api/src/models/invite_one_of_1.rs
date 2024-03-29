/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// InviteOneOf1 : Invite to a group channel



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteOneOf1 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// Invite code
    #[serde(rename = "_id")]
    pub _id: String,
    /// Id of user who created this invite
    #[serde(rename = "creator")]
    pub creator: String,
    /// Id of the group channel this invite points to
    #[serde(rename = "channel")]
    pub channel: String,
}

impl InviteOneOf1 {
    /// Invite to a group channel
    pub fn new(r#type: RHashType, _id: String, creator: String, channel: String) -> InviteOneOf1 {
        InviteOneOf1 {
            r#type,
            _id,
            creator,
            channel,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Group")]
    Group,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Group
    }
}

