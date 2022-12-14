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
pub struct DataCreateChannel {
    /// Channel type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<crate::models::ChannelType>>,
    /// Channel name
    #[serde(rename = "name")]
    pub name: String,
    /// Channel description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this channel is age restricted
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

impl DataCreateChannel {
    pub fn new(name: String) -> DataCreateChannel {
        DataCreateChannel {
            r#type: None,
            name,
            description: None,
            nsfw: None,
        }
    }
}


