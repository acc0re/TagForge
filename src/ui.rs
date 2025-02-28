use app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use crate::app;

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

pub fn draw_select_file(app: &App, frame: &mut Frame) {
    let area = frame.area();

    // Layout für die verschiedenen Abschnitte
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(5),    // Eingabefeld
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Header zeichnen
    draw_header(frame);

    // Erstelle den Eingabe-Abschnitt mit Rahmen
    let input_block = Paragraph::new(app.input_buffer.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "Enter the path to the file you want to edit:",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: true });

    // Render das Eingabe-Widget in den vorgesehenen Bereich
    frame.render_widget(input_block, chunks[1]);

    // Das Footer-Info-Feld
    let footer = Paragraph::new(Line::from(vec![Span::styled(
        "Press Enter to confirm the path.",
        Style::default().fg(Color::Gray),
    )]))
        .block(Block::default().borders(Borders::TOP));

    // Render den Footer
    frame.render_widget(footer, chunks[2]);

    // **********************************
    // Cursor anzeigen
    // **********************************
    let cursor_x = chunks[1].x + app.input_buffer.len() as u16 + 1; // Cursor am Ende des Textes
    let cursor_y = chunks[1].y + 1; // Innerhalb des Eingabefeldes

    frame.set_cursor_position((cursor_x, cursor_y)); // Aktuelle Methode verwenden
}



pub fn draw_editor(frame: &mut Frame) {
    todo!()
}
