/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// FieldsRole : Optional fields on server object

/// Optional fields on server object
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FieldsRole {
    #[serde(rename = "Colour")]
    Colour,

}

impl ToString for FieldsRole {
    fn to_string(&self) -> String {
        match self {
            Self::Colour => String::from("Colour"),
        }
    }
}

impl Default for FieldsRole {
    fn default() -> FieldsRole {
        Self::Colour
    }
}




