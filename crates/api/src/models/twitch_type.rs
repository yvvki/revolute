/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// TwitchType : Type of remote Twitch content

/// Type of remote Twitch content
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TwitchType {
    #[serde(rename = "Channel")]
    Channel,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Clip")]
    Clip,

}

impl ToString for TwitchType {
    fn to_string(&self) -> String {
        match self {
            Self::Channel => String::from("Channel"),
            Self::Video => String::from("Video"),
            Self::Clip => String::from("Clip"),
        }
    }
}

impl Default for TwitchType {
    fn default() -> TwitchType {
        Self::Channel
    }
}




