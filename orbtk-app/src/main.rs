#![windows_subsystem = "windows"]

use orbtk::prelude::*;

fn main() {
    Application::new()
    .window(|ctx|{
        Window::new()
        .title("OrbTk - minimal example")
        .position((100.0,100.0))
        .size(600.0,600.0)
        .child(TextBlock::new().text("OrbTk").build(ctx))
        .build(ctx)
    })
    .run();
}
