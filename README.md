# Outline CLI

A command-line tool for interacting with the Outline documentation server API. This tool allows you to read, create, update, and delete documents in your Outline instance from the terminal.

## Features

- **Document Management**: Create, read, update, delete, list, and search documents
- **Collection Management**: List and view collections
- **Secure Authentication**: API tokens stored securely in system keyring
- **Configuration Management**: Easy setup and configuration
- **Full API Support**: Built on the official Outline API

## Installation

### Prerequisites

- Rust 1.70 or later
- Access to an Outline instance (e.g., <https://outline.yourdomain.com>)
- An Outline API token (get this from Settings > API & Apps in your Outline instance)

### Build from source

```bash
cargo build --release
```

The binary will be available at `target/release/outline-cli`

## Quick Start

### 1. Configure your Outline instance

```bash
outline-cli config set-instance https://outline.yourdomain.com
```

### 2. Set your API token

Get your API token from your Outline instance (Settings > API & Apps), then:

```bash
outline-cli auth set-token YOUR_API_TOKEN_HERE
```

The token will be stored securely in your system's keyring.

### 3. Verify authentication

```bash
outline-cli auth status
```

### 4. Start using the CLI

```bash
# List all documents
outline-cli documents list

# Search for documents
outline-cli documents search "query"

# Get a specific document
outline-cli documents get DOCUMENT_ID

# List collections
outline-cli collections list
```

## Usage

### Authentication Commands

```bash
# Set API token
outline-cli auth set-token <token>

# Check authentication status
outline-cli auth status

# Logout (clear credentials)
outline-cli auth logout
```

### Configuration Commands

```bash
# Set Outline instance URL
outline-cli config set-instance <url>

# Show current configuration
outline-cli config show
```

### Document Commands

```bash
# List documents
outline-cli documents list [--collection-id <id>] [--offset <n>] [--limit <n>]

# Get a document
outline-cli documents get <id> [--text-only]

# Create a document
outline-cli documents create --title "Title" --text "Content" [--collection-id <id>] [--publish]

# Update a document
outline-cli documents update <id> [--title "New Title"] [--text "New Content"] [--publish]

# Delete a document
outline-cli documents delete <id> [--permanent]

# Search documents
outline-cli documents search <query> [--collection-id <id>] [--offset <n>] [--limit <n>]
```

### Collection Commands

```bash
# List collections
outline-cli collections list [--offset <n>] [--limit <n>]
```

## Examples

### Creating a new document

```bash
outline-cli documents create \
  --title "Getting Started" \
  --text "# Welcome\n\nThis is a new document" \
  --collection-id abc123 \
  --publish
```

### Searching for documents

```bash
outline-cli documents search "installation guide"
```

### Getting document content (text only)

```bash
outline-cli documents get doc-id-123 --text-only
```

### Updating a document

```bash
outline-cli documents update doc-id-123 \
  --title "Updated Title" \
  --text "Updated content"
```

## Configuration

The CLI stores configuration in `~/.outline-cli/config.toml`:

```toml
instance_url = "https://outline.yourdomain.com"
output_format = "text"
```

API tokens are stored securely in your system's keyring and are never saved to disk in plain text.

## Architecture

The project is structured as follows:

- `src/api/` - API client and type definitions
- `src/auth/` - Authentication and credential management
- `src/commands/` - CLI command implementations
- `src/config/` - Configuration management
- `src/main.rs` - CLI entry point

## Future Enhancements

- OAuth2 authentication flow (in addition to API tokens)
- Export documents to various formats (PDF, HTML, etc.)
- Batch operations
- Interactive mode
- Document templates
- More collection operations (create, update, delete)

## API Documentation

This tool is built using the official Outline API. For more details, see:
<https://www.getoutline.com/developers>

## License

This project is provided as-is for use with Outline documentation servers.
