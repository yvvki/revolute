/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// BotInformation : Bot information for if the user is a bot



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotInformation {
    /// Id of the owner of this bot
    #[serde(rename = "owner")]
    pub owner: String,
}

impl BotInformation {
    /// Bot information for if the user is a bot
    pub fn new(owner: String) -> BotInformation {
        BotInformation {
            owner,
        }
    }
}

