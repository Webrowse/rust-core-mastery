
// A function sitting at the very top (Root)
fn call_police() {
    println!("call police from Root");
}

mod house {
    fn clean_living_room() {
        println!("Living room cleaned.");
    }

    pub mod bedroom {
        pub fn emergency() {
            // 1. ABSOLUTE PATH (crate)
            // Regardless of where we are, go to the very top and find 'call_police'
            crate::call_police();
        }

        pub fn normal_day() {
            // 2. RELATIVE PATH (super)
            // Go up one level (to 'house') and find 'clean_living_room'
            super::clean_living_room();
            
            // We can also chain super!
            // super::super::call_police(); // This would go to house -> root
        }
        
        pub fn self_check() {
            // 3. RELATIVE PATH (self)
            // Refers to the current module. Usually implied, but can be explicit.
            self::make_bed();
        }

        fn make_bed() {
            println!("Bed made.");
        }
    }
}

fn main() {
    println!("Testing Navigation");
    house::bedroom::emergency();
    house::bedroom::normal_day();
    house::bedroom::self_check();
}