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

impl Collection {
    /// Get the icon for this collection, mapping icon names to appropriate glyphs/emojis
    pub fn icon(&self) -> &str {
        self.icon
            .as_deref()
            .map(|i| crate::icon::collection_icon_to_string(i))
            .unwrap_or("📁")
    }
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
// Export/Import Operations Request Types
// ============================================================================

/// Request to export a collection
#[derive(Debug, Clone, Serialize)]
pub struct ExportCollectionRequest {
    pub id: String,
    pub format: super::common::ExportFormat,
}

impl ExportCollectionRequest {
    pub fn new(id: String, format: super::common::ExportFormat) -> Self {
        Self { id, format }
    }
}

/// Request to export all collections
#[derive(Debug, Clone, Serialize)]
pub struct ExportAllCollectionsRequest {
    pub format: super::common::ExportFormat,
}

impl ExportAllCollectionsRequest {
    pub fn new(format: super::common::ExportFormat) -> Self {
        Self { format }
    }
}

/// Request to import a file into a collection
#[derive(Debug, Clone, Serialize)]
pub struct ImportFileToCollectionRequest {
    pub id: String,
    #[serde(skip)]
    pub file: Vec<u8>,
    pub format: super::common::ImportFormat,
}

impl ImportFileToCollectionRequest {
    pub fn new(id: String, file: Vec<u8>, format: super::common::ImportFormat) -> Self {
        Self { id, file, format }
    }

    pub fn builder(id: String) -> ImportFileToCollectionRequestBuilder {
        ImportFileToCollectionRequestBuilder::new(id)
    }
}

/// Builder for ImportFileToCollectionRequest
#[derive(Debug)]
pub struct ImportFileToCollectionRequestBuilder {
    id: String,
    file: Option<Vec<u8>>,
    format: Option<super::common::ImportFormat>,
}

impl ImportFileToCollectionRequestBuilder {
    pub fn new(id: String) -> Self {
        Self {
            id,
            file: None,
            format: None,
        }
    }

    pub fn file(mut self, file: Vec<u8>) -> Self {
        self.file = Some(file);
        self
    }

    pub fn format(mut self, format: super::common::ImportFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn build(self) -> ImportFileToCollectionRequest {
        ImportFileToCollectionRequest {
            id: self.id,
            file: self.file.expect("file is required"),
            format: self.format.expect("format is required"),
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

    // ========================================================================
    // Export/Import Operations Tests
    // ========================================================================

    #[test]
    fn test_export_collection_request_new() {
        use crate::types::common::ExportFormat;
        let req = ExportCollectionRequest::new("col123".to_string(), ExportFormat::Markdown);
        assert_eq!(req.id, "col123");
        assert!(matches!(req.format, ExportFormat::Markdown));
    }

    #[test]
    fn test_export_collection_request_serialization() {
        use crate::types::common::ExportFormat;
        let req = ExportCollectionRequest::new("col123".to_string(), ExportFormat::Html);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col123\""));
        assert!(json.contains("\"format\":\"html\""));
    }

    #[test]
    fn test_export_collection_request_all_formats() {
        use crate::types::common::ExportFormat;

        let md_req = ExportCollectionRequest::new("col1".to_string(), ExportFormat::Markdown);
        let html_req = ExportCollectionRequest::new("col1".to_string(), ExportFormat::Html);
        let pdf_req = ExportCollectionRequest::new("col1".to_string(), ExportFormat::Pdf);

        assert!(matches!(md_req.format, ExportFormat::Markdown));
        assert!(matches!(html_req.format, ExportFormat::Html));
        assert!(matches!(pdf_req.format, ExportFormat::Pdf));
    }

    #[test]
    fn test_export_collection_request_format_serialization() {
        use crate::types::common::ExportFormat;

        let req = ExportCollectionRequest::new("col1".to_string(), ExportFormat::Markdown);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"format\":\"markdown\""));

        let req = ExportCollectionRequest::new("col1".to_string(), ExportFormat::Html);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"format\":\"html\""));

        let req = ExportCollectionRequest::new("col1".to_string(), ExportFormat::Pdf);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"format\":\"pdf\""));
    }

    #[test]
    fn test_export_all_collections_request_new() {
        use crate::types::common::ExportFormat;
        let req = ExportAllCollectionsRequest::new(ExportFormat::Markdown);
        assert!(matches!(req.format, ExportFormat::Markdown));
    }

    #[test]
    fn test_export_all_collections_request_serialization() {
        use crate::types::common::ExportFormat;
        let req = ExportAllCollectionsRequest::new(ExportFormat::Html);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"format\":\"html\""));
    }

    #[test]
    fn test_export_all_collections_request_all_formats() {
        use crate::types::common::ExportFormat;

        let md_req = ExportAllCollectionsRequest::new(ExportFormat::Markdown);
        let html_req = ExportAllCollectionsRequest::new(ExportFormat::Html);
        let pdf_req = ExportAllCollectionsRequest::new(ExportFormat::Pdf);

        assert!(matches!(md_req.format, ExportFormat::Markdown));
        assert!(matches!(html_req.format, ExportFormat::Html));
        assert!(matches!(pdf_req.format, ExportFormat::Pdf));
    }

    #[test]
    fn test_import_file_to_collection_request_new() {
        use crate::types::common::ImportFormat;
        let file_data = vec![1, 2, 3, 4, 5];
        let req = ImportFileToCollectionRequest::new(
            "col123".to_string(),
            file_data.clone(),
            ImportFormat::Markdown,
        );
        assert_eq!(req.id, "col123");
        assert_eq!(req.file, file_data);
        assert!(matches!(req.format, ImportFormat::Markdown));
    }

    #[test]
    fn test_import_file_to_collection_request_builder() {
        use crate::types::common::ImportFormat;
        let file_data = vec![10, 20, 30];
        let req = ImportFileToCollectionRequest::builder("col456".to_string())
            .file(file_data.clone())
            .format(ImportFormat::Html)
            .build();

        assert_eq!(req.id, "col456");
        assert_eq!(req.file, file_data);
        assert!(matches!(req.format, ImportFormat::Html));
    }

    #[test]
    fn test_import_file_to_collection_request_all_formats() {
        use crate::types::common::ImportFormat;
        let file_data = vec![1, 2, 3];

        let md_req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            file_data.clone(),
            ImportFormat::Markdown,
        );
        let html_req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            file_data.clone(),
            ImportFormat::Html,
        );
        let docx_req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            file_data.clone(),
            ImportFormat::Docx,
        );
        let notion_req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            file_data.clone(),
            ImportFormat::Notion,
        );
        let confluence_req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            file_data.clone(),
            ImportFormat::Confluence,
        );

        assert!(matches!(md_req.format, ImportFormat::Markdown));
        assert!(matches!(html_req.format, ImportFormat::Html));
        assert!(matches!(docx_req.format, ImportFormat::Docx));
        assert!(matches!(notion_req.format, ImportFormat::Notion));
        assert!(matches!(confluence_req.format, ImportFormat::Confluence));
    }

    #[test]
    fn test_import_file_to_collection_request_serialization() {
        use crate::types::common::ImportFormat;
        let file_data = vec![1, 2, 3, 4, 5];
        let req = ImportFileToCollectionRequest::new(
            "col123".to_string(),
            file_data.clone(),
            ImportFormat::Docx,
        );

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"col123\""));
        assert!(json.contains("\"format\":\"docx\""));
        // File data should be skipped in serialization
        assert!(!json.contains("file"));
    }

    #[test]
    fn test_import_file_to_collection_request_file_skipped() {
        use crate::types::common::ImportFormat;
        let file_data = vec![100, 200, 250];
        let req = ImportFileToCollectionRequest::new(
            "col123".to_string(),
            file_data,
            ImportFormat::Notion,
        );

        let json = serde_json::to_string(&req).unwrap();
        // Verify file field is not serialized
        assert!(!json.contains("\"file\""));
    }

    #[test]
    fn test_import_file_to_collection_request_empty_file() {
        use crate::types::common::ImportFormat;
        let req = ImportFileToCollectionRequest::new(
            "col789".to_string(),
            vec![],
            ImportFormat::Markdown,
        );
        assert_eq!(req.file.len(), 0);
    }

    #[test]
    fn test_import_file_to_collection_request_large_file() {
        use crate::types::common::ImportFormat;
        let large_file = vec![0u8; 1_000_000]; // 1MB
        let req = ImportFileToCollectionRequest::new(
            "col999".to_string(),
            large_file.clone(),
            ImportFormat::Html,
        );
        assert_eq!(req.file.len(), 1_000_000);
        assert_eq!(req.file, large_file);
    }

    #[test]
    #[should_panic(expected = "file is required")]
    fn test_import_file_to_collection_request_builder_missing_file() {
        use crate::types::common::ImportFormat;
        ImportFileToCollectionRequest::builder("col123".to_string())
            .format(ImportFormat::Markdown)
            .build();
    }

    #[test]
    #[should_panic(expected = "format is required")]
    fn test_import_file_to_collection_request_builder_missing_format() {
        ImportFileToCollectionRequest::builder("col123".to_string())
            .file(vec![1, 2, 3])
            .build();
    }

    #[test]
    fn test_import_file_to_collection_request_builder_method_chaining() {
        use crate::types::common::ImportFormat;
        let file_data = vec![5, 10, 15, 20];
        let req = ImportFileToCollectionRequest::builder("col888".to_string())
            .format(ImportFormat::Confluence)
            .file(file_data.clone())
            .build();

        assert_eq!(req.id, "col888");
        assert_eq!(req.file, file_data);
        assert!(matches!(req.format, ImportFormat::Confluence));
    }

    #[test]
    fn test_import_format_serialization() {
        use crate::types::common::ImportFormat;

        let req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            vec![1],
            ImportFormat::Markdown,
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"format\":\"markdown\""));

        let req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            vec![1],
            ImportFormat::Html,
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"format\":\"html\""));

        let req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            vec![1],
            ImportFormat::Docx,
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"format\":\"docx\""));

        let req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            vec![1],
            ImportFormat::Notion,
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"format\":\"notion\""));

        let req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            vec![1],
            ImportFormat::Confluence,
        );
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"format\":\"confluence\""));
    }

    #[test]
    fn test_export_collection_edge_cases() {
        use crate::types::common::ExportFormat;

        // Empty string ID
        let req = ExportCollectionRequest::new("".to_string(), ExportFormat::Markdown);
        assert_eq!(req.id, "");

        // Unicode ID
        let req = ExportCollectionRequest::new("📚col".to_string(), ExportFormat::Html);
        assert_eq!(req.id, "📚col");
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.len() > 0);
    }

    #[test]
    fn test_export_all_collections_edge_cases() {
        use crate::types::common::ExportFormat;

        // Just verify all formats work
        for format in [ExportFormat::Markdown, ExportFormat::Html, ExportFormat::Pdf] {
            let req = ExportAllCollectionsRequest::new(format);
            let json = serde_json::to_string(&req).unwrap();
            assert!(json.len() > 0);
        }
    }

    #[test]
    fn test_import_file_binary_data() {
        use crate::types::common::ImportFormat;

        // Test with binary data (not UTF-8)
        let binary_data = vec![0xFF, 0xFE, 0xFD, 0xFC];
        let req = ImportFileToCollectionRequest::new(
            "col123".to_string(),
            binary_data.clone(),
            ImportFormat::Docx,
        );
        assert_eq!(req.file, binary_data);
    }

    #[test]
    fn test_import_file_utf8_data() {
        use crate::types::common::ImportFormat;

        // Test with UTF-8 text data
        let text = "# Hello World\n\nTest document".as_bytes().to_vec();
        let req = ImportFileToCollectionRequest::new(
            "col123".to_string(),
            text.clone(),
            ImportFormat::Markdown,
        );
        assert_eq!(req.file, text);
        assert_eq!(String::from_utf8(req.file).unwrap(), "# Hello World\n\nTest document");
    }

    #[test]
    fn test_all_export_import_types_are_cloneable() {
        use crate::types::common::{ExportFormat, ImportFormat};

        let export_req = ExportCollectionRequest::new("col1".to_string(), ExportFormat::Markdown);
        let export_req_clone = export_req.clone();
        assert_eq!(export_req.id, export_req_clone.id);

        let export_all_req = ExportAllCollectionsRequest::new(ExportFormat::Html);
        let export_all_req_clone = export_all_req.clone();
        assert!(matches!(export_all_req_clone.format, ExportFormat::Html));

        let import_req = ImportFileToCollectionRequest::new(
            "col1".to_string(),
            vec![1, 2, 3],
            ImportFormat::Docx,
        );
        let import_req_clone = import_req.clone();
        assert_eq!(import_req.id, import_req_clone.id);
        assert_eq!(import_req.file, import_req_clone.file);
    }
}
