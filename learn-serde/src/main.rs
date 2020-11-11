//! Serde
//! 
//! [data structure]
//!     ||
//!   [layer]
//!     ||
//! [data format]
//! 

use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point{x: 1, y: 2};
    let serialized = serde_json::to_string(&p).unwrap();
    println!("serialized = {}", serialized);
}
