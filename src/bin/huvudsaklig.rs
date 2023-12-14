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
    buffer.set_text("Hello\n in the hood\n Rust! 🦀\n", attrs, Shaping::Advanced);
    
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
    
    // Draw the buffer (for performance, instead use SwashCache directly)
    buffer.draw(&mut swash_cache, text_color, |x, y, w, h, color| {
        // Fill in your code here for drawing rectangles
        println!("Drawing: {:?}, {:?}, {:?}, {:?}, {:?}", x, y, w, h, color);
    });


    // GPUWU main loop here
    pollster::block_on(run());
}

