use anyhow::*;
use crate::texture::Texture;

pub struct Text {
    text_buffer: cosmic_text::Buffer,
    texture: crate::texture::Texture,
    texture_bind_group: wgpu::BindGroup,
    instance_buffer: wgpu::Buffer,
    instances: Vec<TextInstanceRaw>,
}

// Describes one instance of the text, that is the position, width and height
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextInstanceRaw {
    pub position: [f32; 2], // x, y
    pub size: [f32; 2],     // width, height
}

impl TextInstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TextInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // Arbitrary, shared with vertices
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
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
        self.set_vertex_buffer(0, text_object.instance_buffer.slice(..));
        self.draw_indexed(0..6, 0, 0..text_object.instances.len() as u32);
    }
}

impl Text {
    // TODO: use type system to make sure Text and this render pipeline are connected
    pub fn create_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        texture_bind_group_layout: &wgpu::BindGroupLayout
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("text_shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Text Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        // Create the render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[TextInstanceRaw::desc()],
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
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        instances: Vec<TextInstanceRaw>,
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
        // TODO: fix width/height size problem
        println!("width: {:?}, height; {:?}", width, height);
        println!("width: {:?}, height; {:?}", width.round() as u32, height.round() as u32);
        let mut imgbuf = image::ImageBuffer::new(width.round() as u32, height.round() as u32);

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
            Some("Text Texture"),
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

        // Instance buffer
        use wgpu::util::DeviceExt;
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let result = Text {
            text_buffer,
            texture,
            texture_bind_group,
            instance_buffer,
            instances,
        };
        Ok(result)

        //text_buffer: cosmic_text::Buffer,
        //texture: crate::texture::Texture,
        //position: TextVertex,
    }
}

