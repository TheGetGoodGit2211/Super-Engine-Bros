pub struct Shader {
    pub module: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(device: &wgpu::Device, source: &str, label: Option<&str>) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        Self { module }
    }
}
