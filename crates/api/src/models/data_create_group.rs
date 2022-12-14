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
pub struct DataCreateGroup {
    /// Group name
    #[serde(rename = "name")]
    pub name: String,
    /// Group description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Array of user IDs to add to the group  Must be friends with these users.
    #[serde(rename = "users")]
    pub users: Vec<String>,
    /// Whether this group is age-restricted
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

impl DataCreateGroup {
    pub fn new(name: String, users: Vec<String>) -> DataCreateGroup {
        DataCreateGroup {
            name,
            description: None,
            users,
            nsfw: None,
        }
    }
}


