use std::error::Error;

use crossterm::{
    event::{self, Event, KeyCode},
};

use tui::{
    backend::Backend,
    Terminal
};

pub struct App {
    openai_token: String,
    should_exit: bool,
}

impl App {
    fn exit(&mut self) {
        self.should_exit = true
    }

    fn on_key(&mut self) -> Result<(), Box<dyn Error>> {
        if let Event::Key(e) = event::read()? {
            match e.code {
                KeyCode::Esc => self.exit(),
                _ => {}
            }
        }
        Ok(())
    }

    pub fn new(openai_token: String) -> App {
        App {
            openai_token,
            should_exit: false,
        }
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        loop {
            terminal.draw(|f| {})?;
            App::on_key(&mut self)?;

            if self.should_exit { return Ok(()) }
        }
    }
}