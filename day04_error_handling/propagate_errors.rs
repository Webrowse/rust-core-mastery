// Use of ? operator to propagate errors in Rust

use std::fs;
use std::num::ParseIntError;
use std::io;



#[derive(Debug)]
enum AppError {
    Io(io::Error), 
    Parse(ParseIntError),
    InvalidInput(String),
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

fn read_and_parse_number(path: &str) -> Result<i32, AppError> {
    let content = fs::read_to_string(path)?; 
    let number: i32 = content.trim().parse()?;

    if number < 0 {
        return Err(AppError::InvalidInput( 
            format!("Negative number: {},", number)
        ));
    }
    Ok(number)
}

fn main() {
    match read_and_parse_number("number.txt") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error occurred: {:?}", e),
    }
}