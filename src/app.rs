use std::error::Error;

use crossterm::{
    event::{self, Event, KeyCode},
};

pub struct App {
    should_exit: bool,
}

impl App {
    fn exit(&mut self) {
        self.should_exit = true
    }

    fn on_key(&mut self) -> Result<(), Box<dyn Error>> {
        if let Event::Key(e) = event::read()? {
            match e.code {
                KeyCode::Char('q') => self.exit(),
                _ => {}
            }
        }
        Ok(())
    }

    pub fn new() -> App {
        App {
            should_exit: false,
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        loop {
            App::on_key(&mut self)?;

            if self.should_exit { return Ok(()) }
        }
    }
}