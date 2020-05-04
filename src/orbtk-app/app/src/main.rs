use orbtk::prelude::*;

fn main() {
    Application::new()
    .window(|ctx| {
        Window::create()
        .title("OrbTk - minimal example")
        .position((100.0,100.0))
        .size(640.0,480.0)
        .resizeable(true)
        .child(Button::create().text("Click").build(ctx))
        .build(ctx)
    })
    .run();
}
