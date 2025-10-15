use crate::app::App;
use crate::modals::{ModalType, get_action_list};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Render modal dialogs
pub fn render_modal(f: &mut Frame, app: &App) {
    if !app.modal.is_open() {
        return;
    }

    match &app.modal.modal_type {
        ModalType::None => {}
        ModalType::ActionMenu => render_action_menu(f, app),
        ModalType::Help => render_help(f),
        ModalType::CommandInput => render_command_input(f),
        ModalType::TextInput { title, prompt, value, .. } => {
            render_text_input(f, title, prompt, value);
        }
        ModalType::MultiInput { title, fields, current_field, .. } => {
            render_multi_input(f, title, fields, *current_field);
        }
        ModalType::Confirmation { title, message, .. } => {
            render_confirmation(f, title, message);
        }
        ModalType::List { title, items, state } => {
            render_list(f, title, items, state);
        }
        ModalType::Message { title, message } => {
            render_message(f, title, message);
        }
        ModalType::AuthSetup { selected } => {
            render_auth_setup(f, *selected);
        }
    }
}

fn render_action_menu(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 70, f.area());

    // Clear the area
    f.render_widget(Clear, area);

    // Group actions by category
    let actions = get_action_list();
    let mut categories: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();

    for action in &actions {
        let category = action.category().to_string();
        let entry = format!("{}", action.description());
        categories.entry(category).or_insert_with(Vec::new).push(entry);
    }

    // Build list items
    let mut list_items = Vec::new();
    for (category, items) in categories {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("─── {} ───", category),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))));
        for item in items {
            list_items.push(ListItem::new(format!("  {}", item)));
        }
    }

    let list = List::new(list_items)
        .block(
            Block::default()
                .title(" Actions (↑/↓ navigate, Enter select, Esc close) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut app.modal.menu_state.clone());
}

fn render_help(f: &mut Frame) {
    let area = centered_rect(80, 80, f.area());

    f.render_widget(Clear, area);

    let help_text = vec![
        Line::from(Span::styled("Outline TUI - Help", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(Span::styled("Navigation", Style::default().fg(Color::Cyan))),
        Line::from("  Tab          - Switch between sidebar and editor"),
        Line::from("  ↑/↓ or j/k   - Navigate lists"),
        Line::from("  PageUp/Down  - Jump by page"),
        Line::from("  Home/End     - Jump to start/end"),
        Line::from("  Enter        - Open selected document"),
        Line::from("  r            - Refresh data"),
        Line::from("  q            - Quit application"),
        Line::from(""),
        Line::from(Span::styled("Actions", Style::default().fg(Color::Cyan))),
        Line::from("  m or :       - Open action menu"),
        Line::from("  h or ?       - Show this help"),
        Line::from("  Esc          - Close modal/menu"),
        Line::from(""),
        Line::from(Span::styled("Document Actions", Style::default().fg(Color::Cyan))),
        Line::from("  c            - Create document (as child of selected)"),
        Line::from("  e            - Edit document (in vim mode)"),
        Line::from("  u            - Update document"),
        Line::from("  d            - Delete document"),
        Line::from("  /            - Search documents"),
        Line::from("  a            - Archive document"),
        Line::from("  s            - Star document"),
        Line::from("  x            - Export document"),
        Line::from(""),
        Line::from(Span::styled("Vim Editing (in Edit mode)", Style::default().fg(Color::Cyan))),
        Line::from("  i/a          - Insert mode"),
        Line::from("  v            - Visual mode"),
        Line::from("  hjkl         - Movement"),
        Line::from("  u            - Undo | Ctrl+r - Redo"),
        Line::from("  Esc          - Save & exit to view"),
        Line::from(""),
        Line::from(Span::styled("Press Esc to close", Style::default().fg(Color::Gray))),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn render_command_input(f: &mut Frame) {
    let area = centered_rect(60, 10, f.area());

    f.render_widget(Clear, area);

    let paragraph = Paragraph::new("Command input not yet implemented")
        .block(
            Block::default()
                .title(" Command ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

    f.render_widget(paragraph, area);
}

fn render_text_input(f: &mut Frame, title: &str, prompt: &str, value: &str) {
    let area = centered_rect(60, 20, f.area());

    f.render_widget(Clear, area);

    let text = vec![
        Line::from(Span::styled(prompt, Style::default().fg(Color::Gray))),
        Line::from(""),
        Line::from(Span::raw(value)),
        Line::from(""),
        Line::from(Span::styled("Press Enter to submit, Esc to cancel", Style::default().fg(Color::Gray))),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

    f.render_widget(paragraph, area);
}

fn render_multi_input(f: &mut Frame, title: &str, fields: &[crate::modals::InputField], current_field: usize) {
    let area = centered_rect(70, 30 + fields.len() as u16 * 3, f.area());

    f.render_widget(Clear, area);

    let mut lines = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        let is_current = i == current_field;
        let style = if is_current {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        lines.push(Line::from(Span::styled(&field.label, style)));
        lines.push(Line::from(Span::raw(if field.value.is_empty() {
            format!("  [{}]", field.placeholder)
        } else {
            format!("  {}", field.value)
        })));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Tab: Next field | Shift+Tab: Previous | Enter: Submit | Esc: Cancel",
        Style::default().fg(Color::Gray),
    )));

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

    f.render_widget(paragraph, area);
}

fn render_confirmation(f: &mut Frame, title: &str, message: &str) {
    let area = centered_rect(50, 15, f.area());

    f.render_widget(Clear, area);

    let text = vec![
        Line::from(Span::raw(message)),
        Line::from(""),
        Line::from(Span::styled("Press Enter to confirm, Esc to cancel", Style::default().fg(Color::Gray))),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn render_list(f: &mut Frame, title: &str, items: &[String], state: &ratatui::widgets::ListState) {
    let area = centered_rect(70, 70, f.area());

    f.render_widget(Clear, area);

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|s| ListItem::new(s.as_str()))
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut state.clone());
}

fn render_message(f: &mut Frame, title: &str, message: &str) {
    let area = centered_rect(60, 30, f.area());

    f.render_widget(Clear, area);

    let paragraph = Paragraph::new(message)
        .block(
            Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn render_auth_setup(f: &mut Frame, selected: usize) {
    let area = centered_rect(70, 50, f.area());

    f.render_widget(Clear, area);

    let options = vec![
        ("1", "OAuth2 (Recommended)", "Automatic refresh, more secure"),
        ("2", "API Token", "Simple setup with manual token"),
        ("Q", "Exit", "Configure authentication via CLI"),
    ];

    let mut lines = vec![
        Line::from(Span::styled(
            "Authentication Required",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("No authentication configured. Choose a method:"),
        Line::from(""),
    ];

    for (i, (key, title, desc)) in options.iter().enumerate() {
        let style = if i == selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let marker = if i == selected { ">> " } else { "   " };

        lines.push(Line::from(vec![
            Span::raw(marker),
            Span::styled(*key, style.add_modifier(Modifier::BOLD)),
            Span::styled(". ", style),
            Span::styled(*title, style),
        ]));
        lines.push(Line::from(Span::styled(
            format!("     {}", desc),
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑/↓: Navigate | Enter/1/2: Select | Q: Quit",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Outline TUI - Setup ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
