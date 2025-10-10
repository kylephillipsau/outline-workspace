use crate::app::{App, EditorMode, FocusedPane};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use tui_markdown::from_str as markdown_from_str;

/// Render the document editor/viewer pane
pub fn render_editor(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.focused_pane == FocusedPane::Editor;

    // Split into title and content areas
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(0),     // Content
        ])
        .split(area);

    // Render title
    render_document_title(f, app, is_focused, chunks[0]);

    // Render content
    render_document_content(f, app, is_focused, chunks[1]);
}

/// Render the document title
fn render_document_title(f: &mut Frame, app: &App, is_focused: bool, area: Rect) {
    let title_text = if let Some(doc) = &app.current_document {
        let emoji = doc.emoji.as_deref().unwrap_or("📄");
        vec![
            Span::raw(emoji),
            Span::raw(" "),
            Span::styled(
                &doc.title,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]
    } else {
        vec![Span::styled(
            "No document selected",
            Style::default().fg(Color::Gray),
        )]
    };

    let mode_indicator = match app.editor_mode {
        EditorMode::View => Span::styled(" [VIEW]", Style::default().fg(Color::Green)),
        EditorMode::Edit => Span::styled(" [EDIT]", Style::default().fg(Color::Yellow)),
    };

    // Add collaboration status indicator
    let collab_indicator = {
        use outline_api::collaboration::ConnectionStatus;
        match &app.collaboration_status {
            ConnectionStatus::Connected => Span::styled(" [COLLAB]", Style::default().fg(Color::Cyan)),
            ConnectionStatus::Connecting => Span::styled(" [CONNECTING...]", Style::default().fg(Color::Yellow)),
            ConnectionStatus::Disconnected => Span::raw(""),
            ConnectionStatus::Error(e) => Span::styled(
                format!(" [ERROR: {}]", e),
                Style::default().fg(Color::Red)
            ),
        }
    };

    let mut title_line = title_text;
    title_line.push(mode_indicator);
    title_line.push(collab_indicator);

    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::Gray
    };

    let title = Paragraph::new(Line::from(title_line)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );

    f.render_widget(title, area);
}

/// Render the document content
fn render_document_content(f: &mut Frame, app: &App, is_focused: bool, area: Rect) {
    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::Gray
    };

    let content_text = if app.document_text.is_empty() {
        if app.current_document.is_some() {
            Text::from("Loading document content...")
        } else {
            Text::from("Select a document from the sidebar to view its content.")
        }
    } else {
        // Parse markdown and convert to ratatui Text
        markdown_from_str(&app.document_text)
    };

    let content = Paragraph::new(content_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Content ")
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll_offset, 0));

    f.render_widget(content, area);
}
