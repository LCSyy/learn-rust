//! A rust ui framework design, Qt-like style.

pub mod rf {
    pub struct App {
        rf_name: String,
        rf_engine: Option<Engine>
    }

    impl App {
        pub fn new() -> Self {
            App {
                rf_name: String::new(),
                rf_engine: None
            }
        }

        pub fn run(&self) {
            println!("Setup Context");
            println!("Init engine");
            println!("Run");
        }

        pub fn name(mut self, n: &str) -> Self {
            self.rf_name = n.to_string();
            self
        }

        pub fn engine(mut self, e: Engine) -> Self {
            self.rf_engine = Some(e);
            self
        }
    }

    pub struct Engine {
        src: String
    }

    impl Engine {
        pub fn new() -> Self {
            Engine {
                src: String::new()
            }
        }

        pub fn source(mut self, src: &str) -> Self {
            self.src = src.to_string();
            self
        }
    }

}

