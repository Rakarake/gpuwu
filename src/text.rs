pub struct TextVertex {
    x: u32
}

impl TextVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        //                       position             something     color
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Uint32x2, 3 => Float32x4];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

use std::ops::Range;
use cosmic_text::Buffer;
// Draw text without transformations, for now
pub trait DrawTextSimple<'a> {
    fn draw_text_simple(&mut self, text_buffer: &'a Buffer);
}
