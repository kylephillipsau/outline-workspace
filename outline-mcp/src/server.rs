use anyhow::{Context, Result};
use outline_api::{
    OutlineClient,
    CreateDocumentRequest, UpdateDocumentRequest, SearchDocumentsRequest,
    ListDocumentsRequest, ListCollectionsRequest,
};
use rmcp::{
    model::*,
    tool_router, tool, tool_handler,
    handler::server::{ServerHandler, tool::ToolRouter, wrapper::Parameters},
    ServiceExt,
};
use rmcp::model::{ErrorCode, ServerInfo, ServerCapabilities, ProtocolVersion, Implementation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ============================================================================
// Search Response Types (optimized for context limits)
// ============================================================================

/// Maximum length for document text snippets in search results
const SNIPPET_MAX_LENGTH: usize = 300;

/// A summarized search result optimized for LLM context limits.
/// Use outline_documents_get to retrieve full document content.
#[derive(Debug, Serialize)]
pub struct SearchResultSummary {
    /// Document ID - use with outline_documents_get for full content
    pub id: String,
    /// Document title
    pub title: String,
    /// Relevance score (higher is more relevant)
    pub ranking: f32,
    /// Text snippet showing where the query matched
    pub context: String,
    /// Truncated preview of document content (first 300 chars)
    pub snippet: String,
    /// Full document text (only included if include_content=true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Collection containing this document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// Last updated timestamp
    pub updated_at: String,
}

/// Optimized search response with pagination info
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    /// Search results with summarized document info
    pub results: Vec<SearchResultSummary>,
    /// Total number of results (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    /// Whether more results are available
    pub has_more: bool,
    /// Offset for next page (use as offset parameter to get more)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<u32>,
}

use crate::config::Config;

/// Main MCP server for Outline operations
#[derive(Clone)]
pub struct OutlineServer {
    client: Arc<OutlineClient>,
    tool_router: ToolRouter<Self>,
}

// ============================================================================
// Tool Input Types
// ============================================================================

/// Parameters for listing documents
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListDocumentsParams {
    /// Filter to a specific collection. Get IDs from outline_collections_list.
    #[serde(rename = "collectionId")]
    pub collection_id: Option<String>,
    /// Maximum documents to return (default: 25)
    pub limit: Option<u32>,
    /// Number of documents to skip for pagination
    pub offset: Option<u32>,
}

/// Parameters for getting a single document
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDocumentParams {
    /// The document's unique identifier (UUID)
    pub id: String,
}

/// Parameters for creating a new document
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateDocumentParams {
    /// Document title (appears in navigation and search)
    pub title: String,
    /// Document content in Markdown format
    pub text: String,
    /// Place in a specific collection (get IDs from outline_collections_list)
    #[serde(rename = "collectionId")]
    pub collection_id: Option<String>,
    /// Nest under another document for hierarchical structure
    #[serde(rename = "parentDocumentId")]
    pub parent_document_id: Option<String>,
    /// Set true to publish immediately, false (default) creates draft
    pub publish: Option<bool>,
}

/// Parameters for updating an existing document
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateDocumentParams {
    /// Document identifier to update
    pub id: String,
    /// New document title (optional)
    pub title: Option<String>,
    /// New document content in Markdown (replaces entire content)
    pub text: Option<String>,
    /// Set true to publish a draft document
    pub publish: Option<bool>,
}

/// Parameters for deleting a document
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteDocumentParams {
    /// Document identifier to delete
    pub id: String,
    /// Set true for permanent deletion (cannot be undone). Default: false (moves to trash)
    pub permanent: Option<bool>,
}

/// Parameters for searching documents
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchDocumentsParams {
    /// Search terms (searches titles and content)
    pub query: String,
    /// Limit search to a specific collection
    #[serde(rename = "collectionId")]
    pub collection_id: Option<String>,
    /// Maximum results to return (default: 25, max: 100)
    pub limit: Option<u32>,
    /// Number of results to skip for pagination
    pub offset: Option<u32>,
    /// Include full document text in results (default: false).
    /// When false, only a 300-char snippet is returned to reduce context size.
    /// Use outline_documents_get to retrieve full content for specific documents.
    #[serde(default, rename = "includeContent")]
    pub include_content: bool,
}

/// Parameters for listing collections
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCollectionsParams {
    /// Maximum collections to return (default: 25)
    pub limit: Option<u32>,
    /// Number of collections to skip for pagination
    pub offset: Option<u32>,
}

/// Parameters for getting a single collection
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCollectionParams {
    /// The collection's unique identifier (UUID)
    pub id: String,
}

// ============================================================================
// Server Implementation
// ============================================================================

#[tool_router]
impl OutlineServer {
    /// Create a new Outline MCP server
    pub async fn new() -> Result<Self> {
        tracing::info!("Initializing Outline MCP server");

        // Load config to get instance URL
        let config = Config::load()?;
        let api_base_url = config.get_api_base_url()
            .context("Failed to get API base URL from config")?;

        tracing::info!("Using API base URL: {}", api_base_url);

        // Create client with auto auth (uses keyring)
        let client = OutlineClient::new(api_base_url)?;

        // Test authentication
        match outline_api::auth::get_access_token().await {
            Ok(_) => tracing::info!("Authentication configured"),
            Err(e) => {
                tracing::warn!("No authentication found: {}. Tools will fail until you run 'outline-cli auth set-token'", e);
            }
        }

        Ok(Self {
            client: Arc::new(client),
            tool_router: Self::tool_router(),
        })
    }

    // ========================================================================
    // Document Tools
    // ========================================================================

    /// List documents in your Outline workspace.
    ///
    /// Use this tool to browse and discover documents. Supports pagination for large workspaces.
    ///
    /// Parameters:
    /// - collectionId (optional): Filter documents to a specific collection. Get collection IDs from outline_collections_list.
    /// - limit (optional): Maximum number of documents to return. Default is 25.
    /// - offset (optional): Number of documents to skip for pagination. Use with limit for paging.
    ///
    /// Returns: JSON array of document objects, each containing:
    /// - id: Unique document identifier (use with outline_documents_get)
    /// - title: Document title
    /// - emoji: Document emoji icon (if set)
    /// - createdAt/updatedAt: ISO 8601 timestamps
    /// - publishedAt: When the document was published (null if draft)
    /// - collectionId: ID of the containing collection
    ///
    /// Example usage patterns:
    /// - List all documents: Call with no parameters
    /// - List documents in a collection: {"collectionId": "abc123"}
    /// - Paginate results: {"limit": 10, "offset": 20}
    #[tool(annotations(
        title = "List Documents",
        read_only_hint = true,
        destructive_hint = false,
        idempotent_hint = true,
        open_world_hint = false
    ))]
    async fn outline_documents_list(
        &self,
        params: Parameters<ListDocumentsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        let request = ListDocumentsRequest {
            backlink_document_id: None,
            collection_id: params.collection_id,
            direction: None,
            limit: params.limit,
            offset: params.offset,
            parent_document_id: None,
            sort: None,
            template: None,
            user_id: None,
        };

        let response = self.client.list_documents(request).await
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Get a specific document by ID.
    ///
    /// Retrieves the complete document including full text content. Use this after finding
    /// a document via outline_documents_list or outline_documents_search.
    ///
    /// Parameters:
    /// - id (required): The document's unique identifier (UUID format)
    ///
    /// Returns: JSON object containing:
    /// - id: Document identifier
    /// - title: Document title
    /// - text: Full document content in Markdown format
    /// - emoji: Document emoji icon (if set)
    /// - createdAt/updatedAt: ISO 8601 timestamps
    /// - createdBy: User object of document creator
    /// - collectionId: ID of the containing collection
    /// - parentDocumentId: ID of parent document (if nested)
    /// - publishedAt: Publication timestamp (null if draft)
    /// - revision: Version number for conflict detection
    ///
    /// Example: {"id": "abc123-def456-..."}
    #[tool(annotations(
        title = "Get Document",
        read_only_hint = true,
        destructive_hint = false,
        idempotent_hint = true,
        open_world_hint = false
    ))]
    async fn outline_documents_get(
        &self,
        params: Parameters<GetDocumentParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        let document = self.client.get_document(params.id).await
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        let json = serde_json::to_string_pretty(&document)
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Create a new document in Outline.
    ///
    /// Creates a new document with the specified title and content. Documents can be created
    /// as drafts or published immediately. Can be placed in a collection or nested under
    /// a parent document.
    ///
    /// Parameters:
    /// - title (required): Document title (appears in navigation and search)
    /// - text (required): Document content in Markdown format. Supports headings, lists, code blocks, links, images, and tables.
    /// - collectionId (optional): Place document in a specific collection. Get IDs from outline_collections_list.
    /// - parentDocumentId (optional): Nest under another document. Creates hierarchical structure.
    /// - publish (optional): Set to true to publish immediately. Default false creates a draft.
    ///
    /// Returns: JSON object containing the created document with:
    /// - id: New document's unique identifier
    /// - title, text, emoji, collectionId, parentDocumentId
    /// - createdAt/updatedAt: ISO 8601 timestamps
    /// - revision: Initial version number (1)
    ///
    /// Example - Create draft: {"title": "Meeting Notes", "text": "# Meeting Notes\n\n## Attendees\n..."}
    /// Example - Create in collection: {"title": "API Docs", "text": "...", "collectionId": "abc123", "publish": true}
    #[tool(annotations(
        title = "Create Document",
        read_only_hint = false,
        destructive_hint = false,
        idempotent_hint = false,
        open_world_hint = false
    ))]
    async fn outline_documents_create(
        &self,
        params: Parameters<CreateDocumentParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        let request = CreateDocumentRequest {
            title: params.title,
            text: params.text,
            collection_id: params.collection_id,
            parent_document_id: params.parent_document_id,
            template_id: None,
            template: None,
            emoji: None,
            publish: params.publish,
        };

        let document = self.client.create_document(request).await
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        let json = serde_json::to_string_pretty(&document)
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Update an existing document.
    ///
    /// Modifies an existing document's title, content, or publish status. All update fields
    /// are optional - only specified fields will be changed. Use outline_documents_get first
    /// to retrieve current content if making partial text edits.
    ///
    /// Parameters:
    /// - id (required): Document identifier to update
    /// - title (optional): New document title
    /// - text (optional): New document content in Markdown format. Replaces entire content.
    /// - publish (optional): Set to true to publish a draft document
    ///
    /// Returns: JSON object containing the updated document with all fields
    ///
    /// Example - Update title: {"id": "abc123", "title": "New Title"}
    /// Example - Update content: {"id": "abc123", "text": "# Updated Content\n\nNew text here..."}
    /// Example - Publish draft: {"id": "abc123", "publish": true}
    /// Example - Full update: {"id": "abc123", "title": "New Title", "text": "New content", "publish": true}
    #[tool(annotations(
        title = "Update Document",
        read_only_hint = false,
        destructive_hint = false,
        idempotent_hint = true,
        open_world_hint = false
    ))]
    async fn outline_documents_update(
        &self,
        params: Parameters<UpdateDocumentParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        // Log the text being updated to help debug escaping issues
        if let Some(ref text) = params.text {
            tracing::debug!("Updating document {} with text length: {}", params.id, text.len());
            // Log first 200 chars to see if brackets are escaped
            let preview = if text.len() > 200 {
                format!("{}...", &text[..200])
            } else {
                text.clone()
            };
            tracing::info!("Text preview:\n{}", preview);

            // Check if brackets are escaped
            if text.contains("!\\[") {
                tracing::warn!("WARNING: Text contains escaped brackets !\\[");
            }
        }

        let request = UpdateDocumentRequest {
            id: params.id,
            title: params.title.clone(),
            text: params.text.clone(),
            emoji: None,
            append: None,
            publish: params.publish,
            done: None,
        };

        let document = self.client.update_document(request).await
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        // Log what was returned after update
        tracing::debug!("Document updated. Text length: {}", document.text.len());
        if document.text.contains("!\\[") {
            tracing::warn!("WARNING: Returned text contains escaped brackets !\\[");
        }

        let json = serde_json::to_string_pretty(&document)
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Delete a document from Outline.
    ///
    /// Removes a document from the workspace. By default, documents are moved to trash
    /// where they can be recovered. Use permanent=true for irreversible deletion.
    ///
    /// WARNING: Permanent deletion cannot be undone. The document and all its history
    /// will be lost forever.
    ///
    /// Parameters:
    /// - id (required): Document identifier to delete
    /// - permanent (optional): Set to true for permanent deletion. Default false moves to trash.
    ///
    /// Returns: Success message string:
    /// - "Document moved to trash" (default behavior)
    /// - "Document permanently deleted" (when permanent=true)
    ///
    /// Example - Move to trash: {"id": "abc123"}
    /// Example - Permanent delete: {"id": "abc123", "permanent": true}
    #[tool(annotations(
        title = "Delete Document",
        read_only_hint = false,
        destructive_hint = true,
        idempotent_hint = true,
        open_world_hint = false
    ))]
    async fn outline_documents_delete(
        &self,
        params: Parameters<DeleteDocumentParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;
        let permanent = params.permanent.unwrap_or(false);

        self.client.delete_document(params.id, permanent).await
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        let message = if permanent {
            "Document permanently deleted"
        } else {
            "Document moved to trash"
        };

        Ok(CallToolResult::success(vec![Content::text(message.to_string())]))
    }

    /// Search for documents by text query.
    ///
    /// Full-text search across all documents in your Outline workspace. Searches document
    /// titles and content. Returns ranked results with context snippets.
    ///
    /// **Context Optimization**: By default, only document snippets (300 chars) are returned
    /// to reduce context size. Use outline_documents_get to retrieve full content for
    /// specific documents of interest.
    ///
    /// Parameters:
    /// - query (required): Search terms. Supports natural language queries.
    /// - collectionId (optional): Limit search to a specific collection
    /// - limit (optional): Maximum results to return. Default 25, max 100.
    /// - offset (optional): Number of results to skip for pagination.
    /// - includeContent (optional): Set true to include full document text. Default false.
    ///
    /// Returns: JSON object with:
    /// - results: Array of search results, each containing:
    ///   - id: Document ID (use with outline_documents_get for full content)
    ///   - title: Document title
    ///   - ranking: Relevance score (higher = more relevant)
    ///   - context: Text snippet showing where query matched
    ///   - snippet: First 300 chars of document (always included)
    ///   - text: Full document content (only if includeContent=true)
    ///   - collection_id: Parent collection ID
    ///   - updated_at: Last modified timestamp
    /// - has_more: Whether more results are available
    /// - next_offset: Offset value for fetching next page
    ///
    /// Tips for effective searching:
    /// - Use specific terms for precise results
    /// - Use pagination (offset) for large result sets
    /// - Call outline_documents_get for full content of relevant documents
    ///
    /// Example - Basic search: {"query": "API authentication"}
    /// Example - With pagination: {"query": "deployment", "limit": 10, "offset": 10}
    /// Example - Full content: {"query": "config", "includeContent": true, "limit": 5}
    #[tool(annotations(
        title = "Search Documents",
        read_only_hint = true,
        destructive_hint = false,
        idempotent_hint = true,
        open_world_hint = false
    ))]
    async fn outline_documents_search(
        &self,
        params: Parameters<SearchDocumentsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        // Enforce limits: default 25, max 100
        let limit = params.limit.unwrap_or(25).min(100);

        let request = SearchDocumentsRequest {
            query: params.query,
            collection_id: params.collection_id,
            user_id: None,
            date_filter: None,
            include_archived: None,
            include_drafts: None,
            offset: params.offset,
            limit: Some(limit),
        };

        let response = self.client.search_documents(request).await
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        // Transform results to optimized format
        let result_count = response.data.len() as u32;
        let results: Vec<SearchResultSummary> = response.data.into_iter().map(|r| {
            // Create snippet: first SNIPPET_MAX_LENGTH chars, ending at word boundary
            let text = &r.document.text;
            let snippet = if text.len() <= SNIPPET_MAX_LENGTH {
                text.clone()
            } else {
                // Find last space before limit to avoid cutting words
                let truncate_at = text[..SNIPPET_MAX_LENGTH]
                    .rfind(|c: char| c.is_whitespace())
                    .unwrap_or(SNIPPET_MAX_LENGTH);
                format!("{}...", &text[..truncate_at])
            };

            SearchResultSummary {
                id: r.document.id,
                title: r.document.title,
                ranking: r.ranking,
                context: r.context,
                snippet,
                text: if params.include_content { Some(r.document.text) } else { None },
                collection_id: r.document.collection_id,
                updated_at: r.document.updated_at,
            }
        }).collect();

        // Calculate pagination info
        let current_offset = params.offset.unwrap_or(0);
        let has_more = result_count >= limit;
        let next_offset = if has_more { Some(current_offset + result_count) } else { None };

        let search_response = SearchResponse {
            results,
            total: None, // Outline API doesn't provide total count
            has_more,
            next_offset,
        };

        let json = serde_json::to_string_pretty(&search_response)
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    // ========================================================================
    // Collection Tools
    // ========================================================================

    /// List all collections in your Outline workspace.
    ///
    /// Collections are top-level organizational containers for documents. Use this tool
    /// to discover available collections and get their IDs for filtering document operations.
    ///
    /// Parameters:
    /// - limit (optional): Maximum number of collections to return. Default 25.
    /// - offset (optional): Number to skip for pagination.
    ///
    /// Returns: JSON object with collections array, each containing:
    /// - id: Collection identifier (use with other tools' collectionId parameter)
    /// - name: Collection display name
    /// - description: Collection description (if set)
    /// - color: Hex color code for UI display
    /// - icon: Collection icon identifier
    /// - permission: User's permission level (read, read_write, admin)
    /// - documentCount: Number of documents in collection
    ///
    /// Example - List all: Call with no parameters
    /// Example - Paginate: {"limit": 10, "offset": 10}
    #[tool(annotations(
        title = "List Collections",
        read_only_hint = true,
        destructive_hint = false,
        idempotent_hint = true,
        open_world_hint = false
    ))]
    async fn outline_collections_list(
        &self,
        params: Parameters<ListCollectionsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        let request = ListCollectionsRequest {
            limit: params.limit,
            offset: params.offset,
        };

        let response = self.client.list_collections(request).await
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Get a specific collection by ID.
    ///
    /// Retrieves detailed information about a single collection including its metadata
    /// and document structure.
    ///
    /// Parameters:
    /// - id (required): Collection identifier (UUID format)
    ///
    /// Returns: JSON object containing:
    /// - id: Collection identifier
    /// - name: Collection display name
    /// - description: Collection description
    /// - color: Hex color code
    /// - icon: Icon identifier
    /// - permission: User's permission level
    /// - documentCount: Total documents in collection
    /// - createdAt/updatedAt: ISO 8601 timestamps
    ///
    /// Example: {"id": "abc123-def456-..."}
    #[tool(annotations(
        title = "Get Collection",
        read_only_hint = true,
        destructive_hint = false,
        idempotent_hint = true,
        open_world_hint = false
    ))]
    async fn outline_collections_get(
        &self,
        params: Parameters<GetCollectionParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        let collection = self.client.get_collection(params.id).await
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        let json = serde_json::to_string_pretty(&collection)
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

// Implement ServerHandler trait
#[tool_handler]
impl ServerHandler for OutlineServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("MCP server for Outline documentation workspace. Provides access to documents, collections, and search capabilities.".to_string()),
        }
    }
}

// ============================================================================
// Server Runner
// ============================================================================

/// Run the MCP server with stdio transport
pub async fn run() -> Result<()> {
    // Create the service
    let service = OutlineServer::new().await
        .context("Failed to create Outline server")?;

    tracing::info!("Server created successfully");
    tracing::info!("Serving tools:");
    tracing::info!("  - outline_documents_list");
    tracing::info!("  - outline_documents_get");
    tracing::info!("  - outline_documents_create");
    tracing::info!("  - outline_documents_update");
    tracing::info!("  - outline_documents_delete");
    tracing::info!("  - outline_documents_search");
    tracing::info!("  - outline_collections_list");
    tracing::info!("  - outline_collections_get");

    // Create stdio transport
    use tokio::io::{stdin, stdout};
    let transport = (stdin(), stdout());

    tracing::info!("Starting MCP server on stdio...");

    // Start the server
    let server = service.serve(transport).await
        .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?;

    // Wait for shutdown
    let quit_reason = server.waiting().await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    tracing::info!("Server shutdown: {:?}", quit_reason);

    Ok(())
}
