#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ScreenUniform {
    pub size: [f32; 2],
    pub padding: [f32; 2],
}

impl ScreenUniform {
    pub fn new(width: f32, height: f32) -> Self {
        ScreenUniform {
            size: [width, height],
            padding: [0.0, 0.0],
        }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        self.size = [width, height];
    }
}
