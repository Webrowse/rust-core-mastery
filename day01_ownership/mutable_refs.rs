Core concept: Exclusive Mutable Borrowing.

In Rust, exclusive mutable borrowing allows you to modify data 
while ensuring that no other references (mutable or immutable) exist to that data at the same time. 
When you borrow a value mutably, you can only have one mutable reference to that value, which prevents data races.

fn main() {
    let mut s1 = String::from("hello"); // s1 is the owner of the String

    {
        let r1 = &mut s1; // r1 is a mutable reference to s1

        r1.push_str(", world!"); // We can modify s1 through r1

        println!("r1: {}", r1); // This works fine
    } // r1 goes out of scope here, so we can create another reference to s1

    println!("s1 after r1 scope: {}", s1); // s1 has been modified

    let r2 = &mut s1; // Now we can create a new mutable reference to s1

    r2.push_str(" Welcome to Rust!"); // We can modify s1 through r2

    println!("r2: {}", r2); // This works fine
}