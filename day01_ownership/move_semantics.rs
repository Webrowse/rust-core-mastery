Ownership Rules:
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
fn main() {
    let s1 = String::from("hello"); // s1 is the owner of the String

    let s2 = s1; // Ownership of the String is moved from s1 to s2

    // println!("{}", s1); // This line would cause a compile-time error because s1 is no longer valid

    println!("{}", s2); // This works because s2 is now the owner of the String
}


Move Semantics:
When a variable is assigned to another variable, the ownership of the value is moved to the new variable.
The original variable can no longer be used unless the value implements the Copy trait.

fn main() {
    let x = 5; // x is an integer, which implements the Copy trait

    let y = x; // x is copied to y, both x and y are valid

    println!("x: {}, y: {}", x, y); // This works fine

    let s1 = String::from("hello"); // s1 is the owner of the String

    let s2 = s1; // Ownership of the String is moved from s1 to s2

    // println!("{}", s1); // This line would cause a compile-time error because s1 is no longer valid

    println!("{}", s2); // This works because s2 is now the owner of the String
}

Clone trait vs Copy trait:
The Clone trait allows for deep copying of data, while the Copy trait allows for shallow copying.
Types that implement the Copy trait can be duplicated simply by copying bits, 
while types that implement the Clone trait require an explicit method call to create a deep copy.

fn main() {
    let s1 = String::from("hello"); // s1 is the owner of the String

    let s2 = s1.clone(); // Deep copy of the String is created, s1 and s2 are both valid

    println!("s1: {}, s2: {}", s1, s2); // This works fine

    let x = 5; // x is an integer, which implements the Copy trait

    let y = x; // x is copied to y, both x and y are valid

    println!("x: {}, y: {}", x, y); // This works fine
}