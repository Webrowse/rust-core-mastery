// Demo of the need for the lifetime annotations

// Function that returns a reference to the longer of two string slices

// The returned ref mist be valid for he duration of the shorter of the 2 input refs, hence 'a lifetime annotation

fn logest<'a>(x:&'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = "xyz";

    // compiler will make sure that the returned reference is valid as long as both input references are valid
    let result = logest(string1.as_str(), string2);

    // result will be valid as long as both string1 and string2 are valid
    println!("The longest string is {}", result);
}