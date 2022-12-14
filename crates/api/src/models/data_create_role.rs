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
pub struct DataCreateRole {
    /// Role name
    #[serde(rename = "name")]
    pub name: String,
    /// Ranking position  Smaller values take priority.
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

impl DataCreateRole {
    pub fn new(name: String) -> DataCreateRole {
        DataCreateRole {
            name,
            rank: None,
        }
    }
}


