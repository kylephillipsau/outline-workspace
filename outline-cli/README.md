# outline-cli

A command-line interface for interacting with the Outline documentation server. Manage documents, collections, and configuration directly from your terminal.

## Features

- **Document Management**: Create, read, update, delete, list, and search documents
- **Collection Management**: List and view collections
- **Secure Authentication**: API tokens stored in system keyring
- **Scriptable**: Perfect for automation and CI/CD workflows
- **Tree View**: Visual document hierarchy with emojis
- **Full API Coverage**: Complete access to Outline's API

## Installation

From the workspace root:

```bash
cargo build --release -p outline-cli
```

The binary will be at `../target/release/outline-cli`

## Quick Start

### 1. Configure your Outline instance

```bash
outline-cli config set-instance https://outline.yourdomain.com
```

### 2. Authenticate

Get your API token from Outline (Settings > API & Apps):

```bash
outline-cli auth set-token YOUR_API_TOKEN
```

### 3. Verify setup

```bash
outline-cli auth status
```

### 4. Start using

```bash
# List documents in tree view
outline-cli documents list

# Search for documents
outline-cli documents search "installation guide"

# Get specific document
outline-cli documents get <document-id>
```

## Commands

### Authentication

```bash
# Set API token (stored in system keyring)
outline-cli auth set-token <token>

# Check authentication status
outline-cli auth status

# Clear credentials
outline-cli auth logout
```

### Configuration

```bash
# Set Outline instance URL
outline-cli config set-instance <url>

# Show current configuration
outline-cli config show
```

### Documents

```bash
# List all documents
outline-cli documents list

# List with filters
outline-cli documents list --collection-id <id> --limit 50

# Get a document
outline-cli documents get <id>

# Get only document text (useful for piping)
outline-cli documents get <id> --text-only

# Create a document
outline-cli documents create \
  --title "My Document" \
  --text "# Content\n\nDocument text here" \
  --collection-id <id> \
  --publish

# Update a document
outline-cli documents update <id> \
  --title "Updated Title" \
  --text "Updated content"

# Delete a document
outline-cli documents delete <id>

# Permanently delete (cannot be undone)
outline-cli documents delete <id> --permanent

# Search documents
outline-cli documents search "query"
outline-cli documents search "query" --collection-id <id>
```

### Collections

```bash
# List all collections
outline-cli collections list

# List with pagination
outline-cli collections list --offset 10 --limit 25
```

## Examples

### Create a document from a file

```bash
outline-cli documents create \
  --title "Installation Guide" \
  --text "$(cat installation.md)" \
  --collection-id abc123 \
  --publish
```

### Pipe document content

```bash
# Get document text and process it
outline-cli documents get doc-123 --text-only | grep "TODO"

# Save document to file
outline-cli documents get doc-123 --text-only > document.md
```

### Batch operations with shell scripts

```bash
#!/bin/bash
# Search and list matching documents
for doc in $(outline-cli documents search "migration" | jq -r '.id'); do
  echo "Processing: $doc"
  outline-cli documents get $doc --text-only
done
```

## Output

The CLI outputs structured data that's easy to parse:

- Document lists show hierarchical tree view with indentation
- Collection lists show icons and metadata
- Search results include relevance scores
- All outputs are human-readable by default

## Configuration File

Located at `~/.config/outline-cli/config.toml`:

```toml
api_base_url = "https://outline.yourdomain.com"
```

You can also set the API URL via environment variable:
```bash
export OUTLINE_API_URL="https://outline.yourdomain.com"
```

## Security

- API tokens are **never** stored in plain text
- Credentials are stored in system keyring (Windows Credential Manager, macOS Keychain, Linux Secret Service)
- Configuration file contains no sensitive data

## Development

Run from source:
```bash
cargo run -p outline-cli -- documents list
```

Run tests:
```bash
cargo test -p outline-cli
```

## Troubleshooting

**"API token not found in keyring"**
- Run `outline-cli auth set-token <token>` to store your token

**"API base URL not configured"**
- Run `outline-cli config set-instance <url>` to set your Outline URL

**"Failed to create keyring entry"**
- Ensure your system keyring service is running
- On Linux, you may need `gnome-keyring` or `kwallet`

## License

Part of the Outline Toolkit project.
