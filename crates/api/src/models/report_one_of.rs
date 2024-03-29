/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ReportOneOf : Report is waiting for triage / action



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReportOneOf {
    #[serde(rename = "status")]
    pub status: Status,
}

impl ReportOneOf {
    /// Report is waiting for triage / action
    pub fn new(status: Status) -> ReportOneOf {
        ReportOneOf {
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

