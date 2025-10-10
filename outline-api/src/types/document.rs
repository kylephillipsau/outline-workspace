use serde::{Deserialize, Serialize};

use super::common::{DateFilter, PaginationResponse, SortDirection};
use super::user::User;

/// Document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default)]
    pub url_id: String,
    pub collection_id: Option<String>,
    pub parent_document_id: Option<String>,
    pub published_at: Option<String>,
    pub archived_at: Option<String>,
    pub deleted_at: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_viewed_at: Option<String>,
    #[serde(default)]
    pub revision: u32,
    #[serde(default)]
    pub full_width: bool,
    #[serde(default)]
    pub template: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<Vec<User>>,
}

/// Document sort options
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DocumentSort {
    Title,
    Index,
    UpdatedAt,
    CreatedAt,
}

impl Default for DocumentSort {
    fn default() -> Self {
        Self::UpdatedAt
    }
}

// ============================================================================
// Request Types
// ============================================================================

/// Request to list documents
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDocumentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlink_document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<SortDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<DocumentSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ListDocumentsRequest {
    pub fn builder() -> ListDocumentsRequestBuilder {
        ListDocumentsRequestBuilder::default()
    }
}

/// Builder for ListDocumentsRequest
#[derive(Debug, Default)]
pub struct ListDocumentsRequestBuilder {
    backlink_document_id: Option<String>,
    collection_id: Option<String>,
    direction: Option<SortDirection>,
    limit: Option<u32>,
    offset: Option<u32>,
    parent_document_id: Option<String>,
    sort: Option<DocumentSort>,
    template: Option<bool>,
    user_id: Option<String>,
}

impl ListDocumentsRequestBuilder {
    pub fn backlink_document_id(mut self, id: String) -> Self {
        self.backlink_document_id = Some(id);
        self
    }

    pub fn collection_id(mut self, id: String) -> Self {
        self.collection_id = Some(id);
        self
    }

    pub fn direction(mut self, direction: SortDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn parent_document_id(mut self, id: String) -> Self {
        self.parent_document_id = Some(id);
        self
    }

    pub fn sort(mut self, sort: DocumentSort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn template(mut self, template: bool) -> Self {
        self.template = Some(template);
        self
    }

    pub fn user_id(mut self, id: String) -> Self {
        self.user_id = Some(id);
        self
    }

    pub fn build(self) -> ListDocumentsRequest {
        ListDocumentsRequest {
            backlink_document_id: self.backlink_document_id,
            collection_id: self.collection_id,
            direction: self.direction,
            limit: self.limit,
            offset: self.offset,
            parent_document_id: self.parent_document_id,
            sort: self.sort,
            template: self.template,
            user_id: self.user_id,
        }
    }
}

/// Request to get document info
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentInfoRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
}

impl DocumentInfoRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            share_id: None,
        }
    }

    pub fn with_share_id(mut self, share_id: String) -> Self {
        self.share_id = Some(share_id);
        self
    }
}

/// Request to create a document
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocumentRequest {
    pub title: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
}

impl CreateDocumentRequest {
    pub fn new(title: String, text: String) -> Self {
        Self {
            title,
            text,
            collection_id: None,
            parent_document_id: None,
            template_id: None,
            template: None,
            emoji: None,
            publish: None,
        }
    }

    pub fn builder() -> CreateDocumentRequestBuilder {
        CreateDocumentRequestBuilder::default()
    }
}

/// Builder for CreateDocumentRequest
#[derive(Debug, Default)]
pub struct CreateDocumentRequestBuilder {
    title: Option<String>,
    text: Option<String>,
    collection_id: Option<String>,
    parent_document_id: Option<String>,
    template_id: Option<String>,
    template: Option<bool>,
    emoji: Option<String>,
    publish: Option<bool>,
}

impl CreateDocumentRequestBuilder {
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn collection_id(mut self, id: String) -> Self {
        self.collection_id = Some(id);
        self
    }

    pub fn parent_document_id(mut self, id: String) -> Self {
        self.parent_document_id = Some(id);
        self
    }

    pub fn template_id(mut self, id: String) -> Self {
        self.template_id = Some(id);
        self
    }

    pub fn template(mut self, template: bool) -> Self {
        self.template = Some(template);
        self
    }

    pub fn emoji(mut self, emoji: String) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn publish(mut self, publish: bool) -> Self {
        self.publish = Some(publish);
        self
    }

    pub fn build(self) -> CreateDocumentRequest {
        CreateDocumentRequest {
            title: self.title.expect("title is required"),
            text: self.text.expect("text is required"),
            collection_id: self.collection_id,
            parent_document_id: self.parent_document_id,
            template_id: self.template_id,
            template: self.template,
            emoji: self.emoji,
            publish: self.publish,
        }
    }
}

/// Request to update a document
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}

impl UpdateDocumentRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            title: None,
            text: None,
            emoji: None,
            append: None,
            publish: None,
            done: None,
        }
    }

    pub fn builder(id: String) -> UpdateDocumentRequestBuilder {
        UpdateDocumentRequestBuilder::new(id)
    }
}

/// Builder for UpdateDocumentRequest
#[derive(Debug)]
pub struct UpdateDocumentRequestBuilder {
    id: String,
    title: Option<String>,
    text: Option<String>,
    emoji: Option<String>,
    append: Option<bool>,
    publish: Option<bool>,
    done: Option<bool>,
}

impl UpdateDocumentRequestBuilder {
    pub fn new(id: String) -> Self {
        Self {
            id,
            title: None,
            text: None,
            emoji: None,
            append: None,
            publish: None,
            done: None,
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn emoji(mut self, emoji: String) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn append(mut self, append: bool) -> Self {
        self.append = Some(append);
        self
    }

    pub fn publish(mut self, publish: bool) -> Self {
        self.publish = Some(publish);
        self
    }

    pub fn done(mut self, done: bool) -> Self {
        self.done = Some(done);
        self
    }

    pub fn build(self) -> UpdateDocumentRequest {
        UpdateDocumentRequest {
            id: self.id,
            title: self.title,
            text: self.text,
            emoji: self.emoji,
            append: self.append,
            publish: self.publish,
            done: self.done,
        }
    }
}

/// Request to delete a document
#[derive(Debug, Clone, Serialize)]
pub struct DeleteDocumentRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

impl DeleteDocumentRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            permanent: None,
        }
    }

    pub fn permanent(mut self, permanent: bool) -> Self {
        self.permanent = Some(permanent);
        self
    }
}

/// Request to search documents
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchDocumentsRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_filter: Option<DateFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_drafts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl SearchDocumentsRequest {
    pub fn new(query: String) -> Self {
        Self {
            query,
            collection_id: None,
            user_id: None,
            date_filter: None,
            include_archived: None,
            include_drafts: None,
            offset: None,
            limit: None,
        }
    }

    pub fn builder(query: String) -> SearchDocumentsRequestBuilder {
        SearchDocumentsRequestBuilder::new(query)
    }
}

/// Builder for SearchDocumentsRequest
#[derive(Debug)]
pub struct SearchDocumentsRequestBuilder {
    query: String,
    collection_id: Option<String>,
    user_id: Option<String>,
    date_filter: Option<DateFilter>,
    include_archived: Option<bool>,
    include_drafts: Option<bool>,
    offset: Option<u32>,
    limit: Option<u32>,
}

impl SearchDocumentsRequestBuilder {
    pub fn new(query: String) -> Self {
        Self {
            query,
            collection_id: None,
            user_id: None,
            date_filter: None,
            include_archived: None,
            include_drafts: None,
            offset: None,
            limit: None,
        }
    }

    pub fn collection_id(mut self, id: String) -> Self {
        self.collection_id = Some(id);
        self
    }

    pub fn user_id(mut self, id: String) -> Self {
        self.user_id = Some(id);
        self
    }

    pub fn date_filter(mut self, filter: DateFilter) -> Self {
        self.date_filter = Some(filter);
        self
    }

    pub fn include_archived(mut self, include: bool) -> Self {
        self.include_archived = Some(include);
        self
    }

    pub fn include_drafts(mut self, include: bool) -> Self {
        self.include_drafts = Some(include);
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

    pub fn build(self) -> SearchDocumentsRequest {
        SearchDocumentsRequest {
            query: self.query,
            collection_id: self.collection_id,
            user_id: self.user_id,
            date_filter: self.date_filter,
            include_archived: self.include_archived,
            include_drafts: self.include_drafts,
            offset: self.offset,
            limit: self.limit,
        }
    }
}

// ============================================================================
// Response Types
// ============================================================================

/// Response from listing documents
#[derive(Debug, Deserialize)]
pub struct ListDocumentsResponse {
    pub data: Vec<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationResponse>,
}

/// Search result item with ranking and context
#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub ranking: f32,
    pub context: String,
    pub document: Document,
}

/// Response from searching documents
#[derive(Debug, Deserialize)]
pub struct SearchDocumentsResponse {
    pub data: Vec<SearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationResponse>,
}

// ============================================================================
// Organization Operations Request Types
// ============================================================================

/// Request to archive a document
#[derive(Debug, Clone, Serialize)]
pub struct ArchiveDocumentRequest {
    pub id: String,
}

impl ArchiveDocumentRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Request to unarchive a document
#[derive(Debug, Clone, Serialize)]
pub struct UnarchiveDocumentRequest {
    pub id: String,
}

impl UnarchiveDocumentRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Request to star a document
#[derive(Debug, Clone, Serialize)]
pub struct StarDocumentRequest {
    pub id: String,
}

impl StarDocumentRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Request to unstar a document
#[derive(Debug, Clone, Serialize)]
pub struct UnstarDocumentRequest {
    pub id: String,
}

impl UnstarDocumentRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Request to unpublish a document
#[derive(Debug, Clone, Serialize)]
pub struct UnpublishDocumentRequest {
    pub id: String,
}

impl UnpublishDocumentRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Request to convert a document into a template
#[derive(Debug, Clone, Serialize)]
pub struct TemplatizeDocumentRequest {
    pub id: String,
}

impl TemplatizeDocumentRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Request to move a document
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveDocumentRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
}

impl MoveDocumentRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            collection_id: None,
            parent_document_id: None,
            index: None,
        }
    }

    pub fn builder(id: String) -> MoveDocumentRequestBuilder {
        MoveDocumentRequestBuilder::new(id)
    }
}

/// Builder for MoveDocumentRequest
#[derive(Debug)]
pub struct MoveDocumentRequestBuilder {
    id: String,
    collection_id: Option<String>,
    parent_document_id: Option<String>,
    index: Option<u32>,
}

impl MoveDocumentRequestBuilder {
    pub fn new(id: String) -> Self {
        Self {
            id,
            collection_id: None,
            parent_document_id: None,
            index: None,
        }
    }

    pub fn collection_id(mut self, id: String) -> Self {
        self.collection_id = Some(id);
        self
    }

    pub fn parent_document_id(mut self, id: String) -> Self {
        self.parent_document_id = Some(id);
        self
    }

    pub fn index(mut self, index: u32) -> Self {
        self.index = Some(index);
        self
    }

    pub fn build(self) -> MoveDocumentRequest {
        MoveDocumentRequest {
            id: self.id,
            collection_id: self.collection_id,
            parent_document_id: self.parent_document_id,
            index: self.index,
        }
    }
}

/// Request to restore a document
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreDocumentRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
}

impl RestoreDocumentRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            revision_id: None,
            collection_id: None,
        }
    }

    pub fn with_revision(mut self, revision_id: String) -> Self {
        self.revision_id = Some(revision_id);
        self
    }

    pub fn with_collection(mut self, collection_id: String) -> Self {
        self.collection_id = Some(collection_id);
        self
    }
}

// ============================================================================
// Listing Variants Request Types
// ============================================================================

/// Request to list recently viewed documents
#[derive(Debug, Clone, Serialize)]
pub struct ViewedDocumentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ViewedDocumentsRequest {
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

impl Default for ViewedDocumentsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request to list draft documents
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl DraftsRequest {
    pub fn new() -> Self {
        Self {
            collection_id: None,
            offset: None,
            limit: None,
        }
    }

    pub fn for_collection(collection_id: String) -> Self {
        Self {
            collection_id: Some(collection_id),
            offset: None,
            limit: None,
        }
    }
}

impl Default for DraftsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request to list template documents
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplatesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl TemplatesRequest {
    pub fn new() -> Self {
        Self {
            collection_id: None,
            offset: None,
            limit: None,
        }
    }

    pub fn for_collection(collection_id: String) -> Self {
        Self {
            collection_id: Some(collection_id),
            offset: None,
            limit: None,
        }
    }
}

impl Default for TemplatesRequest {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Collaboration Operations Request Types
// ============================================================================

/// Request to add a user to a document
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddUserToDocumentRequest {
    pub id: String,
    pub user_id: String,
    pub permission: super::common::Permission,
}

impl AddUserToDocumentRequest {
    pub fn new(id: String, user_id: String, permission: super::common::Permission) -> Self {
        Self {
            id,
            user_id,
            permission,
        }
    }
}

/// Request to remove a user from a document
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveUserFromDocumentRequest {
    pub id: String,
    pub user_id: String,
}

impl RemoveUserFromDocumentRequest {
    pub fn new(id: String, user_id: String) -> Self {
        Self { id, user_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::common::{DateFilter, Permission, SortDirection};

    // ============================================================================
    // Core Request Types Tests
    // ============================================================================

    #[test]
    fn test_list_documents_request_serialization_with_all_fields() {
        let req = ListDocumentsRequest::builder()
            .backlink_document_id("backlink123".to_string())
            .collection_id("col123".to_string())
            .direction(SortDirection::Asc)
            .limit(50)
            .offset(10)
            .parent_document_id("parent456".to_string())
            .sort(DocumentSort::Title)
            .template(true)
            .user_id("user789".to_string())
            .build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"backlinkDocumentId\":\"backlink123\""));
        assert!(json.contains("\"collectionId\":\"col123\""));
        assert!(json.contains("\"direction\":\"asc\""));
        assert!(json.contains("\"limit\":50"));
        assert!(json.contains("\"offset\":10"));
        assert!(json.contains("\"parentDocumentId\":\"parent456\""));
        assert!(json.contains("\"sort\":\"title\""));
        assert!(json.contains("\"template\":true"));
        assert!(json.contains("\"userId\":\"user789\""));
    }

    #[test]
    fn test_list_documents_request_builder_empty() {
        let req = ListDocumentsRequest::builder().build();
        let json = serde_json::to_string(&req).unwrap();
        // Empty request should serialize to {}
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_list_documents_request_builder_partial() {
        let req = ListDocumentsRequest::builder()
            .collection_id("col123".to_string())
            .limit(25)
            .build();

        assert_eq!(req.collection_id, Some("col123".to_string()));
        assert_eq!(req.limit, Some(25));
        assert_eq!(req.offset, None);
        assert_eq!(req.sort, None);
    }

    #[test]
    fn test_list_documents_request_optional_fields_omitted() {
        let req = ListDocumentsRequest::builder()
            .collection_id("col123".to_string())
            .build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"collectionId\":\"col123\""));
        assert!(!json.contains("limit"));
        assert!(!json.contains("offset"));
        assert!(!json.contains("sort"));
    }

    #[test]
    fn test_document_info_request_new() {
        let req = DocumentInfoRequest::new("doc123".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(!json.contains("shareId"));
    }

    #[test]
    fn test_document_info_request_with_share_id() {
        let req = DocumentInfoRequest::new("doc123".to_string())
            .with_share_id("share456".to_string());

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(json.contains("\"shareId\":\"share456\""));
    }

    #[test]
    fn test_create_document_request_new() {
        let req = CreateDocumentRequest::new("Test Title".to_string(), "Test Content".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"title\":\"Test Title\""));
        assert!(json.contains("\"text\":\"Test Content\""));
        assert!(!json.contains("collectionId"));
        assert!(!json.contains("emoji"));
    }

    #[test]
    fn test_create_document_request_builder_all_fields() {
        let req = CreateDocumentRequest::builder()
            .title("New Doc".to_string())
            .text("Content here".to_string())
            .collection_id("col789".to_string())
            .parent_document_id("parent123".to_string())
            .template_id("template456".to_string())
            .template(true)
            .emoji("📄".to_string())
            .publish(true)
            .build();

        assert_eq!(req.title, "New Doc");
        assert_eq!(req.text, "Content here");
        assert_eq!(req.collection_id, Some("col789".to_string()));
        assert_eq!(req.parent_document_id, Some("parent123".to_string()));
        assert_eq!(req.template_id, Some("template456".to_string()));
        assert_eq!(req.template, Some(true));
        assert_eq!(req.emoji, Some("📄".to_string()));
        assert_eq!(req.publish, Some(true));
    }

    #[test]
    fn test_create_document_request_builder_camel_case() {
        let req = CreateDocumentRequest::builder()
            .title("Title".to_string())
            .text("Text".to_string())
            .collection_id("col123".to_string())
            .parent_document_id("parent456".to_string())
            .template_id("tpl789".to_string())
            .build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"collectionId\":\"col123\""));
        assert!(json.contains("\"parentDocumentId\":\"parent456\""));
        assert!(json.contains("\"templateId\":\"tpl789\""));
    }

    #[test]
    #[should_panic(expected = "title is required")]
    fn test_create_document_request_builder_missing_title() {
        CreateDocumentRequest::builder()
            .text("Content".to_string())
            .build();
    }

    #[test]
    #[should_panic(expected = "text is required")]
    fn test_create_document_request_builder_missing_text() {
        CreateDocumentRequest::builder()
            .title("Title".to_string())
            .build();
    }

    #[test]
    fn test_update_document_request_new() {
        let req = UpdateDocumentRequest::new("doc123".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(!json.contains("title"));
        assert!(!json.contains("text"));
    }

    #[test]
    fn test_update_document_request_builder_all_fields() {
        let req = UpdateDocumentRequest::builder("doc123".to_string())
            .title("Updated Title".to_string())
            .text("Updated Content".to_string())
            .emoji("✏️".to_string())
            .append(true)
            .publish(true)
            .done(true)
            .build();

        assert_eq!(req.id, "doc123");
        assert_eq!(req.title, Some("Updated Title".to_string()));
        assert_eq!(req.text, Some("Updated Content".to_string()));
        assert_eq!(req.emoji, Some("✏️".to_string()));
        assert_eq!(req.append, Some(true));
        assert_eq!(req.publish, Some(true));
        assert_eq!(req.done, Some(true));
    }

    #[test]
    fn test_update_document_request_builder_partial() {
        let req = UpdateDocumentRequest::builder("doc123".to_string())
            .title("New Title".to_string())
            .build();

        assert_eq!(req.id, "doc123");
        assert_eq!(req.title, Some("New Title".to_string()));
        assert_eq!(req.text, None);
        assert_eq!(req.emoji, None);
    }

    #[test]
    fn test_update_document_request_camel_case() {
        let req = UpdateDocumentRequest::builder("doc123".to_string())
            .title("Title".to_string())
            .append(true)
            .build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(json.contains("\"title\":\"Title\""));
        assert!(json.contains("\"append\":true"));
    }

    #[test]
    fn test_delete_document_request_new() {
        let req = DeleteDocumentRequest::new("doc123".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(!json.contains("permanent"));
    }

    #[test]
    fn test_delete_document_request_permanent() {
        let req = DeleteDocumentRequest::new("doc123".to_string()).permanent(true);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(json.contains("\"permanent\":true"));
    }

    #[test]
    fn test_search_documents_request_new() {
        let req = SearchDocumentsRequest::new("search query".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"query\":\"search query\""));
        assert!(!json.contains("collectionId"));
        assert!(!json.contains("userId"));
    }

    #[test]
    fn test_search_documents_request_builder_all_fields() {
        let req = SearchDocumentsRequest::builder("test query".to_string())
            .collection_id("col123".to_string())
            .user_id("user456".to_string())
            .date_filter(DateFilter::Week)
            .include_archived(true)
            .include_drafts(false)
            .offset(20)
            .limit(100)
            .build();

        assert_eq!(req.query, "test query");
        assert_eq!(req.collection_id, Some("col123".to_string()));
        assert_eq!(req.user_id, Some("user456".to_string()));
        assert!(matches!(req.date_filter, Some(DateFilter::Week)));
        assert_eq!(req.include_archived, Some(true));
        assert_eq!(req.include_drafts, Some(false));
        assert_eq!(req.offset, Some(20));
        assert_eq!(req.limit, Some(100));
    }

    #[test]
    fn test_search_documents_request_camel_case() {
        let req = SearchDocumentsRequest::builder("query".to_string())
            .collection_id("col123".to_string())
            .date_filter(DateFilter::Month)
            .include_archived(true)
            .include_drafts(true)
            .build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"collectionId\":\"col123\""));
        assert!(json.contains("\"dateFilter\":\"month\""));
        assert!(json.contains("\"includeArchived\":true"));
        assert!(json.contains("\"includeDrafts\":true"));
    }

    // ============================================================================
    // Organization Operations Tests
    // ============================================================================

    #[test]
    fn test_archive_document_request_serialization() {
        let req = ArchiveDocumentRequest::new("doc123".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
    }

    #[test]
    fn test_unarchive_document_request_serialization() {
        let req = UnarchiveDocumentRequest::new("doc456".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc456\""));
    }

    #[test]
    fn test_star_document_request_serialization() {
        let req = StarDocumentRequest::new("doc789".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc789\""));
    }

    #[test]
    fn test_unstar_document_request_serialization() {
        let req = UnstarDocumentRequest::new("doc012".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc012\""));
    }

    #[test]
    fn test_unpublish_document_request_serialization() {
        let req = UnpublishDocumentRequest::new("doc345".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc345\""));
    }

    #[test]
    fn test_templatize_document_request_serialization() {
        let req = TemplatizeDocumentRequest::new("doc678".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc678\""));
    }

    #[test]
    fn test_move_document_request_new() {
        let req = MoveDocumentRequest::new("doc123".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(!json.contains("collectionId"));
        assert!(!json.contains("parentDocumentId"));
        assert!(!json.contains("index"));
    }

    #[test]
    fn test_move_document_request_builder_all_fields() {
        let req = MoveDocumentRequest::builder("doc123".to_string())
            .collection_id("col456".to_string())
            .parent_document_id("parent789".to_string())
            .index(5)
            .build();

        assert_eq!(req.id, "doc123");
        assert_eq!(req.collection_id, Some("col456".to_string()));
        assert_eq!(req.parent_document_id, Some("parent789".to_string()));
        assert_eq!(req.index, Some(5));
    }

    #[test]
    fn test_move_document_request_builder_partial() {
        let req = MoveDocumentRequest::builder("doc123".to_string())
            .collection_id("col456".to_string())
            .index(3)
            .build();

        assert_eq!(req.id, "doc123");
        assert_eq!(req.collection_id, Some("col456".to_string()));
        assert_eq!(req.parent_document_id, None);
        assert_eq!(req.index, Some(3));
    }

    #[test]
    fn test_move_document_request_camel_case() {
        let req = MoveDocumentRequest::builder("doc123".to_string())
            .collection_id("col456".to_string())
            .parent_document_id("parent789".to_string())
            .build();

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"collectionId\":\"col456\""));
        assert!(json.contains("\"parentDocumentId\":\"parent789\""));
    }

    #[test]
    fn test_restore_document_request_new() {
        let req = RestoreDocumentRequest::new("doc123".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(!json.contains("revisionId"));
        assert!(!json.contains("collectionId"));
    }

    #[test]
    fn test_restore_document_request_with_revision() {
        let req = RestoreDocumentRequest::new("doc123".to_string())
            .with_revision("rev456".to_string());

        assert_eq!(req.id, "doc123");
        assert_eq!(req.revision_id, Some("rev456".to_string()));
        assert_eq!(req.collection_id, None);
    }

    #[test]
    fn test_restore_document_request_with_collection() {
        let req = RestoreDocumentRequest::new("doc123".to_string())
            .with_collection("col789".to_string());

        assert_eq!(req.id, "doc123");
        assert_eq!(req.revision_id, None);
        assert_eq!(req.collection_id, Some("col789".to_string()));
    }

    #[test]
    fn test_restore_document_request_with_both() {
        let req = RestoreDocumentRequest::new("doc123".to_string())
            .with_revision("rev456".to_string())
            .with_collection("col789".to_string());

        assert_eq!(req.id, "doc123");
        assert_eq!(req.revision_id, Some("rev456".to_string()));
        assert_eq!(req.collection_id, Some("col789".to_string()));
    }

    #[test]
    fn test_restore_document_request_camel_case() {
        let req = RestoreDocumentRequest::new("doc123".to_string())
            .with_revision("rev456".to_string())
            .with_collection("col789".to_string());

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"revisionId\":\"rev456\""));
        assert!(json.contains("\"collectionId\":\"col789\""));
    }

    // ============================================================================
    // Listing Variants Tests
    // ============================================================================

    #[test]
    fn test_viewed_documents_request_new() {
        let req = ViewedDocumentsRequest::new();
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_viewed_documents_request_default() {
        let req = ViewedDocumentsRequest::default();
        assert_eq!(req.offset, None);
        assert_eq!(req.limit, None);
    }

    #[test]
    fn test_viewed_documents_request_with_pagination() {
        let req = ViewedDocumentsRequest::with_pagination(10, 50);
        assert_eq!(req.offset, Some(10));
        assert_eq!(req.limit, Some(50));

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"offset\":10"));
        assert!(json.contains("\"limit\":50"));
    }

    #[test]
    fn test_drafts_request_new() {
        let req = DraftsRequest::new();
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_drafts_request_default() {
        let req = DraftsRequest::default();
        assert_eq!(req.collection_id, None);
        assert_eq!(req.offset, None);
        assert_eq!(req.limit, None);
    }

    #[test]
    fn test_drafts_request_for_collection() {
        let req = DraftsRequest::for_collection("col123".to_string());
        assert_eq!(req.collection_id, Some("col123".to_string()));
        assert_eq!(req.offset, None);
        assert_eq!(req.limit, None);
    }

    #[test]
    fn test_drafts_request_camel_case() {
        let req = DraftsRequest::for_collection("col123".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"collectionId\":\"col123\""));
    }

    #[test]
    fn test_templates_request_new() {
        let req = TemplatesRequest::new();
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_templates_request_default() {
        let req = TemplatesRequest::default();
        assert_eq!(req.collection_id, None);
        assert_eq!(req.offset, None);
        assert_eq!(req.limit, None);
    }

    #[test]
    fn test_templates_request_for_collection() {
        let req = TemplatesRequest::for_collection("col456".to_string());
        assert_eq!(req.collection_id, Some("col456".to_string()));
        assert_eq!(req.offset, None);
        assert_eq!(req.limit, None);
    }

    #[test]
    fn test_templates_request_camel_case() {
        let req = TemplatesRequest::for_collection("col456".to_string());
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"collectionId\":\"col456\""));
    }

    // ============================================================================
    // Collaboration Tests
    // ============================================================================

    #[test]
    fn test_add_user_to_document_request_new() {
        let req = AddUserToDocumentRequest::new(
            "doc123".to_string(),
            "user456".to_string(),
            Permission::Read,
        );

        assert_eq!(req.id, "doc123");
        assert_eq!(req.user_id, "user456");
        assert!(matches!(req.permission, Permission::Read));
    }

    #[test]
    fn test_add_user_to_document_request_serialization() {
        let req = AddUserToDocumentRequest::new(
            "doc123".to_string(),
            "user456".to_string(),
            Permission::ReadWrite,
        );

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(json.contains("\"userId\":\"user456\""));
        assert!(json.contains("\"permission\":\"read_write\""));
    }

    #[test]
    fn test_add_user_to_document_request_camel_case() {
        let req = AddUserToDocumentRequest::new(
            "doc123".to_string(),
            "user456".to_string(),
            Permission::Read,
        );

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"userId\":\"user456\""));
    }

    #[test]
    fn test_remove_user_from_document_request_new() {
        let req = RemoveUserFromDocumentRequest::new("doc123".to_string(), "user789".to_string());

        assert_eq!(req.id, "doc123");
        assert_eq!(req.user_id, "user789");
    }

    #[test]
    fn test_remove_user_from_document_request_serialization() {
        let req = RemoveUserFromDocumentRequest::new("doc123".to_string(), "user789".to_string());

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(json.contains("\"userId\":\"user789\""));
    }

    #[test]
    fn test_remove_user_from_document_request_camel_case() {
        let req = RemoveUserFromDocumentRequest::new("doc123".to_string(), "user789".to_string());

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"userId\":\"user789\""));
    }

    // ============================================================================
    // DocumentSort Tests
    // ============================================================================

    #[test]
    fn test_document_sort_default() {
        let sort = DocumentSort::default();
        assert!(matches!(sort, DocumentSort::UpdatedAt));
    }

    #[test]
    fn test_document_sort_serialization() {
        let sort_title = DocumentSort::Title;
        let sort_index = DocumentSort::Index;
        let sort_updated = DocumentSort::UpdatedAt;
        let sort_created = DocumentSort::CreatedAt;

        assert_eq!(
            serde_json::to_string(&sort_title).unwrap(),
            "\"title\""
        );
        assert_eq!(
            serde_json::to_string(&sort_index).unwrap(),
            "\"index\""
        );
        assert_eq!(
            serde_json::to_string(&sort_updated).unwrap(),
            "\"updatedAt\""
        );
        assert_eq!(
            serde_json::to_string(&sort_created).unwrap(),
            "\"createdAt\""
        );
    }

    // ============================================================================
    // Edge Cases and Integration Tests
    // ============================================================================

    #[test]
    fn test_list_documents_request_builder_method_chaining() {
        let req = ListDocumentsRequest::builder()
            .collection_id("col1".to_string())
            .limit(10)
            .offset(5)
            .sort(DocumentSort::Title)
            .direction(SortDirection::Asc)
            .build();

        assert_eq!(req.collection_id, Some("col1".to_string()));
        assert_eq!(req.limit, Some(10));
        assert_eq!(req.offset, Some(5));
        assert!(matches!(req.sort, Some(DocumentSort::Title)));
        assert!(matches!(req.direction, Some(SortDirection::Asc)));
    }

    #[test]
    fn test_create_document_builder_method_chaining() {
        let req = CreateDocumentRequest::builder()
            .title("Title".to_string())
            .text("Text".to_string())
            .emoji("📝".to_string())
            .publish(true)
            .template(false)
            .build();

        assert_eq!(req.title, "Title");
        assert_eq!(req.text, "Text");
        assert_eq!(req.emoji, Some("📝".to_string()));
        assert_eq!(req.publish, Some(true));
        assert_eq!(req.template, Some(false));
    }

    #[test]
    fn test_update_document_builder_method_chaining() {
        let req = UpdateDocumentRequest::builder("doc123".to_string())
            .title("New Title".to_string())
            .text("New Text".to_string())
            .append(false)
            .done(true)
            .build();

        assert_eq!(req.id, "doc123");
        assert_eq!(req.title, Some("New Title".to_string()));
        assert_eq!(req.text, Some("New Text".to_string()));
        assert_eq!(req.append, Some(false));
        assert_eq!(req.done, Some(true));
    }

    #[test]
    fn test_search_documents_builder_method_chaining() {
        let req = SearchDocumentsRequest::builder("test".to_string())
            .collection_id("col1".to_string())
            .user_id("user1".to_string())
            .include_archived(true)
            .include_drafts(false)
            .limit(50)
            .build();

        assert_eq!(req.query, "test");
        assert_eq!(req.collection_id, Some("col1".to_string()));
        assert_eq!(req.user_id, Some("user1".to_string()));
        assert_eq!(req.include_archived, Some(true));
        assert_eq!(req.include_drafts, Some(false));
        assert_eq!(req.limit, Some(50));
    }

    #[test]
    fn test_move_document_builder_method_chaining() {
        let req = MoveDocumentRequest::builder("doc1".to_string())
            .collection_id("col1".to_string())
            .parent_document_id("parent1".to_string())
            .index(10)
            .build();

        assert_eq!(req.id, "doc1");
        assert_eq!(req.collection_id, Some("col1".to_string()));
        assert_eq!(req.parent_document_id, Some("parent1".to_string()));
        assert_eq!(req.index, Some(10));
    }

    #[test]
    fn test_restore_document_method_chaining() {
        let req = RestoreDocumentRequest::new("doc1".to_string())
            .with_revision("rev1".to_string())
            .with_collection("col1".to_string());

        assert_eq!(req.id, "doc1");
        assert_eq!(req.revision_id, Some("rev1".to_string()));
        assert_eq!(req.collection_id, Some("col1".to_string()));
    }

    #[test]
    fn test_delete_document_method_chaining() {
        let req = DeleteDocumentRequest::new("doc1".to_string()).permanent(true);

        assert_eq!(req.id, "doc1");
        assert_eq!(req.permanent, Some(true));
    }

    #[test]
    fn test_document_info_method_chaining() {
        let req = DocumentInfoRequest::new("doc1".to_string())
            .with_share_id("share1".to_string());

        assert_eq!(req.id, "doc1");
        assert_eq!(req.share_id, Some("share1".to_string()));
    }

    #[test]
    fn test_empty_optional_fields_serialization() {
        // Test that None values are properly skipped in serialization
        let req = UpdateDocumentRequest::new("doc123".to_string());
        let json = serde_json::to_string(&req).unwrap();

        // Should only contain the id field
        assert!(json.contains("\"id\":\"doc123\""));
        assert!(!json.contains("title"));
        assert!(!json.contains("text"));
        assert!(!json.contains("emoji"));
        assert!(!json.contains("append"));
        assert!(!json.contains("publish"));
        assert!(!json.contains("done"));
    }

    #[test]
    fn test_all_organization_operations_have_id() {
        // Ensure all organization operation requests have the id field
        let archive = ArchiveDocumentRequest::new("id".to_string());
        let unarchive = UnarchiveDocumentRequest::new("id".to_string());
        let star = StarDocumentRequest::new("id".to_string());
        let unstar = UnstarDocumentRequest::new("id".to_string());
        let unpublish = UnpublishDocumentRequest::new("id".to_string());
        let templatize = TemplatizeDocumentRequest::new("id".to_string());

        assert_eq!(archive.id, "id");
        assert_eq!(unarchive.id, "id");
        assert_eq!(star.id, "id");
        assert_eq!(unstar.id, "id");
        assert_eq!(unpublish.id, "id");
        assert_eq!(templatize.id, "id");
    }
}
