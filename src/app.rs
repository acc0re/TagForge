use crate::{input, ui, xml};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{io, path::PathBuf};

pub enum AppState {
    MainMenu,
    Editor,
    Exiting,
}

pub struct App {
    pub running: bool,
    pub state: AppState,
    pub file_path: Option<PathBuf>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            state: AppState::MainMenu,
            file_path: None,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<(), io::Error> {
        while self.running {
            terminal.draw(|frame| ui::draw_menu(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        input::handle_input(self)
    }
}
