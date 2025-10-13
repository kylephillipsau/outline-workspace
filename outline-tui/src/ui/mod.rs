pub mod sidebar;
pub mod editor;
pub mod modal;

use crate::app::{App, FocusedPane};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub use sidebar::render_sidebar;
pub use editor::render_editor;
pub use modal::render_modal;

/// Render the entire UI
pub fn render(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Create the main layout: header, content, footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    // Render header
    render_header(f, chunks[0]);

    // Split content into sidebar and editor
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),  // Sidebar
            Constraint::Percentage(75),  // Editor
        ])
        .split(chunks[1]);

    // Render sidebar and editor
    render_sidebar(f, app, content_chunks[0]);
    render_editor(f, app, content_chunks[1]);

    // Render footer
    render_footer(f, app, chunks[2]);

    // Render modal on top of everything
    render_modal(f, app);
}

/// Render the header
fn render_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled("Outline ", Style::default().fg(Color::Cyan)),
        Span::styled(
            "TUI",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" - "),
        Span::styled(
            "Team Knowledge Base",
            Style::default().fg(Color::Gray),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(title, area);
}

/// Render the footer with help text and status
fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.focused_pane {
        FocusedPane::Sidebar => {
            "↑/↓: Navigate | Enter: Open | Tab: Switch pane | q: Quit"
        }
        FocusedPane::Editor => {
            "Tab: Switch pane | e: Edit mode | Esc: View mode | ↑/↓: Scroll | q: Quit"
        }
    };

    let status_text = app
        .status_message
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("");

    let footer_text = if !status_text.is_empty() {
        format!("{} | {}", help_text, status_text)
    } else {
        help_text.to_string()
    };

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}
