use std::error::Error;
use std::fs::File;
use std::{io};
use std::io::{BufReader, ErrorKind};
use reqwest::{Client, Response};
use serde::Serialize;
use serde::Deserialize;
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

pub async fn read_or_request() -> Result<String, Box<dyn Error>> {
    if is_found()? {
        if is_passed().await? { Ok(token()?.token) }
        else { Ok(request().await?) }
    } else { Ok(request().await?) }
}

async fn request() -> Result<String, Box<dyn Error>> {
    todo!()
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