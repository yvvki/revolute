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
pub struct DataCreateEmoji {
    /// Server name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parent")]
    pub parent: Box<crate::models::DataCreateEmojiParent>,
    /// Whether the emoji is mature
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

impl DataCreateEmoji {
    pub fn new(name: String, parent: crate::models::DataCreateEmojiParent) -> DataCreateEmoji {
        DataCreateEmoji {
            name,
            parent: Box::new(parent),
            nsfw: None,
        }
    }
}


