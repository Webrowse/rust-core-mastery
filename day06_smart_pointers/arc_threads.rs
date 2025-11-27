

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // ex1: Simple Read-Only Sharing with Arc
    let s_data = Arc::new(vec![10,20,30]); // Wrap a vec[] in an Arc
    let mut handles = vec![];

    for i in 1..=10 {
        let shared = Arc::clone(&s_data); // Clone the Arc for each thread
        let handle = thread::spawn(move || {        // Move the cloned Arc into the thread
            println!("Thread {:2} sees data: {:?}", i, shared);
        });
        handles.push(handle);   // Store the thread handle
    }

    for handle in handles {         // Wait for all threads to finish
        handle.join().unwrap();     // Handle any panics
    }

    // ex2: Mutable Sharing with Arc and Mutex
    let m_data = Arc::new(Mutex::new(vec![10,20,30])); // Wrap a vec[] in an Arc and Mutex
    let mut handles2 = vec![];
    for i in 1..=10 {
        let shared = Arc::clone(&m_data); // Clone the Arc for each thread
        let handle = thread::spawn(move || {        // Move the cloned Arc into the thread
            let mut data = shared.lock().unwrap(); // Lock the Mutex to get mutable access
            data.push(i * 10);                     // Modify the data
            println!("Thread {:2} updated data: {:?}", i, *data);
        });
        handles2.push(handle);   // Store the thread handle
    }
    for handle in handles2 {        // Wait for all threads to finish
        handle.join().unwrap();     // Returns, Ok(output) or Err(payload)
    }
}