use wgpu::{BindGroupLayout, Device, PipelineLayout};
pub fn make(device: &Device, texture_bind_group_layout: &BindGroupLayout) -> PipelineLayout {
    return device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[texture_bind_group_layout],
        push_constant_ranges: &[],
    });
}
