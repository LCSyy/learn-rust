use std::process::Command;
use std::{time, thread};
use math3d::{EchoItem,Rectangle};

fn main() {
    let mut cls = Command::new("printf");
    cls.arg("\\033c");
    let sixteens = time::Duration::from_millis(100);

    let rect = Rectangle::new(0,0,10,10);
    loop {
        rect.draw();

        cls.spawn().unwrap();
        thread::sleep(sixteens);
    }
}
