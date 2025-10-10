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

### Documents

- `list_documents()` - List documents with optional filters
- `get_document()` - Get a specific document by ID
- `create_document()` - Create a new document
- `update_document()` - Update an existing document
- `delete_document()` - Delete or archive a document
- `search_documents()` - Search documents by query

### Collections

- `list_collections()` - List all collections
- More collection operations coming soon

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
