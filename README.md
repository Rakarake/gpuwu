# GPUwU

Gotta do
* 

Ok, create render objects that hold all the render state.

```rust
let uwu = gpuwu::UwU::new();
let duck = gpuwu::Render3DObject::new("objs/cube.obj", (0,0,0))
    .grid(4)
    .build();

let debug_text = gpuwu::Debug::new("Haha, lol", (10.0, 12.0));

let mut state: i32 = 0;

uwu.start(|uwu_renderer: Renderer, delta: f64, input: Input|
    uwu_renderer.render([
        duck,
        debug_text,
    ]);
    state += 1;
);

```

