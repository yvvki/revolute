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
pub struct DataEditBot {
    /// Bot username
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Whether the bot can be added by anyone
    #[serde(rename = "public", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub public: Option<Option<bool>>,
    /// Whether analytics should be gathered for this bot  Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    #[serde(rename = "analytics", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub analytics: Option<Option<bool>>,
    /// Interactions URL
    #[serde(rename = "interactions_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub interactions_url: Option<Option<String>>,
    /// Fields to remove from bot object
    #[serde(rename = "remove", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remove: Option<Option<Vec<crate::models::FieldsBot>>>,
}

impl DataEditBot {
    pub fn new() -> DataEditBot {
        DataEditBot {
            name: None,
            public: None,
            analytics: None,
            interactions_url: None,
            remove: None,
        }
    }
}


