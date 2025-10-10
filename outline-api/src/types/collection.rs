use serde::{Deserialize, Serialize};

use super::common::{CollectionPermission, PaginationResponse};

/// Collection structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<CollectionPermission>,
}

// ============================================================================
// Request Types
// ============================================================================

/// Request to list collections
#[derive(Debug, Clone, Serialize)]
pub struct ListCollectionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ListCollectionsRequest {
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

impl Default for ListCollectionsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request to get collection info
#[derive(Debug, Clone, Serialize)]
pub struct CollectionInfoRequest {
    pub id: String,
}

impl CollectionInfoRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Request to create a collection
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<CollectionPermission>,
}

impl CreateCollectionRequest {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            color: None,
            private: None,
            permission: None,
        }
    }

    pub fn builder(name: String) -> CreateCollectionRequestBuilder {
        CreateCollectionRequestBuilder::new(name)
    }
}

/// Builder for CreateCollectionRequest
#[derive(Debug)]
pub struct CreateCollectionRequestBuilder {
    name: String,
    description: Option<String>,
    color: Option<String>,
    private: Option<bool>,
    permission: Option<CollectionPermission>,
}

impl CreateCollectionRequestBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            color: None,
            private: None,
            permission: None,
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }

    pub fn private(mut self, private: bool) -> Self {
        self.private = Some(private);
        self
    }

    pub fn permission(mut self, permission: CollectionPermission) -> Self {
        self.permission = Some(permission);
        self
    }

    pub fn build(self) -> CreateCollectionRequest {
        CreateCollectionRequest {
            name: self.name,
            description: self.description,
            color: self.color,
            private: self.private,
            permission: self.permission,
        }
    }
}

/// Request to update a collection
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCollectionRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<CollectionPermission>,
}

impl UpdateCollectionRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: None,
            description: None,
            color: None,
            permission: None,
        }
    }

    pub fn builder(id: String) -> UpdateCollectionRequestBuilder {
        UpdateCollectionRequestBuilder::new(id)
    }
}

/// Builder for UpdateCollectionRequest
#[derive(Debug)]
pub struct UpdateCollectionRequestBuilder {
    id: String,
    name: Option<String>,
    description: Option<String>,
    color: Option<String>,
    permission: Option<CollectionPermission>,
}

impl UpdateCollectionRequestBuilder {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: None,
            description: None,
            color: None,
            permission: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }

    pub fn permission(mut self, permission: CollectionPermission) -> Self {
        self.permission = Some(permission);
        self
    }

    pub fn build(self) -> UpdateCollectionRequest {
        UpdateCollectionRequest {
            id: self.id,
            name: self.name,
            description: self.description,
            color: self.color,
            permission: self.permission,
        }
    }
}

/// Request to delete a collection
#[derive(Debug, Clone, Serialize)]
pub struct DeleteCollectionRequest {
    pub id: String,
}

impl DeleteCollectionRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

// ============================================================================
// Response Types
// ============================================================================

/// Response from listing collections
#[derive(Debug, Deserialize)]
pub struct ListCollectionsResponse {
    pub data: Vec<Collection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationResponse>,
}
