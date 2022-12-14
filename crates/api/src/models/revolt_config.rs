/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RevoltConfig {
    /// Revolt API Version
    #[serde(rename = "revolt")]
    pub revolt: String,
    #[serde(rename = "features")]
    pub features: Box<crate::models::RevoltConfigFeatures>,
    /// WebSocket URL
    #[serde(rename = "ws")]
    pub ws: String,
    /// URL pointing to the client serving this node
    #[serde(rename = "app")]
    pub app: String,
    /// Web Push VAPID public key
    #[serde(rename = "vapid")]
    pub vapid: String,
}

impl RevoltConfig {
    pub fn new(revolt: String, features: crate::models::RevoltConfigFeatures, ws: String, app: String, vapid: String) -> RevoltConfig {
        RevoltConfig {
            revolt,
            features: Box::new(features),
            ws,
            app,
            vapid,
        }
    }
}


