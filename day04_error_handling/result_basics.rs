// WHEN FAILING IS A POSSIBILITY, WE HANDLE THAT WITH Result<T, E>

// In Rust, the `Result<T, E>` type is used for functions that can succeed or fail.
// It is an enum with two variants: `Ok(T)` for success and `Err(E)` for failure.

// on sucess, return the f64 result
// on failure, return a String error message
fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    // 'match' is commonly used to handle Result values
    // Example 1: Successful division
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: Division by zero
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Using 'unwrap' to get the value or panic on error. live by the sword... method, unrecommended for production code.
    let result = safe_divide(20.0, 4.0).unwrap();
    println!("Unwrapped Result: {}", result);

    //Using 'expect' to get the value or panic with a custom message on error. A better wrapper over unwrap.
    let result = safe_divide(20.0, 0.0).expect("Failed to divide");
    println!("Expected Result: {}", result);

    // Using 'unwrap_or' to provide a default value on error
    let result = safe_divide(15.0, 0.0).unwrap_or(-1.0);
    println!("Unwrapped or Default Result: {}", result);
}