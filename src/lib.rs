mod texture;
mod camera;
mod render;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use render::RenderState;
use log::info;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

// Code to create the window
use winit::window::Window;
use winit::dpi::PhysicalSize;
#[cfg(not(target_arch="wasm32"))]
fn create_window(event_loop: &EventLoop<()>) -> Window {
    WindowBuilder::new()
       .with_inner_size(PhysicalSize::new(400, 400))
       .with_min_inner_size(PhysicalSize::new(400, 400))
       .build(&event_loop).unwrap()
}
#[cfg(target_arch="wasm32")]
fn create_window(event_loop: &EventLoop<()>) -> Window {
    use winit::platform::web::WindowBuilderExtWebSys;
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let canvas_element = doc.get_element_by_id("gpuwu-canvas")?;
            info!("YES, ALRIGHT");
            let canvas = canvas_element
                .dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
            Some(WindowBuilder::new()
                .with_canvas(Some(canvas))
                .build(&event_loop).unwrap())
        })
        .expect("Could not connect canvas and winit window.")
}

// Main program loop
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {
    // Initialize the right logging
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
            //console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    info!("Hello There! OwO");

    let event_loop = EventLoop::new().unwrap();

    let window = create_window(&event_loop);

    info!("Window: {:?}", window.inner_size());

    // Create render state
    let mut state = RenderState::new(window).await;
    // Event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
                // Idk about this
                state.window().request_redraw();
            },
            // When new frames are wanted
            Event::AboutToWait => {
                state.update();
            },
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                     => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }).unwrap();
}

