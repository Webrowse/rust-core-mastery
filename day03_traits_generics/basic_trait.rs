// CORE CONCEPT: POLYMORPHISM WITH TRAITS -> Different types sharing common behavior

// Define a trait
trait Summarizable {
    fn summarize(&self) -> String;

    // Default impl 9can be overwritten)
    fn summarize_brief(&self) -> String {
        String::from("(Read more...)")
    }
}

struct Article {
    title: String,
    author: String,
    content: String,
}

// Implement the trait for Article
impl Summarizable for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
    fn summarize_brief(&self) -> String {
        format!("{}... by {}", &self.content.chars().take(10).collect::<String>(), self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

// Implement the trait for Tweet
impl Summarizable for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_brief(&self) -> String {
        format!("{}: {}...", self.username, self.content.chars().take(10).collect::<String>())
    }
}

fn main() {
    let article = Article {
        title: String::from("Rust with me"),
        author: "Romy".into(),
        content: String::from("A list of Rust concepts with the examples of code concepts..."),
    };

    let tweet = Tweet {
        username: String::from("rustasian"),
        content: String::from("Trait is the code of rust, add lifetime and call it a day!"),
    };

    println!("Article Summary: {}", article.summarize());
    println!("Tweet Summary: {}", tweet.summarize());
    println!("Tweet Brief Summary: {}", tweet.summarize_brief());   
    println!("Article Brief Summary: {}", article.summarize_brief());
}

