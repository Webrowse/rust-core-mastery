

use tokio::fs;
use std::io::Error;

const FILE_NAME: &str = "async_data.txt";
const TEST_CONTENT: &str = "Written by tokio code";

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Writing in file: {}", FILE_NAME);

    // Tokio::fs::write creates or overwrites the file.
    fs::write(FILE_NAME, TEST_CONTENT).await?;

    println!("written successfully");

    // Read operation
    let content = fs::read_to_string(FILE_NAME).await?;

    println!("Content of file: {}", content);

    // Clean-Up file
    fs::remove_file(FILE_NAME).await?;
    println!("File deleted successfully");

    // final verification

    if content == TEST_CONTENT {
        println!("\nSTATUS: Success! Read data matches the wriiten data");
    } else {
        eprintln!("\nStatus: Error: Content Mismatch");
    }

    Ok(())
}
/*
    Rust's standard library `std::fs` is *blocking*. A call to `std::fs::write` 
    would block the entire OS thread until the operation completes.

    the main event loop doesn't stop, 
    it delegates the blocking work elsewhere and awaits the result.
*/