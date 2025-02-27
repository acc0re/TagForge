use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw_menu(frame: &mut Frame) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new(Line::from(vec![
        Span::styled("TagForge - XML Editor", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]))
        .block(Block::default().borders(Borders::BOTTOM));

    let menu_options = Paragraph::new(vec![
        Line::from(Span::styled("(o) Open file", Style::default().fg(Color::LightBlue))),
        Line::from(Span::styled("(q) Exit", Style::default().fg(Color::Red))),
    ])
        .block(Block::default()
            .borders(Borders::ALL)
            .title(Span::styled("Main", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)))
        )
        .wrap(Wrap { trim: true });

    // Footer-Info
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("A terminal-based XML editor for efficient document management.", Style::default().fg(Color::Gray)),
    ]))
        .block(Block::default().borders(Borders::TOP));

    // Rendering
    frame.render_widget(header, chunks[0]);
    frame.render_widget(menu_options, chunks[1]);
    frame.render_widget(footer, chunks[2]);
}
