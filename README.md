# GPUwU

The *render objects* hold references to resources on the GPU.

```rust
// UwU is the handy "get things on the screen struct"
let uwu = gpuwu::UwU::new();
let duck = gpuwu::Render3DObject::new("objs/cube.obj", (0,0,0))
    .grid(4)
    .build();

// Coordinate system is in pixels troglodyte style
let debug_text = gpuwu::Text::new("Haha, lol 😂\nYeah rite", (10.0, 12.0));

let mut state: i32 = 0;

uwu.gogogo(|renderer: &mut gpuwu::Renderer, input: &mut gpuwu::Input|
    // get, get_pressed, get_realeased are bool. get_axis is float
    if input.get(gpuwu::InputThing::KeyA, gpuwu::InputMode::Hold) {
        renderer.render([
            duck,
            debug_text,
        ]);
    }
    state += 1;
);
```

