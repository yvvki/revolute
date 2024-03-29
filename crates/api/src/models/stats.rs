/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Stats : Server Stats



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Stats {
    /// Index usage information
    #[serde(rename = "indices")]
    pub indices: ::std::collections::HashMap<String, Vec<crate::models::Index>>,
    /// Collection stats
    #[serde(rename = "coll_stats")]
    pub coll_stats: ::std::collections::HashMap<String, crate::models::CollectionStats>,
}

impl Stats {
    /// Server Stats
    pub fn new(indices: ::std::collections::HashMap<String, Vec<crate::models::Index>>, coll_stats: ::std::collections::HashMap<String, crate::models::CollectionStats>) -> Stats {
        Stats {
            indices,
            coll_stats,
        }
    }
}


