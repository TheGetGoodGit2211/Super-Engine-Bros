mod app;
mod state;
mod vertex;
mod renderer;
mod shader;
mod pipelines;
mod screen_uniform;
mod quad_instance;

use app::App;

use winit::event_loop::{ControlFlow, EventLoop};


fn main() {

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
