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

    // ========================================================================
    // Document Organization Operations
    // ========================================================================

    /// Archive a document
    pub async fn archive_document(&self, id: String) -> Result<Document> {
        let request = ArchiveDocumentRequest::new(id);
        let response: ApiResponse<Document> = self.post("documents.archive", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to archive document"))
    }

    /// Unarchive a document
    pub async fn unarchive_document(&self, id: String) -> Result<Document> {
        let request = UnarchiveDocumentRequest::new(id);
        let response: ApiResponse<Document> = self.post("documents.unarchive", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to unarchive document"))
    }

    /// Star a document
    pub async fn star_document(&self, id: String) -> Result<Document> {
        let request = StarDocumentRequest::new(id);
        let response: ApiResponse<Document> = self.post("documents.star", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to star document"))
    }

    /// Unstar a document
    pub async fn unstar_document(&self, id: String) -> Result<Document> {
        let request = UnstarDocumentRequest::new(id);
        let response: ApiResponse<Document> = self.post("documents.unstar", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to unstar document"))
    }

    /// Unpublish a document (convert to draft)
    pub async fn unpublish_document(&self, id: String) -> Result<Document> {
        let request = UnpublishDocumentRequest::new(id);
        let response: ApiResponse<Document> = self.post("documents.unpublish", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to unpublish document"))
    }

    /// Convert a document into a template
    pub async fn templatize_document(&self, id: String) -> Result<Document> {
        let request = TemplatizeDocumentRequest::new(id);
        let response: ApiResponse<Document> = self.post("documents.templatize", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to templatize document"))
    }

    /// Move a document
    pub async fn move_document(&self, request: MoveDocumentRequest) -> Result<Document> {
        let response: ApiResponse<Document> = self.post("documents.move", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to move document"))
    }

    /// Restore a document from trash or to a previous revision
    pub async fn restore_document(&self, request: RestoreDocumentRequest) -> Result<Document> {
        let response: ApiResponse<Document> = self.post("documents.restore", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to restore document"))
    }

    // ========================================================================
    // Document Listing Variants
    // ========================================================================

    /// List recently viewed documents
    pub async fn list_viewed_documents(&self, request: ViewedDocumentsRequest) -> Result<ListDocumentsResponse> {
        self.post("documents.viewed", &request).await
    }

    /// List draft documents
    pub async fn list_drafts(&self, request: DraftsRequest) -> Result<ListDocumentsResponse> {
        self.post("documents.drafts", &request).await
    }

    /// List template documents
    pub async fn list_templates(&self, request: TemplatesRequest) -> Result<ListDocumentsResponse> {
        self.post("documents.templates", &request).await
    }

    // ========================================================================
    // Document Collaboration Operations
    // ========================================================================

    /// Add a user to a document
    pub async fn add_user_to_document(&self, request: AddUserToDocumentRequest) -> Result<()> {
        let _response: ApiResponse<serde_json::Value> = self.post("documents.add_user", &request).await?;
        Ok(())
    }

    /// Remove a user from a document
    pub async fn remove_user_from_document(&self, request: RemoveUserFromDocumentRequest) -> Result<()> {
        let _response: ApiResponse<serde_json::Value> = self.post("documents.remove_user", &request).await?;
        Ok(())
    }

    // ========================================================================
    // Document Import/Export Operations
    // ========================================================================

    /// Import a document from external formats (Markdown, HTML, Docx, Notion, Confluence)
    pub async fn import_document(&self, request: ImportDocumentRequest) -> Result<Document> {
        let url = format!("{}/{}", self.base_url, "documents.import");

        let mut headers = HeaderMap::new();

        if let Some(token) = &self.api_token {
            let auth_value = format!("Bearer {}", token);
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&auth_value)
                    .context("Failed to create authorization header")?,
            );
        }

        // Create multipart form for file upload
        let file_part = reqwest::multipart::Part::bytes(request.file)
            .file_name("import.file");

        let mut form = reqwest::multipart::Form::new()
            .text("collectionId", request.collection_id.clone())
            .part("file", file_part);

        if let Some(parent_id) = request.parent_document_id {
            form = form.text("parentDocumentId", parent_id);
        }

        if let Some(publish) = request.publish {
            form = form.text("publish", publish.to_string());
        }

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await
            .context("Failed to send import request")?;

        let status = response.status();
        let body = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            return Err(anyhow!(
                "Import request failed with status {}: {}",
                status,
                body
            ));
        }

        let api_response: ApiResponse<Document> = serde_json::from_str(&body)
            .context("Failed to parse import response")?;

        api_response.data.ok_or_else(|| anyhow!("Failed to import document"))
    }

    /// Export a document in various formats (Markdown, HTML, PDF)
    pub async fn export_document(&self, request: ExportDocumentRequest) -> Result<Vec<u8>> {
        let url = format!("{}/{}", self.base_url, "documents.export");

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
            .json(&request)
            .send()
            .await
            .context("Failed to send export request")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.context("Failed to read error response")?;
            return Err(anyhow!(
                "Export request failed with status {}: {}",
                status,
                body
            ));
        }

        response
            .bytes()
            .await
            .context("Failed to read export data")
            .map(|b| b.to_vec())
    }

    // ========================================================================
    // Collection Operations
    // ========================================================================

    /// List collections
    pub async fn list_collections(&self, request: ListCollectionsRequest) -> Result<ListCollectionsResponse> {
        self.post("collections.list", &request).await
    }

    /// Get collection by ID
    pub async fn get_collection(&self, id: String) -> Result<Collection> {
        let request = CollectionInfoRequest::new(id);
        let response: ApiResponse<Collection> = self.post("collections.info", &request).await?;
        response.data.ok_or_else(|| anyhow!("Collection not found"))
    }

    /// Create a new collection
    pub async fn create_collection(&self, request: CreateCollectionRequest) -> Result<Collection> {
        let response: ApiResponse<Collection> = self.post("collections.create", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to create collection"))
    }

    /// Update a collection
    pub async fn update_collection(&self, request: UpdateCollectionRequest) -> Result<Collection> {
        let response: ApiResponse<Collection> = self.post("collections.update", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to update collection"))
    }

    /// Delete a collection
    pub async fn delete_collection(&self, id: String) -> Result<()> {
        let request = DeleteCollectionRequest::new(id);
        let _response: ApiResponse<serde_json::Value> = self.post("collections.delete", &request).await?;
        Ok(())
    }

    /// Move a collection to a different position
    pub async fn move_collection(&self, request: MoveCollectionRequest) -> Result<Collection> {
        let response: ApiResponse<Collection> = self.post("collections.move", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to move collection"))
    }

    /// List documents in a collection
    pub async fn list_collection_documents(&self, request: CollectionDocumentsRequest) -> Result<ListDocumentsResponse> {
        self.post("collections.documents", &request).await
    }

    // ========================================================================
    // Collection Member Management
    // ========================================================================

    /// Add a user to a collection
    pub async fn add_user_to_collection(&self, request: AddUserToCollectionRequest) -> Result<()> {
        let _response: ApiResponse<serde_json::Value> = self.post("collections.add_user", &request).await?;
        Ok(())
    }

    /// Remove a user from a collection
    pub async fn remove_user_from_collection(&self, request: RemoveUserFromCollectionRequest) -> Result<()> {
        let _response: ApiResponse<serde_json::Value> = self.post("collections.remove_user", &request).await?;
        Ok(())
    }

    /// Add a group to a collection
    pub async fn add_group_to_collection(&self, request: AddGroupToCollectionRequest) -> Result<()> {
        let _response: ApiResponse<serde_json::Value> = self.post("collections.add_group", &request).await?;
        Ok(())
    }

    /// Remove a group from a collection
    pub async fn remove_group_from_collection(&self, request: RemoveGroupFromCollectionRequest) -> Result<()> {
        let _response: ApiResponse<serde_json::Value> = self.post("collections.remove_group", &request).await?;
        Ok(())
    }

    /// List collection members
    pub async fn list_collection_memberships(&self, request: CollectionMembershipsRequest) -> Result<serde_json::Value> {
        self.post("collections.memberships", &request).await
    }

    // ========================================================================
    // Collection Export/Import Operations
    // ========================================================================

    /// Export a collection in the specified format
    pub async fn export_collection(&self, request: ExportCollectionRequest) -> Result<Vec<u8>> {
        let url = format!("{}/{}", self.base_url, "collections.export");

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
            .json(&request)
            .send()
            .await
            .context("Failed to send export request")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.context("Failed to read error response")?;
            return Err(anyhow!(
                "Export request failed with status {}: {}",
                status,
                body
            ));
        }

        response
            .bytes()
            .await
            .context("Failed to read export data")
            .map(|b| b.to_vec())
    }

    /// Export all collections in the specified format
    pub async fn export_all_collections(&self, request: ExportAllCollectionsRequest) -> Result<Vec<u8>> {
        let url = format!("{}/{}", self.base_url, "collections.export_all");

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
            .json(&request)
            .send()
            .await
            .context("Failed to send export all request")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.context("Failed to read error response")?;
            return Err(anyhow!(
                "Export all request failed with status {}: {}",
                status,
                body
            ));
        }

        response
            .bytes()
            .await
            .context("Failed to read export data")
            .map(|b| b.to_vec())
    }

    /// Import a file into a collection
    pub async fn import_file_to_collection(&self, request: ImportFileToCollectionRequest) -> Result<serde_json::Value> {
        let url = format!("{}/{}", self.base_url, "collections.import_file");

        let mut headers = HeaderMap::new();

        if let Some(token) = &self.api_token {
            let auth_value = format!("Bearer {}", token);
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&auth_value)
                    .context("Failed to create authorization header")?,
            );
        }

        // Create multipart form for file upload
        let file_part = reqwest::multipart::Part::bytes(request.file)
            .file_name("import.file");

        let form = reqwest::multipart::Form::new()
            .text("id", request.id)
            .text("format", format!("{:?}", request.format).to_lowercase())
            .part("file", file_part);

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await
            .context("Failed to send import request")?;

        let status = response.status();
        let body = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            return Err(anyhow!(
                "Import request failed with status {}: {}",
                status,
                body
            ));
        }

        serde_json::from_str(&body).context("Failed to parse import response")
    }

    // ========================================================================
    // User Operations
    // ========================================================================

    /// Get user information
    pub async fn get_user(&self, request: UserInfoRequest) -> Result<User> {
        let response: ApiResponse<User> = self.post("users.info", &request).await?;
        response.data.ok_or_else(|| anyhow!("User not found"))
    }

    /// Update user profile
    pub async fn update_user(&self, request: UpdateUserRequest) -> Result<User> {
        let response: ApiResponse<User> = self.post("users.update", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to update user"))
    }

    /// List users
    pub async fn list_users(&self, request: ListUsersRequest) -> Result<ListUsersResponse> {
        self.post("users.list", &request).await
    }

    // ========================================================================
    // User Admin Operations
    // ========================================================================

    /// Suspend a user account
    pub async fn suspend_user(&self, id: String) -> Result<User> {
        let request = SuspendUserRequest::new(id);
        let response: ApiResponse<User> = self.post("users.suspend", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to suspend user"))
    }

    /// Activate a suspended user account
    pub async fn activate_user(&self, id: String) -> Result<User> {
        let request = ActivateUserRequest::new(id);
        let response: ApiResponse<User> = self.post("users.activate", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to activate user"))
    }

    /// Delete a user account
    pub async fn delete_user(&self, id: String) -> Result<User> {
        let request = DeleteUserRequest::new(id);
        let response: ApiResponse<User> = self.post("users.delete", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to delete user"))
    }

    /// Promote a user to admin
    pub async fn promote_user(&self, id: String) -> Result<User> {
        let request = PromoteUserRequest::new(id);
        let response: ApiResponse<User> = self.post("users.promote", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to promote user"))
    }

    /// Demote an admin to regular user
    pub async fn demote_user(&self, id: String) -> Result<User> {
        let request = DemoteUserRequest::new(id);
        let response: ApiResponse<User> = self.post("users.demote", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to demote user"))
    }

    /// Invite a new user to the team
    pub async fn invite_user(&self, request: InviteUserRequest) -> Result<User> {
        let response: ApiResponse<User> = self.post("users.invite", &request).await?;
        response.data.ok_or_else(|| anyhow!("Failed to invite user"))
    }
}
