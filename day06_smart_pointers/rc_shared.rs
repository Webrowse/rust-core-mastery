use std::rc::Rc;

struct RcDemo {
    val: i32,
}

fn main() {
    let rc_init = RcDemo { val: 10 };
    let rc2 = Rc::new(&rc_init); // Wrap RcDemo in an Rc
    let rc3 = Rc::clone(&rc2); // Cloned rc2
    let rc4 = Rc::clone(&rc2); // Cloned again
    
    println!("Strong count after rc4: {}", Rc::strong_count(&rc4));

    {
        let rc5 = Rc::clone(&rc4); // Increase reference count
        println!("Strong count after rc5: {}", Rc::strong_count(&rc4));
    } // rc5 goes out of scope here

    println!("Strong count after rc5 goes out of scope: {}", Rc::strong_count(&rc4));
    println!("Strong count after rc5 goes out of scope: {}", Rc::strong_count(&rc3));
    println!("Strong count after rc5 goes out of scope: {}", Rc::strong_count(&rc2));
}