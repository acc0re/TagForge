use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw_header(frame: &mut Frame) {
    let header = Paragraph::new(Line::from(vec![Span::styled(
        "TagForge - XML Editor",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]))
    .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(header, frame.area());
}

pub fn draw_main_menu(frame: &mut Frame) {

    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(5),    // Menu
            Constraint::Length(3), // Footer
        ])
        .split(area);

    draw_header(frame);

    let menu_options = Paragraph::new(vec![
        Line::from(Span::styled(
            "(o) Open file",
            Style::default().fg(Color::LightBlue),
        )),
        Line::from(Span::styled("(q) Exit", Style::default().fg(Color::Red))),
    ])
    .block(
        Block::default().borders(Borders::ALL).title(Span::styled(
            "Main Menu",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
    )
    .wrap(Wrap { trim: true });

    // Footer-Info
    let footer = Paragraph::new(Line::from(vec![Span::styled(
        "A terminal-based XML editor for efficient document management.",
        Style::default().fg(Color::Gray),
    )]))
    .block(Block::default().borders(Borders::TOP));

    // Rendering
    frame.render_widget(menu_options, chunks[1]);
    frame.render_widget(footer, chunks[2]);
}

pub fn draw_select_file(frame: &mut Frame) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(5),    // Input
            Constraint::Length(3), // Footer
        ])
        .split(area);

    draw_header(frame);

    let input_prompt = Paragraph::new(vec![Line::from(Span::styled(
        "Enter the file path:",
        Style::default().fg(Color::LightBlue),
    ))])
    .block(
        Block::default().borders(Borders::ALL).title(Span::styled(
            "Select File",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
    )
    .wrap(Wrap { trim: true });

    // Footer-Info
    let footer = Paragraph::new(Line::from(vec![Span::styled(
        "Press Enter to confirm the path.",
        Style::default().fg(Color::Gray),
    )]))
    .block(Block::default().borders(Borders::TOP));

    // Rendering
    frame.render_widget(input_prompt, chunks[1]);
    frame.render_widget(footer, chunks[2]);
}

pub fn draw_editor(frame: &mut Frame) {
    todo!()
}
