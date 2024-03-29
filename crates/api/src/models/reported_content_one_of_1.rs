/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ReportedContentOneOf1 : Report a server



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReportedContentOneOf1 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// ID of the server
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "report_reason")]
    pub report_reason: crate::models::ContentReportReason,
}

impl ReportedContentOneOf1 {
    /// Report a server
    pub fn new(r#type: RHashType, id: String, report_reason: crate::models::ContentReportReason) -> ReportedContentOneOf1 {
        ReportedContentOneOf1 {
            r#type,
            id,
            report_reason,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Server")]
    Server,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Server
    }
}

