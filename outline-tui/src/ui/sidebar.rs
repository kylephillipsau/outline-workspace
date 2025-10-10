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
    let is_focused = app.focused_pane == FocusedPane::Sidebar;

    // Build list items from sidebar items
    let items: Vec<ListItem> = app
        .sidebar_items
        .iter()
        .map(|item| {
            let indent = "  ".repeat(item.indent_level());
            let icon = item.icon();
            let title = item.title();

            let line = Line::from(vec![
                Span::raw(indent),
                Span::raw(icon),
                Span::raw(" "),
                Span::raw(title),
            ]);

            ListItem::new(line)
        })
        .collect();

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
