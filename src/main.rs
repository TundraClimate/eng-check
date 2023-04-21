use std::error::Error;
use std::{fs, io};
use std::fs::File;

use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    Terminal,
};

fn setup_files() -> Result<(), Box<dyn Error>> {
    if let Some(s) = dirs::config_dir() {
        let s = s.join("eng-check");
        fs::create_dir_all(&s)?;
        File::create(s.join(".token"))?;
        File::create(s.join("eng-check.conf"))?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_files()?;

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}