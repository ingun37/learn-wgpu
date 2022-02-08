use wgpu::{Device, ShaderModule};
pub fn make(device: &Device) -> ShaderModule {
    return device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });
}
