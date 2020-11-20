//! how to write trait extension method
//! 
//! [reference](http://xion.io/post/code/rust-extension-traits.html)

pub trait One {
    fn hello(&self) -> String;
}

impl<T> One for T {
    fn hello(&self) -> String {
        String::from("Hello")
    }
}

