use crate::app::{App, AppState};
use crossterm::event::{self, KeyCode, KeyEvent};
use std::{io, path::PathBuf};

pub fn handle_input(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(200))? {
        if let event::Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == crossterm::event::KeyEventKind::Press {
                match &app.state {
                    AppState::MainMenu => match code {
                        KeyCode::Char('o') => app.state = AppState::SelectFile,
                        KeyCode::Char('q') => app.running = false,
                        _ => {}
                    },
                    AppState::Editor => match code {
                        KeyCode::Char('q') => app.state = AppState::MainMenu,
                        _ => {}
                    },
                    AppState::SelectFile => match code {
                        KeyCode::Char(c) => {
                            // Charakter zur Eingabe hinzufügen
                            app.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            // Entfernt das letzte Zeichen
                            app.input_buffer.pop();
                        }
                        KeyCode::Enter => {
                            // Pfad aus `input_buffer` übernehmen und den Status ändern
                            if !app.input_buffer.is_empty() {
                                app.file_path = Some(PathBuf::from(app.input_buffer.clone()));
                                app.input_buffer.clear(); // Buffer leeren
                                app.state = AppState::Editor; // Zum Editor wechseln
                            }
                        }
                        KeyCode::Esc | KeyCode::Char('q') => {
                            // Zurück zum Main Menu
                            app.input_buffer.clear();
                            app.state = AppState::MainMenu;
                        }
                        _ => {}
                    },
                    AppState::Exiting => {}
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
