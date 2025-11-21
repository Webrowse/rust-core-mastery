Core Concept: Slices (&str - String Slice, &[T] - Array Slice)

In Rust, slices are a way to reference a contiguous sequence of elements in a collection without taking ownership of the collection itself.
Slices are useful for working with parts of data structures like strings and arrays without needing to copy or own the data.

fn main() {
    // String Slice Example
    let s = String::from("Hello, Rustaceans!");
    let hello_slice: &str = &s[0..5]; // Slicing the first 5 characters
    let rustaceans_slice: &str = &s[7..]; // Slicing from index 7 to the end

    println!("Full string: {}", s);
    println!("Hello slice: {}", hello_slice);
    println!("Rustaceans slice: {}", rustaceans_slice);

    // Array Slice Example
    let arr = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[1..4]; // Slicing elements from index 1 to 3

    println!("Full array: {:?}", arr);
    println!("Array slice: {:?}", slice);
}

