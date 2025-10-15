use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render the authentication setup page
pub fn render_auth_page(f: &mut Frame, app: &App) {
    let area = f.area();

    // Create a centered layout
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(area);

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(outer_layout[1]);

    let content_area = inner_layout[1];

    // Build content based on auth_selected state
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Outline TUI", Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Authentication Required",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("No authentication configured. Choose a method:"),
        Line::from(""),
    ];

    // Options
    let options = [
        ("OAuth2 (Recommended)", "Automatic refresh, more secure"),
        ("API Token", "Simple setup with manual token"),
        ("Exit", "Configure via CLI instead"),
    ];

    for (i, (title, desc)) in options.iter().enumerate() {
        let is_selected = i == app.auth_selected;
        let style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let marker = if is_selected { "  ▶ " } else { "    " };
        let number = format!("{}. ", i + 1);

        lines.push(Line::from(vec![
            Span::raw(marker),
            Span::styled(number, style.add_modifier(Modifier::BOLD)),
            Span::styled(*title, style),
        ]));
        lines.push(Line::from(vec![
            Span::raw("      "),
            Span::styled(*desc, Style::default().fg(Color::Gray)),
        ]));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Navigation:",
        Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  ↑/↓ or j/k    Navigate options",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        "  Enter or 1/2  Select option",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        "  q             Quit",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, content_area);
}

/// Render API token input screen
pub fn render_api_token_input(f: &mut Frame, app: &App) {
    let area = f.area();

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(area);

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(outer_layout[1]);

    let content_area = inner_layout[1];

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("API Token Setup", Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from("Enter your Outline API token:"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Token: ", Style::default().fg(Color::Gray)),
            Span::styled(&app.api_token_input, Style::default().fg(Color::White)),
            Span::styled("_", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Get your API token from Outline settings:",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  https://app.getoutline.com/settings/tokens",
            Style::default().fg(Color::Blue),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Navigation:",
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "  Enter         Save token and continue",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  Esc           Cancel and go back",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, content_area);
}
