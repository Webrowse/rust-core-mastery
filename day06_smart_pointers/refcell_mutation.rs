use std::cell::RefCell;

#[derive(Debug)]
struct Counter {
    value: RefCell<i32>,
}

fn main() {
    let c = Counter {
        value: RefCell::new(10),
    };

    *c.value.borrow_mut() += 5;
    println!("After the first mutation: {:?}", c);

    {
        let mut v = c.value.borrow_mut();
        *v += 20;
        println!("after second mutation : {:?}", v);
    }

    let borrow_immutable = c.value.borrow();
    println!("immutable borrow: {}", *borrow_immutable);

    drop(borrow_immutable);             // dropped immutable ref because we cannot have mutable ref and immutable at same time

    *c.value.borrow_mut() += 1;         // this line would have paniced without dropping mutable ref.
    println!("after releasing immutable borrow: {:?}", c);
}