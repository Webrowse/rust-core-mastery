// PLACING RESTRICTIONS ON THE GENERIC TYPES USING TRAIT BOUNDS

use std::fmt::Display;

// Simple Trait Bound syntax
// T must implement Display so we can print it
fn print_value<T: Display>(value: T) {
    println!("Value: {}", value);
}

// Multiple Trait Bounds
// T must implement both Display and Clone
fn duplicate_and_print<T: Display + Clone>(value: T) {
    let cloned_value = value.clone();
    println!("Original: {}, Cloned: {}", value, cloned_value);
}   

// Using where clause for better readability
fn compare_and_print<T>(a: T, b: T)
where
    T: Display + PartialOrd
{
    if a > b {
        println!("{} is greater than {}", a, b);        
    } else if a < b {
        println!("{} is less than {}", a, b);
    } else {
        println!("{} is equal to {}", a, b);
    }
}

fn main() {
    print_value(42);
    print_value("Hello, Rust!");

    // duplicate_and_print requires Clone trait
    duplicate_and_print(String::from("Clone me!")); 
    compare_and_print(10, 20);
    compare_and_print("banana", "cpjdjhfjfj");


}
