use wgpu::{Texture, TextureView};

pub fn make(t: &Texture) -> TextureView {
    return t.create_view(&wgpu::TextureViewDescriptor::default());
}
