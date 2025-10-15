use serde::{Deserialize, Serialize};

/// Standard API response wrapper
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
}

/// Pagination information for requests
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Pagination response from API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationResponse {
    pub limit: u32,
    pub offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_path: Option<String>,
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::Desc
    }
}

/// Export format for documents and collections
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Markdown,
    Html,
    Pdf,
}

/// Import format for documents and collections
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImportFormat {
    Markdown,
    Html,
    Docx,
    Notion,
    Confluence,
}

/// Permission level for documents and collections
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    Read,
    ReadWrite,
}

/// Collection permission level
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionPermission {
    Read,
    ReadWrite,
}

impl Default for CollectionPermission {
    fn default() -> Self {
        Self::ReadWrite
    }
}

/// User role in the team
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Member,
    Viewer,
}

/// Date filter for search queries
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DateFilter {
    Day,
    Week,
    Month,
    Year,
}

/// User filter for listing users
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserFilter {
    All,
    Admins,
    Suspended,
    Active,
    Invited,
}

/// User sort field
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserSort {
    Name,
    Email,
    LastActive,
    CreatedAt,
}
