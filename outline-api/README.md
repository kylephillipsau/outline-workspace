# outline-api

A Rust library providing a type-safe client for the Outline API. This library is shared by both `outline-cli` and `outline-tui`.

## Features

- **Type-safe API client**: Full Rust types for all API requests and responses
- **Async/await support**: Built on `tokio` and `reqwest`
- **Secure authentication**: API tokens stored in system keyring
- **Comprehensive coverage**: Documents, collections, search, and more
- **Error handling**: Proper error propagation with `anyhow`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
outline-api = { path = "../outline-api" }
```

### Basic Example

```rust
use outline_api::{OutlineClient, auth};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Get stored API token from keyring
    let api_token = auth::get_api_token()?;

    // Create client
    let client = OutlineClient::new("https://outline.example.com".to_string())?
        .with_token(api_token);

    // List collections
    let collections = client.list_collections(None, None).await?;
    for collection in collections.data {
        println!("Collection: {}", collection.name);
    }

    // Get a document
    let document = client.get_document("doc-id".to_string()).await?;
    println!("Document: {}", document.title);

    Ok(())
}
```

### Authentication

The library supports two authentication methods:

#### Option 1: OAuth2 (Recommended)

OAuth2 provides automatic token refresh and better security:

```rust
use outline_api::auth::{self, OAuth2Config};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Configure OAuth2 credentials (one-time setup)
    let config = OAuth2Config {
        client_id: "your-client-id".to_string(),
        client_secret: "your-client-secret".to_string(),
        auth_url: "https://app.getoutline.com/oauth/authorize".to_string(),
        token_url: "https://app.getoutline.com/oauth/token".to_string(),
        redirect_url: "http://localhost:8080/callback".to_string(),
    };
    auth::set_oauth2_config(&config)?;

    // Start OAuth2 authorization flow (opens browser)
    let scopes = vec!["read".to_string(), "write".to_string()];
    let tokens = auth::oauth2_authorize(config, scopes).await?;

    println!("Access token stored! Expires at: {:?}", tokens.expires_at);

    // Create client with automatic auth (uses OAuth2 or API token)
    let client = OutlineClient::with_auto_auth("https://outline.example.com".to_string())?;

    // Tokens are automatically refreshed when expired
    let collections = client.list_collections(None, None).await?;

    Ok(())
}
```

**Creating OAuth2 Credentials:**
1. Go to your Outline instance → Settings → API & Apps
2. Create a new OAuth application
3. Set the redirect URL to: `http://localhost:8080/callback`
4. Copy the Client ID and Client Secret

#### Option 2: API Token (Legacy)

Manual API token management using the system keyring:

```rust
use outline_api::auth;

// Store API token
auth::set_api_token("your-api-token")?;

// Retrieve API token
let token = auth::get_api_token()?;

// Check if token exists
if auth::has_api_token() {
    println!("Authenticated");
}

// Delete token
auth::delete_api_token()?;

// Create client with manual token
let client = OutlineClient::new("https://outline.example.com".to_string())?
    .with_token(token);
```

#### Automatic Authentication

The client can automatically detect and use the appropriate authentication method:

```rust
// This will use OAuth2 if configured, otherwise falls back to API token
let client = OutlineClient::with_auto_auth("https://outline.example.com".to_string())?;

// Check which auth method is being used
match auth::get_auth_method() {
    auth::AuthMethod::OAuth2 => println!("Using OAuth2"),
    auth::AuthMethod::ApiToken => println!("Using API Token"),
    auth::AuthMethod::None => println!("Not authenticated"),
}
```

## API Client Methods

### Documents (21 operations)

**Core Operations:**
- `list_documents()` - List documents with optional filters
- `get_document()` - Get a specific document by ID
- `create_document()` - Create a new document
- `update_document()` - Update an existing document
- `delete_document()` - Delete or archive a document
- `search_documents()` - Search documents by query

**Organization:**
- `archive_document()` - Archive a document
- `unarchive_document()` - Restore from archive
- `star_document()` - Star a document for quick access
- `unstar_document()` - Remove star from a document
- `unpublish_document()` - Convert published document to draft
- `templatize_document()` - Convert document into a template
- `move_document()` - Move document to different collection/parent
- `restore_document()` - Restore from trash or to previous revision

**Listing Variants:**
- `list_viewed_documents()` - List recently viewed documents
- `list_drafts()` - List draft documents
- `list_templates()` - List template documents

**Import/Export:**
- `import_document()` - Import document from external formats (Markdown, HTML, Docx, Notion, Confluence)
- `export_document()` - Export document to Markdown, HTML, or PDF

**Collaboration:**
- `add_user_to_document()` - Grant user access to document
- `remove_user_from_document()` - Revoke user access from document

### Collections (15 operations)

**Core Operations:**
- `list_collections()` - List all collections
- `get_collection()` - Get collection by ID
- `create_collection()` - Create a new collection
- `update_collection()` - Update collection properties
- `delete_collection()` - Delete a collection

**Organization:**
- `move_collection()` - Move collection to different position
- `list_collection_documents()` - List documents in a collection

**Import/Export:**
- `export_collection()` - Export collection to Markdown, HTML, or PDF
- `export_all_collections()` - Export all collections to Markdown, HTML, or PDF
- `import_file_to_collection()` - Import files into collection (Markdown, HTML, Docx, Notion, Confluence)

**Member Management:**
- `add_user_to_collection()` - Add user to collection
- `remove_user_from_collection()` - Remove user from collection
- `add_group_to_collection()` - Add group to collection
- `remove_group_from_collection()` - Remove group from collection
- `list_collection_memberships()` - List collection members

### Users (9 operations)

**Profile Operations:**
- `get_user()` - Get user information (current or by ID)
- `update_user()` - Update user profile
- `list_users()` - List all users with filters

**Admin Operations:**
- `suspend_user()` - Suspend a user account
- `activate_user()` - Activate a suspended user
- `delete_user()` - Delete a user account
- `promote_user()` - Promote user to admin
- `demote_user()` - Demote admin to regular user
- `invite_user()` - Invite new user to team

### Comments (7 operations)

**Core Operations:**
- `create_comment()` - Create a comment on a document
- `get_comment()` - Get comment details
- `list_comments()` - List comments on a document
- `update_comment()` - Update a comment
- `delete_comment()` - Delete a comment

**Thread Management:**
- `resolve_comment()` - Mark a comment thread as resolved
- `unresolve_comment()` - Mark a comment thread as unresolved

### Groups (8 operations)

**Core Operations:**
- `create_group()` - Create a new group
- `get_group()` - Get group details
- `list_groups()` - List all groups
- `update_group()` - Update group properties
- `delete_group()` - Delete a group

**Member Management:**
- `add_user_to_group()` - Add a user to a group
- `remove_user_from_group()` - Remove a user from a group
- `list_group_memberships()` - List group members

### Shares (5 operations)

**Core Operations:**
- `create_share()` - Create a public share link
- `get_share()` - Get share details
- `list_shares()` - List all shares
- `update_share()` - Update share settings
- `revoke_share()` - Revoke a share link

### Attachments (4 operations)

**Core Operations:**
- `create_attachment()` - Upload a file attachment
- `delete_attachment()` - Delete an attachment
- `redirect_attachment()` - Get attachment download URL
- `list_attachments()` - List attachments

### Notifications (5 operations)

**Core Operations:**
- `list_notifications()` - List user notifications
- `update_notification()` - Update a notification (mark as read)
- `archive_notification()` - Archive a notification
- `unarchive_notification()` - Unarchive a notification
- `archive_all_notifications()` - Archive all notifications

### Events (1 operation)

**Audit Trail:**
- `list_events()` - List team events (audit log)

### Teams (2 operations)

**Core Operations:**
- `get_team()` - Get team information
- `update_team()` - Update team settings

### Real-Time Collaboration (Optional `collaboration` feature)

**WebSocket-based collaborative editing using Yrs CRDT:**

```rust
use outline_api::collaboration::{start_collaboration, CollaborationEvent};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (mut client, mut events) = start_collaboration(
        "https://outline.example.com".to_string(),
        api_token,
        document_id,
    ).await?;

    // Connect to the collaboration server
    client.connect().await?;

    // Handle collaboration events
    while let Some(event) = events.recv().await {
        match event {
            CollaborationEvent::DocumentUpdated(text) => {
                println!("Document updated: {}", text);
            }
            CollaborationEvent::StatusChanged(status) => {
                println!("Status: {:?}", status);
            }
            CollaborationEvent::UserJoined(user) => {
                println!("User joined: {}", user);
            }
            CollaborationEvent::UserLeft(user) => {
                println!("User left: {}", user);
            }
            CollaborationEvent::Error(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    Ok(())
}
```

**Enable the collaboration feature in your `Cargo.toml`:**

```toml
[dependencies]
outline-api = { path = "../outline-api", features = ["collaboration"] }
```

**Features:**
- Real-time document synchronization using Yjs CRDT
- Conflict-free collaborative editing
- WebSocket connection to Hocuspocus backend
- Presence awareness (user join/leave notifications)
- Automatic state synchronization

## Types

The library exports all necessary types:

```rust
use outline_api::{
    Document,
    Collection,
    User,
    DocumentsListResponse,
    DocumentSearchResponse,
    CollectionsListResponse,
};
```

## Error Handling

All methods return `Result<T>` using `anyhow::Error` for easy error propagation:

```rust
use anyhow::Result;

async fn example() -> Result<()> {
    let client = create_client()?;
    let doc = client.get_document("id".to_string()).await?;
    Ok(())
}
```

## Dependencies

- `reqwest` - HTTP client
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization
- `anyhow` - Error handling
- `keyring` - Secure credential storage

## Development

Run tests:
```bash
cargo test -p outline-api
```

Build:
```bash
cargo build -p outline-api
```

## License

Part of the Outline Toolkit project.
