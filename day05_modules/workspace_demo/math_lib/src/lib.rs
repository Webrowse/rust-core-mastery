

pub fn add_two (x: i32) -> i32 {
    x.checked_add(2).expect("Overflow when adding two")
}