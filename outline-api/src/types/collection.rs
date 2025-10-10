use serde::{Deserialize, Serialize};

use super::common::{CollectionPermission, PaginationResponse};

/// Collection structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<CollectionPermission>,
}

// ============================================================================
// Request Types
// ============================================================================

/// Request to list collections
#[derive(Debug, Clone, Serialize)]
pub struct ListCollectionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ListCollectionsRequest {
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
        }
    }

    pub fn with_pagination(offset: u32, limit: u32) -> Self {
        Self {
            offset: Some(offset),
            limit: Some(limit),
        }
    }
}

impl Default for ListCollectionsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request to get collection info
#[derive(Debug, Clone, Serialize)]
pub struct CollectionInfoRequest {
    pub id: String,
}

impl CollectionInfoRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Request to create a collection
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<CollectionPermission>,
}

impl CreateCollectionRequest {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            color: None,
            private: None,
            permission: None,
        }
    }

    pub fn builder(name: String) -> CreateCollectionRequestBuilder {
        CreateCollectionRequestBuilder::new(name)
    }
}

/// Builder for CreateCollectionRequest
#[derive(Debug)]
pub struct CreateCollectionRequestBuilder {
    name: String,
    description: Option<String>,
    color: Option<String>,
    private: Option<bool>,
    permission: Option<CollectionPermission>,
}

impl CreateCollectionRequestBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            color: None,
            private: None,
            permission: None,
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }

    pub fn private(mut self, private: bool) -> Self {
        self.private = Some(private);
        self
    }

    pub fn permission(mut self, permission: CollectionPermission) -> Self {
        self.permission = Some(permission);
        self
    }

    pub fn build(self) -> CreateCollectionRequest {
        CreateCollectionRequest {
            name: self.name,
            description: self.description,
            color: self.color,
            private: self.private,
            permission: self.permission,
        }
    }
}

/// Request to update a collection
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCollectionRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<CollectionPermission>,
}

impl UpdateCollectionRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: None,
            description: None,
            color: None,
            permission: None,
        }
    }

    pub fn builder(id: String) -> UpdateCollectionRequestBuilder {
        UpdateCollectionRequestBuilder::new(id)
    }
}

/// Builder for UpdateCollectionRequest
#[derive(Debug)]
pub struct UpdateCollectionRequestBuilder {
    id: String,
    name: Option<String>,
    description: Option<String>,
    color: Option<String>,
    permission: Option<CollectionPermission>,
}

impl UpdateCollectionRequestBuilder {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: None,
            description: None,
            color: None,
            permission: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }

    pub fn permission(mut self, permission: CollectionPermission) -> Self {
        self.permission = Some(permission);
        self
    }

    pub fn build(self) -> UpdateCollectionRequest {
        UpdateCollectionRequest {
            id: self.id,
            name: self.name,
            description: self.description,
            color: self.color,
            permission: self.permission,
        }
    }
}

/// Request to delete a collection
#[derive(Debug, Clone, Serialize)]
pub struct DeleteCollectionRequest {
    pub id: String,
}

impl DeleteCollectionRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

// ============================================================================
// Response Types
// ============================================================================

/// Response from listing collections
#[derive(Debug, Deserialize)]
pub struct ListCollectionsResponse {
    pub data: Vec<Collection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationResponse>,
}

// ============================================================================
// Organization Operations Request Types
// ============================================================================

/// Request to move a collection
#[derive(Debug, Clone, Serialize)]
pub struct MoveCollectionRequest {
    pub id: String,
    pub index: u32,
}

impl MoveCollectionRequest {
    pub fn new(id: String, index: u32) -> Self {
        Self { id, index }
    }
}

/// Request to list documents in a collection
#[derive(Debug, Clone, Serialize)]
pub struct CollectionDocumentsRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl CollectionDocumentsRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            offset: None,
            limit: None,
        }
    }

    pub fn with_pagination(id: String, offset: u32, limit: u32) -> Self {
        Self {
            id,
            offset: Some(offset),
            limit: Some(limit),
        }
    }
}

// ============================================================================
// Member Management Request Types
// ============================================================================

/// Request to add a user to a collection
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddUserToCollectionRequest {
    pub id: String,
    pub user_id: String,
    pub permission: super::common::Permission,
}

impl AddUserToCollectionRequest {
    pub fn new(id: String, user_id: String, permission: super::common::Permission) -> Self {
        Self {
            id,
            user_id,
            permission,
        }
    }
}

/// Request to remove a user from a collection
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveUserFromCollectionRequest {
    pub id: String,
    pub user_id: String,
}

impl RemoveUserFromCollectionRequest {
    pub fn new(id: String, user_id: String) -> Self {
        Self { id, user_id }
    }
}

/// Request to add a group to a collection
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGroupToCollectionRequest {
    pub id: String,
    pub group_id: String,
    pub permission: super::common::Permission,
}

impl AddGroupToCollectionRequest {
    pub fn new(id: String, group_id: String, permission: super::common::Permission) -> Self {
        Self {
            id,
            group_id,
            permission,
        }
    }
}

/// Request to remove a group from a collection
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveGroupFromCollectionRequest {
    pub id: String,
    pub group_id: String,
}

impl RemoveGroupFromCollectionRequest {
    pub fn new(id: String, group_id: String) -> Self {
        Self { id, group_id }
    }
}

/// Request to list collection members
#[derive(Debug, Clone, Serialize)]
pub struct CollectionMembershipsRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl CollectionMembershipsRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            query: None,
            offset: None,
            limit: None,
        }
    }

    pub fn with_query(id: String, query: String) -> Self {
        Self {
            id,
            query: Some(query),
            offset: None,
            limit: None,
        }
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::common::{CollectionPermission, Permission};

    // ========================================================================
    // Core Request Types Tests
    // ========================================================================

    #[test]
    fn test_list_collections_request_new() {
        let req = ListCollectionsRequest::new();
        assert!(req.offset.is_none());
        assert!(req.limit.is_none());
    }

    #[test]
    fn test_list_collections_request_default() {
        let req = ListCollectionsRequest::default();
        assert!(req.offset.is_none());
        assert!(req.limit.is_none());
    }

    #[test]
    fn test_list_collections_request_with_pagination() {
        let req = ListCollectionsRequest::with_pagination(10, 50);
        assert_eq!(req.offset, Some(10));
        assert_eq!(req.limit, Some(50));
    }

    #[test]
    fn test_list_collections_request_serialization() {
        let req = ListCollectionsRequest::with_pagination(20, 100);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"offset\":20"));
        assert!(json.contains("\"limit\":100"));
    }

    #[test]
    fn test_list_collections_request_optional_fields_omitted() {
        let req = ListCollectionsRequest::new();
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_collection_info_request_new() {
        let req = CollectionInfoRequest::new("col123".to_string());
        assert_eq!(req.id, "col123");
    }

    #[test]
    fn test_collection_info_request_serialization() {
        let req = CollectionInfoRequest::new("col123".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col123\""));
    }

    #[test]
    fn test_create_collection_request_new() {
        let req = CreateCollectionRequest::new("My Collection".to_string());
        assert_eq!(req.name, "My Collection");
        assert!(req.description.is_none());
        assert!(req.color.is_none());
        assert!(req.private.is_none());
        assert!(req.permission.is_none());
    }

    #[test]
    fn test_create_collection_request_builder() {
        let req = CreateCollectionRequest::builder("My Collection".to_string())
            .description("Test collection".to_string())
            .color("#FF0000".to_string())
            .private(true)
            .permission(CollectionPermission::Read)
            .build();

        assert_eq!(req.name, "My Collection");
        assert_eq!(req.description, Some("Test collection".to_string()));
        assert_eq!(req.color, Some("#FF0000".to_string()));
        assert_eq!(req.private, Some(true));
        assert!(matches!(req.permission, Some(CollectionPermission::Read)));
    }

    #[test]
    fn test_create_collection_request_builder_partial() {
        let req = CreateCollectionRequest::builder("Minimal Collection".to_string())
            .description("Just a description".to_string())
            .build();

        assert_eq!(req.name, "Minimal Collection");
        assert_eq!(req.description, Some("Just a description".to_string()));
        assert!(req.color.is_none());
        assert!(req.private.is_none());
        assert!(req.permission.is_none());
    }

    #[test]
    fn test_create_collection_request_serialization_camel_case() {
        let req = CreateCollectionRequest::builder("Test".to_string())
            .private(true)
            .build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"name\":\"Test\""));
        assert!(json.contains("\"private\":true"));
    }

    #[test]
    fn test_create_collection_request_optional_fields_omitted() {
        let req = CreateCollectionRequest::new("Simple".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"name\":\"Simple\""));
        assert!(!json.contains("description"));
        assert!(!json.contains("color"));
        assert!(!json.contains("private"));
        assert!(!json.contains("permission"));
    }

    #[test]
    fn test_update_collection_request_new() {
        let req = UpdateCollectionRequest::new("col456".to_string());
        assert_eq!(req.id, "col456");
        assert!(req.name.is_none());
        assert!(req.description.is_none());
        assert!(req.color.is_none());
        assert!(req.permission.is_none());
    }

    #[test]
    fn test_update_collection_request_builder() {
        let req = UpdateCollectionRequest::builder("col789".to_string())
            .name("Updated Name".to_string())
            .description("Updated description".to_string())
            .color("#00FF00".to_string())
            .permission(CollectionPermission::ReadWrite)
            .build();

        assert_eq!(req.id, "col789");
        assert_eq!(req.name, Some("Updated Name".to_string()));
        assert_eq!(req.description, Some("Updated description".to_string()));
        assert_eq!(req.color, Some("#00FF00".to_string()));
        assert!(matches!(
            req.permission,
            Some(CollectionPermission::ReadWrite)
        ));
    }

    #[test]
    fn test_update_collection_request_builder_partial() {
        let req = UpdateCollectionRequest::builder("col999".to_string())
            .name("New Name".to_string())
            .build();

        assert_eq!(req.id, "col999");
        assert_eq!(req.name, Some("New Name".to_string()));
        assert!(req.description.is_none());
        assert!(req.color.is_none());
        assert!(req.permission.is_none());
    }

    #[test]
    fn test_update_collection_request_serialization_camel_case() {
        let req = UpdateCollectionRequest::builder("col111".to_string())
            .name("Test".to_string())
            .build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col111\""));
        assert!(json.contains("\"name\":\"Test\""));
    }

    #[test]
    fn test_update_collection_request_optional_fields_omitted() {
        let req = UpdateCollectionRequest::new("col222".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col222\""));
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("description"));
        assert!(!json.contains("color"));
        assert!(!json.contains("permission"));
    }

    #[test]
    fn test_delete_collection_request_new() {
        let req = DeleteCollectionRequest::new("col999".to_string());
        assert_eq!(req.id, "col999");
    }

    #[test]
    fn test_delete_collection_request_serialization() {
        let req = DeleteCollectionRequest::new("col999".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col999\""));
    }

    // ========================================================================
    // Organization Operations Tests
    // ========================================================================

    #[test]
    fn test_move_collection_request_new() {
        let req = MoveCollectionRequest::new("col123".to_string(), 5);
        assert_eq!(req.id, "col123");
        assert_eq!(req.index, 5);
    }

    #[test]
    fn test_move_collection_request_serialization() {
        let req = MoveCollectionRequest::new("col123".to_string(), 5);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col123\""));
        assert!(json.contains("\"index\":5"));
    }

    #[test]
    fn test_move_collection_request_zero_index() {
        let req = MoveCollectionRequest::new("col456".to_string(), 0);
        assert_eq!(req.index, 0);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"index\":0"));
    }

    #[test]
    fn test_collection_documents_request_new() {
        let req = CollectionDocumentsRequest::new("col789".to_string());
        assert_eq!(req.id, "col789");
        assert!(req.offset.is_none());
        assert!(req.limit.is_none());
    }

    #[test]
    fn test_collection_documents_request_with_pagination() {
        let req = CollectionDocumentsRequest::with_pagination("col789".to_string(), 10, 25);
        assert_eq!(req.id, "col789");
        assert_eq!(req.offset, Some(10));
        assert_eq!(req.limit, Some(25));
    }

    #[test]
    fn test_collection_documents_request_serialization() {
        let req = CollectionDocumentsRequest::with_pagination("col789".to_string(), 20, 50);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col789\""));
        assert!(json.contains("\"offset\":20"));
        assert!(json.contains("\"limit\":50"));
    }

    #[test]
    fn test_collection_documents_request_optional_fields_omitted() {
        let req = CollectionDocumentsRequest::new("col999".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col999\""));
        assert!(!json.contains("offset"));
        assert!(!json.contains("limit"));
    }

    // ========================================================================
    // Member Management Tests
    // ========================================================================

    #[test]
    fn test_add_user_to_collection_request_new() {
        let req = AddUserToCollectionRequest::new(
            "col123".to_string(),
            "user456".to_string(),
            Permission::Read,
        );
        assert_eq!(req.id, "col123");
        assert_eq!(req.user_id, "user456");
        assert!(matches!(req.permission, Permission::Read));
    }

    #[test]
    fn test_add_user_to_collection_request_read_write() {
        let req = AddUserToCollectionRequest::new(
            "col123".to_string(),
            "user789".to_string(),
            Permission::ReadWrite,
        );
        assert!(matches!(req.permission, Permission::ReadWrite));
    }

    #[test]
    fn test_add_user_to_collection_request_serialization_camel_case() {
        let req = AddUserToCollectionRequest::new(
            "col123".to_string(),
            "user456".to_string(),
            Permission::Read,
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col123\""));
        assert!(json.contains("\"userId\":\"user456\""));
        assert!(json.contains("\"permission\":\"read\""));
    }

    #[test]
    fn test_remove_user_from_collection_request_new() {
        let req = RemoveUserFromCollectionRequest::new("col123".to_string(), "user456".to_string());
        assert_eq!(req.id, "col123");
        assert_eq!(req.user_id, "user456");
    }

    #[test]
    fn test_remove_user_from_collection_request_serialization_camel_case() {
        let req = RemoveUserFromCollectionRequest::new("col123".to_string(), "user456".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col123\""));
        assert!(json.contains("\"userId\":\"user456\""));
    }

    #[test]
    fn test_add_group_to_collection_request_new() {
        let req = AddGroupToCollectionRequest::new(
            "col123".to_string(),
            "group456".to_string(),
            Permission::ReadWrite,
        );
        assert_eq!(req.id, "col123");
        assert_eq!(req.group_id, "group456");
        assert!(matches!(req.permission, Permission::ReadWrite));
    }

    #[test]
    fn test_add_group_to_collection_request_read_permission() {
        let req = AddGroupToCollectionRequest::new(
            "col789".to_string(),
            "group999".to_string(),
            Permission::Read,
        );
        assert!(matches!(req.permission, Permission::Read));
    }

    #[test]
    fn test_add_group_to_collection_request_serialization_camel_case() {
        let req = AddGroupToCollectionRequest::new(
            "col123".to_string(),
            "group456".to_string(),
            Permission::ReadWrite,
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col123\""));
        assert!(json.contains("\"groupId\":\"group456\""));
        assert!(json.contains("\"permission\":\"read_write\""));
    }

    #[test]
    fn test_remove_group_from_collection_request_new() {
        let req =
            RemoveGroupFromCollectionRequest::new("col123".to_string(), "group456".to_string());
        assert_eq!(req.id, "col123");
        assert_eq!(req.group_id, "group456");
    }

    #[test]
    fn test_remove_group_from_collection_request_serialization_camel_case() {
        let req =
            RemoveGroupFromCollectionRequest::new("col123".to_string(), "group456".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col123\""));
        assert!(json.contains("\"groupId\":\"group456\""));
    }

    #[test]
    fn test_collection_memberships_request_new() {
        let req = CollectionMembershipsRequest::new("col123".to_string());
        assert_eq!(req.id, "col123");
        assert!(req.query.is_none());
        assert!(req.offset.is_none());
        assert!(req.limit.is_none());
    }

    #[test]
    fn test_collection_memberships_request_with_query() {
        let req = CollectionMembershipsRequest::with_query("col123".to_string(), "john".to_string());
        assert_eq!(req.id, "col123");
        assert_eq!(req.query, Some("john".to_string()));
        assert!(req.offset.is_none());
        assert!(req.limit.is_none());
    }

    #[test]
    fn test_collection_memberships_request_serialization() {
        let req = CollectionMembershipsRequest::with_query("col123".to_string(), "jane".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col123\""));
        assert!(json.contains("\"query\":\"jane\""));
    }

    #[test]
    fn test_collection_memberships_request_optional_fields_omitted() {
        let req = CollectionMembershipsRequest::new("col999".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col999\""));
        assert!(!json.contains("query"));
        assert!(!json.contains("offset"));
        assert!(!json.contains("limit"));
    }

    // ========================================================================
    // Edge Cases and Additional Tests
    // ========================================================================

    #[test]
    fn test_create_collection_builder_method_chaining() {
        let req = CreateCollectionRequest::builder("Test".to_string())
            .color("#FFFFFF".to_string())
            .description("Desc".to_string())
            .private(false)
            .permission(CollectionPermission::ReadWrite)
            .build();

        assert_eq!(req.name, "Test");
        assert_eq!(req.color, Some("#FFFFFF".to_string()));
        assert_eq!(req.description, Some("Desc".to_string()));
        assert_eq!(req.private, Some(false));
        assert!(matches!(
            req.permission,
            Some(CollectionPermission::ReadWrite)
        ));
    }

    #[test]
    fn test_update_collection_builder_method_chaining() {
        let req = UpdateCollectionRequest::builder("col123".to_string())
            .color("#000000".to_string())
            .name("New".to_string())
            .description("New Desc".to_string())
            .permission(CollectionPermission::Read)
            .build();

        assert_eq!(req.id, "col123");
        assert_eq!(req.name, Some("New".to_string()));
        assert_eq!(req.color, Some("#000000".to_string()));
        assert_eq!(req.description, Some("New Desc".to_string()));
        assert!(matches!(req.permission, Some(CollectionPermission::Read)));
    }

    #[test]
    fn test_collection_permission_serialization() {
        let req = CreateCollectionRequest::builder("Test".to_string())
            .permission(CollectionPermission::Read)
            .build();
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"permission\":\"read\""));

        let req = CreateCollectionRequest::builder("Test".to_string())
            .permission(CollectionPermission::ReadWrite)
            .build();
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"permission\":\"read_write\""));
    }

    #[test]
    fn test_permission_serialization() {
        let req = AddUserToCollectionRequest::new(
            "col1".to_string(),
            "user1".to_string(),
            Permission::Read,
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"permission\":\"read\""));

        let req = AddUserToCollectionRequest::new(
            "col1".to_string(),
            "user1".to_string(),
            Permission::ReadWrite,
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"permission\":\"read_write\""));
    }

    #[test]
    fn test_empty_string_values() {
        let req = CreateCollectionRequest::builder("".to_string()).build();
        assert_eq!(req.name, "");

        let req = CollectionInfoRequest::new("".to_string());
        assert_eq!(req.id, "");
    }

    #[test]
    fn test_special_characters_in_strings() {
        let req = CreateCollectionRequest::builder("Test & \"Collection\"".to_string())
            .description("Special chars: <>&'\"".to_string())
            .build();

        let json = serde_json::to_string(&req).unwrap();
        // Verify JSON escaping works
        assert!(json.contains("Test & \\\"Collection\\\""));
    }

    #[test]
    fn test_large_index_values() {
        let req = MoveCollectionRequest::new("col1".to_string(), u32::MAX);
        assert_eq!(req.index, u32::MAX);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(&format!("\"index\":{}", u32::MAX)));
    }

    #[test]
    fn test_pagination_boundary_values() {
        let req = ListCollectionsRequest::with_pagination(0, 1);
        assert_eq!(req.offset, Some(0));
        assert_eq!(req.limit, Some(1));

        let req = ListCollectionsRequest::with_pagination(u32::MAX, u32::MAX);
        assert_eq!(req.offset, Some(u32::MAX));
        assert_eq!(req.limit, Some(u32::MAX));
    }

    #[test]
    fn test_unicode_in_collection_names() {
        let req = CreateCollectionRequest::builder("📚 Books Collection".to_string())
            .description("测试 Test עברית".to_string())
            .build();

        assert_eq!(req.name, "📚 Books Collection");
        assert_eq!(req.description, Some("测试 Test עברית".to_string()));

        let json = serde_json::to_string(&req).unwrap();
        // Verify it serializes without errors
        assert!(json.len() > 0);
    }
}
