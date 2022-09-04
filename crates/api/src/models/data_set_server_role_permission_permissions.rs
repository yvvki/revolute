/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// DataSetServerRolePermissionPermissions : Allow / deny values for the role in this server.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataSetServerRolePermissionPermissions {
    /// Allow bit flags
    #[serde(rename = "allow")]
    pub allow: i32,
    /// Disallow bit flags
    #[serde(rename = "deny")]
    pub deny: i32,
}

impl DataSetServerRolePermissionPermissions {
    /// Allow / deny values for the role in this server.
    pub fn new(allow: i32, deny: i32) -> DataSetServerRolePermissionPermissions {
        DataSetServerRolePermissionPermissions {
            allow,
            deny,
        }
    }
}

