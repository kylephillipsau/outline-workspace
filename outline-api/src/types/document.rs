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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
