use std::sync::Arc;

use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::{event_loop::OwnedDisplayHandle, window::Window};

use crate::{pipelines::hello_pipeline::HelloPipeline, shader::Shader, vertex::Vertex};

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_format: wgpu::TextureFormat,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub instance: wgpu::Instance,
    shader: Shader,
    pipeline: HelloPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl Renderer {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>) -> Self{
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display),
        ));
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        const VERTICES: &[Vertex] = &[
            // Top-Left (Full Red, 100% Opaque)
            Vertex { position: [-0.5, 0.5, 0.0, 1.0], color: [1.0, 0.0, 0.0, 1.0] },
            // Bottom-Left (Full Green, 0% Alpha / Fully Transparent!)
            Vertex { position: [-0.5, -0.5, 0.0, 1.0], color: [0.0, 1.0, 0.0, 0.0] },
            // Bottom-Right (Full Blue, 100% Opaque)
            Vertex { position: [0.5, -0.5, 0.0, 1.0], color: [0.0, 0.0, 1.0, 1.0] },
            // Top-Right (Yellow: Red + Green, 100% Opaque)
            Vertex { position: [0.5, 0.5, 0.0, 1.0], color: [1.0, 1.0, 0.0, 1.0] },
        ];
        const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Hello Vert Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Hello IDX Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let shader = Shader::new(&device, include_str!("./shaders/hello.wgsl"), None);

        let pipeline = HelloPipeline::new(&device, surface_format, &shader);

        let renderer = Renderer {
            device,
            queue,
            surface,
            surface_format,
            size,
            instance,
            shader,
            pipeline,
            vertex_buffer,
            index_buffer
        };

        renderer.configure_surface();

        renderer
    }

    pub fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
    }

    pub fn render(&mut self, window: &Window) {
        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => return,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("No error scope registered, so validation errors will panic")
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                self.configure_surface();
                return;
            }
        };
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            let pipeline = &self.pipeline;

            render_pass.set_pipeline(&pipeline.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..6, 0, 0..1);
        }

        self.queue.submit([encoder.finish()]);
        window.pre_present_notify();
        self.queue.present(surface_texture);
    }
}
