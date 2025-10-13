use serde::{Deserialize, Serialize};
use super::User;

// ============================================================================
// Comment Type
// ============================================================================

/// A comment on a document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub document_id: String,
    pub parent_comment_id: Option<String>,
    pub created_by: User,
    pub created_at: String,
    pub updated_at: String,
    pub data: CommentData,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<User>,
}

/// Comment data structure containing text and optional position
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentData {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<CommentPosition>,
}

impl CommentData {
    pub fn new(text: String) -> Self {
        Self {
            text,
            position: None,
        }
    }

    pub fn with_position(mut self, position: CommentPosition) -> Self {
        self.position = Some(position);
        self
    }
}

/// Position of a comment in the document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentPosition {
    pub line: u32,
    pub character: u32,
}

impl CommentPosition {
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

// ============================================================================
// Create Comment
// ============================================================================

/// Request to create a comment on a document
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentRequest {
    pub document_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_comment_id: Option<String>,
    pub data: CommentData,
}

impl CreateCommentRequest {
    pub fn new(document_id: String, data: CommentData) -> Self {
        Self {
            document_id,
            parent_comment_id: None,
            data,
        }
    }

    pub fn parent_comment_id(mut self, parent_comment_id: String) -> Self {
        self.parent_comment_id = Some(parent_comment_id);
        self
    }
}

// ============================================================================
// Get Comment
// ============================================================================

/// Request to get comment details
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentInfoRequest {
    pub id: String,
}

impl CommentInfoRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

// ============================================================================
// List Comments
// ============================================================================

/// Request to list comments on a document
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListCommentsRequest {
    pub document_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ListCommentsRequest {
    pub fn new(document_id: String) -> Self {
        Self {
            document_id,
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

/// Response from listing comments
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCommentsResponse {
    pub data: Vec<Comment>,
    pub pagination: super::Pagination,
}

// ============================================================================
// Update Comment
// ============================================================================

/// Request to update a comment
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommentRequest {
    pub id: String,
    pub data: CommentData,
}

impl UpdateCommentRequest {
    pub fn new(id: String, data: CommentData) -> Self {
        Self { id, data }
    }
}

// ============================================================================
// Delete Comment
// ============================================================================

/// Request to delete a comment
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommentRequest {
    pub id: String,
}

impl DeleteCommentRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

// ============================================================================
// Resolve Comment
// ============================================================================

/// Request to mark a comment thread as resolved
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveCommentRequest {
    pub id: String,
}

impl ResolveCommentRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

// ============================================================================
// Unresolve Comment
// ============================================================================

/// Request to mark a comment thread as unresolved
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnresolveCommentRequest {
    pub id: String,
}

impl UnresolveCommentRequest {
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
    fn test_comment_data_builder() {
        let data = CommentData::new("Test comment".to_string());
        assert_eq!(data.text, "Test comment");
        assert!(data.position.is_none());

        let data_with_position = data.with_position(CommentPosition::new(10, 5));
        assert!(data_with_position.position.is_some());
        let pos = data_with_position.position.unwrap();
        assert_eq!(pos.line, 10);
        assert_eq!(pos.character, 5);
    }

    #[test]
    fn test_create_comment_request_builder() {
        let data = CommentData::new("Test comment".to_string());
        let request = CreateCommentRequest::new("doc-123".to_string(), data.clone());
        assert_eq!(request.document_id, "doc-123");
        assert!(request.parent_comment_id.is_none());

        let request_with_parent = request.parent_comment_id("parent-456".to_string());
        assert_eq!(request_with_parent.parent_comment_id, Some("parent-456".to_string()));
    }

    #[test]
    fn test_list_comments_request_builder() {
        let request = ListCommentsRequest::new("doc-123".to_string());
        assert_eq!(request.document_id, "doc-123");
        assert!(request.offset.is_none());
        assert!(request.limit.is_none());

        let request_with_pagination = request.offset(10).limit(20);
        assert_eq!(request_with_pagination.offset, Some(10));
        assert_eq!(request_with_pagination.limit, Some(20));
    }

    #[test]
    fn test_comment_position() {
        let position = CommentPosition::new(42, 15);
        assert_eq!(position.line, 42);
        assert_eq!(position.character, 15);
    }

    #[test]
    fn test_serialize_create_comment_request() {
        let data = CommentData::new("Test comment".to_string());
        let request = CreateCommentRequest::new("doc-123".to_string(), data);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["documentId"], "doc-123");
        assert_eq!(json["data"]["text"], "Test comment");
    }

    #[test]
    fn test_serialize_list_comments_request() {
        let request = ListCommentsRequest::new("doc-123".to_string())
            .offset(5)
            .limit(10);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["documentId"], "doc-123");
        assert_eq!(json["offset"], 5);
        assert_eq!(json["limit"], 10);
    }
}
