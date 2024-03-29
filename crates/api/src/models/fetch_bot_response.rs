/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// FetchBotResponse : Bot Response



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FetchBotResponse {
    #[serde(rename = "bot")]
    pub bot: Box<crate::models::FetchBotResponseBot>,
    #[serde(rename = "user")]
    pub user: Box<crate::models::FetchBotResponseUser>,
}

impl FetchBotResponse {
    /// Bot Response
    pub fn new(bot: crate::models::FetchBotResponseBot, user: crate::models::FetchBotResponseUser) -> FetchBotResponse {
        FetchBotResponse {
            bot: Box::new(bot),
            user: Box::new(user),
        }
    }
}


