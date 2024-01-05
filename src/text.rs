use anyhow::*;
use crate::texture::Texture;
use crate::render::Vertex;

pub struct Text {
    text_buffer: cosmic_text::Buffer,
    texture: crate::texture::Texture,
    texture_bind_group: wgpu::BindGroup,
    position: TextVertex,
    size: (f32, f32),
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextVertex {
    pub position: [u32; 2],
}

impl Vertex for TextVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TextVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Uint32x2,
                },
            ],
        }
    }
}

pub trait DrawText<'a> {
    fn draw_text(
        &mut self,
        text_object: &'a Text,
    );
}

impl<'a, 'b> DrawText<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_text(&mut self, text_object: &'b Text) {
        self.set_bind_group(0, &text_object.texture_bind_group, &[]);
        self.draw(0..3, 0..1);
    }
}

impl Text {
    // TODO: use type system to make sure Text and this render pipeline are connected
    pub fn create_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        texture_bind_group_layout: &wgpu::BindGroupLayout
    ) -> wgpu::RenderPipeline {

        // TODO: change shader to the right one
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("text_shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        // Create the render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[TextVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            // The stencil buffer is us
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });
        render_pipeline
        
    }
    // Simple text from any string
    // If None is provided as width or height, that dimension is unbounded
    // Returns the resulted dimensions
    // TODO: make some sanity checks when converting between integer types
    pub fn new_from_str(
        text: &str,
        size: (Option<usize>, Option<usize>),
        color: cosmic_text::Color,
        font_system: &mut cosmic_text::FontSystem,
        swash_cache: &mut cosmic_text::SwashCache,
        metrics: cosmic_text::Metrics,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        position: (u32, u32),
        texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<Self> {
        use cosmic_text::{Attrs, Buffer, Shaping};

        let mut text_buffer = Buffer::new(font_system, metrics);
        let mut buffer = text_buffer.borrow_with(font_system);

        // Set a size for the text buffer, in pixels
        let width = if let Some(size_width) = size.0 {
            size_width as f32
        } else {
            f32::MAX
        };
        let height = if let Some(size_width) = size.1 {
            size_width as f32
        } else {
            f32::MAX
        };
        buffer.set_size(width, height);

        // Attributes indicate what font to choose
        let attrs = Attrs::new();

        // Finally, set the text!
        buffer.set_text(&text, attrs, Shaping::Advanced);

        // Perform shaping as desired
        buffer.shape_until_scroll();

        // Set up the canvas
        let width = buffer.size().0;
        let height = metrics.line_height * buffer.layout_runs().count() as f32;

        // Create image to give to texture
        let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

        // Draw to the canvas
        buffer.draw(swash_cache, color, |x, y, w, h, color| {
            let a = color.a();
            if a == 0
                || x < 0
                || x >= width as i32
                || y < 0
                || y >= height as i32
                || w != 1
                || h != 1
            {
                // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
                return;
            }

            // Scale by alpha (mimics blending with black)
            let scale = |c: u8| (c as i32 * a as i32 / 255).clamp(0, 255) as u8;

            let r = scale(color.r());
            let g = scale(color.g());
            let b = scale(color.b());
            let a = scale(color.a());

            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            *pixel = image::Rgba([r, g, b, a]);
        });

        // Create GPU texture
        let texture = crate::texture::Texture::from_image(
            device,
            queue,
            &image::DynamicImage::ImageRgba8(imgbuf),
            label,

        )?;

        // Create texture's bind group
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        let result = Text {
            text_buffer,
            texture,
            position: TextVertex { position: [position.0, position.1] },
            size: (width, height),
            texture_bind_group,
        };
        Ok(result)

    //text_buffer: cosmic_text::Buffer,
    //texture: crate::texture::Texture,
    //position: TextVertex,
    }
}

