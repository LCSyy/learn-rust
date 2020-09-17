//! Qt like ui frameworks writen in rust.

pub mod rf {
    pub struct App {

    }

    impl App {
        pub fn new() -> Self {
            App {}
        }

        pub fn exec(&self) -> i32 {
            println!("Exec app.");
            0
        }
    }

    pub struct Engine {

    }

    impl Engine {
        pub fn new() -> Self {
            Engine {}
        }

        pub fn source(&self, src: &str) {
            println!("load rf source:{}", src);
        }
    }

}

