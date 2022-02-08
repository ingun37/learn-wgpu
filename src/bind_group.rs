use wgpu::{BindGroup, BindGroupLayout, Device, Sampler, TextureView};
pub fn make(
    device: &Device,
    texture_bind_group_layout: &BindGroupLayout,
    diffuse_texture_view: &TextureView,
    diffuse_sampler: &Sampler,
) -> BindGroup {
    return device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(diffuse_texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(diffuse_sampler),
            },
        ],
        label: Some("diffuse_bind_group"),
    });
}
