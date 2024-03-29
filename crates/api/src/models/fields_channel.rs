/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// FieldsChannel : Optional fields on channel object

/// Optional fields on channel object
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FieldsChannel {
    #[serde(rename = "Description")]
    Description,
    #[serde(rename = "Icon")]
    Icon,
    #[serde(rename = "DefaultPermissions")]
    DefaultPermissions,

}

impl ToString for FieldsChannel {
    fn to_string(&self) -> String {
        match self {
            Self::Description => String::from("Description"),
            Self::Icon => String::from("Icon"),
            Self::DefaultPermissions => String::from("DefaultPermissions"),
        }
    }
}

impl Default for FieldsChannel {
    fn default() -> FieldsChannel {
        Self::Description
    }
}




