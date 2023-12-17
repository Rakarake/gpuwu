use gpuwu::run;

use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};
use std::fmt::Write;

fn main() {
    // A FontSystem provides access to detected system fonts, create one per application
    let mut font_system = FontSystem::new();

    // A SwashCache stores rasterized glyphs, create one per application
    let mut swash_cache = SwashCache::new();

    // Text metrics indicate the font size and line height of a buffer
    const FONT_SIZE: f32 = 14.0;
    const LINE_HEIGHT: f32 = FONT_SIZE * 1.2;
    let metrics = Metrics::new(FONT_SIZE, LINE_HEIGHT);

    // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
    let mut buffer = Buffer::new(&mut font_system, metrics);

    let mut buffer = buffer.borrow_with(&mut font_system);

    // Set a size for the text buffer, in pixels
    let width = 80.0;
    let height = f32::MAX; // The height is unbounded
    buffer.set_size(width, height);

    // Attributes indicate what font to choose
    let attrs = Attrs::new();

    // Add some text!
    let text = std::env::args()
        .nth(1)
        .unwrap_or(" Hi, Rust! 🦀 ".to_string());
    buffer.set_text(&text, attrs, Shaping::Advanced);

    // Perform shaping as desired
    buffer.shape_until_scroll();

    // Default text color (0xFF, 0xFF, 0xFF is white)
    const TEXT_COLOR: Color = Color::rgb(0xFF, 0xFF, 0xFF);

    // Set up the canvas
    let width = buffer.size().0;
    let height = LINE_HEIGHT * buffer.layout_runs().count() as f32;
    let mut canvas = vec![vec![None; width as usize]; height as usize];
    // Render the canvas to width x height
    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    // Draw to the canvas
    buffer.draw(&mut swash_cache, TEXT_COLOR, |x, y, w, h, color| {
        println!("x: {:?}, y: {:?}", x, y);
        let a = color.a();
        if a == 0 || x < 0 || x >= width as i32 || y < 0 || y >= height as i32 || w != 1 || h != 1 {
            // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
            return;
        }

        // Scale by alpha (mimics blending with black)
        let scale = |c: u8| (c as i32 * a as i32 / 255).clamp(0, 255) as u8;

        let r = scale(color.r());
        let g = scale(color.g());
        let b = scale(color.b());
        canvas[y as usize][x as usize] = Some((r, g, b));

        let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
        *pixel = image::Rgb([r, g, b]);
    });
    imgbuf.save("fractal.png").unwrap();

    //// Cool fractal
    //let imgx = 800;
    //let imgy = 800;

    //let scalex = 3.0 / imgx as f32;
    //let scaley = 3.0 / imgy as f32;

    //// Create a new ImgBuf with width: imgx and height: imgy
    //let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    //// Iterate over the coordinates and pixels of the image
    //for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //    let r = (0.3 * x as f32) as u8;
    //    let b = (0.3 * y as f32) as u8;
    //    *pixel = image::Rgb([r, 0, b]);
    //}

    //// A redundant loop to demonstrate reading image data
    //for x in 0..imgx {
    //    for y in 0..imgy {
    //        let cx = y as f32 * scalex - 1.5;
    //        let cy = x as f32 * scaley - 1.5;

    //        let c = num_complex::Complex::new(-0.4, 0.6);
    //        let mut z = num_complex::Complex::new(cx, cy);

    //        let mut i = 0;
    //        while i < 255 && z.norm() <= 2.0 {
    //            z = z * z + c;
    //            i += 1;
    //        }

    //        let pixel = imgbuf.get_pixel_mut(x, y);
    //        let image::Rgb(data) = *pixel;
    //        *pixel = image::Rgb([data[0], i as u8, data[2]]);
    //    }
    //}

    //// Save the image as “fractal.png”, the format is deduced from the path
    //imgbuf.save("fractal.png").unwrap();

    // GPUWU main loop here
    pollster::block_on(run());
}
