Core Concept: Borrow Checker Lifetimes and Scope Rules.

The borrow checker enforces rules about how long references are valid.
A reference must always be valid for the duration of its use, and the borrow checker ensures that
references do not outlive the data they point to. This prevents dangling references and ensures memory safety.

fn main() {
    let s1 = String::from("hello"); // s1 is the owner of the String

    {
        let r1 = &s1; // r1 is an immutable reference to s1

        println!("r1: {}", r1); // This works fine
    } // r1 goes out of scope here

    // r1 can no longer be used here since it is out of scope

    let r2 = &s1; // Now we can create a new immutable reference to s1

    println!("r2: {}", r2); // This works fine
}