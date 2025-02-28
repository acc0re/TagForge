use crate::{input, ui, xml};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::Write;
use std::{io, path::PathBuf};

#[derive(PartialEq, Clone)]
pub enum AppState {
    MainMenu,
    SelectFile,
    Editor,
    Exiting,
}

pub struct App {
    pub running: bool,
    pub state: AppState,
    pub file_path: Option<PathBuf>,
    pub input_buffer: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            state: AppState::MainMenu,
            file_path: None,
            input_buffer: String::new(),
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<(), io::Error> {
        while self.running {
            match self.state {
                AppState::MainMenu => terminal.draw(|frame| ui::draw_main_menu(frame))?,
                AppState::SelectFile => terminal.draw(|frame| ui::draw_select_file(self, frame))?,
                AppState::Editor => terminal.draw(|frame| ui::draw_editor(frame))?,
                AppState::Exiting => break,
            };

            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        input::handle_input(self)
    }
}
