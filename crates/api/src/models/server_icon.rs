/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ServerIcon : Icon attachment



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerIcon {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Tag / bucket this file was uploaded to
    #[serde(rename = "tag")]
    pub tag: String,
    /// Original filename
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::FileMetadata>,
    /// Raw content type of this file
    #[serde(rename = "content_type")]
    pub content_type: String,
    /// Size of this file (in bytes)
    #[serde(rename = "size")]
    pub size: i32,
    /// Whether this file was deleted
    #[serde(rename = "deleted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Option<bool>>,
    /// Whether this file was reported
    #[serde(rename = "reported", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reported: Option<Option<bool>>,
    #[serde(rename = "message_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Option<String>>,
    #[serde(rename = "user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Option<String>>,
    #[serde(rename = "server_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<Option<String>>,
    /// Id of the object this file is associated with
    #[serde(rename = "object_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<Option<String>>,
}

impl ServerIcon {
    /// Icon attachment
    pub fn new(_id: String, tag: String, filename: String, metadata: crate::models::FileMetadata, content_type: String, size: i32) -> ServerIcon {
        ServerIcon {
            _id,
            tag,
            filename,
            metadata: Box::new(metadata),
            content_type,
            size,
            deleted: None,
            reported: None,
            message_id: None,
            user_id: None,
            server_id: None,
            object_id: None,
        }
    }
}


