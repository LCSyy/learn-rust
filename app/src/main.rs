// use rf::{ App, Engine };

// #[derive(rf::Object, rf::Widget)]
// struct MyButton {

// }

// impl MyButton {
//     fn new() -> Self {
//         MyButton {}
//     }

//     fn property(&self) {

//     }
// }

// struct Util {

// }

// impl Util {
//     fn new() -> Self {
//         Util {}
//     }
// }

// fn main() -> std::io::Result<()> {
//     App::new()
//     .engine(
//         Engine::new()
//         .source("res://main.rf")
//         .register_type("MyApp","Button",||{
//             MyButton::new().property()
//         })
//         .register_singleton("MyApp","Util",||{
//             Util::new()
//         })
//         .property("Version","1.0.0")
//     )
//     .run()
// }

use app::rf::{ App, Engine };

fn main() {
    App::new()
    .name("rust app")
    .engine(
        Engine::new().source("res://main.rf")
    )
    .run();
}
