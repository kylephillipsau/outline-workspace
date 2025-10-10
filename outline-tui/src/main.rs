mod app;
mod config;
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
    // Initialize logging (writes to outline-tui.log if RUST_LOG is set)
    if std::env::var("RUST_LOG").is_ok() {
        let file = std::fs::File::create("outline-tui.log")?;
        tracing_subscriber::fmt()
            .with_writer(file)
            .with_ansi(false)
            .init();
        info!("Outline TUI starting...");
    }

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
    match app.focused_pane {
        FocusedPane::Sidebar => handle_sidebar_keys(app, key, modifiers).await?,
        FocusedPane::Editor => handle_editor_keys(app, key, modifiers).await?,
    }

    Ok(())
}

fn handle_mouse_event(app: &mut App, mouse: MouseEvent) -> Result<()> {
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
        _ => {}
    }
    Ok(())
}

async fn handle_sidebar_keys(
    app: &mut App,
    key: KeyCode,
    _modifiers: KeyModifiers,
) -> Result<()> {
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
        _ => {}
    }

    Ok(())
}

async fn handle_editor_keys(
    app: &mut App,
    key: KeyCode,
    _modifiers: KeyModifiers,
) -> Result<()> {
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
        _ => {}
    }

    Ok(())
}

/// Create an authenticated API client
fn create_api_client() -> Result<OutlineClient> {
    let config = Config::load()?;
    let api_base_url = config.get_api_base_url()?;
    let api_token = auth::get_api_token()?;
    Ok(OutlineClient::new(api_base_url)?.with_token(api_token))
}

async fn load_collections_and_documents(app: &mut App) -> Result<()> {
    app.is_loading = true;

    let client = create_api_client()?;

    // Load collections
    let collections_response = client.list_collections(None, None).await?;

    // Build sidebar items
    let mut sidebar_items = Vec::new();

    for collection in collections_response.data {
        sidebar_items.push(SidebarItem::Collection(collection.clone()));

        // Load documents for this collection
        if let Ok(docs_response) = client
            .list_documents(None, Some(collection.id.clone()), None, None, None, None, None, None, None)
            .await
        {
            for doc in docs_response.data {
                // Calculate indent level based on parent
                let indent_level = if doc.parent_document_id.is_some() {
                    2
                } else {
                    1
                };

                sidebar_items.push(SidebarItem::Document(doc, indent_level));
            }
        }
    }

    app.sidebar_items = sidebar_items;
    app.is_loading = false;

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

