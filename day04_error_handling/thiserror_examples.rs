// 'thiserror' is an external crate that provides easy way to define custom error types in Rust.
// 90% of professional rust libraries define their error types using this crate.


// cargo add thiserror

use thiserror::Error;
use std::fs;
use std::io;


#[derive(Error, Debug)]   // Error is from thiserror crate
enum DataError {
    // Simple error Message
    #[error("Data cannot be empty")]
    EmptyData,

    // Dynamic error message with details (using field)
    #[error("Invalid ID provided: {0} is not numeric")]
    InvalidId(String),

    // Wrapping another error type (source error)
    // #[from] automatically creates conversion from io::Error to DataError
    #[error("IO Error occurred: {0}")]
    IoError(#[from] io::Error),     // This allows using ? operator on File I/O operations

    // Custom error message with multiple fields
    #[error("JSON parsing failed: {0}")]
    ParseError(String),
}

// Function to read data from a file and validate it
fn read_config_file(path: &str) -> Result<String, DataError> {

    // if fs::read_to_string fails, the io::Error is automatically converted to DataError::IoError
    let content = fs::read_to_string(path)?;

    if content.is_empty() {
        return Err(DataError::EmptyData);
    }

    Ok(content)
}

fn parse_id(id_str: &str) -> Result<u32, DataError> {
    id_str.trim().parse::<u32>().map_err(|_| DataError::InvalidId(id_str.to_string()))
}

fn main() {
    // Example 1: Reading a config file
    match read_config_file("config.txt") {
        Ok(data) => println!("Config Data: {}", data),
        Err(e) => println!("Error reading config file: {:?}", e),
    }

    // Example 2: Parsing an ID
    match parse_id("abc123") {
        Ok(id) => println!("Parsed ID: {}", id),
        Err(e) => println!("Error parsing ID: {:?}", e),
    }
}