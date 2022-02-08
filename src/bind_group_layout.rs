use wgpu::{BindGroupLayout, Device};
pub fn make(device: &Device) -> BindGroupLayout {
    return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(
                    // SamplerBindingType::Comparison is only for TextureSampleType::Depth
                    // SamplerBindingType::Filtering if the sample_type of the texture is:
                    //     TextureSampleType::Float { filterable: true }
                    // Otherwise you'll get an error.
                    wgpu::SamplerBindingType::Filtering,
                ),
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
    });
}
