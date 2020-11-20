use learn_rust::me_str;
use learn_rust::trait_extension_method::One;

fn hello() {}

fn main() {
    me_str!{
        "name": "Rust",
        "age": 24
    };
    println!("{}",hello.hello());
}
