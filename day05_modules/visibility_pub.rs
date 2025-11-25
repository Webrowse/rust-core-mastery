mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,  // private field
    }

    impl Breakfast {
        // Public associated function to create a Breakfast instance
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub(crate) fn internal_works() {
        println!("This function is visible within the crate.");
    }
}

fn main() {
    // Creating a Breakfast instance using the public associated function
    let mut meal = back_of_house::Breakfast::summer("HotBread");

    // Modifying the public field
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast for breakfast.", meal.toast);

    // Trying to change the private field will result in a compile-time error
    // meal.seasonal_fruit = String::from("blueberries"); // Error!
}