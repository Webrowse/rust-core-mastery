use std::fs;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Context(String),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> AppError {
        AppError::Parse(err)
    }
}
//low-level function with no context
// Raw error bubble
fn read_number(path: &str) -> Result<i32, AppError> {
    let content = fs::read_to_string(path)?;
    let number: i32 = content.trim().parse()?;
    Ok(number)
}

//med-level function adding context
fn load_config_value(path: &str) -> Result<i32, AppError> {
    let number = read_number(path).map_err(|e| {
        AppError::Context(format!("Failed to load config value from {}: {:?}", path, e))
    })?;
    Ok(number)
}

//high-level function adding more context
fn initialize_application(config_path: &str) -> Result<i32, AppError> {
    load_config_value(config_path).map_err(|_|{
        AppError::Context(format!("Application failed to initialize with config: {}", config_path))
    })
}

fn main() {
    match initialize_application("config.txt") {
        Ok(value) => println!("Application initialized with value: {}", value),
        Err(e) => println!("Error: {:?}", e),
    }
}

