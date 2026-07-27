use std::sync::Arc;

use winit::{
    event_loop::OwnedDisplayHandle,
    window::Window,
};

use crate::renderer::Renderer;

pub struct State {
    window: Arc<Window>,
    pub renderer: Renderer,
}

impl State {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>) -> State {
        let renderer = Renderer::new(display, window.clone()).await;
        State {
            window,
            renderer,
        }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn render(&mut self) {
        self.renderer.render(&self.window);
    }
}
