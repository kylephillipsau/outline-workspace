use outline_api::{Collection, Document};
use outline_api::collaboration::{CollaborationClient, CollaborationEvent, ConnectionStatus, DocumentSync};
use ratatui::layout::Rect;
use ratatui::widgets::ListState;
use tokio::sync::mpsc;
use tui_textarea::TextArea;
use crate::modals::Modal;

/// Which pane is currently focused
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedPane {
    Sidebar,
    Editor,
}

/// Current mode of the editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    View,
    Edit,
}

/// Vim mode for text editing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
}

/// Application view state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    /// Authentication setup page
    AuthSetup,
    /// Main application view
    Main,
}

/// Auth setup selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthChoice {
    OAuth2,
    ApiToken,
    Exit,
}

/// Item in the sidebar tree
#[derive(Debug, Clone)]
pub enum SidebarItem {
    Collection(Collection),
    Document(Document, usize), // Document and its indent level
}

impl SidebarItem {
    pub fn title(&self) -> &str {
        match self {
            SidebarItem::Collection(c) => &c.name,
            SidebarItem::Document(d, _) => &d.title,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            SidebarItem::Collection(c) => c.icon(),
            SidebarItem::Document(d, _) => d.icon(),
        }
    }

    pub fn indent_level(&self) -> usize {
        match self {
            SidebarItem::Collection(_) => 0,
            SidebarItem::Document(_, level) => *level,
        }
    }
}

/// Application state
pub struct App {
    /// Whether the app should quit
    pub should_quit: bool,

    /// Current application view
    pub view: AppView,

    /// Currently focused pane
    pub focused_pane: FocusedPane,

    /// Editor mode
    pub editor_mode: EditorMode,

    /// Auth setup selected option
    pub auth_selected: usize,

    /// API token input buffer (for auth setup)
    pub api_token_input: String,

    /// Text editor widget (for edit mode)
    pub textarea: TextArea<'static>,

    /// Vim mode state
    pub vim_mode: VimMode,

    /// Sidebar items (collections and documents in tree order)
    pub sidebar_items: Vec<SidebarItem>,

    /// Sidebar list state for navigation
    pub sidebar_state: ListState,

    /// Currently loaded document
    pub current_document: Option<Document>,

    /// Document text content
    pub document_text: String,

    /// Scroll offset for document viewer
    pub scroll_offset: u16,

    /// Status message to display
    pub status_message: Option<String>,

    /// Whether data is currently loading
    pub is_loading: bool,

    /// Collaboration client (if enabled for current document)
    #[allow(dead_code)]
    pub collaboration_client: Option<CollaborationClient>,

    /// Collaboration event receiver
    #[allow(dead_code)]
    pub collaboration_rx: Option<mpsc::Receiver<CollaborationEvent>>,

    /// Document sync handler
    #[allow(dead_code)]
    pub document_sync: Option<DocumentSync>,

    /// Collaboration connection status
    #[allow(dead_code)]
    pub collaboration_status: ConnectionStatus,

    /// Modal dialog state
    pub modal: Modal,

    /// Sidebar rendered area (for mouse click detection)
    pub sidebar_area: Option<Rect>,

    /// Pending document creation context (parent_id, collection_id)
    pub pending_doc_create: Option<(Option<String>, Option<String>)>,
}

impl App {
    pub fn new() -> Self {
        let mut sidebar_state = ListState::default();
        sidebar_state.select(Some(0));

        Self {
            should_quit: false,
            view: AppView::Main,
            focused_pane: FocusedPane::Sidebar,
            editor_mode: EditorMode::View,
            auth_selected: 0,
            api_token_input: String::new(),
            textarea: TextArea::default(),
            vim_mode: VimMode::Normal,
            sidebar_items: Vec::new(),
            sidebar_state,
            current_document: None,
            document_text: String::new(),
            scroll_offset: 0,
            status_message: None,
            is_loading: false,
            collaboration_client: None,
            collaboration_rx: None,
            document_sync: None,
            collaboration_status: ConnectionStatus::Disconnected,
            modal: Modal::new(),
            sidebar_area: None,
            pending_doc_create: None,
        }
    }

    /// Handle navigation within the sidebar
    pub fn sidebar_next(&mut self) {
        if self.sidebar_items.is_empty() {
            return;
        }

        let i = match self.sidebar_state.selected() {
            Some(i) => {
                if i >= self.sidebar_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.sidebar_state.select(Some(i));
    }

    pub fn sidebar_previous(&mut self) {
        if self.sidebar_items.is_empty() {
            return;
        }

        let i = match self.sidebar_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.sidebar_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.sidebar_state.select(Some(i));
    }

    /// Jump down by a page (10 items)
    pub fn sidebar_page_down(&mut self) {
        if self.sidebar_items.is_empty() {
            return;
        }

        let i = match self.sidebar_state.selected() {
            Some(i) => (i + 10).min(self.sidebar_items.len() - 1),
            None => 0,
        };
        self.sidebar_state.select(Some(i));
    }

    /// Jump up by a page (10 items)
    pub fn sidebar_page_up(&mut self) {
        if self.sidebar_items.is_empty() {
            return;
        }

        let i = match self.sidebar_state.selected() {
            Some(i) => i.saturating_sub(10),
            None => 0,
        };
        self.sidebar_state.select(Some(i));
    }

    /// Jump to first item
    pub fn sidebar_home(&mut self) {
        if !self.sidebar_items.is_empty() {
            self.sidebar_state.select(Some(0));
        }
    }

    /// Jump to last item
    pub fn sidebar_end(&mut self) {
        if !self.sidebar_items.is_empty() {
            self.sidebar_state.select(Some(self.sidebar_items.len() - 1));
        }
    }

    /// Get the currently selected sidebar item
    pub fn selected_sidebar_item(&self) -> Option<&SidebarItem> {
        self.sidebar_state
            .selected()
            .and_then(|i| self.sidebar_items.get(i))
    }

    /// Switch focus between panes
    pub fn toggle_focus(&mut self) {
        self.focused_pane = match self.focused_pane {
            FocusedPane::Sidebar => FocusedPane::Editor,
            FocusedPane::Editor => FocusedPane::Sidebar,
        };
    }

    /// Toggle editor mode
    pub fn toggle_editor_mode(&mut self) {
        self.editor_mode = match self.editor_mode {
            EditorMode::View => EditorMode::Edit,
            EditorMode::Edit => EditorMode::View,
        };
    }

    /// Scroll document view down
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    /// Scroll document view up
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Scroll down by a page (10 lines)
    pub fn scroll_page_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(10);
    }

    /// Scroll up by a page (10 lines)
    pub fn scroll_page_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(10);
    }

    /// Scroll to top of document
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    /// Scroll to bottom of document
    pub fn scroll_to_bottom(&mut self) {
        // Set to a large value; the paragraph widget will clamp it automatically
        self.scroll_offset = u16::MAX;
    }

    /// Set status message
    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
    }

    /// Load document text into the textarea editor
    pub fn load_text_into_editor(&mut self) {
        let lines: Vec<String> = self.document_text.lines().map(|s| s.to_string()).collect();
        self.textarea = if lines.is_empty() {
            TextArea::default()
        } else {
            TextArea::new(lines)
        };
        self.vim_mode = VimMode::Normal;
    }

    /// Get text from the textarea editor
    pub fn get_text_from_editor(&self) -> String {
        self.textarea.lines().join("\n")
    }

    /// Start collaboration for the current document
    #[allow(dead_code)]
    pub async fn start_collaboration(
        &mut self,
        api_base_url: String,
        api_token: String,
        document_id: String,
    ) -> anyhow::Result<()> {
        use outline_api::collaboration::start_collaboration;

        // Stop any existing collaboration
        self.stop_collaboration().await;

        // Create document sync
        let doc_sync = DocumentSync::new();

        // Initialize sync with current document text
        doc_sync.set_text(&self.document_text)?;

        // Start collaboration client
        let (mut client, rx) = start_collaboration(api_base_url, api_token, document_id).await?;

        // Connect to WebSocket
        client.connect().await?;

        // Store collaboration state
        self.collaboration_client = Some(client);
        self.collaboration_rx = Some(rx);
        self.document_sync = Some(doc_sync);
        self.collaboration_status = ConnectionStatus::Connecting;

        Ok(())
    }

    /// Stop collaboration
    #[allow(dead_code)]
    pub async fn stop_collaboration(&mut self) {
        if let Some(client) = &self.collaboration_client {
            let _ = client.disconnect().await;
        }
        self.collaboration_client = None;
        self.collaboration_rx = None;
        self.document_sync = None;
        self.collaboration_status = ConnectionStatus::Disconnected;
    }

    /// Process collaboration events (call this regularly in event loop)
    pub fn process_collaboration_events(&mut self) {
        // Collect all available events first to avoid borrow checker issues
        let mut events = Vec::new();
        if let Some(rx) = &mut self.collaboration_rx {
            while let Ok(event) = rx.try_recv() {
                events.push(event);
            }
        }

        // Process collected events
        for event in events {
            match event {
                CollaborationEvent::StatusChanged(status) => {
                    self.collaboration_status = status.clone();
                    self.set_status(format!("Collaboration: {:?}", status));
                }
                CollaborationEvent::DocumentUpdated(content) => {
                    self.document_text = content.clone();

                    // If in edit mode, update the textarea with remote changes
                    if self.editor_mode == EditorMode::Edit {
                        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                        self.textarea = if lines.is_empty() {
                            TextArea::default()
                        } else {
                            TextArea::new(lines)
                        };
                    }

                    if let Some(doc_sync) = &self.document_sync {
                        if let Ok(synced_text) = doc_sync.get_text() {
                            self.document_text = synced_text;
                        }
                    }
                }
                CollaborationEvent::UserJoined(user) => {
                    self.set_status(format!("User joined: {}", user));
                }
                CollaborationEvent::UserLeft(user) => {
                    self.set_status(format!("User left: {}", user));
                }
                CollaborationEvent::Error(err) => {
                    self.set_status(format!("Collaboration error: {}", err));
                }
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
