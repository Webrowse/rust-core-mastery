// HOLDING REFS IN A STRUCT WITH LIFETIMES

// It shows how a struct can hold a reference with an explicit lifetime annotation.
// It must be ensured that the data being referenced outlives the struct instance.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Ego is the enemy. Stay humble.");

    let first_part = novel.split('.').next().expect("No fullstop found!");

    let i = ImportantExcerpt {           // First_part must live as long as i    
        part: first_part,
    };

    println!("Excerpt: {}", i.part);

    // if first_part goes out of scope here, i.part would be a dangling reference
    // if novel goes out of scope here, first_part would be a dangling reference
}