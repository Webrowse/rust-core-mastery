Core Concept: Safety & Lifetimes

Defination of dangling reference:
a reference that points to a memory location that has been deallocated or is no longer valid.

How borrow checker prevents dangling references:
The borrow checker enforces rules about how long references are valid.
A reference must always be valid for the duration of its use, and the borrow checker ensures that
references do not outlive the data they point to. This prevents dangling references and ensures memory safety

e.g. cannot return reference to local variable from a function-scope.

fn main() {
    let r;                // Declare a reference variable r

    {
        let s = String::from("hello"); // s is created in this inner scope
        r = &s;                        // r borrows s
    } // s goes out of scope here, and is dropped

    // println!("r: {}", r); // This line would cause a compile-time error because r is a dangling reference
}
