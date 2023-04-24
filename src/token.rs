use std::{error::Error, fs, fs::File, io::{self, BufReader, ErrorKind}};
use crossterm::event::{Event, KeyCode, read};

use reqwest::{Client, Response};

use serde::{Serialize, Deserialize};

use serde_json::json;
use tui::backend::Backend;
use tui::{Frame, Terminal};
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph};

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

struct TokenProcessor {
    buffer: String,
    width: u16,
}

impl TokenProcessor {
    fn request<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        loop {
            self.width = self.buffer.len() as u16;
            terminal.draw(|f| self.draw(f))?;
            if let Event::Key(k) = read()? {
                match k.code {
                    KeyCode::Enter => {
                        break;
                    },
                    KeyCode::Char(c) => {
                        self.buffer.push(c);
                    },
                    KeyCode::Backspace => {
                        self.buffer.pop();
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let outline = Block::default().title("OpenAI token").borders(Borders::ALL);
        let input_area = Block::default().borders(Borders::ALL);
        let input = Paragraph::new(self.buffer.as_ref()).block(input_area)
            .alignment(Alignment::Left);
        let chunks = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
            ].as_ref())
            .split(f.size());
        f.set_cursor(
            chunks[0].x + self.width + 1,
            chunks[0].y + 1,
        );
        f.render_widget(outline, f.size());
        f.render_widget(input, chunks[0]);
    }

    async fn is_passed(&self) -> Result<bool, Box<dyn Error>> {
        let client = Client::new();
        let prompt = "test message";
        let token = &self.buffer;
        let url = "https://api.openai.com/v1/chat/completions";

        let res = client.post(url)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [{
                    "role": "user",
                    "content": prompt
                }]
            }))
            .send()
            .await?;
        Ok(res.status().is_success())
    }
}

pub async fn read_or_request<B: Backend>(terminal: &mut Terminal<B>) -> Result<String, Box<dyn Error>> {
    if is_found()? {
        if is_passed().await? { Ok(token()?.token) } else { Ok(request(terminal).await?) }
    } else { Ok(request(terminal).await?) }
}

async fn request<B: Backend>(terminal: &mut Terminal<B>) -> Result<String, Box<dyn Error>> {
    let mut processor = TokenProcessor { buffer: String::new(), width: 0 };

    while !processor.is_passed().await? {
        processor.buffer.drain(..);
        processor.request(terminal)?;
    }
    Ok(processor.buffer)
}

async fn call_user(prompt: &str) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let token = token()?.token;
    let url = "https://api.openai.com/v1/chat/completions";

    let res = client.post(url)
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [{
                "role": "user",
                "content": prompt
            }]
        }))
        .send()
        .await?;
    Ok(res)
}

fn is_found() -> Result<bool, Box<dyn Error>> {
    if let Some(_) = dirs::config_dir() {
        if let Ok(Token { token: _ }) = token() { return Ok(true); }
    }
    Ok(false)
}

async fn is_passed() -> Result<bool, Box<dyn Error>> {
    let prompt = "test message";
    let res = call_user(prompt).await?;

    Ok(res.status().is_success())
}

fn set(token: String) -> Result<(), Box<dyn Error>> {
    let token = Token { token };
    let json = serde_json::to_string(&token)?;
    if let Some(p) = dirs::config_dir() {
        let path = p.join("eng-check").join(".token");
        fs::write(path, json)?;
    }
    Ok(())
}

fn token() -> Result<Token, Box<dyn Error>> {
    if let Some(path) = dirs::config_dir() {
        let path = path.join("eng-check").join(".token");
        let token: Token = serde_json::from_reader(BufReader::new(File::open(path)?))?;
        return Ok(token);
    }
    Err(Box::new(io::Error::new(ErrorKind::InvalidData, "invalid configuration-dir")))
}

#[test]
fn test() {
    assert!(is_found().unwrap());
}