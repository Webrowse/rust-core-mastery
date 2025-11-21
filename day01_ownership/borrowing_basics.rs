Core Concept: Immutable Borrowing.

In Rust, immutable borrowing allows you to read data without taking ownership of it. 
When you borrow a value immutably, you can have multiple references to that value at the same time, 
but you cannot modify it through those references. This is useful for ensuring data safety and preventing unintended side effects.

fn main() {
    let s1 = String::from("hello"); // s1 is the owner of the String

    let r1 = &s1; // r1 is an immutable reference to s1
    let r2 = &s1; // r2 is another immutable reference to s1

    println!("r1: {}, r2: {}", r1, r2); // This works fine, both r1 and r2 can read s1

    // s1.push_str(", world!"); // This line would cause a compile-time error because s1 is borrowed immutably

    println!("s1: {}", s1); // s1 can still be used here since it was not modified
}

