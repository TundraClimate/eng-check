use std::error::Error;
use std::fs;
use std::fs::File;

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
    Ok(())
}