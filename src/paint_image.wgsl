
// This just draws the resulting image

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    tex_coords: vec2<f32>
};

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    
    const pos = array(
        vec2( 1.0, 1.0),
        vec2(-1.0, 1.0),
        vec2(-1.0,-1.0),
        vec2( 1.0,-1.0)
    );
    
    const pos_texture = array(
        vec2( 1.0, 0.0),
        vec2( 0.0, 0.0),
        vec2( 0.0, 1.0),
        vec2( 1.0, 1.0)
    );
    
    out.clip_position = vec4<f32>(pos[vertex_index], 0.0, 1.0);
    out.tex_coords = pos_texture[vertex_index];
    return out;
}
 
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
