/*
___________             ___________
\__    ___/____     ____\_   _____/__________  ____   ____
  |    |  \__  \   / ___\|    __)/  _ \_  __ \/ ___\_/ __ \
  |    |   / __ \_/ /_/  >     \(  <_> )  | \/ /_/  >  ___/
  |____|  (____  /\___  /\___  / \____/|__|  \___  / \___  >
               \//_____/     \/             /_____/      \/
 */

mod app;
mod ui;
mod input;
mod xml;

use app::App;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<(), io::Error> {
    color_eyre::install().expect("Failed to install color-eyre");

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = app.run(&mut terminal);

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
