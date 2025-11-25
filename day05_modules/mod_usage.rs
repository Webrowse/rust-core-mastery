// DEFINING AND USING VIA USE

//'mod' keyword makes the module
mod front_of_house {
    pub mod hosting {               // Nested module
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

// Bringing the module into scope with 'use'
use crate::front_of_house::hosting;

fn main() {
    // Now we can use the function directly
    hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();     // This works without 'use'

    println!("Module usage example complete.");
}



// 'use' is a shortcut to bring paths into scope
// It helps avoid repeating long paths  