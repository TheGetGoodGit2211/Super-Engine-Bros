use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::{event_loop::OwnedDisplayHandle, window::Window};

use crate::{pipelines::hello_pipeline::HelloPipeline, quad_buffers::QuadBuffers, quad_instance::QuadInstance, screen_uniform::ScreenUniform, shader::Shader, surface_context::SurfaceContext};

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: HelloPipeline,
    // vertex_buffer: wgpu::Buffer,
    // index_buffer: wgpu::Buffer,
    quad_buffers: QuadBuffers,
    screen_buffer: wgpu::Buffer,
    screen_bind_group: wgpu::BindGroup,
    screen_uniform: ScreenUniform,
    surface_context: SurfaceContext,
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

        let quad_buffers = QuadBuffers::new(&device, 10_000);

        let surface_context = SurfaceContext::new(window, adapter, instance);
        surface_context.configure(&device);

        // let indices: &[u16] = &[0, 1, 2, 2, 3, 0];
        //
        // let quad = QuadInstance::new([20.0, 20.0], [80.0, 80.0], [1.0, 0.0,  0.0, 1.0]);
        //
        // let vertices = &quad.to_verts();
        //
        // let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        //     label: Some("Hello Vert Buffer"),
        //     contents: bytemuck::cast_slice(vertices),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });
        //
        // let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        //     label: Some("Hello IDX Buffer"),
        //     contents: bytemuck::cast_slice(indices),
        //     usage: wgpu::BufferUsages::INDEX,
        // });

        let shader = Shader::new(&device, include_str!("./shaders/pix_test.wgsl"), None);

        let (screen_uniform, screen_buffer, screen_bind_group_layout, screen_bind_group) =
        Self::create_screen_uniform(&device, surface_context.size.width as f32, surface_context.size.height as f32);

        let pipeline = HelloPipeline::new(&device, surface_context.surface_format, &shader, &[&screen_bind_group_layout]);

        let renderer = Renderer {
            device,
            queue,
            pipeline,
            // vertex_buffer,
            // index_buffer,
            quad_buffers,
            screen_buffer,
            screen_bind_group,
            screen_uniform,
            surface_context,
        };


        renderer
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.surface_context.resize(new_size, &self.device);
        self.screen_uniform.update(new_size.width as f32, new_size.height as f32);
        self.queue.write_buffer(&self.screen_buffer, 0, bytemuck::cast_slice(&[self.screen_uniform]));
    }

    pub fn render(&mut self, window: &Window) {
        self.quad_buffers.clear();

        let quad = QuadInstance::new([20.0, 20.0], [80.0, 80.0], [1.0, 0.0, 0.0, 1.0]);

        self.quad_buffers.push(quad);
        self.quad_buffers.upload(&self.queue);

        let ctx = &self.surface_context;
        let surface_texture = match ctx.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => return,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                ctx.configure(&self.device);
                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                ctx.configure(&self.device);
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("No error scope registered, so validation errors will panic")
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                ctx.configure(&self.device);
                return;
            }
        };
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(ctx.surface_format.add_srgb_suffix()),
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
            render_pass.set_bind_group(0, &self.screen_bind_group, &[]);
            self.quad_buffers.draw(&mut render_pass);
        }

        self.queue.submit([encoder.finish()]);
        window.pre_present_notify();
        self.queue.present(surface_texture);
    }

    fn create_screen_uniform(
        device: &wgpu::Device,
        width: f32,
        height: f32,
    ) -> (ScreenUniform, wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let uniform = ScreenUniform::new(width, height);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Screen Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Screen Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Screen Bind Group"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        (uniform, buffer, layout, bind_group)
    }
}
