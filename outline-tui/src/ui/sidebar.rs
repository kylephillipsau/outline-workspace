use crate::app::{App, FocusedPane};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

/// Render the sidebar with collections and documents
pub fn render_sidebar(f: &mut Frame, app: &mut App, area: Rect) {
    // Store the sidebar area for mouse click detection
    app.sidebar_area = Some(area);

    let is_focused = app.focused_pane == FocusedPane::Sidebar;

    // Build list items from sidebar items with tree indicators
    let items: Vec<ListItem> = if app.is_loading && app.sidebar_items.is_empty() {
        // Show loading indicator when no items yet
        vec![
            ListItem::new(Line::from("")),
            ListItem::new(Line::from(Span::styled("Loading data...", Style::default().fg(Color::Yellow)))),
            ListItem::new(Line::from("")),
            ListItem::new(Line::from(Span::styled("Fetching collections and documents", Style::default().fg(Color::Gray)))),
            ListItem::new(Line::from(Span::styled("This may take a moment...", Style::default().fg(Color::DarkGray)))),
        ]
    } else {
        app.sidebar_items
            .iter()
            .map(|item| {
            let indent_level = item.indent_level();
            let icon = item.icon();
            let title = item.title();

            // Build display line with simple spacing
            let line = if indent_level == 0 {
                // Collections: icon + space + title
                Line::from(vec![
                    Span::raw(icon),
                    Span::raw("  "),
                    Span::raw(title),
                ])
            } else if indent_level == 1 {
                // Root documents: indent + icon + space + title
                Line::from(vec![
                    Span::raw("  "),
                    Span::raw(icon),
                    Span::raw("  "),
                    Span::raw(title),
                ])
            } else {
                // Child documents: indent + tree + icon + space + title
                let base_indent = "  ".repeat(indent_level - 1);
                Line::from(vec![
                    Span::raw(base_indent),
                    Span::styled("└─ ", Style::default().fg(Color::DarkGray)),
                    Span::raw(icon),
                    Span::raw("  "),
                    Span::raw(title),
                ])
            };

            ListItem::new(line)
            })
            .collect()
    };

    // Determine border color based on focus
    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::Gray
    };

    let title = if app.is_loading {
        " Collections & Documents (Loading...) "
    } else {
        " Collections & Documents "
    };

    // Create the list widget
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(border_color)),
        )
        .highlight_style(
            Style::default()
                .bg(if is_focused { Color::Blue } else { Color::DarkGray })
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(if is_focused { "▶ " } else { "  " });

    // Render the list with state
    f.render_stateful_widget(list, area, &mut app.sidebar_state);
}
