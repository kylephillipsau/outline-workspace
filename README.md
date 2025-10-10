# Outline Toolkit

A collection of Rust tools for interacting with the Outline documentation server. This workspace includes a shared API library, a command-line interface (CLI), and a terminal user interface (TUI) for managing your Outline knowledge base.

## Components

### 📚 outline-api
A shared Rust library providing:
- Outline API client implementation
- Type-safe API request/response models
- Secure credential management via system keyring
- Authentication helpers

Used by both the CLI and TUI applications.

### ⌨️ outline-cli
A command-line interface for Outline:
- Create, read, update, delete, list, and search documents
- Manage collections
- Scriptable and automation-friendly
- Full API coverage

[See outline-cli README](outline-cli/README.md)

### 🖥️ outline-tui
An interactive terminal user interface for Outline:
- Obsidian-inspired layout with sidebar navigation
- Browse collections and documents in a tree view
- View and edit documents with live preview
- Keyboard-driven navigation
- (Future: Real-time collaborative editing with CRDT support)

[See outline-tui README](outline-tui/README.md)

## Installation

### Prerequisites

- Rust 1.70 or later
- Access to an Outline instance (e.g., `https://outline.yourdomain.com`)
- An Outline API token (get this from Settings > API & Apps in your Outline instance)

### Build from source

Build all components:
```bash
cargo build --release --workspace
```

Or build individual components:
```bash
# CLI only
cargo build --release -p outline-cli

# TUI only
cargo build --release -p outline-tui
```

The binaries will be available in `target/release/`:
- `outline-cli` - Command-line interface
- `outline-tui` - Terminal user interface

## Quick Start

### 1. Configure your Outline instance

```bash
# For CLI
outline-cli config set-instance https://outline.yourdomain.com

# For TUI (shared config location)
# Edit ~/.config/outline-cli/config.toml
```

### 2. Set your API token

Get your API token from your Outline instance (Settings > API & Apps), then:

```bash
outline-cli auth set-token YOUR_API_TOKEN_HERE
```

The token will be stored securely in your system's keyring and is shared between CLI and TUI.

### 3. Run the applications

**CLI:**
```bash
# List all documents
outline-cli documents list

# Search for documents
outline-cli documents search "query"

# Get a specific document
outline-cli documents get DOCUMENT_ID
```

**TUI:**
```bash
# Launch the interactive interface
outline-tui
```

Navigate with:
- `↑/↓` or `j/k`: Navigate sidebar
- `Enter`: Open selected document
- `Tab`: Switch between sidebar and editor
- `r`: Refresh data
- `q`: Quit

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

Both CLI and TUI share configuration stored in `~/.config/outline-cli/config.toml`:

```toml
api_base_url = "https://outline.yourdomain.com"
```

API tokens are stored securely in your system's keyring and are never saved to disk in plain text.

## Workspace Structure

```
outline-cli/
├── Cargo.toml              # Workspace definition
├── outline-api/            # Shared API library
│   ├── src/
│   │   ├── client.rs       # API client implementation
│   │   ├── types.rs        # Request/response types
│   │   └── auth.rs         # Keyring authentication
│   └── Cargo.toml
├── outline-cli/            # Command-line interface
│   ├── src/
│   │   ├── commands/       # CLI command implementations
│   │   ├── config/         # Configuration management
│   │   └── main.rs         # CLI entry point
│   └── Cargo.toml
└── outline-tui/            # Terminal user interface
    ├── src/
    │   ├── app.rs          # Application state
    │   ├── ui/             # UI rendering components
    │   ├── config.rs       # Configuration
    │   └── main.rs         # TUI entry point
    └── Cargo.toml
```

## Future Enhancements

### outline-cli
- OAuth2 authentication flow
- Export documents to various formats (PDF, HTML, etc.)
- Batch operations
- Document templates
- More collection operations (create, update, delete)

### outline-tui
- Real-time collaborative editing with Yjs/CRDT
- Markdown syntax highlighting and rendering
- Full-featured text editor with vim keybindings
- Search within documents
- Multi-pane document viewing
- Inline image preview

## API Documentation

This tool is built using the official Outline API. For more details, see:
<https://www.getoutline.com/developers>

## License

This project is provided as-is for use with Outline documentation servers.
