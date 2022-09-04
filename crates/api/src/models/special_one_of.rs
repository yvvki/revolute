/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// SpecialOneOf : No remote content



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpecialOneOf {
    #[serde(rename = "type")]
    pub _type: Type,
}

impl SpecialOneOf {
    /// No remote content
    pub fn new(_type: Type) -> SpecialOneOf {
        SpecialOneOf {
            _type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "None")]
    None,
}

impl Default for Type {
    fn default() -> Type {
        Self::None
    }
}
