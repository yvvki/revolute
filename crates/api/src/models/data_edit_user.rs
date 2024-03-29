/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataEditUser {
    /// New display name
    #[serde(rename = "display_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<Option<String>>,
    /// Attachment Id for avatar
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<Box<crate::models::DataEditUserStatus>>>,
    #[serde(rename = "profile", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Option<Box<crate::models::DataEditUserProfile>>>,
    /// Bitfield of user badges
    #[serde(rename = "badges", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub badges: Option<Option<i32>>,
    /// Enum of user flags
    #[serde(rename = "flags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Option<i32>>,
    /// Fields to remove from user object
    #[serde(rename = "remove", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remove: Option<Option<Vec<crate::models::FieldsUser>>>,
}

impl DataEditUser {
    pub fn new() -> DataEditUser {
        DataEditUser {
            display_name: None,
            avatar: None,
            status: None,
            profile: None,
            badges: None,
            flags: None,
            remove: None,
        }
    }
}


