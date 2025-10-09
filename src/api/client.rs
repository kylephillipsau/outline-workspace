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
    pub async fn list_documents(
        &self,
        backlink_document_id: Option<String>,
        collection_id: Option<String>,
        direction: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
        parent_document_id: Option<String>,
        sort: Option<String>,
        template: Option<bool>,
        user_id: Option<String>,
    ) -> Result<DocumentsListResponse> {
        let request = DocumentsListRequest {
            backlink_document_id,
            collection_id,
            direction,
            limit,
            offset,
            parent_document_id,
            sort,
            template,
            user_id,
        };

        self.post("documents.list", &request).await
    }

    /// Get document by ID
    pub async fn get_document(&self, id: String) -> Result<Document> {
        let request = DocumentInfoRequest { id };
        let response: ApiResponse<Document> = self.post("documents.info", &request).await?;

        response.data.ok_or_else(|| anyhow!("Document not found"))
    }

    /// Create a new document
    pub async fn create_document(
        &self,
        title: String,
        text: String,
        collection_id: Option<String>,
        parent_document_id: Option<String>,
        emoji: Option<String>,
        publish: Option<bool>,
    ) -> Result<Document> {
        let request = DocumentCreateRequest {
            title,
            text,
            collection_id,
            parent_document_id,
            emoji,
            publish,
        };

        let response: ApiResponse<Document> = self.post("documents.create", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to create document"))
    }

    /// Update a document
    pub async fn update_document(
        &self,
        id: String,
        title: Option<String>,
        text: Option<String>,
        emoji: Option<String>,
        publish: Option<bool>,
    ) -> Result<Document> {
        let request = DocumentUpdateRequest {
            id,
            title,
            text,
            emoji,
            publish,
            done: None,
        };

        let response: ApiResponse<Document> = self.post("documents.update", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to update document"))
    }

    /// Delete a document
    pub async fn delete_document(&self, id: String, permanent: bool) -> Result<()> {
        let request = DocumentDeleteRequest {
            id,
            permanent: Some(permanent),
        };

        let _response: ApiResponse<serde_json::Value> =
            self.post("documents.delete", &request).await?;
        Ok(())
    }

    /// Search documents
    pub async fn search_documents(
        &self,
        query: String,
        collection_id: Option<String>,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<DocumentSearchResponse> {
        let request = DocumentSearchRequest {
            query,
            collection_id,
            offset,
            limit,
        };

        self.post("documents.search", &request).await
    }

    /// List collections
    pub async fn list_collections(
        &self,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<CollectionsListResponse> {
        let request = CollectionsListRequest { offset, limit };

        self.post("collections.list", &request).await
    }
}
