use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::quad_instance::QuadInstance;

pub struct QuadBuffers {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    vertices: Vec<crate::vertex::Vertex>,
    max_quads: usize,
}

impl QuadBuffers {
    pub fn new(device: &wgpu::Device, max_quads: usize) -> QuadBuffers {
        let max_verts = max_quads * 4;

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dynamic Buffer of Vertices"),
            size: (max_verts * size_of::<crate::vertex::Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = Self::create_index_buffer(device, max_quads);

        QuadBuffers {
            vertex_buffer,
            index_buffer,
            vertices: Vec::with_capacity(max_verts),
            max_quads,
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }
    
    pub fn push(&mut self, quad: QuadInstance) {
        if self.vertices.len() / 4 >= self.max_quads {
            return;
        }

        self.vertices.extend_from_slice(&quad.to_verts());
    }

    pub fn upload(&self, queue: &wgpu::Queue) {
        if !self.vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        }
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..6, 0, 0..1);
    }

    fn create_index_buffer(device: &wgpu::Device, max_quads: usize) -> wgpu::Buffer {
        let max_indices = max_quads * 6;
        let mut indices: Vec<u16> = Vec::with_capacity(max_indices);

        for i in 0..(max_quads as u16) {
            let offset = i * 4;

            indices.extend_from_slice(&[
                offset, offset + 1, offset + 2,
                offset + 2, offset + 3, offset
            ]);
        }

        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Pre-Allocated Buffer of Indices"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        })
    }
}
