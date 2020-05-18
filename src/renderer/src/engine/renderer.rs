pub struct Renderer {
    state: RendererState
}

struct RendererState {
    running: bool,
    name: String
}

impl Renderer {
    pub fn new() -> Self {
        Renderer { state: RendererState::new() }
    }

    /// 进入主循环。
    pub fn exec(&mut self) -> bool {

        // before enter main loop
        println!("Starting {} ...", self.state.name);

        let mut count = 100_000_000;
        while self.state.running {
            // prepare frame
            count -= 1;
            
            // prepare render

            // render

            // end frame
            if count <= 0 {
                self.state.running = false;
            }
        }

        // after exit main loop
        println!("Stoped {}.", self.state.name);

        true
    }
}


impl RendererState {
    fn new() -> Self {
        RendererState {
            running: true,
            name: String::from("Renderer")
        }
    }
}