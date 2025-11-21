Core Concept: Borrow Scope Usage Based Rules.

fn main() {
    let mut a = String::from("rusk");
    
    let b = &mut a;
    b.push_str(" rist");
    println!("{}", b);
    
    let c = &mut a;                 // This is allowed in Non-Lexical Lifetimes (NLL), because b is no longer used after its last usage.
    c.push_str(" Add_c");
    println!("{}",c);               // In usual Rust borrowing rules, this would cause a compile-time error.
}

In above code, Non-Lexical Lifetimes (NLL) allows the mutable borrow `b` to end its lifetime
after its last usage, enabling the creation of another mutable borrow `c` of `a`.

Usual rule suggests that only one mutable reference can exist at a time, but NLL relaxes this by allowing
the borrow checker to determine the actual usage of references, thus preventing unnecessary borrow conflicts and enhancing flexibility
while maintaining memory safety.