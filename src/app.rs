use std::error::Error;

use crossterm::{
    event::{self, Event, KeyCode},
};

use tui::{backend::Backend, Frame, Terminal};
use tui::layout::Rect;

use crate::{ui};

pub struct App {
    openai_token: String,
    buffer: String,
    response: Vec<String>,
    should_exit: bool,
}

impl App {
    pub fn buffer(&self) -> &String {
        &self.buffer
    }
    
    pub fn response(&self) -> &Vec<String> {
        &self.response
    }

    fn exit(&mut self) {
        self.should_exit = true
    }

    fn on_key(&mut self) -> Result<(), Box<dyn Error>> {
        if let Event::Key(e) = event::read()? {
            match e.code {
                KeyCode::Esc => self.exit(),
                KeyCode::Char(c) => self.buffer.push(c),
                KeyCode::Backspace => { self.buffer.pop(); },
                KeyCode::Enter => {
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        ui::main_ui(&self, f, area);
    }

    pub fn new(openai_token: String) -> App {
        App {
            openai_token,
            buffer: String::new(),
            response: vec![],
            should_exit: false,
        }
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        loop {
            terminal.draw(|f| self.render(f, f.size()))?;
            App::on_key(&mut self)?;

            if self.should_exit { return Ok(()) }
        }
    }
}