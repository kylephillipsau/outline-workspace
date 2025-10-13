mod actions;
mod app;
mod config;
mod executor;
mod modals;
mod ui;

use anyhow::Result;
use app::{App, EditorMode, FocusedPane, SidebarItem};
use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEvent, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use outline_api::{auth, OutlineClient};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tracing::{info, debug};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging (always write to outline-tui.log)
    let log_file = std::fs::File::create("outline-tui.log")?;
    tracing_subscriber::fmt()
        .with_writer(log_file)
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .init();
    info!("Outline TUI starting...");

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    // Load initial data
    if let Err(e) = load_collections_and_documents(app).await {
        app.set_status(format!("Error loading data: {}", e));
    }

    loop {
        terminal.draw(|f| ui::render(f, app))?;

        // Process collaboration events
        app.process_collaboration_events();

        // Handle events (only process key press, not release)
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    // Only handle key press events, ignore release events
                    if key.kind == KeyEventKind::Press {
                        handle_key_event(app, key.code, key.modifiers).await?;
                    }
                }
                Event::Mouse(mouse) => {
                    handle_mouse_event(app, mouse)?;
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

async fn handle_key_event(
    app: &mut App,
    key: KeyCode,
    modifiers: KeyModifiers,
) -> Result<()> {
    // If a modal is open, handle modal keys first
    if app.modal.is_open() {
        handle_modal_keys(app, key, modifiers).await?;
    } else {
        // Normal key handling
        match app.focused_pane {
            FocusedPane::Sidebar => handle_sidebar_keys(app, key, modifiers).await?,
            FocusedPane::Editor => handle_editor_keys(app, key, modifiers).await?,
        }
    }

    Ok(())
}

async fn handle_modal_keys(
    app: &mut App,
    key: KeyCode,
    modifiers: KeyModifiers,
) -> Result<()> {
    use modals::ModalType;

    match &app.modal.modal_type {
        ModalType::ActionMenu => {
            match key {
                KeyCode::Esc => {
                    app.modal.close();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.modal.menu_previous();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.modal.menu_next();
                }
                KeyCode::Enter => {
                    if let Some(action) = app.modal.get_selected_action() {
                        app.modal.close();
                        execute_action_with_prompt(app, action).await?;
                    }
                }
                _ => {}
            }
        }
        ModalType::Help => {
            if matches!(key, KeyCode::Esc | KeyCode::Char('q')) {
                app.modal.close();
            }
        }
        ModalType::TextInput { .. } => {
            match key {
                KeyCode::Esc => {
                    app.modal.close();
                }
                KeyCode::Enter => {
                    if let Some(action) = app.modal.get_pending_action() {
                        if let Some(input) = app.modal.get_text_input() {
                            app.modal.close();
                            execute_action_direct(app, action, vec![input]).await?;
                        }
                    }
                }
                KeyCode::Backspace => {
                    app.modal.handle_backspace();
                }
                KeyCode::Char(c) => {
                    app.modal.handle_char(c);
                }
                _ => {}
            }
        }
        ModalType::MultiInput { .. } => {
            match key {
                KeyCode::Esc => {
                    app.modal.close();
                }
                KeyCode::Enter => {
                    if let Some(action) = app.modal.get_pending_action() {
                        if let Some(inputs) = app.modal.get_multi_input() {
                            app.modal.close();
                            execute_action_direct(app, action, inputs).await?;
                        }
                    }
                }
                KeyCode::Tab => {
                    if modifiers.contains(KeyModifiers::SHIFT) {
                        app.modal.previous_field();
                    } else {
                        app.modal.next_field();
                    }
                }
                KeyCode::Backspace => {
                    app.modal.handle_backspace();
                }
                KeyCode::Char(c) => {
                    app.modal.handle_char(c);
                }
                _ => {}
            }
        }
        ModalType::Confirmation { .. } => {
            match key {
                KeyCode::Esc | KeyCode::Char('n') => {
                    app.modal.close();
                }
                KeyCode::Enter | KeyCode::Char('y') => {
                    if let Some(action) = app.modal.get_pending_action() {
                        app.modal.close();
                        execute_action_direct(app, action, Vec::new()).await?;
                    }
                }
                _ => {}
            }
        }
        ModalType::List { .. } | ModalType::Message { .. } => {
            match key {
                KeyCode::Esc | KeyCode::Enter => {
                    app.modal.close();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.modal.menu_previous();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.modal.menu_next();
                }
                _ => {}
            }
        }
        _ => {
            if matches!(key, KeyCode::Esc) {
                app.modal.close();
            }
        }
    }

    Ok(())
}

fn handle_mouse_event(app: &mut App, mouse: MouseEvent) -> Result<()> {
    // Don't handle mouse events if a modal is open
    if app.modal.is_open() {
        return Ok(());
    }

    // Handle mouse wheel scrolling in both panes
    match mouse.kind {
        MouseEventKind::ScrollUp => {
            match app.focused_pane {
                FocusedPane::Sidebar => app.sidebar_previous(),
                FocusedPane::Editor => app.scroll_up(),
            }
        }
        MouseEventKind::ScrollDown => {
            match app.focused_pane {
                FocusedPane::Sidebar => app.sidebar_next(),
                FocusedPane::Editor => app.scroll_down(),
            }
        }
        MouseEventKind::Down(_) => {
            // Check if click is in the sidebar area (for item selection)
            if let Some(sidebar_area) = app.sidebar_area {
                // Check if the click is within the sidebar area
                if mouse.column >= sidebar_area.x
                    && mouse.column < sidebar_area.x + sidebar_area.width
                    && mouse.row >= sidebar_area.y
                    && mouse.row < sidebar_area.y + sidebar_area.height
                {
                    // Focus the sidebar if not already focused
                    app.focused_pane = FocusedPane::Sidebar;

                    // Calculate which item was clicked
                    // Account for: top border with title (1 line)
                    let header_offset = 1;

                    // Check if click is in the content area (below header, above bottom border)
                    if mouse.row >= sidebar_area.y + header_offset
                        && mouse.row < sidebar_area.y + sidebar_area.height - 1
                        && !app.sidebar_items.is_empty()
                    {
                        // Calculate the clicked item index
                        let clicked_row = mouse.row - sidebar_area.y - header_offset;

                        // Get the current scroll offset from the list state
                        let offset = app.sidebar_state.offset();
                        let item_index = (clicked_row as usize) + offset;

                        // Update selection if the index is valid
                        if item_index < app.sidebar_items.len() {
                            app.sidebar_state.select(Some(item_index));
                        }
                    }

                    return Ok(());
                }
            }

            // Handle left click to switch panes (if not in sidebar)
            // Get terminal size to calculate pane boundaries
            let (width, height) = crossterm::terminal::size()?;

            // Skip if click is in header (first 3 lines) or footer (last 3 lines)
            if mouse.row < 3 || mouse.row >= height - 3 {
                return Ok(());
            }

            // Calculate sidebar width (25% of total width)
            let sidebar_width = (width * 25) / 100;

            // Determine which pane was clicked based on X coordinate
            let new_pane = if mouse.column < sidebar_width {
                FocusedPane::Sidebar
            } else {
                FocusedPane::Editor
            };

            // Only change focus if clicking a different pane
            if app.focused_pane != new_pane {
                app.focused_pane = new_pane;
            }
        }
        _ => {}
    }
    Ok(())
}

async fn handle_sidebar_keys(
    app: &mut App,
    key: KeyCode,
    _modifiers: KeyModifiers,
) -> Result<()> {
    use actions::Action;

    match key {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.sidebar_previous();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.sidebar_next();
        }
        KeyCode::PageUp => {
            app.sidebar_page_up();
        }
        KeyCode::PageDown => {
            app.sidebar_page_down();
        }
        KeyCode::Home => {
            app.sidebar_home();
        }
        KeyCode::End => {
            app.sidebar_end();
        }
        KeyCode::Tab => {
            app.toggle_focus();
        }
        KeyCode::Enter => {
            // Load the selected document
            if let Some(item) = app.selected_sidebar_item() {
                if let SidebarItem::Document(doc, _) = item {
                    let doc_id = doc.id.clone();
                    if let Err(e) = load_document(app, doc_id).await {
                        app.set_status(format!("Error loading document: {}", e));
                    }
                }
            }
        }
        KeyCode::Char('r') => {
            // Refresh data
            app.set_status("Refreshing...".to_string());
            if let Err(e) = load_collections_and_documents(app).await {
                app.set_status(format!("Error refreshing: {}", e));
            } else {
                app.set_status("Refreshed!".to_string());
            }
        }
        // Keyboard shortcuts
        KeyCode::Char('m') | KeyCode::Char(':') => {
            app.modal.show_action_menu();
        }
        KeyCode::Char('h') | KeyCode::Char('?') => {
            app.modal.show_help();
        }
        KeyCode::Char('c') => {
            execute_action_with_prompt(app, Action::CreateDocument).await?;
        }
        KeyCode::Char('/') => {
            execute_action_with_prompt(app, Action::SearchDocuments).await?;
        }
        _ => {}
    }

    Ok(())
}

async fn handle_editor_keys(
    app: &mut App,
    key: KeyCode,
    _modifiers: KeyModifiers,
) -> Result<()> {
    use actions::Action;

    match key {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Tab => {
            app.toggle_focus();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.scroll_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.scroll_down();
        }
        KeyCode::PageUp => {
            app.scroll_page_up();
        }
        KeyCode::PageDown => {
            app.scroll_page_down();
        }
        KeyCode::Home => {
            app.scroll_to_top();
        }
        KeyCode::End => {
            app.scroll_to_bottom();
        }
        KeyCode::Char('e') => {
            app.toggle_editor_mode();
        }
        KeyCode::Esc => {
            if app.editor_mode == EditorMode::Edit {
                app.toggle_editor_mode();
            }
        }
        // Keyboard shortcuts
        KeyCode::Char('m') | KeyCode::Char(':') => {
            app.modal.show_action_menu();
        }
        KeyCode::Char('h') | KeyCode::Char('?') => {
            app.modal.show_help();
        }
        KeyCode::Char('u') => {
            execute_action_with_prompt(app, Action::UpdateDocument).await?;
        }
        KeyCode::Char('d') => {
            execute_action_with_prompt(app, Action::DeleteDocument).await?;
        }
        KeyCode::Char('a') => {
            execute_action_direct(app, Action::ArchiveDocument, Vec::new()).await?;
        }
        KeyCode::Char('s') => {
            execute_action_direct(app, Action::StarDocument, Vec::new()).await?;
        }
        KeyCode::Char('x') => {
            execute_action_with_prompt(app, Action::ExportDocument).await?;
        }
        _ => {}
    }

    Ok(())
}

/// Execute an action that requires user input
async fn execute_action_with_prompt(app: &mut App, action: actions::Action) -> Result<()> {
    use executor::{action_requires_input, get_input_fields_for_action};

    if action_requires_input(&action) {
        let fields = get_input_fields_for_action(&action, app);
        if fields.is_empty() {
            // No specific fields, use a confirmation
            app.modal.show_confirmation(
                "Confirm Action".to_string(),
                format!("Execute: {}", action.description()),
                action,
            );
        } else if fields.len() == 1 {
            // Single field input
            let field = &fields[0];
            app.modal.show_text_input(
                action.description().to_string(),
                field.label.clone(),
                action,
            );
        } else {
            // Multiple fields
            app.modal.show_multi_input(
                action.description().to_string(),
                fields,
                action,
            );
        }
    } else {
        // Execute directly
        execute_action_direct(app, action, Vec::new()).await?;
    }

    Ok(())
}

/// Execute an action directly with given input values
async fn execute_action_direct(app: &mut App, action: actions::Action, input_values: Vec<String>) -> Result<()> {
    use executor::execute_action;

    app.set_status("Executing...".to_string());

    let client = create_api_client()?;
    match execute_action(action.clone(), app, &client, input_values).await {
        Ok(message) => {
            app.modal.show_message("Success".to_string(), message);
            // Refresh data after certain actions
            if should_refresh_after_action(&action) {
                let _ = load_collections_and_documents(app).await;
            }
        }
        Err(e) => {
            app.modal.show_message("Error".to_string(), format!("Failed: {}", e));
        }
    }

    Ok(())
}

/// Check if we should refresh data after an action
fn should_refresh_after_action(action: &actions::Action) -> bool {
    use actions::Action;
    matches!(action,
        Action::CreateDocument | Action::DeleteDocument | Action::ArchiveDocument |
        Action::UnarchiveDocument | Action::CreateCollection | Action::DeleteCollection |
        Action::MoveDocument | Action::MoveCollection
    )
}

/// Create an authenticated API client
fn create_api_client() -> Result<OutlineClient> {
    let config = Config::load()?;
    let api_base_url = config.get_api_base_url()?;
    let api_token = auth::get_api_token()?;
    Ok(OutlineClient::new(api_base_url)?.with_token(api_token))
}

async fn load_collections_and_documents(app: &mut App) -> Result<()> {
    use outline_api::{ListCollectionsRequest, ListDocumentsRequest, Document};
    use std::collections::HashMap;

    app.is_loading = true;

    let client = create_api_client()?;

    // Load collections
    let request = ListCollectionsRequest::new();
    let collections_response = client.list_collections(request).await?;

    // Build sidebar items
    let mut sidebar_items = Vec::new();

    for collection in collections_response.data {
        sidebar_items.push(SidebarItem::Collection(collection.clone()));

        // Load documents for this collection
        let docs_request = ListDocumentsRequest {
            backlink_document_id: None,
            collection_id: Some(collection.id.clone()),
            direction: None,
            limit: None,
            offset: None,
            parent_document_id: None,
            sort: None,
            template: None,
            user_id: None,
        };

        if let Ok(docs_response) = client.list_documents(docs_request).await {
            debug!("Loaded {} documents from list API for collection {}", docs_response.data.len(), collection.id);

            // Fetch full document info to get emojis (simple approach)
            let mut docs_with_emoji = Vec::new();
            for doc in docs_response.data {
                debug!("Fetching full document info for: {} ({})", doc.title, doc.id);

                // Fetch full document to get emoji
                match client.get_document(doc.id.clone()).await {
                    Ok(full_doc) => {
                        debug!("  Got full doc - emoji: {:?}, title: {}", full_doc.emoji, full_doc.title);
                        docs_with_emoji.push(full_doc);
                    }
                    Err(e) => {
                        debug!("  Failed to get full doc: {}, using list version", e);
                        docs_with_emoji.push(doc); // Fallback to doc without emoji
                    }
                }
            }

            debug!("Finished loading full document info, building tree...");

            // Build tree structure
            let mut doc_map: HashMap<String, Document> = HashMap::new();
            let mut children_map: HashMap<Option<String>, Vec<String>> = HashMap::new();

            for doc in docs_with_emoji {
                let doc_id = doc.id.clone();
                let parent_id = doc.parent_document_id.clone();

                doc_map.insert(doc_id.clone(), doc);
                children_map.entry(parent_id).or_insert_with(Vec::new).push(doc_id);
            }

            // Find root documents (no parent or parent not in this collection)
            let roots: Vec<String> = doc_map
                .keys()
                .filter(|id| {
                    let doc = &doc_map[*id];
                    doc.parent_document_id.is_none() ||
                    !doc_map.contains_key(doc.parent_document_id.as_ref().unwrap())
                })
                .cloned()
                .collect();

            // Recursively add documents in tree order
            fn add_document_tree(
                doc_id: &str,
                doc_map: &HashMap<String, Document>,
                children_map: &HashMap<Option<String>, Vec<String>>,
                sidebar_items: &mut Vec<SidebarItem>,
                indent_level: usize,
            ) {
                if let Some(doc) = doc_map.get(doc_id) {
                    sidebar_items.push(SidebarItem::Document(doc.clone(), indent_level));

                    // Add children
                    if let Some(child_ids) = children_map.get(&Some(doc_id.to_string())) {
                        for child_id in child_ids {
                            add_document_tree(
                                child_id,
                                doc_map,
                                children_map,
                                sidebar_items,
                                indent_level + 1,
                            );
                        }
                    }
                }
            }

            // Add all root documents and their children
            for root_id in roots {
                add_document_tree(&root_id, &doc_map, &children_map, &mut sidebar_items, 1);
            }
        }
    }

    app.sidebar_items = sidebar_items;
    app.is_loading = false;

    // Log summary of what was loaded
    let doc_count = app.sidebar_items.iter().filter(|item| matches!(item, SidebarItem::Document(_, _))).count();
    let emoji_count = app.sidebar_items.iter().filter(|item| {
        if let SidebarItem::Document(doc, _) = item {
            doc.emoji.is_some()
        } else {
            false
        }
    }).count();

    info!("Loaded {} documents, {} have emojis", doc_count, emoji_count);

    Ok(())
}

async fn load_document(app: &mut App, doc_id: String) -> Result<()> {
    info!("Loading document: {}", doc_id);
    let client = create_api_client()?;

    debug!("Fetching document from API...");
    let document = client.get_document(doc_id.clone()).await?;
    app.document_text = document.text.clone();
    app.current_document = Some(document);
    app.scroll_offset = 0;
    info!("Document loaded successfully");

    // COLLABORATION DISABLED: Prevents UI freezing
    // The WebSocket connection blocks even with timeout, causing the TUI to freeze
    // when opening documents. Collaboration can be re-enabled once we implement
    // a proper non-blocking background task architecture.

    // TODO: Implement collaboration in a separate background task that doesn't block the UI

    Ok(())
}

