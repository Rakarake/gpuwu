//use wgpu::util::DeviceExt;
//
//#[repr(C)]
//#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
//struct TextVertex {
//    position: [f32; 3]
//    w: u32,
//    h: u32,
//    color: [],
//}
//
//pub struct Text {
//    text_buff: cosmic_text::Buffer,
//    gpu_buff: wgpu::Buffer
//}
//
//impl Text {
//    const ATTRIBS: [wgpu::VertexAttribute; 3] =
//        //                       position             something     color
//        wgpu::vertex_attr_array![0 => Float32x3, 1 => Uint32x2, 3 => Float32x4];
//
//    fn desc() -> wgpu::VertexBufferLayout<'static> {
//        wgpu::VertexBufferLayout {
//            array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
//            step_mode: wgpu::VertexStepMode::Vertex,
//            attributes: &Self::ATTRIBS,
//        }
//    }
//
//    // Simple text from any string
//    fn from_str(
//        text: &str,
//        font_system: &mut cosmic_text::FontSystem,
//        swash_cache: &mut cosmic_text::SwashCache,
//        metrics: cosmic_text::Metrics,
//        device: &wgpu::Device,
//        queue: &wgpu::Queue,
//        layout: &wgpu::BindGroupLayout,
//    ) -> Self {
//        // A cosmic_text Buffer should be created for each "widget"
//        let text_buff = cosmic_text::Buffer::new(font_system, metrics);
//        text_buff.set_size(font_system, 80.0, 25.0);
//        let attrs = cosmic_text::Attrs::new();
//        text_buff.set_text(font_system, text, attrs, cosmic_text::Shaping::Advanced);
//        text_buff.shape_until_scroll(font_system);
//        let text_color = cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF);
//        let mut vertices = Vec::new();
//        text_buff.draw(font_system, swash_cache, text_color, |x, y, w, h, color| {
//            // Fill in your code here for drawing rectangles
//            println!("Drawing: {:?}, {:?}, {:?}, {:?}, {:?}", x, y, w, h, color);
//            vertices.push(TextVertex { x, y, w, h, color });
//        });
//
//        // The GPU buffer
//        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//            label: Some("Vertex Buffer"),
//            contents: bytemuck::cast_slice(&[1,2,3]),
//            usage: wgpu::BufferUsages::VERTEX,
//        });
//        //Text { buff:  }
//        Text { text_buff: text_buff, gpu_buff:  }
//    }
//}
//
//use std::ops::Range;
//use cosmic_text::Buffer;
//// Draw text without transformations, for now
//pub trait DrawTextSimple<'a> {
//    fn draw_text_simple(&mut self, text_buffer: &'a Buffer);
//}
