use anyhow::*;

pub struct Text {
    text_buffer: cosmic_text::Buffer,
    texture: crate::texture::Texture,
}

impl Text {
    // TODO: use type system to make sure Text and this render pipeline are connected
    //pub fn create_render_pipeline() -> wgpu::RenderPipeline {
    //    
    //}
    // Simple text from any string
    // If None is provided as width or height, that dimension is unbounded
    // Returns the resulted dimensions
    // TODO: make some sanity checks when converting between integer types
    pub fn from_str(
        text: &str,
        size: (Option<usize>, Option<usize>),
        color: cosmic_text::Color,
        font_system: &mut cosmic_text::FontSystem,
        swash_cache: &mut cosmic_text::SwashCache,
        metrics: cosmic_text::Metrics,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
    ) -> Result<(Self, (usize, usize))> {
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
        let result = Text { text_buffer, texture };
        Ok((result, (width as usize, height as usize)))
    }
}

