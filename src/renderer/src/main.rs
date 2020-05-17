/*
GraphicsItem
Renderer
Abstract Graphics Layer
*/
/*
workspace
|- package1 
|  |- Cargo.toml
|  |- src/
|  |  |- main.rs -------------- binary crate root
|  |  |- lib.rs  -------------- lib crate root
|  |  |- bin/
|  |  |  |- <other_binary_crate>.rs
|  |  |  |- ...
|- package2
|- Cargo.toml

scope
path - refer to  an item in the module tree.
pub use - re-exporting

module
|- mod <module_name>
|- <module_name>.rs
|- <module_name>/
|  |- [mod.rs]
|  |- <child_module_name>.rs
*/

// if statement
// let if
// loop statement
// let loop -> break statement;
// while
// for, need a range for

extern crate glium;

use glium::{ glutin, Surface };

// use renderer::math::{ self, Vector2 };

fn main() {
    let events_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
    .with_inner_size(glutin::dpi::LogicalSize::new(640.0,80.0))
    .with_title("Hello World!");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    events_loop.run(move |ev, _, control_flow| {

        let mut frame = display.draw();
        frame.clear_color(0.8, 0.2, 0.25, 1.0);
        frame.finish().unwrap();

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return
            },
            _ => {}
        }
    });
}
