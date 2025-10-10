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

The library provides secure credential storage using the system keyring:

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
```

## API Client Methods

### Documents (19 operations)

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

**Collaboration:**
- `add_user_to_document()` - Grant user access to document
- `remove_user_from_document()` - Revoke user access from document

### Collections (12 operations)

**Core Operations:**
- `list_collections()` - List all collections
- `get_collection()` - Get collection by ID
- `create_collection()` - Create a new collection
- `update_collection()` - Update collection properties
- `delete_collection()` - Delete a collection

**Organization:**
- `move_collection()` - Move collection to different position
- `list_collection_documents()` - List documents in a collection

**Member Management:**
- `add_user_to_collection()` - Add user to collection
- `remove_user_from_collection()` - Remove user from collection
- `add_group_to_collection()` - Add group to collection
- `remove_group_from_collection()` - Remove group from collection
- `list_collection_memberships()` - List collection members

### Users (3 operations)

- `get_user()` - Get user information (current or by ID)
- `update_user()` - Update user profile
- `list_users()` - List all users with filters

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
