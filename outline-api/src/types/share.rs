use serde::{Deserialize, Serialize};
use super::{Document, User};

// ============================================================================
// Share Type
// ============================================================================

/// A public share link for a document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub id: String,
    pub document_id: String,
    pub document: Option<Document>,
    pub url: String,
    pub published: bool,
    pub include_child_documents: bool,
    pub created_by: User,
    pub created_at: String,
    pub updated_at: String,
    pub last_accessed_at: Option<String>,
}

// ============================================================================
// Create Share
// ============================================================================

/// Request to create a public share link
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateShareRequest {
    pub document_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_child_documents: Option<bool>,
}

impl CreateShareRequest {
    pub fn new(document_id: String) -> Self {
        Self {
            document_id,
            published: None,
            include_child_documents: None,
        }
    }

    pub fn published(mut self, published: bool) -> Self {
        self.published = Some(published);
        self
    }

    pub fn include_child_documents(mut self, include: bool) -> Self {
        self.include_child_documents = Some(include);
        self
    }
}

// ============================================================================
// Get Share
// ============================================================================

/// Request to get share details
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareInfoRequest {
    pub id: String,
}

impl ShareInfoRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

// ============================================================================
// List Shares
// ============================================================================

/// Request to list all shares
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListSharesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ListSharesRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn document_id(mut self, document_id: String) -> Self {
        self.document_id = Some(document_id);
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
}

/// Response from listing shares
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSharesResponse {
    pub data: Vec<Share>,
    pub pagination: super::Pagination,
}

// ============================================================================
// Update Share
// ============================================================================

/// Request to update share settings
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateShareRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_child_documents: Option<bool>,
}

impl UpdateShareRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            published: None,
            include_child_documents: None,
        }
    }

    pub fn published(mut self, published: bool) -> Self {
        self.published = Some(published);
        self
    }

    pub fn include_child_documents(mut self, include: bool) -> Self {
        self.include_child_documents = Some(include);
        self
    }
}

// ============================================================================
// Revoke Share
// ============================================================================

/// Request to revoke a share link
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeShareRequest {
    pub id: String,
}

impl RevokeShareRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_share_request_builder() {
        let request = CreateShareRequest::new("doc-123".to_string());
        assert_eq!(request.document_id, "doc-123");
        assert!(request.published.is_none());
        assert!(request.include_child_documents.is_none());

        let request_with_options = request
            .published(true)
            .include_child_documents(true);
        assert_eq!(request_with_options.published, Some(true));
        assert_eq!(request_with_options.include_child_documents, Some(true));
    }

    #[test]
    fn test_update_share_request_builder() {
        let request = UpdateShareRequest::new("share-123".to_string());
        assert_eq!(request.id, "share-123");
        assert!(request.published.is_none());
        assert!(request.include_child_documents.is_none());

        let request_updated = request
            .published(false)
            .include_child_documents(false);
        assert_eq!(request_updated.published, Some(false));
        assert_eq!(request_updated.include_child_documents, Some(false));
    }

    #[test]
    fn test_list_shares_request_builder() {
        let request = ListSharesRequest::new();
        assert!(request.document_id.is_none());
        assert!(request.offset.is_none());
        assert!(request.limit.is_none());

        let request_with_filters = request
            .document_id("doc-123".to_string())
            .offset(10)
            .limit(20);
        assert_eq!(request_with_filters.document_id, Some("doc-123".to_string()));
        assert_eq!(request_with_filters.offset, Some(10));
        assert_eq!(request_with_filters.limit, Some(20));
    }

    #[test]
    fn test_revoke_share_request() {
        let request = RevokeShareRequest::new("share-123".to_string());
        assert_eq!(request.id, "share-123");
    }

    #[test]
    fn test_serialize_create_share_request() {
        let request = CreateShareRequest::new("doc-123".to_string())
            .published(true)
            .include_child_documents(true);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["documentId"], "doc-123");
        assert_eq!(json["published"], true);
        assert_eq!(json["includeChildDocuments"], true);
    }

    #[test]
    fn test_serialize_update_share_request() {
        let request = UpdateShareRequest::new("share-123".to_string())
            .published(false);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["id"], "share-123");
        assert_eq!(json["published"], false);
    }

    #[test]
    fn test_serialize_list_shares_request() {
        let request = ListSharesRequest::new()
            .document_id("doc-123".to_string())
            .offset(5)
            .limit(10);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["documentId"], "doc-123");
        assert_eq!(json["offset"], 5);
        assert_eq!(json["limit"], 10);
    }
}
