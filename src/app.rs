use crate::{input, ui, xml};
use crossterm::{execute, terminal::{Clear, ClearType}};
use ratatui::{Terminal, backend::CrosstermBackend};
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
    pub previous_state: Option<AppState>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            state: AppState::MainMenu,
            file_path: None,
            previous_state: None,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<(), io::Error> {
        while self.running {
            if self.previous_state.as_ref() != Some(&self.state) {
                print!("state changed");
                execute!(terminal.backend_mut(), Clear(ClearType::All))?;
                self.previous_state = Some(self.state.clone());
            }

            match self.state {
                AppState::MainMenu => terminal.draw(|frame| ui::draw_main_menu(frame))?,
                AppState::SelectFile => terminal.draw(|frame| ui::draw_select_file(frame))?,
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
