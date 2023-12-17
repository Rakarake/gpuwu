use gpuwu::run;

fn main() {
    use cosmic_text::{Attrs, Color, FontSystem, SwashCache, Buffer, Metrics, Shaping};
    
    // A FontSystem provides access to detected system fonts, create one per application
    let mut font_system = FontSystem::new();
    
    // A SwashCache stores rasterized glyphs, create one per application
    let mut swash_cache = SwashCache::new();
    
    // Text metrics indicate the font size and line height of a buffer
    let metrics = Metrics::new(14.0, 20.0);
    
    // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
    let mut buffer = Buffer::new(&mut font_system, metrics);
    
    // Borrow buffer together with the font system for more convenient method calls
    let mut buffer = buffer.borrow_with(&mut font_system);
    
    // Set a size for the text buffer, in pixels
    buffer.set_size(80.0, 25.0);
    
    // Attributes indicate what font to choose
    let attrs = Attrs::new();
    
    // Add some text!
    buffer.set_text("Hello in the hood Rust ksldjflksjdf lkjsdlfkj slkdjflkjsdlkfjlskdjfl;aksjdf;lkaj! 🦀\n", attrs, Shaping::Advanced);
    
    // Perform shaping as desired
    buffer.shape_until_scroll();
    
    // Inspect the output runs
    for run in buffer.layout_runs() {
        for glyph in run.glyphs.iter() {
            println!("{:#?}", glyph);
        }
    }

    // Create a default text color
    let text_color = Color::rgb(0xFF, 0xFF, 0xFF);
    

    let imgx = 80;
    let imgy = 25;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Draw the buffer (for performance, instead use SwashCache directly)
    let mut pixels = [[false; 80]; 25];
    buffer.draw(&mut swash_cache, text_color, |x, y, w, h, color| {
        // Fill in your code here for drawing rectangles
        //println!("Drawing: {:?}, {:?}, {:?}, {:?}, {:?}", x, y, w, h, color);
        pixels[y as usize][x as usize] = true;
        let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
        *pixel = image::Rgb([244_u8, 244_u8, 244_u8]);
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

