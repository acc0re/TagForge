use crossterm::{
    event::{self, EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::Layout,
    widgets::{Block, Borders},
};
use std::{io, time::Duration};

pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> App {
        Self { running: true }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<(), io::Error> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::default()
            .title("TagForge - XML Editor")
            .borders(Borders::ALL);
        frame.render_widget(block, area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match code {
                KeyCode::Char('q') => self.running = false,
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}
