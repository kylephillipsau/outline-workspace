use serde::{Deserialize, Serialize};
use super::User;

// ============================================================================
// Group Type
// ============================================================================

/// A team group for organizing users
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub member_count: Option<u32>,
}

// ============================================================================
// Create Group
// ============================================================================

/// Request to create a new group
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateGroupRequest {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

// ============================================================================
// Get Group
// ============================================================================

/// Request to get group details
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupInfoRequest {
    pub id: String,
}

impl GroupInfoRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

// ============================================================================
// List Groups
// ============================================================================

/// Request to list all groups
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ListGroupsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Response from listing groups
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupsResponse {
    pub data: Vec<Group>,
    pub pagination: super::Pagination,
}

// ============================================================================
// Update Group
// ============================================================================

/// Request to update group properties
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGroupRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateGroupRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: None,
            description: None,
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
}

// ============================================================================
// Delete Group
// ============================================================================

/// Request to delete a group
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteGroupRequest {
    pub id: String,
}

impl DeleteGroupRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

// ============================================================================
// Add User to Group
// ============================================================================

/// Request to add a user to a group
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddUserToGroupRequest {
    pub id: String,
    pub user_id: String,
}

impl AddUserToGroupRequest {
    pub fn new(id: String, user_id: String) -> Self {
        Self { id, user_id }
    }
}

// ============================================================================
// Remove User from Group
// ============================================================================

/// Request to remove a user from a group
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveUserFromGroupRequest {
    pub id: String,
    pub user_id: String,
}

impl RemoveUserFromGroupRequest {
    pub fn new(id: String, user_id: String) -> Self {
        Self { id, user_id }
    }
}

// ============================================================================
// Group Memberships
// ============================================================================

/// Request to list group members
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembershipsRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GroupMembershipsRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            offset: None,
            limit: None,
        }
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Group membership entry
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembership {
    pub id: String,
    pub user_id: String,
    pub group_id: String,
    pub user: User,
}

/// Response from listing group memberships
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembershipsResponse {
    pub data: Vec<GroupMembership>,
    pub pagination: super::Pagination,
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_group_request_builder() {
        let request = CreateGroupRequest::new("Engineering".to_string());
        assert_eq!(request.name, "Engineering");
        assert!(request.description.is_none());

        let request_with_desc = request.description("Engineering team".to_string());
        assert_eq!(request_with_desc.description, Some("Engineering team".to_string()));
    }

    #[test]
    fn test_update_group_request_builder() {
        let request = UpdateGroupRequest::new("group-123".to_string());
        assert_eq!(request.id, "group-123");
        assert!(request.name.is_none());
        assert!(request.description.is_none());

        let request_updated = request
            .name("New Name".to_string())
            .description("New Description".to_string());
        assert_eq!(request_updated.name, Some("New Name".to_string()));
        assert_eq!(request_updated.description, Some("New Description".to_string()));
    }

    #[test]
    fn test_list_groups_request_builder() {
        let request = ListGroupsRequest::new();
        assert!(request.offset.is_none());
        assert!(request.limit.is_none());

        let request_with_pagination = request.offset(10).limit(20);
        assert_eq!(request_with_pagination.offset, Some(10));
        assert_eq!(request_with_pagination.limit, Some(20));
    }

    #[test]
    fn test_group_memberships_request_builder() {
        let request = GroupMembershipsRequest::new("group-123".to_string());
        assert_eq!(request.id, "group-123");
        assert!(request.offset.is_none());
        assert!(request.limit.is_none());

        let request_with_pagination = request.offset(5).limit(15);
        assert_eq!(request_with_pagination.offset, Some(5));
        assert_eq!(request_with_pagination.limit, Some(15));
    }

    #[test]
    fn test_add_user_to_group_request() {
        let request = AddUserToGroupRequest::new("group-123".to_string(), "user-456".to_string());
        assert_eq!(request.id, "group-123");
        assert_eq!(request.user_id, "user-456");
    }

    #[test]
    fn test_remove_user_from_group_request() {
        let request = RemoveUserFromGroupRequest::new("group-123".to_string(), "user-456".to_string());
        assert_eq!(request.id, "group-123");
        assert_eq!(request.user_id, "user-456");
    }

    #[test]
    fn test_serialize_create_group_request() {
        let request = CreateGroupRequest::new("Engineering".to_string())
            .description("Engineering team".to_string());

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["name"], "Engineering");
        assert_eq!(json["description"], "Engineering team");
    }

    #[test]
    fn test_serialize_update_group_request() {
        let request = UpdateGroupRequest::new("group-123".to_string())
            .name("New Name".to_string());

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["id"], "group-123");
        assert_eq!(json["name"], "New Name");
    }
}
