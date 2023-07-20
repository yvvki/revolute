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
pub struct DataReportContent {
    #[serde(rename = "content")]
    pub content: Box<crate::models::DataReportContentContent>,
    /// Additional report description
    #[serde(rename = "additional_context", skip_serializing_if = "Option::is_none")]
    pub additional_context: Option<String>,
}

impl DataReportContent {
    pub fn new(content: crate::models::DataReportContentContent) -> DataReportContent {
        DataReportContent {
            content: Box::new(content),
            additional_context: None,
        }
    }
}

