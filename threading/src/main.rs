use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    // move data in 
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // wait for spawned threads to finish
    handle.join().unwrap();

    // main thread actions
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
