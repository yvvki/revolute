/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// FieldsBot : Optional fields on bot object

/// Optional fields on bot object
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FieldsBot {
    #[serde(rename = "Token")]
    Token,
    #[serde(rename = "InteractionsURL")]
    InteractionsUrl,

}

impl ToString for FieldsBot {
    fn to_string(&self) -> String {
        match self {
            Self::Token => String::from("Token"),
            Self::InteractionsUrl => String::from("InteractionsURL"),
        }
    }
}

impl Default for FieldsBot {
    fn default() -> FieldsBot {
        Self::Token
    }
}




