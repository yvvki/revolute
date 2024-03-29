/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// InviteBotDestinationAnyOf : Invite to a server



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteBotDestinationAnyOf {
    /// Server Id
    #[serde(rename = "server")]
    pub server: String,
}

impl InviteBotDestinationAnyOf {
    /// Invite to a server
    pub fn new(server: String) -> InviteBotDestinationAnyOf {
        InviteBotDestinationAnyOf {
            server,
        }
    }
}


