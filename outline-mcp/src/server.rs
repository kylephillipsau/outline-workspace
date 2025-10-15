use anyhow::{Context, Result};
use outline_api::{
    OutlineClient,
    CreateDocumentRequest, UpdateDocumentRequest, SearchDocumentsRequest,
    ListDocumentsRequest, ListCollectionsRequest,
};
use rmcp::{
    model::*,
    tool_router, tool,
    handler::server::{ServerHandler, tool::ToolRouter, wrapper::Parameters},
    ServiceExt,
};
use rmcp::model::ErrorCode;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListDocumentsParams {
    #[serde(rename = "collectionId")]
    pub collection_id: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDocumentParams {
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateDocumentParams {
    pub title: String,
    pub text: String,
    #[serde(rename = "collectionId")]
    pub collection_id: Option<String>,
    #[serde(rename = "parentDocumentId")]
    pub parent_document_id: Option<String>,
    pub publish: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateDocumentParams {
    pub id: String,
    pub title: Option<String>,
    pub text: Option<String>,
    pub publish: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteDocumentParams {
    pub id: String,
    pub permanent: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchDocumentsParams {
    pub query: String,
    #[serde(rename = "collectionId")]
    pub collection_id: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCollectionsParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCollectionParams {
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

    /// List documents in your Outline workspace. Optionally filter by collection ID.
    #[tool]
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

    /// Get a specific document by ID. Returns the full document including title, text content, and metadata.
    #[tool]
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

    /// Create a new document in Outline. Requires title and text content. Optionally set collection ID, parent document, and publish status.
    #[tool]
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

    /// Update an existing document. You can update the title, text content, or publish status.
    #[tool]
    async fn outline_documents_update(
        &self,
        params: Parameters<UpdateDocumentParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        let request = UpdateDocumentRequest {
            id: params.id,
            title: params.title,
            text: params.text,
            emoji: None,
            append: None,
            publish: params.publish,
            done: None,
        };

        let document = self.client.update_document(request).await
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        let json = serde_json::to_string_pretty(&document)
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    /// Delete a document. By default, moves to trash. Set permanent=true to permanently delete (cannot be undone).
    #[tool]
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

    /// Search for documents by text query. Optionally filter by collection ID.
    #[tool]
    async fn outline_documents_search(
        &self,
        params: Parameters<SearchDocumentsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = params.0;

        let request = SearchDocumentsRequest {
            query: params.query,
            collection_id: params.collection_id,
            user_id: None,
            date_filter: None,
            include_archived: None,
            include_drafts: None,
            offset: None,
            limit: params.limit,
        };

        let response = self.client.search_documents(request).await
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| ErrorData::new(ErrorCode(-32000), e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    // ========================================================================
    // Collection Tools
    // ========================================================================

    /// List all collections in your Outline workspace.
    #[tool]
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

    /// Get a specific collection by ID. Returns collection details and metadata.
    #[tool]
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
impl ServerHandler for OutlineServer {}

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
