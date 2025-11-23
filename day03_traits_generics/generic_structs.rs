// REDUCING CODE DUPLICATION WITH GENERIC STRUCTS

// A struct can hold coordinates of any type (Ints, floats, etc.)
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}   

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

// A struct with mixed generic types
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &U {
        &self.y
    }
    
}

fn main() {
    // Integer Point
    let int_point = Point { x: 10, y: 20 };

    // Float Point
    let float_point = Point { x: 1.5, y: 2.5 };

    // Mixed Point
    let mixed_point = MixedPoint { x: 5, y: 3.14 };

    println!("Integer Point: {:?}", int_point.get_x());
    println!("Float Point: {:?}", float_point.get_y());
    println!("Mixed Point: {:?}, {:?}", mixed_point.get_y(), mixed_point.get_x());
}