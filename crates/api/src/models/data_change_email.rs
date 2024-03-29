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
pub struct DataChangeEmail {
    /// Valid email address
    #[serde(rename = "email")]
    pub email: String,
    /// Current password
    #[serde(rename = "current_password")]
    pub current_password: String,
}

impl DataChangeEmail {
    pub fn new(email: String, current_password: String) -> DataChangeEmail {
        DataChangeEmail {
            email,
            current_password,
        }
    }
}


