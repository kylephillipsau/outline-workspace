use serde::{Deserialize, Serialize};

use super::common::{PaginationResponse, SortDirection, UserFilter, UserSort};

/// User structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_suspended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_viewer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_at: Option<String>,
}

// ============================================================================
// Request Types
// ============================================================================

/// Request to get user information
#[derive(Debug, Clone, Serialize)]
pub struct UserInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UserInfoRequest {
    /// Get current user's info
    pub fn new() -> Self {
        Self { id: None }
    }

    /// Get specific user's info
    pub fn for_user(id: String) -> Self {
        Self { id: Some(id) }
    }
}

impl Default for UserInfoRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request to update user profile
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl UpdateUserRequest {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            avatar_url: None,
            language: None,
        }
    }

    pub fn builder() -> UpdateUserRequestBuilder {
        UpdateUserRequestBuilder::default()
    }
}

impl Default for UpdateUserRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for UpdateUserRequest
#[derive(Debug, Default)]
pub struct UpdateUserRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    avatar_url: Option<String>,
    language: Option<String>,
}

impl UpdateUserRequestBuilder {
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn avatar_url(mut self, url: String) -> Self {
        self.avatar_url = Some(url);
        self
    }

    pub fn language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    pub fn build(self) -> UpdateUserRequest {
        UpdateUserRequest {
            id: self.id,
            name: self.name,
            avatar_url: self.avatar_url,
            language: self.language,
        }
    }
}

/// Request to list users
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<UserFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<UserSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<SortDirection>,
}

impl ListUsersRequest {
    pub fn new() -> Self {
        Self {
            query: None,
            filter: None,
            offset: None,
            limit: None,
            sort: None,
            direction: None,
        }
    }

    pub fn builder() -> ListUsersRequestBuilder {
        ListUsersRequestBuilder::default()
    }
}

impl Default for ListUsersRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for ListUsersRequest
#[derive(Debug, Default)]
pub struct ListUsersRequestBuilder {
    query: Option<String>,
    filter: Option<UserFilter>,
    offset: Option<u32>,
    limit: Option<u32>,
    sort: Option<UserSort>,
    direction: Option<SortDirection>,
}

impl ListUsersRequestBuilder {
    pub fn query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }

    pub fn filter(mut self, filter: UserFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn sort(mut self, sort: UserSort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn direction(mut self, direction: SortDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn build(self) -> ListUsersRequest {
        ListUsersRequest {
            query: self.query,
            filter: self.filter,
            offset: self.offset,
            limit: self.limit,
            sort: self.sort,
            direction: self.direction,
        }
    }
}

// ============================================================================
// Response Types
// ============================================================================

/// Response from listing users
#[derive(Debug, Deserialize)]
pub struct ListUsersResponse {
    pub data: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationResponse>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::common::{SortDirection, UserFilter, UserSort};

    // ========================================================================
    // UserInfoRequest Tests
    // ========================================================================

    #[test]
    fn test_user_info_request_current_user() {
        let req = UserInfoRequest::new();
        assert_eq!(req.id, None);
    }

    #[test]
    fn test_user_info_request_specific_user() {
        let req = UserInfoRequest::for_user("user123".to_string());
        assert_eq!(req.id, Some("user123".to_string()));
    }

    #[test]
    fn test_user_info_request_default() {
        let req = UserInfoRequest::default();
        assert_eq!(req.id, None);
    }

    #[test]
    fn test_user_info_request_serialize_without_id() {
        let req = UserInfoRequest::new();
        let json = serde_json::to_string(&req).unwrap();
        // When id is None, it should be skipped in serialization
        assert!(!json.contains("\"id\""));
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_user_info_request_serialize_with_id() {
        let req = UserInfoRequest::for_user("user456".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\""));
        assert!(json.contains("user456"));
    }

    // ========================================================================
    // UpdateUserRequest Tests
    // ========================================================================

    #[test]
    fn test_update_user_request_new() {
        let req = UpdateUserRequest::new();
        assert_eq!(req.id, None);
        assert_eq!(req.name, None);
        assert_eq!(req.avatar_url, None);
        assert_eq!(req.language, None);
    }

    #[test]
    fn test_update_user_request_default() {
        let req = UpdateUserRequest::default();
        assert_eq!(req.id, None);
        assert_eq!(req.name, None);
        assert_eq!(req.avatar_url, None);
        assert_eq!(req.language, None);
    }

    #[test]
    fn test_update_user_request_builder_empty() {
        let req = UpdateUserRequest::builder().build();
        assert_eq!(req.id, None);
        assert_eq!(req.name, None);
        assert_eq!(req.avatar_url, None);
        assert_eq!(req.language, None);
    }

    #[test]
    fn test_update_user_request_builder_with_id() {
        let req = UpdateUserRequest::builder()
            .id("user789".to_string())
            .build();
        assert_eq!(req.id, Some("user789".to_string()));
        assert_eq!(req.name, None);
    }

    #[test]
    fn test_update_user_request_builder_with_name() {
        let req = UpdateUserRequest::builder()
            .name("John Doe".to_string())
            .build();
        assert_eq!(req.name, Some("John Doe".to_string()));
        assert_eq!(req.id, None);
    }

    #[test]
    fn test_update_user_request_builder_with_avatar_url() {
        let req = UpdateUserRequest::builder()
            .avatar_url("https://example.com/avatar.png".to_string())
            .build();
        assert_eq!(
            req.avatar_url,
            Some("https://example.com/avatar.png".to_string())
        );
    }

    #[test]
    fn test_update_user_request_builder_with_language() {
        let req = UpdateUserRequest::builder()
            .language("en_US".to_string())
            .build();
        assert_eq!(req.language, Some("en_US".to_string()));
    }

    #[test]
    fn test_update_user_request_builder_with_all_fields() {
        let req = UpdateUserRequest::builder()
            .id("user123".to_string())
            .name("Jane Smith".to_string())
            .avatar_url("https://example.com/jane.png".to_string())
            .language("fr_FR".to_string())
            .build();

        assert_eq!(req.id, Some("user123".to_string()));
        assert_eq!(req.name, Some("Jane Smith".to_string()));
        assert_eq!(
            req.avatar_url,
            Some("https://example.com/jane.png".to_string())
        );
        assert_eq!(req.language, Some("fr_FR".to_string()));
    }

    #[test]
    fn test_update_user_request_builder_chaining() {
        let req = UpdateUserRequest::builder()
            .name("Alice".to_string())
            .language("es_ES".to_string())
            .build();

        assert_eq!(req.name, Some("Alice".to_string()));
        assert_eq!(req.language, Some("es_ES".to_string()));
    }

    #[test]
    fn test_update_user_request_serialize_empty() {
        let req = UpdateUserRequest::new();
        let json = serde_json::to_string(&req).unwrap();
        // All fields are None, should serialize to empty object
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_update_user_request_serialize_with_camel_case() {
        let req = UpdateUserRequest::builder()
            .name("Bob".to_string())
            .avatar_url("https://example.com/bob.jpg".to_string())
            .build();

        let json = serde_json::to_string(&req).unwrap();
        // Should use camelCase for field names
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"avatarUrl\""));
        assert!(!json.contains("\"avatar_url\""));
        assert!(json.contains("Bob"));
    }

    #[test]
    fn test_update_user_request_serialize_skips_none_fields() {
        let req = UpdateUserRequest::builder()
            .name("Charlie".to_string())
            .build();

        let json = serde_json::to_string(&req).unwrap();
        // Only name should be present, other fields should be skipped
        assert!(json.contains("\"name\""));
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"avatarUrl\""));
        assert!(!json.contains("\"language\""));
    }

    // ========================================================================
    // ListUsersRequest Tests
    // ========================================================================

    #[test]
    fn test_list_users_request_new() {
        let req = ListUsersRequest::new();
        assert_eq!(req.query, None);
        assert!(req.filter.is_none());
        assert_eq!(req.offset, None);
        assert_eq!(req.limit, None);
        assert!(req.sort.is_none());
        assert!(req.direction.is_none());
    }

    #[test]
    fn test_list_users_request_default() {
        let req = ListUsersRequest::default();
        assert_eq!(req.query, None);
        assert!(req.filter.is_none());
        assert_eq!(req.offset, None);
        assert_eq!(req.limit, None);
        assert!(req.sort.is_none());
        assert!(req.direction.is_none());
    }

    #[test]
    fn test_list_users_request_builder_empty() {
        let req = ListUsersRequest::builder().build();
        assert_eq!(req.query, None);
        assert!(req.filter.is_none());
        assert_eq!(req.offset, None);
        assert_eq!(req.limit, None);
        assert!(req.sort.is_none());
        assert!(req.direction.is_none());
    }

    #[test]
    fn test_list_users_request_builder_with_query() {
        let req = ListUsersRequest::builder()
            .query("admin".to_string())
            .build();
        assert_eq!(req.query, Some("admin".to_string()));
    }

    #[test]
    fn test_list_users_request_builder_with_filter() {
        let req = ListUsersRequest::builder()
            .filter(UserFilter::Admins)
            .build();
        assert!(matches!(req.filter, Some(UserFilter::Admins)));
    }

    #[test]
    fn test_list_users_request_builder_with_filter_active() {
        let req = ListUsersRequest::builder()
            .filter(UserFilter::Active)
            .build();
        assert!(matches!(req.filter, Some(UserFilter::Active)));
    }

    #[test]
    fn test_list_users_request_builder_with_filter_suspended() {
        let req = ListUsersRequest::builder()
            .filter(UserFilter::Suspended)
            .build();
        assert!(matches!(req.filter, Some(UserFilter::Suspended)));
    }

    #[test]
    fn test_list_users_request_builder_with_offset() {
        let req = ListUsersRequest::builder().offset(10).build();
        assert_eq!(req.offset, Some(10));
    }

    #[test]
    fn test_list_users_request_builder_with_limit() {
        let req = ListUsersRequest::builder().limit(50).build();
        assert_eq!(req.limit, Some(50));
    }

    #[test]
    fn test_list_users_request_builder_with_sort_name() {
        let req = ListUsersRequest::builder().sort(UserSort::Name).build();
        assert!(matches!(req.sort, Some(UserSort::Name)));
    }

    #[test]
    fn test_list_users_request_builder_with_sort_email() {
        let req = ListUsersRequest::builder().sort(UserSort::Email).build();
        assert!(matches!(req.sort, Some(UserSort::Email)));
    }

    #[test]
    fn test_list_users_request_builder_with_sort_last_active() {
        let req = ListUsersRequest::builder()
            .sort(UserSort::LastActive)
            .build();
        assert!(matches!(req.sort, Some(UserSort::LastActive)));
    }

    #[test]
    fn test_list_users_request_builder_with_sort_created_at() {
        let req = ListUsersRequest::builder()
            .sort(UserSort::CreatedAt)
            .build();
        assert!(matches!(req.sort, Some(UserSort::CreatedAt)));
    }

    #[test]
    fn test_list_users_request_builder_with_direction_asc() {
        let req = ListUsersRequest::builder()
            .direction(SortDirection::Asc)
            .build();
        assert!(matches!(req.direction, Some(SortDirection::Asc)));
    }

    #[test]
    fn test_list_users_request_builder_with_direction_desc() {
        let req = ListUsersRequest::builder()
            .direction(SortDirection::Desc)
            .build();
        assert!(matches!(req.direction, Some(SortDirection::Desc)));
    }

    #[test]
    fn test_list_users_request_builder_with_pagination() {
        let req = ListUsersRequest::builder()
            .offset(20)
            .limit(100)
            .build();
        assert_eq!(req.offset, Some(20));
        assert_eq!(req.limit, Some(100));
    }

    #[test]
    fn test_list_users_request_builder_with_all_fields() {
        let req = ListUsersRequest::builder()
            .query("developer".to_string())
            .filter(UserFilter::Active)
            .offset(30)
            .limit(25)
            .sort(UserSort::Name)
            .direction(SortDirection::Asc)
            .build();

        assert_eq!(req.query, Some("developer".to_string()));
        assert!(matches!(req.filter, Some(UserFilter::Active)));
        assert_eq!(req.offset, Some(30));
        assert_eq!(req.limit, Some(25));
        assert!(matches!(req.sort, Some(UserSort::Name)));
        assert!(matches!(req.direction, Some(SortDirection::Asc)));
    }

    #[test]
    fn test_list_users_request_builder_with_filters_and_sorting() {
        let req = ListUsersRequest::builder()
            .query("admin".to_string())
            .filter(UserFilter::Admins)
            .sort(UserSort::Email)
            .direction(SortDirection::Desc)
            .limit(50)
            .build();

        assert_eq!(req.query, Some("admin".to_string()));
        assert!(matches!(req.filter, Some(UserFilter::Admins)));
        assert_eq!(req.limit, Some(50));
        assert!(matches!(req.sort, Some(UserSort::Email)));
        assert!(matches!(req.direction, Some(SortDirection::Desc)));
    }

    #[test]
    fn test_list_users_request_serialize_empty() {
        let req = ListUsersRequest::new();
        let json = serde_json::to_string(&req).unwrap();
        // All fields are None, should serialize to empty object
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_list_users_request_serialize_with_camel_case() {
        let req = ListUsersRequest::builder()
            .query("test".to_string())
            .sort(UserSort::LastActive)
            .build();

        let json = serde_json::to_string(&req).unwrap();
        // Should use camelCase for sort field
        assert!(json.contains("\"query\""));
        assert!(json.contains("\"sort\""));
        assert!(json.contains("\"lastActive\""));
    }

    #[test]
    fn test_list_users_request_serialize_with_filter() {
        let req = ListUsersRequest::builder()
            .filter(UserFilter::Admins)
            .build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"filter\""));
        assert!(json.contains("\"admins\""));
    }

    #[test]
    fn test_list_users_request_serialize_with_direction() {
        let req = ListUsersRequest::builder()
            .direction(SortDirection::Asc)
            .build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"direction\""));
        assert!(json.contains("\"asc\""));
    }

    #[test]
    fn test_list_users_request_serialize_skips_none_fields() {
        let req = ListUsersRequest::builder()
            .query("search".to_string())
            .limit(10)
            .build();

        let json = serde_json::to_string(&req).unwrap();
        // Only query and limit should be present
        assert!(json.contains("\"query\""));
        assert!(json.contains("\"limit\""));
        assert!(!json.contains("\"filter\""));
        assert!(!json.contains("\"offset\""));
        assert!(!json.contains("\"sort\""));
        assert!(!json.contains("\"direction\""));
    }

    #[test]
    fn test_list_users_request_serialize_with_numeric_values() {
        let req = ListUsersRequest::builder().offset(0).limit(999).build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"offset\":0"));
        assert!(json.contains("\"limit\":999"));
    }

    // ========================================================================
    // Builder Default Tests
    // ========================================================================

    #[test]
    fn test_update_user_request_builder_default() {
        let builder = UpdateUserRequestBuilder::default();
        let req = builder.build();
        assert_eq!(req.id, None);
        assert_eq!(req.name, None);
        assert_eq!(req.avatar_url, None);
        assert_eq!(req.language, None);
    }

    #[test]
    fn test_list_users_request_builder_default() {
        let builder = ListUsersRequestBuilder::default();
        let req = builder.build();
        assert_eq!(req.query, None);
        assert!(req.filter.is_none());
        assert_eq!(req.offset, None);
        assert_eq!(req.limit, None);
        assert!(req.sort.is_none());
        assert!(req.direction.is_none());
    }

    // ========================================================================
    // Edge Case Tests
    // ========================================================================

    #[test]
    fn test_update_user_request_with_empty_strings() {
        let req = UpdateUserRequest::builder()
            .name("".to_string())
            .language("".to_string())
            .build();

        assert_eq!(req.name, Some("".to_string()));
        assert_eq!(req.language, Some("".to_string()));
    }

    #[test]
    fn test_list_users_request_with_empty_query() {
        let req = ListUsersRequest::builder().query("".to_string()).build();
        assert_eq!(req.query, Some("".to_string()));
    }

    #[test]
    fn test_list_users_request_with_zero_offset() {
        let req = ListUsersRequest::builder().offset(0).build();
        assert_eq!(req.offset, Some(0));
    }

    #[test]
    fn test_list_users_request_with_zero_limit() {
        let req = ListUsersRequest::builder().limit(0).build();
        assert_eq!(req.limit, Some(0));
    }

    #[test]
    fn test_user_info_request_clone() {
        let req1 = UserInfoRequest::for_user("user999".to_string());
        let req2 = req1.clone();
        assert_eq!(req1.id, req2.id);
    }

    #[test]
    fn test_update_user_request_clone() {
        let req1 = UpdateUserRequest::builder()
            .name("Test User".to_string())
            .build();
        let req2 = req1.clone();
        assert_eq!(req1.name, req2.name);
    }

    #[test]
    fn test_list_users_request_clone() {
        let req1 = ListUsersRequest::builder()
            .query("test".to_string())
            .build();
        let req2 = req1.clone();
        assert_eq!(req1.query, req2.query);
    }
}
