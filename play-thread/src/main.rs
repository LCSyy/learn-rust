//! Rust Fearless Concurrency
//! 

use std::thread;

fn main() {
    let h = thread::spawn(||{
        println!("Hello");
    });

    println!("Hello {}", thread::current().name().unwrap());

    h.join().unwrap();
}
