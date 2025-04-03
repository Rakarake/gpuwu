# GPUwU

NOTE: this is just a simple API draft.

Abstracts rendering of simple 2D objects, window creation,
the event loop and window related action such as copy-pasting.

Objects bind to low level resources and thus shallowly embedded.
Because of rust 😆, a generic state is passed by reference to the callback
functions.

```rust
// UwU is the handy "get things on the screen struct"
let uwu = gpuwu::UwU<i32>::new()
    .window_title("What's this?")
    .build();

// Simple rectangles
let background = gpuwu::Rectangle::new(uwu)
    .position(2.0, 2.0)
    .size(60.0, 80.0)
    .build();

// Coordinate system is in pixels
let text = gpuwu::Text::new(uwu)
    .text("Bigger Chungus 🐰")
    .position(10.0, 12.0)
    .size(30.0, 30.0)
    .build();

let mut state: i32 = 0;

// Can choose different callbacks
uwu.gogogo(&mut state)
    .on_update(| state: &mut i32, renderer: &mut gpuwu::Renderer | {
        if state % 2 == 0 {
            renderer.render([
                background,
                text,
            ]);
        }
    })
    .on_event(| state: &mut i32, event: gpuwu::Event | {
        match event {
            gpuwu::Event::InputPressed(gpuwu::Input::A) => {
                state += 1;
            }
        }
    })
    .owO();
```

