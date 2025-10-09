# Outline CLI - Quick Start Guide

## Building the Project

```bash
# Build in release mode for production use
cargo build --release

# The binary will be at:
# target/release/outline-cli.exe (Windows)
# target/release/outline-cli (Linux/macOS)
```

## Initial Setup

### 1. Configure Instance URL

```bash
outline-cli config set-instance https://outline.yourdomain.com
```

### 2. Get Your API Token

1. Log into your Outline instance
2. Go to Settings > API & Apps
3. Create a new API key
4. Copy the token

### 3. Store Your API Token

```bash
outline-cli auth set-token <your-api-token>
```

This stores the token securely in your system keyring (Windows Credential Manager on Windows).

## Common Commands

### Verify Setup

```bash
# Check auth status
outline-cli auth status

# View current config
outline-cli config show
```

### Working with Documents

```bash
# List all documents
outline-cli documents list

# List documents in a specific collection
outline-cli documents list --collection-id <collection-id>

# Get a specific document
outline-cli documents get <document-id>

# Get just the document text
outline-cli documents get <document-id> --text-only

# Search for documents
outline-cli documents search "search term"

# Create a new document
outline-cli documents create \
  --title "My Document" \
  --text "# Hello\n\nThis is my document" \
  --collection-id <collection-id> \
  --publish

# Update a document
outline-cli documents update <document-id> \
  --title "Updated Title" \
  --text "Updated content"

# Delete a document (archive)
outline-cli documents delete <document-id>

# Permanently delete a document
outline-cli documents delete <document-id> --permanent
```

### Working with Collections

```bash
# List all collections
outline-cli collections list
```

## Tips

1. **Finding Collection IDs**: Run `outline-cli collections list` to see all collection IDs
2. **Finding Document IDs**: Run `outline-cli documents list` or search to find document IDs
3. **Markdown Support**: The `--text` parameter supports full Markdown syntax
4. **Pagination**: Use `--offset` and `--limit` flags for pagination on list/search commands

## Getting Help

```bash
# General help
outline-cli --help

# Command-specific help
outline-cli documents --help
outline-cli documents create --help
```

## Troubleshooting

### "API token not found"

Run `outline-cli auth set-token <token>` to store your API token.

### "Instance URL not configured"

Run `outline-cli config set-instance <url>` to set your Outline instance URL.

### Authentication Errors

1. Check your token is valid: `outline-cli auth status`
2. Verify instance URL: `outline-cli config show`
3. Generate a new API token from your Outline instance

## Configuration Files

- **Config file**: `~/.outline-cli/config.toml` (stores instance URL and preferences)
- **API token**: Stored in system keyring (Windows Credential Manager, macOS Keychain, Linux Secret Service)

## Next Steps

Now that you have the basic CLI working, you can:

1. Integrate it into scripts for automation
2. Create document templates
3. Build workflows for documentation updates
4. Export documents for offline use

For OAuth2 authentication support and additional features, see the main README.md file.
