// APPLYING TO ALL THAT IMPLEMENT A TRAIT

// This is how stardard library gives methods like to_string() to all types that implement the 'Display' trait.

trait Printable {
    fn format_pretty(&self) -> String;
}

use std::fmt::Display;

// Implement Printable for all types that implement Display
impl<T: Display> Printable for T {
    fn format_pretty(&self) -> String {
        format!("*** {} ***", self)
    }
}   

struct Cat {
    name: String,
    age: u8,
}

// fmt is a function required by the Display trait, we are manually redefining it here for demonstration.
impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let number = 42;
    let text = "Hello, tsur!";
    let cat = Cat {
        name: String::from("Tom"),
        age: 3,
    };
    println!("{}", number.format_pretty());
    println!("{}", text.format_pretty());
    println!("{}", cat.format_pretty());
}