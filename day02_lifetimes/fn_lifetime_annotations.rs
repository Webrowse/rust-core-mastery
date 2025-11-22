// LIFETIME ELISION RULES

// Lifetime elision rules allow the compiler to infer lifetimes in certain situations
// without explicit annotations. There are three main rules for lifetime elision:
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all
//    output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
//    the lifetime of self is assigned to all output lifetime parameters.

fn get_first_word(s: &str) -> &str {  // No explicit lifetime annotations needed due to elision rules (rule 2)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// annotation is required because 2 input parameters, and none is &self
// output lifetime tied to the lifetime of x, so lifetime of y is irrelevant here

fn choose_first<'a>(x: &'a str, y: &str) -> &'a str { // 'a is needed for x and return value, but not for y
    x
}

fn main() {
    let sentence = String::from("Hello world from Rust");
    let first_word = get_first_word(&sentence);         // one input parameter, so elision rule 2 applies, no need for annotation
    println!("First word: {}", first_word);

    let str1 = String::from("First string");
    let str2 = String::from("Second string");
    let chosen = choose_first(&str1, &str2);
    println!("Chosen string: {}", chosen);
}