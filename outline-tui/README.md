# outline-tui

An interactive terminal user interface (TUI) for Outline. Navigate your knowledge base with an Obsidian-inspired layout, browse collections and documents in a tree view, and view/edit documents—all from your terminal.

## Features

- **Obsidian-inspired Layout**: Sidebar navigation + document viewer
- **Tree View Navigation**: Browse collections and documents hierarchically
- **Document Viewer**: Read documents with full content display
- **Keyboard-driven**: Vim-style navigation (hjkl) and intuitive shortcuts
- **Real-time Updates**: Refresh data on demand
- **Dual-pane Interface**: Focus switching between sidebar and editor
- **Visual Indicators**: Emojis, icons, and focused pane highlighting

## Coming Soon

- Real-time collaborative editing with CRDT (Yjs)
- Markdown rendering and syntax highlighting
- Full-featured text editor with vim keybindings
- Search within documents
- Multi-pane document viewing
- Document creation and editing

## Installation

From the workspace root:

```bash
cargo build --release -p outline-tui
```

The binary will be at `../target/release/outline-tui`

## Quick Start

### 1. Configure Authentication

The TUI shares configuration with `outline-cli`. Set up your credentials:

```bash
outline-cli config set-instance https://outline.yourdomain.com
outline-cli auth set-token YOUR_API_TOKEN
```

### 2. Launch

```bash
outline-tui
```

### 3. Navigate

The TUI will load your collections and documents automatically.

## Layout

```
┌──────────────────────────────────────────────────┐
│ Outline TUI - Team Knowledge Base                │
├────────────┬─────────────────────────────────────┤
│            │ 📄 Document Title           [VIEW]  │
│ Sidebar    ├─────────────────────────────────────┤
│            │                                      │
│ 📚 Coll... │  Document content appears here      │
│ └─ 📄 Doc  │  with full text and markdown        │
│ └─ 📄 Doc  │                                      │
│            │  Scroll with ↑/↓ or j/k              │
│ 📚 Coll... │                                      │
│ └─ 📄 Doc  │                                      │
│            │                                      │
├────────────┴─────────────────────────────────────┤
│ ↑/↓: Navigate | Enter: Open | Tab: Switch | q: Quit │
└──────────────────────────────────────────────────┘
```

## Keybindings

### Global

- `q` - Quit application
- `Tab` - Switch focus between sidebar and editor

### Sidebar (when focused)

- `↑` / `k` - Move up one item
- `↓` / `j` - Move down one item
- `Page Up` - Jump up 10 items
- `Page Down` - Jump down 10 items
- `Home` - Jump to first item
- `End` - Jump to last item
- **Mouse wheel** - Scroll up/down
- `Enter` - Open selected document
- `r` - Refresh collections and documents
- Highlighted in **cyan** when focused

### Editor (when focused)

- `↑` / `k` - Scroll up one line
- `↓` / `j` - Scroll down one line
- `Page Up` - Scroll up one page (10 lines)
- `Page Down` - Scroll down one page (10 lines)
- `Home` - Jump to top of document
- `End` - Jump to bottom of document
- **Mouse wheel** - Scroll up/down
- `e` - Toggle edit mode (future feature)
- `Esc` - Return to view mode
- Highlighted in **cyan** when focused

## Configuration

The TUI uses the same configuration as the CLI, located at:
```
~/.config/outline-cli/config.toml
```

```toml
api_base_url = "https://outline.yourdomain.com"
```

Alternatively, set via environment variable:
```bash
export OUTLINE_API_URL="https://outline.yourdomain.com"
```

## Features in Detail

### Sidebar Navigation

- Displays all collections with icons
- Shows documents in hierarchical tree structure
- Nested documents are indented
- Visual selection indicator (`▶`) when focused
- Collections and documents show their emoji/icons

### Document Viewer

- Shows document title with emoji
- Displays full document content
- Mode indicator ([VIEW] or [EDIT])
- Scrollable content area
- Loading indicators for async operations

### Status Bar

- Context-sensitive help text
- Status messages (loading, errors, success)
- Different keybinding hints based on focused pane

## Performance

- Efficient async loading with Tokio
- Lazy document loading (only loads when selected)
- Minimal memory footprint
- Fast keyboard navigation with no lag

## Development

Run from source:
```bash
cargo run -p outline-tui
```

Run with debug logging:
```bash
RUST_LOG=debug cargo run -p outline-tui
```

Build for release:
```bash
cargo build --release -p outline-tui
```

## Troubleshooting

**"Error loading data: API token not found"**
- Run `outline-cli auth set-token <token>` first

**"Error loading data: API base URL not configured"**
- Run `outline-cli config set-instance <url>` first

**Terminal garbled after crash**
- Run `reset` in your terminal to restore it

**TUI doesn't display properly**
- Ensure your terminal supports UTF-8 and emojis
- Try a different terminal emulator
- Minimum recommended terminal size: 80x24

## Architecture

The TUI is built with:

- **ratatui** - Terminal UI framework
- **crossterm** - Cross-platform terminal manipulation
- **tokio** - Async runtime
- **outline-api** - Shared API client library

### Code Structure

```
src/
├── main.rs         # Entry point, event loop, key handling
├── app.rs          # Application state and business logic
├── config.rs       # Configuration management
└── ui/
    ├── mod.rs      # Main UI layout and rendering
    ├── sidebar.rs  # Sidebar component
    └── editor.rs   # Document viewer/editor component
```

## Future: Collaborative Editing

The TUI is designed with real-time collaboration in mind:

- **Yjs/CRDT** integration for conflict-free editing
- **WebSocket** connection to Outline's collaboration endpoint
- **Multi-cursor** support showing other users
- **Operational transformation** for seamless concurrent edits

## License

Part of the Outline Toolkit project.
