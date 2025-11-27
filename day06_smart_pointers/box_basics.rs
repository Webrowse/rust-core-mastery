// BOX KEEPS VALUE ON THE HEAP, EVEN THE ONCES THAT USUALLY GO ON THE STACK

fn main() {
    let a = Box::new(90);       //init a new BOX
    println!("Value in box a: {}", a);
    println!("Address for box a: {:p}", a);

    let b = a;              //move ownership of box a to b, It would've been fine since i32 impliment Copy trait, but Box is not Copy
    println!("Value in box b: {}", b);
    println!("Address for box b: {:p}", b);
    //println!("Value in box a: {}", a); //ERROR: value borrowed here after

    struct ValueInBox {
        data: Box<i32>,     //data field is a box that holds an i32
    }

    let v = ValueInBox { data: Box::new(20) }; 
    println!("Value in ValueInBox v: {}", v.data);
    println!("Address ValueInBox v: {:p}", v.data);

    // Now we keep this struct inside a Box
    let boxed_v = Box::new(v);
    println!("Value in boxed ValueInBox: {}", boxed_v.data);
    println!("Address of boxed ValueInBox: {:p}", boxed_v.data);

    println!("Attempt to print memory addresses: {:p}", boxed_v);
}