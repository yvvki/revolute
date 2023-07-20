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
pub struct MemberQueryResponse {
    /// List of members
    #[serde(rename = "members")]
    pub members: Vec<crate::models::Member>,
    /// List of users
    #[serde(rename = "users")]
    pub users: Vec<crate::models::User>,
}

impl MemberQueryResponse {
    pub fn new(members: Vec<crate::models::Member>, users: Vec<crate::models::User>) -> MemberQueryResponse {
        MemberQueryResponse {
            members,
            users,
        }
    }
}

