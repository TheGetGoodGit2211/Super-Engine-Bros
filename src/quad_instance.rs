use crate::vertex::Vertex;

pub struct QuadInstance {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}

impl QuadInstance {
    pub fn new(pos: [f32; 2], size: [f32; 2], color: [f32; 4]) -> QuadInstance {
        QuadInstance { pos, size, color }
    }

    pub fn to_verts(&self) -> [Vertex; 4] {
        let [w, h] = self.size;
        let [x, y] = self.pos;

        [
            Vertex { position: [x, y, 0.0, 1.0], color: self.color },
            Vertex { position: [x, y + h, 0.0, 1.0], color: self.color },
            Vertex { position: [x + w, y + h, 0.0, 1.0], color: self.color },
            Vertex { position: [x + w, y, 0.0, 1.0], color: self.color },
        ]
    }
}
