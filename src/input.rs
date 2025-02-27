use crate::app::{App, AppState};
use crossterm::event::{self, KeyCode, KeyEvent};
use std::{io, path::PathBuf};

pub fn handle_input(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(200))? {
        if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
            match app.state {
                AppState::MainMenu => match code {
                    KeyCode::Char('o') => {
                        println!("Please insert file path:");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        let trimmed = input.trim();
                        if !trimmed.is_empty() {
                            app.file_path = Some(PathBuf::from(trimmed));
                            app.state = AppState::Editor;
                        }
                    }
                    KeyCode::Char('q') => app.running = false,
                    _ => {}
                },
                AppState::Editor => match code {
                    KeyCode::Char('q') => app.state = AppState::MainMenu,
                    _ => {}
                },
                AppState::Exiting => {}
                _ => {}
            }
        }
    }
    Ok(())
}
