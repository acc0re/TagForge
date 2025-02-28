use crate::app::{App, AppState};
use crossterm::event::{self, KeyCode, KeyEvent};
use std::{io, path::PathBuf};

pub fn handle_input(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(200))? {
        if let event::Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == crossterm::event::KeyEventKind::Press {
                match app.state {
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
                        KeyCode::Enter => {
                            app.state = AppState::Editor;
                            app.file_path = Some(PathBuf::from("test.xml"));
                        }
                        KeyCode::Char('q') => app.state = AppState::MainMenu,
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
