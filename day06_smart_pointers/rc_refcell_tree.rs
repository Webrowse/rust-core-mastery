use std::cell::RefCell;

#[derive(Debug)]
struct Counter {
    value: RefCell<i32>,
}

fn main() {
    let c = Counter {
        value: RefCell::new(0),
    };

    *c.value.borrow_mut() += 1;
    *c.value.borrow_mut() += 5;
    *c.value.borrow_mut() += 2;

    println!("Final Value: {:?}", c.value.borrow());

    let borrow1 = c.value.borrow();
    println!("borrow1: {:?}", borrow1);
}