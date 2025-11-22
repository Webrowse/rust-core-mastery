// MULTIPLE AND NAMED LIFETIMES

// Different references can have different lifetimes.
// We can also specify relationships between them.

fn print_and_return<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
    where 
    'b: 'a     // this puts the constraint that 'b must live at least as long as 'a, for compilor to verify
{
    println!("y: {}", y);
    x
}

fn main() {
    let long_lived = String::from("I live long");
    {
        let short_lived = String::from("I live short");

        let result = print_and_return(long_lived.as_str(), short_lived.as_str());

        println!("Result: {}", result);
    } // short_lived goes out of scope here
}   // long_lived goes out of scope here