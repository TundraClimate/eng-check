mod app;
mod token;

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

use crate::{
    app::App,
};

fn setup_files() -> Result<(), Box<dyn Error>> {
    if let Some(s) = dirs::config_dir() {
        let s = s.join("eng-check");
        fs::create_dir_all(&s)?;
        if !s.join(".token").exists() { File::create(s.join(".token"))?; }
        if !s.join("eng-check.conf").exists() { File::create(s.join("eng-check.conf"))?; }
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

    let token = token::read_or_request().await?;
    let app = App::new(token);
    app.run(&mut terminal)?;

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}