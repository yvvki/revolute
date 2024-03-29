/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ReportStatusOneOf : Report is waiting for triage / action



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReportStatusOneOf {
    #[serde(rename = "status")]
    pub status: Status,
}

impl ReportStatusOneOf {
    /// Report is waiting for triage / action
    pub fn new(status: Status) -> ReportStatusOneOf {
        ReportStatusOneOf {
            status,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Created")]
    Created,
}

impl Default for Status {
    fn default() -> Status {
        Self::Created
    }
}

