use anyhow::{anyhow, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::types::*;

/// API client for Outline
pub struct OutlineClient {
    client: reqwest::Client,
    base_url: String,
    api_token: Option<String>,
}

impl OutlineClient {
    /// Create a new API client
    pub fn new(base_url: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url,
            api_token: None,
        })
    }

    /// Set the API token for authentication
    pub fn with_token(mut self, token: String) -> Self {
        self.api_token = Some(token);
        self
    }

    /// Make a POST request to the API
    async fn post<T, R>(&self, endpoint: &str, request: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(token) = &self.api_token {
            let auth_value = format!("Bearer {}", token);
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&auth_value)
                    .context("Failed to create authorization header")?,
            );
        }

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(request)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let body = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            return Err(anyhow!(
                "API request failed with status {}: {}",
                status,
                body
            ));
        }

        serde_json::from_str(&body).context("Failed to parse response JSON")
    }

    /// List documents
    pub async fn list_documents(&self, request: ListDocumentsRequest) -> Result<ListDocumentsResponse> {
        self.post("documents.list", &request).await
    }

    /// Get document by ID
    pub async fn get_document(&self, id: String) -> Result<Document> {
        let request = DocumentInfoRequest::new(id);
        let response: ApiResponse<Document> = self.post("documents.info", &request).await?;

        response.data.ok_or_else(|| anyhow!("Document not found"))
    }

    /// Create a new document
    pub async fn create_document(&self, request: CreateDocumentRequest) -> Result<Document> {
        let response: ApiResponse<Document> = self.post("documents.create", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to create document"))
    }

    /// Update a document
    pub async fn update_document(&self, request: UpdateDocumentRequest) -> Result<Document> {
        let response: ApiResponse<Document> = self.post("documents.update", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to update document"))
    }

    /// Delete a document
    pub async fn delete_document(&self, id: String, permanent: bool) -> Result<()> {
        let request = DeleteDocumentRequest::new(id).permanent(permanent);

        let _response: ApiResponse<serde_json::Value> =
            self.post("documents.delete", &request).await?;
        Ok(())
    }

    /// Search documents
    pub async fn search_documents(&self, request: SearchDocumentsRequest) -> Result<SearchDocumentsResponse> {
        self.post("documents.search", &request).await
    }

    /// List collections
    pub async fn list_collections(&self, request: ListCollectionsRequest) -> Result<ListCollectionsResponse> {
        self.post("collections.list", &request).await
    }
}
