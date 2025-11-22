// STATIC LIFETIME IS VALID TILL THE END OF THE PROGRAM

// This is often used for &str which saved into the binary of the program, doesn't add compute at runtime.

// A constant string literal has a 'static lifetime.

const MESSAGE: &'static str = "Hello, I have a static lifetime!"; // const are written in Capital letters by convention

fn process_static_data ( data: &'static str) {
    println!("Processing static data: {}", data);
}

fn main() {
    // Using the static string literal
    process_static_data(MESSAGE);

    // Using another static string literal directly
    process_static_data("This is another static string literal.");

    let s: &'static = String::from("making static by annotation example"); // leak makes the String have 'static lifetime by leaking memory
    process_static_data(s);
}