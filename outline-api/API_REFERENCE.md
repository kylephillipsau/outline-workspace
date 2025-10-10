# Outline API Reference & Implementation Plan

This document provides a comprehensive reference for the Outline API endpoints and details the implementation plan for the `outline-api` Rust crate.

## Table of Contents

1. [Overview](#overview)
2. [Implementation Plan](#implementation-plan)
3. [API Endpoints by Resource](#api-endpoints-by-resource)
4. [Type Definitions](#type-definitions)
5. [Authentication](#authentication)
6. [Error Handling](#error-handling)

---

## Overview

The `outline-api` crate provides a complete Rust client for the Outline knowledge base server. The implementation follows KISS, YAGNI, and DRY principles with a modular, trait-based design.

### Design Principles

- **Trait-based API**: Clean separation between interface and implementation
- **Feature flags**: Optional dependencies for collaboration features
- **Type safety**: Strongly-typed request/response structures
- **Progressive enhancement**: Core features first, advanced features later

### Architecture

```
outline-api/
├── src/
│   ├── lib.rs              # Public API surface
│   ├── client.rs           # HTTP client implementation
│   ├── auth.rs             # Authentication handling
│   ├── types/              # Type definitions
│   │   ├── mod.rs
│   │   ├── document.rs     # Document types
│   │   ├── collection.rs   # Collection types
│   │   ├── user.rs         # User types
│   │   ├── comment.rs      # Comment types
│   │   ├── group.rs        # Group types
│   │   ├── share.rs        # Share types
│   │   ├── attachment.rs   # Attachment types
│   │   ├── notification.rs # Notification types
│   │   ├── event.rs        # Event types
│   │   └── team.rs         # Team types
│   ├── resources/          # Resource-specific API implementations
│   │   ├── mod.rs
│   │   ├── documents.rs    # Document operations
│   │   ├── collections.rs  # Collection operations
│   │   ├── users.rs        # User operations
│   │   ├── comments.rs     # Comment operations
│   │   ├── groups.rs       # Group operations
│   │   ├── shares.rs       # Share operations
│   │   ├── attachments.rs  # Attachment operations
│   │   ├── notifications.rs # Notification operations
│   │   ├── events.rs       # Event operations
│   │   └── teams.rs        # Team operations
│   └── collaboration/      # Real-time collaboration (optional)
│       ├── mod.rs
│       ├── websocket.rs    # WebSocket connection
│       └── sync.rs         # CRDT sync engine
```

---

## Implementation Plan

### Phase 1: Core Infrastructure (IMMEDIATE)

**Goal**: Establish foundation for all API operations

#### Tasks:
1. **Type System Refinement**
   - Move types to `types/` module structure
   - Create comprehensive type definitions for all resources
   - Add builder patterns for complex request types

2. **Client Architecture**
   - Implement trait-based API design
   - Add retry logic with exponential backoff
   - Implement request/response logging (debug feature)

3. **Authentication**
   - Support API tokens
   - Add token refresh mechanism
   - Implement auth header management

#### Deliverables:
- Complete type definitions for Documents, Collections, Users
- Working `OutlineClient` with authentication
- Basic error handling and retry logic

---

### Phase 2: Essential Resources (HIGH PRIORITY)

**Goal**: Implement most commonly used API operations

#### Resources to Implement:

1. **Documents** (28 operations)
   - CRUD operations: create, update, delete, info
   - Content operations: import, export, restore
   - Organization: move, archive, unarchive, star, unstar
   - Search and listing: list, search, viewed, drafts, templates
   - Collaboration: add_user, remove_user
   - Version control: templatize

2. **Collections** (18 operations)
   - CRUD operations: create, update, delete, info
   - Organization: move, export, add_user, remove_user, add_group, remove_group
   - Listing: list, documents
   - Content: import_file

3. **Users** (15 operations)
   - Profile: info, update
   - Administration: list, suspend, activate, delete, promote, demote, invite
   - Preferences: notifications, update_role

#### Deliverables:
- Complete implementation of Documents API
- Complete implementation of Collections API
- Complete implementation of Users API
- Integration tests for all endpoints

---

### Phase 3: Extended Resources (MEDIUM PRIORITY)

**Goal**: Add supporting features for team collaboration

#### Resources to Implement:

1. **Comments** (8 operations)
   - CRUD: create, update, delete, info
   - Listing: list
   - Reactions: add_reaction, remove_reaction

2. **Groups** (9 operations)
   - CRUD: create, update, delete, info
   - Management: list, add_user, remove_user, memberships

3. **Shares** (6 operations)
   - CRUD: create, update, revoke, info
   - Listing: list

4. **Attachments** (5 operations)
   - Upload: create, redirect
   - Management: delete
   - Listing: list

#### Deliverables:
- Implementation of Comments, Groups, Shares, Attachments APIs
- Integration with Documents for inline attachments and comments

---

### Phase 4: Advanced Features (OPTIONAL)

**Goal**: Complete API coverage and real-time features

#### Resources to Implement:

1. **Notifications** (7 operations)
   - Management: list, update, archive, archive_all, unarchive
   - Subscription: subscribe, unsubscribe

2. **Events** (1 operation)
   - Audit trail: list

3. **Teams** (2 operations)
   - Information: info, update

4. **Real-time Collaboration** (behind feature flag)
   - WebSocket connection for live updates
   - CRDT synchronization (Yjs integration)
   - Presence awareness

#### Deliverables:
- Complete API coverage (100% of Outline endpoints)
- Optional real-time collaboration features
- Comprehensive documentation and examples

---

## API Endpoints by Resource

### Documents API

The Documents API provides 28 operations for managing documents in Outline.

#### Core Operations

##### `documents.create`
Create a new document.

**Request:**
```rust
pub struct CreateDocumentRequest {
    pub title: String,
    pub text: String,
    pub collection_id: Option<String>,
    pub parent_document_id: Option<String>,
    pub template_id: Option<String>,
    pub template: Option<bool>,
    pub publish: Option<bool>,
}
```

**Response:**
```rust
pub struct Document {
    pub id: String,
    pub title: String,
    pub text: String,
    pub url_id: String,
    pub collection_id: Option<String>,
    pub parent_document_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: Option<String>,
    pub archived_at: Option<String>,
    pub deleted_at: Option<String>,
    // ... additional fields
}
```

##### `documents.info`
Retrieve document details by ID.

**Request:**
```rust
pub struct DocumentInfoRequest {
    pub id: String,
    pub share_id: Option<String>,
}
```

##### `documents.update`
Update an existing document.

**Request:**
```rust
pub struct UpdateDocumentRequest {
    pub id: String,
    pub title: Option<String>,
    pub text: Option<String>,
    pub append: Option<bool>,
    pub publish: Option<bool>,
    pub done: Option<bool>,
}
```

##### `documents.delete`
Move a document to trash.

**Request:**
```rust
pub struct DeleteDocumentRequest {
    pub id: String,
    pub permanent: Option<bool>,
}
```

#### Content Operations

##### `documents.import`
Import a document from external formats (Markdown, HTML, Notion, Confluence).

**Request:**
```rust
pub struct ImportDocumentRequest {
    pub file: Vec<u8>,
    pub collection_id: String,
    pub parent_document_id: Option<String>,
    pub publish: Option<bool>,
}
```

##### `documents.export`
Export a document in various formats.

**Request:**
```rust
pub struct ExportDocumentRequest {
    pub id: String,
    pub format: ExportFormat, // markdown, html, pdf
}
```

```rust
pub enum ExportFormat {
    Markdown,
    Html,
    Pdf,
}
```

##### `documents.restore`
Restore a document from trash or to a previous revision.

**Request:**
```rust
pub struct RestoreDocumentRequest {
    pub id: String,
    pub revision_id: Option<String>,
    pub collection_id: Option<String>,
}
```

#### Organization Operations

##### `documents.move`
Move a document to a different collection or parent.

**Request:**
```rust
pub struct MoveDocumentRequest {
    pub id: String,
    pub collection_id: Option<String>,
    pub parent_document_id: Option<String>,
    pub index: Option<u32>,
}
```

##### `documents.archive`
Archive a document.

**Request:**
```rust
pub struct ArchiveDocumentRequest {
    pub id: String,
}
```

##### `documents.unarchive`
Unarchive a document.

**Request:**
```rust
pub struct UnarchiveDocumentRequest {
    pub id: String,
}
```

##### `documents.star`
Star a document for quick access.

**Request:**
```rust
pub struct StarDocumentRequest {
    pub id: String,
}
```

##### `documents.unstar`
Remove star from a document.

**Request:**
```rust
pub struct UnstarDocumentRequest {
    pub id: String,
}
```

##### `documents.unpublish`
Convert a published document to draft.

**Request:**
```rust
pub struct UnpublishDocumentRequest {
    pub id: String,
}
```

#### Listing & Search Operations

##### `documents.list`
List documents with pagination and filtering.

**Request:**
```rust
pub struct ListDocumentsRequest {
    pub collection_id: Option<String>,
    pub parent_document_id: Option<String>,
    pub backlink_document_id: Option<String>,
    pub user_id: Option<String>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub sort: Option<DocumentSort>,
    pub direction: Option<SortDirection>,
}
```

```rust
pub enum DocumentSort {
    Title,
    Index,
    UpdatedAt,
    CreatedAt,
}

pub enum SortDirection {
    Asc,
    Desc,
}
```

##### `documents.search`
Full-text search across documents.

**Request:**
```rust
pub struct SearchDocumentsRequest {
    pub query: String,
    pub collection_id: Option<String>,
    pub user_id: Option<String>,
    pub date_filter: Option<DateFilter>,
    pub include_archived: Option<bool>,
    pub include_drafts: Option<bool>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

```rust
pub enum DateFilter {
    Day,
    Week,
    Month,
    Year,
}
```

##### `documents.viewed`
List recently viewed documents.

**Request:**
```rust
pub struct ViewedDocumentsRequest {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

##### `documents.drafts`
List draft documents.

**Request:**
```rust
pub struct DraftsRequest {
    pub collection_id: Option<String>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

##### `documents.templates`
List template documents.

**Request:**
```rust
pub struct TemplatesRequest {
    pub collection_id: Option<String>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

#### Collaboration Operations

##### `documents.add_user`
Grant a user access to a document.

**Request:**
```rust
pub struct AddUserToDocumentRequest {
    pub id: String,
    pub user_id: String,
    pub permission: Permission,
}
```

```rust
pub enum Permission {
    Read,
    ReadWrite,
}
```

##### `documents.remove_user`
Revoke user access from a document.

**Request:**
```rust
pub struct RemoveUserFromDocumentRequest {
    pub id: String,
    pub user_id: String,
}
```

#### Template Operations

##### `documents.templatize`
Convert a document into a template.

**Request:**
```rust
pub struct TemplatizeDocumentRequest {
    pub id: String,
}
```

---

### Collections API

The Collections API provides 18 operations for managing document collections.

#### Core Operations

##### `collections.create`
Create a new collection.

**Request:**
```rust
pub struct CreateCollectionRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub private: Option<bool>,
    pub permission: Option<CollectionPermission>,
}
```

```rust
pub enum CollectionPermission {
    Read,
    ReadWrite,
}
```

**Response:**
```rust
pub struct Collection {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub permission: CollectionPermission,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    // ... additional fields
}
```

##### `collections.info`
Retrieve collection details.

**Request:**
```rust
pub struct CollectionInfoRequest {
    pub id: String,
}
```

##### `collections.update`
Update collection properties.

**Request:**
```rust
pub struct UpdateCollectionRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub permission: Option<CollectionPermission>,
}
```

##### `collections.delete`
Delete a collection.

**Request:**
```rust
pub struct DeleteCollectionRequest {
    pub id: String,
}
```

#### Organization Operations

##### `collections.move`
Move a collection to a different position.

**Request:**
```rust
pub struct MoveCollectionRequest {
    pub id: String,
    pub index: u32,
}
```

##### `collections.export`
Export entire collection.

**Request:**
```rust
pub struct ExportCollectionRequest {
    pub id: String,
    pub format: ExportFormat,
}
```

##### `collections.export_all`
Export all collections.

**Request:**
```rust
pub struct ExportAllCollectionsRequest {
    pub format: ExportFormat,
}
```

#### Member Management

##### `collections.add_user`
Add a user to a collection.

**Request:**
```rust
pub struct AddUserToCollectionRequest {
    pub id: String,
    pub user_id: String,
    pub permission: Permission,
}
```

##### `collections.remove_user`
Remove a user from a collection.

**Request:**
```rust
pub struct RemoveUserFromCollectionRequest {
    pub id: String,
    pub user_id: String,
}
```

##### `collections.add_group`
Add a group to a collection.

**Request:**
```rust
pub struct AddGroupToCollectionRequest {
    pub id: String,
    pub group_id: String,
    pub permission: Permission,
}
```

##### `collections.remove_group`
Remove a group from a collection.

**Request:**
```rust
pub struct RemoveGroupFromCollectionRequest {
    pub id: String,
    pub group_id: String,
}
```

##### `collections.memberships`
List collection members.

**Request:**
```rust
pub struct CollectionMembershipsRequest {
    pub id: String,
    pub query: Option<String>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

#### Listing Operations

##### `collections.list`
List all collections.

**Request:**
```rust
pub struct ListCollectionsRequest {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

##### `collections.documents`
List documents in a collection.

**Request:**
```rust
pub struct CollectionDocumentsRequest {
    pub id: String,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

#### Content Operations

##### `collections.import_file`
Import documents from a file.

**Request:**
```rust
pub struct ImportFileToCollectionRequest {
    pub id: String,
    pub file: Vec<u8>,
    pub format: ImportFormat,
}
```

```rust
pub enum ImportFormat {
    Markdown,
    Html,
    Docx,
    Notion,
    Confluence,
}
```

---

### Users API

The Users API provides 15 operations for user management.

#### Profile Operations

##### `users.info`
Get user information.

**Request:**
```rust
pub struct UserInfoRequest {
    pub id: Option<String>, // Defaults to current user
}
```

**Response:**
```rust
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub is_admin: bool,
    pub is_suspended: bool,
    pub is_viewer: bool,
    pub created_at: String,
    pub last_active_at: Option<String>,
    // ... additional fields
}
```

##### `users.update`
Update user profile.

**Request:**
```rust
pub struct UpdateUserRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub language: Option<String>,
}
```

#### Administration Operations

##### `users.list`
List all users (admin only).

**Request:**
```rust
pub struct ListUsersRequest {
    pub query: Option<String>,
    pub filter: Option<UserFilter>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub sort: Option<UserSort>,
    pub direction: Option<SortDirection>,
}
```

```rust
pub enum UserFilter {
    All,
    Admins,
    Suspended,
    Active,
    Invited,
}

pub enum UserSort {
    Name,
    Email,
    LastActive,
    CreatedAt,
}
```

##### `users.suspend`
Suspend a user account.

**Request:**
```rust
pub struct SuspendUserRequest {
    pub id: String,
}
```

##### `users.activate`
Activate a suspended user.

**Request:**
```rust
pub struct ActivateUserRequest {
    pub id: String,
}
```

##### `users.delete`
Delete a user account.

**Request:**
```rust
pub struct DeleteUserRequest {
    pub id: String,
}
```

##### `users.promote`
Promote user to admin.

**Request:**
```rust
pub struct PromoteUserRequest {
    pub id: String,
}
```

##### `users.demote`
Demote admin to regular user.

**Request:**
```rust
pub struct DemoteUserRequest {
    pub id: String,
}
```

##### `users.invite`
Invite new user to team.

**Request:**
```rust
pub struct InviteUserRequest {
    pub email: String,
    pub name: String,
    pub role: Option<UserRole>,
}
```

```rust
pub enum UserRole {
    Admin,
    Member,
    Viewer,
}
```

#### Notification Operations

##### `users.notifications`
Get user notification settings.

**Request:**
```rust
pub struct UserNotificationsRequest {
    pub id: Option<String>,
}
```

##### `users.update_notification`
Update notification preferences.

**Request:**
```rust
pub struct UpdateUserNotificationRequest {
    pub event_type: NotificationEventType,
    pub enabled: bool,
}
```

```rust
pub enum NotificationEventType {
    DocumentCreated,
    DocumentUpdated,
    CommentCreated,
    // ... additional event types
}
```

---

### Comments API

The Comments API provides 8 operations for document comments.

##### `comments.create`
Create a comment on a document.

**Request:**
```rust
pub struct CreateCommentRequest {
    pub document_id: String,
    pub parent_comment_id: Option<String>,
    pub data: CommentData,
}
```

```rust
pub struct CommentData {
    pub text: String,
    pub position: Option<CommentPosition>,
}

pub struct CommentPosition {
    pub line: u32,
    pub character: u32,
}
```

##### `comments.info`
Get comment details.

**Request:**
```rust
pub struct CommentInfoRequest {
    pub id: String,
}
```

##### `comments.list`
List comments on a document.

**Request:**
```rust
pub struct ListCommentsRequest {
    pub document_id: String,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

##### `comments.update`
Update a comment.

**Request:**
```rust
pub struct UpdateCommentRequest {
    pub id: String,
    pub data: CommentData,
}
```

##### `comments.delete`
Delete a comment.

**Request:**
```rust
pub struct DeleteCommentRequest {
    pub id: String,
}
```

##### `comments.resolve`
Mark a comment thread as resolved.

**Request:**
```rust
pub struct ResolveCommentRequest {
    pub id: String,
}
```

##### `comments.unresolve`
Mark a comment thread as unresolved.

**Request:**
```rust
pub struct UnresolveCommentRequest {
    pub id: String,
}
```

---

### Groups API

The Groups API provides 9 operations for team group management.

##### `groups.create`
Create a new group.

**Request:**
```rust
pub struct CreateGroupRequest {
    pub name: String,
    pub description: Option<String>,
}
```

##### `groups.info`
Get group details.

**Request:**
```rust
pub struct GroupInfoRequest {
    pub id: String,
}
```

##### `groups.list`
List all groups.

**Request:**
```rust
pub struct ListGroupsRequest {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

##### `groups.update`
Update group properties.

**Request:**
```rust
pub struct UpdateGroupRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}
```

##### `groups.delete`
Delete a group.

**Request:**
```rust
pub struct DeleteGroupRequest {
    pub id: String,
}
```

##### `groups.add_user`
Add a user to a group.

**Request:**
```rust
pub struct AddUserToGroupRequest {
    pub id: String,
    pub user_id: String,
}
```

##### `groups.remove_user`
Remove a user from a group.

**Request:**
```rust
pub struct RemoveUserFromGroupRequest {
    pub id: String,
    pub user_id: String,
}
```

##### `groups.memberships`
List group members.

**Request:**
```rust
pub struct GroupMembershipsRequest {
    pub id: String,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

---

### Shares API

The Shares API provides 6 operations for document sharing.

##### `shares.create`
Create a public share link.

**Request:**
```rust
pub struct CreateShareRequest {
    pub document_id: String,
    pub published: Option<bool>,
    pub include_child_documents: Option<bool>,
}
```

##### `shares.info`
Get share details.

**Request:**
```rust
pub struct ShareInfoRequest {
    pub id: String,
}
```

##### `shares.list`
List all shares.

**Request:**
```rust
pub struct ListSharesRequest {
    pub document_id: Option<String>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
```

##### `shares.update`
Update share settings.

**Request:**
```rust
pub struct UpdateShareRequest {
    pub id: String,
    pub published: Option<bool>,
    pub include_child_documents: Option<bool>,
}
```

##### `shares.revoke`
Revoke a share link.

**Request:**
```rust
pub struct RevokeShareRequest {
    pub id: String,
}
```

---

### Attachments API

The Attachments API provides 5 operations for file attachments.

##### `attachments.create`
Upload a file attachment.

**Request:**
```rust
pub struct CreateAttachmentRequest {
    pub name: String,
    pub document_id: Option<String>,
    pub content_type: String,
    pub size: u64,
    pub data: Vec<u8>,
}
```

##### `attachments.delete`
Delete an attachment.

**Request:**
```rust
pub struct DeleteAttachmentRequest {
    pub id: String,
}
```

##### `attachments.redirect`
Get attachment download URL.

**Request:**
```rust
pub struct RedirectAttachmentRequest {
    pub id: String,
}
```

---

### Notifications API

The Notifications API provides 7 operations for managing user notifications.

##### `notifications.list`
List user notifications.

**Request:**
```rust
pub struct ListNotificationsRequest {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub archived: Option<bool>,
}
```

##### `notifications.update`
Mark notification as read.

**Request:**
```rust
pub struct UpdateNotificationRequest {
    pub id: String,
    pub viewed_at: Option<String>,
}
```

##### `notifications.archive`
Archive a notification.

**Request:**
```rust
pub struct ArchiveNotificationRequest {
    pub id: String,
}
```

##### `notifications.unarchive`
Unarchive a notification.

**Request:**
```rust
pub struct UnarchiveNotificationRequest {
    pub id: String,
}
```

##### `notifications.archive_all`
Archive all notifications.

**Request:**
```rust
pub struct ArchiveAllNotificationsRequest {}
```

---

### Events API

The Events API provides audit trail functionality.

##### `events.list`
List team events (audit log).

**Request:**
```rust
pub struct ListEventsRequest {
    pub name: Option<String>,
    pub actor_id: Option<String>,
    pub document_id: Option<String>,
    pub collection_id: Option<String>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub sort: Option<EventSort>,
    pub direction: Option<SortDirection>,
}
```

```rust
pub enum EventSort {
    CreatedAt,
    Name,
}
```

---

### Teams API

The Teams API provides 2 operations for team management.

##### `teams.info`
Get team information.

**Request:**
```rust
pub struct TeamInfoRequest {}
```

**Response:**
```rust
pub struct Team {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub sharing: bool,
    pub collaborative_editing: bool,
    pub default_collection_id: Option<String>,
    pub domain: Option<String>,
    pub allowed_domains: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}
```

##### `teams.update`
Update team settings.

**Request:**
```rust
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub sharing: Option<bool>,
    pub collaborative_editing: Option<bool>,
    pub default_collection_id: Option<String>,
    pub allowed_domains: Option<Vec<String>>,
}
```

---

## Type Definitions

### Common Types

```rust
// Pagination response wrapper
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

pub struct Pagination {
    pub offset: u32,
    pub limit: u32,
    pub total: Option<u32>,
}

// API response wrapper
pub struct ApiResponse<T> {
    pub data: T,
    pub status: u16,
    pub ok: bool,
}

// Error response
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub status: u16,
}
```

### Document Types

```rust
pub struct Document {
    pub id: String,
    pub url_id: String,
    pub title: String,
    pub text: String,
    pub emoji: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: Option<String>,
    pub archived_at: Option<String>,
    pub deleted_at: Option<String>,
    pub created_by: User,
    pub updated_by: User,
    pub collection_id: Option<String>,
    pub parent_document_id: Option<String>,
    pub last_viewed_at: Option<String>,
    pub revision: u32,
    pub full_width: bool,
    pub template: bool,
    pub collaborators: Vec<User>,
}
```

### Collection Types

```rust
pub struct Collection {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub icon: Option<String>,
    pub permission: CollectionPermission,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub documents: Option<Vec<Document>>,
}
```

---

## Authentication

### API Token Authentication

The Outline API uses Bearer token authentication. Tokens can be created in the Outline settings under API.

```rust
use outline_api::OutlineClient;

let client = OutlineClient::new("https://outline.example.com")
    .with_token("your-api-token");
```

### Token Management

```rust
// Update token at runtime
client.set_token("new-token");

// Clear token
client.clear_token();

// Check if authenticated
if client.is_authenticated() {
    // Make API calls
}
```

---

## Error Handling

### Error Types

```rust
pub enum OutlineError {
    /// HTTP request failed
    RequestFailed(reqwest::Error),

    /// API returned an error response
    ApiError {
        status: u16,
        message: String,
    },

    /// Authentication failed or token invalid
    Unauthorized,

    /// Resource not found
    NotFound,

    /// Rate limit exceeded
    RateLimited {
        retry_after: Option<u64>,
    },

    /// Failed to parse response
    ParseError(serde_json::Error),

    /// Network connectivity issue
    NetworkError,
}
```

### Error Handling Example

```rust
use outline_api::{OutlineClient, OutlineError};

async fn fetch_document(client: &OutlineClient, id: &str) -> Result<Document, OutlineError> {
    match client.documents().info(id).await {
        Ok(doc) => Ok(doc),
        Err(OutlineError::NotFound) => {
            println!("Document not found");
            Err(OutlineError::NotFound)
        }
        Err(OutlineError::RateLimited { retry_after }) => {
            println!("Rate limited, retry after {:?} seconds", retry_after);
            Err(OutlineError::RateLimited { retry_after })
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        }
    }
}
```

---

## Feature Flags

### Available Features

```toml
[dependencies]
outline-api = { version = "0.1.0", features = ["collaboration"] }
```

- `collaboration`: Enable WebSocket and CRDT support for real-time editing
- `compression`: Enable gzip compression for API requests
- `native-tls`: Use native TLS instead of rustls
- `blocking`: Provide synchronous blocking API

### Conditional Compilation Example

```rust
#[cfg(feature = "collaboration")]
pub mod collaboration {
    pub use crate::collaboration::*;
}
```

---

## Usage Examples

### Creating a Document

```rust
use outline_api::{OutlineClient, CreateDocumentRequest};

let client = OutlineClient::new("https://outline.example.com")
    .with_token("your-api-token");

let request = CreateDocumentRequest {
    title: "My New Document".to_string(),
    text: "# Hello World\n\nThis is my document.".to_string(),
    collection_id: Some("collection-id".to_string()),
    parent_document_id: None,
    template_id: None,
    template: Some(false),
    publish: Some(true),
};

let document = client.documents().create(request).await?;
println!("Created document: {}", document.id);
```

### Searching Documents

```rust
use outline_api::{SearchDocumentsRequest, DateFilter};

let request = SearchDocumentsRequest {
    query: "rust programming".to_string(),
    collection_id: None,
    user_id: None,
    date_filter: Some(DateFilter::Month),
    include_archived: Some(false),
    include_drafts: Some(false),
    offset: None,
    limit: Some(10),
};

let results = client.documents().search(request).await?;
for doc in results.data {
    println!("{}: {}", doc.title, doc.url_id);
}
```

### Managing Collections

```rust
use outline_api::{CreateCollectionRequest, CollectionPermission};

let request = CreateCollectionRequest {
    name: "Engineering".to_string(),
    description: Some("Engineering documentation".to_string()),
    color: Some("#FF0000".to_string()),
    private: Some(false),
    permission: Some(CollectionPermission::ReadWrite),
};

let collection = client.collections().create(request).await?;
println!("Created collection: {}", collection.id);
```

---

## Testing Strategy

### Unit Tests

Each resource module should have unit tests for:
- Request serialization
- Response deserialization
- Error handling
- Builder patterns

### Integration Tests

Integration tests should cover:
- End-to-end API calls (with test server)
- Authentication flow
- Error scenarios (404, 401, 429)
- Pagination
- Rate limiting

### Example Integration Test

```rust
#[tokio::test]
async fn test_document_lifecycle() {
    let client = setup_test_client().await;

    // Create
    let doc = client.documents().create(test_doc_request()).await.unwrap();

    // Read
    let fetched = client.documents().info(&doc.id).await.unwrap();
    assert_eq!(doc.id, fetched.id);

    // Update
    let updated = client.documents().update(&doc.id, "New title").await.unwrap();
    assert_eq!(updated.title, "New title");

    // Delete
    client.documents().delete(&doc.id).await.unwrap();
}
```

---

## Roadmap

### Version 0.1.0 (Phase 1) - COMPLETED ✅
- ✅ Basic client architecture
- ✅ Authentication support
- ✅ Type definitions for core resources
- ✅ Modular type system with builder patterns
- ✅ Documents API - Core operations (6/28)
- ✅ Collections API - Basic listing (1/18)

### Version 0.2.0 (Phase 2) - IN PROGRESS ⏳
- ✅ Documents API - Organization operations (19/28 total)
  - ✅ Archive, unarchive, star, unstar, unpublish, templatize
  - ✅ Move, restore
  - ✅ List variants: viewed, drafts, templates
  - ✅ Collaboration: add_user, remove_user
  - ⏸️ Import/export (deferred - file handling complexity)

- ✅ Collections API - Extended operations (12/18 total)
  - ✅ CRUD: create, info, update, delete
  - ✅ Organization: move, list documents
  - ✅ Member management: add/remove users, add/remove groups, list memberships
  - ⏸️ Export operations (deferred - file handling complexity)

- ✅ Users API - Basic operations (3/15)
  - ✅ Profile: info, update
  - ✅ Administration: list
  - ⏸️ Advanced admin ops (deferred to Phase 3)

**Phase 2 Status: 34/61 operations (56%) - Essential operations complete**

### Version 0.3.0 (Phase 3) - PLANNED
- Comments API (8 operations)
- Groups API (9 operations)
- Shares API (6 operations)
- Attachments API (5 operations)
- Users API - Admin operations (suspend, activate, delete, promote, demote, invite)
- Complete test coverage for all operations

### Version 0.4.0 (Phase 4) - FUTURE
- WebSocket support (optional feature)
- CRDT synchronization (optional feature)
- Presence awareness
- Performance optimizations

---

## Contributing

When contributing to the implementation:

1. **Follow the phases**: Implement features in the order outlined above
2. **Add tests**: Every endpoint needs unit and integration tests
3. **Update documentation**: Keep this file in sync with implementation
4. **Use types**: Prefer strongly-typed requests/responses over generic maps
5. **Handle errors**: Use the `OutlineError` enum for all error cases

---

## References

- [Outline GitHub Repository](https://github.com/outline/outline)
- [Outline API Documentation](https://www.getoutline.com/developers)
- [Outline Server Routes](https://github.com/outline/outline/tree/main/server/routes/api)

---

**Last Updated**: 2025-10-10
**Version**: 0.1.0 (Draft)
