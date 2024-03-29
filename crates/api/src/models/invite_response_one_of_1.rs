/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// InviteResponseOneOf1 : Group channel invite



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteResponseOneOf1 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// Invite code
    #[serde(rename = "code")]
    pub code: String,
    /// Id of group channel
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// Name of group channel
    #[serde(rename = "channel_name")]
    pub channel_name: String,
    /// Description of group channel
    #[serde(rename = "channel_description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_description: Option<Option<String>>,
    /// Name of user who created the invite
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[serde(rename = "user_avatar", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_avatar: Option<Option<Box<crate::models::InviteResponseOneOfUserAvatar>>>,
}

impl InviteResponseOneOf1 {
    /// Group channel invite
    pub fn new(r#type: RHashType, code: String, channel_id: String, channel_name: String, user_name: String) -> InviteResponseOneOf1 {
        InviteResponseOneOf1 {
            r#type,
            code,
            channel_id,
            channel_name,
            channel_description: None,
            user_name,
            user_avatar: None,
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

