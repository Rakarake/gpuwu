// Position
struct InstanceInput {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    instance: InstanceInput,
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    // Counter-clock-wise
    let u = f32((in_vertex_index >= 2u) && (in_vertex_index < 5u));
    let v = f32((in_vertex_index >= 1u) && (in_vertex_index < 4u));
    let x = (u * 2.0) - 1.0; //u * instance.size.x + instance.position.x;
    let y = (v * 2.0) - 1.0; //v * instance.size.y + instance.position.y;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    out.tex_coords = vec2<f32>(u, 1.0 - v);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
