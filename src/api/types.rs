use serde::{Deserialize, Serialize};

/// Standard API response wrapper
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
}

/// Pagination information
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: u32,
    pub limit: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 25,
        }
    }
}

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
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<User>,
}

/// User structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

/// Collection structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Request types

/// Request to list documents
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlink_document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Response from listing documents
#[derive(Debug, Deserialize)]
pub struct DocumentsListResponse {
    pub data: Vec<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationResponse>,
}

/// Pagination response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PaginationResponse {
    pub limit: u32,
    pub offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_path: Option<String>,
}

/// Request to get document info
#[derive(Debug, Serialize)]
pub struct DocumentInfoRequest {
    pub id: String,
}

/// Request to create a document
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentCreateRequest {
    pub title: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
}

/// Request to update a document
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentUpdateRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}

/// Request to delete a document
#[derive(Debug, Serialize)]
pub struct DocumentDeleteRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

/// Request to search documents
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSearchRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Search result item with ranking and context
#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub ranking: f32,
    #[allow(dead_code)]
    pub context: String,
    pub document: Document,
}

/// Response from searching documents
#[derive(Debug, Deserialize)]
pub struct DocumentSearchResponse {
    pub data: Vec<SearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationResponse>,
}

/// Request to list collections
#[derive(Debug, Serialize)]
pub struct CollectionsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response from listing collections
#[derive(Debug, Deserialize)]
pub struct CollectionsListResponse {
    pub data: Vec<Collection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(dead_code)]
    pub pagination: Option<PaginationResponse>,
}
