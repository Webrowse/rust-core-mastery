// There can be a variety of errors that can occur in runtime.
// One way to handle them is to define a custom error enum.

// Define a custom error enum to represent different kinds of errors
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSqrt,
    InputTooLarge(f64),
}

// Function to perform safe division, returning a Result type
fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, MathError> {
    if denominator == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(numerator / denominator)
    }
}   

fn safe_sqrt(value: f64) -> Result<f64, MathError> {
    if value < 0.0 {
        Err(MathError::NegativeSqrt)
    } else {
        Ok(value.sqrt())
    }
}


fn main() {

    // Example 1: Division by zero
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    //Example 2: Negative square root
    match safe_sqrt(-4.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {:?}", e),   
    }

    // Example 3: Number too large (not implemented in functions, just for demonstration)
    let large_number: f64 = f32::MAX as f64 * 2.0;
    if large_number > f32::MAX as f64{
        let error = MathError::InputTooLarge(large_number);

        if let MathError::InputTooLarge(val) = error {
            println!("Error: Input too large: {}", val);
        }
    }
}