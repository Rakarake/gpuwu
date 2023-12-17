mod camera;
mod model;
mod render;
mod resources;
mod text;
mod texture;

use log::info;
use render::RenderState;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Code to create the window
use winit::dpi::PhysicalSize;
use winit::window::Window;
#[cfg(not(target_arch = "wasm32"))]
fn create_window(event_loop: &EventLoop<()>) -> (Window, PhysicalSize<u32>) {
    let size = PhysicalSize::new(400, 400);
    (
        WindowBuilder::new()
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap(),
        size,
    )
}
#[cfg(target_arch = "wasm32")]
fn create_window(event_loop: &EventLoop<()>) -> (Window, PhysicalSize<u32>) {
    use winit::platform::web::WindowBuilderExtWebSys;
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let canvas_element = doc.get_element_by_id("gpuwu-canvas")?;
            let canvas = canvas_element
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .ok()?;
            //info!("Webpage canvas title (tooltip): {:?}", canvas.title());
            //info!("Webpage canvas width: {:?}, height: {:?}", canvas.width(), canvas.height());
            let size = PhysicalSize::new(canvas.width(), canvas.height());
            Some((
                WindowBuilder::new()
                    .with_canvas(Some(canvas))
                    .with_inner_size(size)
                    .build(&event_loop)
                    .unwrap(),
                size,
            ))
        })
        .expect("Could not connect canvas and winit window.")
}

// Main program loop
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    // Initialize the right logging
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    info!("Hello There! OwO");

    let event_loop = EventLoop::new();

    let (window, window_size) = create_window(&event_loop);

    info!("Window: {:?}", window.inner_size());

    // Create render state
    let mut state = RenderState::new(window, window_size).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            info!("Resized!");
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}
