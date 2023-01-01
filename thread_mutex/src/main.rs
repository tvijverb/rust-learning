// use std::sync::Mutex;

// fn main() {
//     // creates a new instance Mutex<i32> and initial value of 5
//     // mutex is a smart pointer in rust
//     let m = Mutex::new(5);

//     {
//         // aquire access to the mutex by calling .lock()
//         // panic if the mutex cannot be acquired
//         let mut num = m.lock().unwrap();
//         // increment the mutex's value
//         *num += 1;
//     }
//     // print the mutex's value
//     println!("m = {:?}", m);
// }

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // create a atomic reference count for a mutex instance
    // Arc allows us to have multiple owners of the same mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // new "owned" Arc Mutex for each thread
        let counter = Arc::clone(&counter);
        // create a new thread that consumes the Arc counter
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            // and increment the Mutex by 1
            *num += 1;
        });
        // push the thread handle to a vector
        handles.push(handle);
    }

    // and wait for all handles to finish and extract the result
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}