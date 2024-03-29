/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// FieldsServer : Optional fields on server object

/// Optional fields on server object
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FieldsServer {
    #[serde(rename = "Description")]
    Description,
    #[serde(rename = "Categories")]
    Categories,
    #[serde(rename = "SystemMessages")]
    SystemMessages,
    #[serde(rename = "Icon")]
    Icon,
    #[serde(rename = "Banner")]
    Banner,

}

impl ToString for FieldsServer {
    fn to_string(&self) -> String {
        match self {
            Self::Description => String::from("Description"),
            Self::Categories => String::from("Categories"),
            Self::SystemMessages => String::from("SystemMessages"),
            Self::Icon => String::from("Icon"),
            Self::Banner => String::from("Banner"),
        }
    }
}

impl Default for FieldsServer {
    fn default() -> FieldsServer {
        Self::Description
    }
}




