use serde::{Deserialize, Serialize};

// ============================================================================
// Attachment Type
// ============================================================================

/// A file attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: String,
    pub document_id: Option<String>,
    pub name: String,
    pub content_type: String,
    pub size: u64,
    pub url: String,
    pub created_at: String,
}

// ============================================================================
// Create Attachment
// ============================================================================

/// Request to upload a file attachment
#[derive(Debug, Clone)]
pub struct CreateAttachmentRequest {
    pub name: String,
    pub document_id: Option<String>,
    pub content_type: String,
    pub size: u64,
    pub data: Vec<u8>,
}

impl CreateAttachmentRequest {
    pub fn new(name: String, content_type: String, data: Vec<u8>) -> Self {
        let size = data.len() as u64;
        Self {
            name,
            document_id: None,
            content_type,
            size,
            data,
        }
    }

    pub fn document_id(mut self, document_id: String) -> Self {
        self.document_id = Some(document_id);
        self
    }
}

// ============================================================================
// Delete Attachment
// ============================================================================

/// Request to delete an attachment
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAttachmentRequest {
    pub id: String,
}

impl DeleteAttachmentRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

// ============================================================================
// Redirect Attachment
// ============================================================================

/// Request to get attachment download URL
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RedirectAttachmentRequest {
    pub id: String,
}

impl RedirectAttachmentRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Response containing the redirect URL for an attachment
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedirectAttachmentResponse {
    pub url: String,
}

// ============================================================================
// List Attachments
// ============================================================================

/// Request to list attachments
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListAttachmentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ListAttachmentsRequest {
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

/// Response from listing attachments
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAttachmentsResponse {
    pub data: Vec<Attachment>,
    pub pagination: super::Pagination,
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_attachment_request() {
        let data = vec![1, 2, 3, 4, 5];
        let request = CreateAttachmentRequest::new(
            "test.txt".to_string(),
            "text/plain".to_string(),
            data.clone(),
        );
        assert_eq!(request.name, "test.txt");
        assert_eq!(request.content_type, "text/plain");
        assert_eq!(request.size, 5);
        assert_eq!(request.data, data);
        assert!(request.document_id.is_none());

        let request_with_doc = request.document_id("doc-123".to_string());
        assert_eq!(request_with_doc.document_id, Some("doc-123".to_string()));
    }

    #[test]
    fn test_delete_attachment_request() {
        let request = DeleteAttachmentRequest::new("attachment-123".to_string());
        assert_eq!(request.id, "attachment-123");
    }

    #[test]
    fn test_redirect_attachment_request() {
        let request = RedirectAttachmentRequest::new("attachment-123".to_string());
        assert_eq!(request.id, "attachment-123");
    }

    #[test]
    fn test_list_attachments_request_builder() {
        let request = ListAttachmentsRequest::new();
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
    fn test_serialize_delete_attachment_request() {
        let request = DeleteAttachmentRequest::new("attachment-123".to_string());
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["id"], "attachment-123");
    }

    #[test]
    fn test_serialize_list_attachments_request() {
        let request = ListAttachmentsRequest::new()
            .document_id("doc-123".to_string())
            .offset(5)
            .limit(10);

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["documentId"], "doc-123");
        assert_eq!(json["offset"], 5);
        assert_eq!(json["limit"], 10);
    }

    #[test]
    fn test_attachment_size_calculation() {
        let data = vec![0u8; 1024]; // 1KB
        let request = CreateAttachmentRequest::new(
            "file.bin".to_string(),
            "application/octet-stream".to_string(),
            data,
        );
        assert_eq!(request.size, 1024);
    }
}
