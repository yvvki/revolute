/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// InviteBotDestinationAnyOf1 : Invite to a group



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteBotDestinationAnyOf1 {
    /// Group Id
    #[serde(rename = "group")]
    pub group: String,
}

impl InviteBotDestinationAnyOf1 {
    /// Invite to a group
    pub fn new(group: String) -> InviteBotDestinationAnyOf1 {
        InviteBotDestinationAnyOf1 {
            group,
        }
    }
}


