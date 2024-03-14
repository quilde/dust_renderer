@compute @workgroup_size(16, 16)
fn main_image(@builtin(global_invocation_id) id: vec3u) {
    // Viewport resolution (in pixels)
    let screen_size = textureDimensions(screen);

    // Prevent overdraw for workgroups on the edge of the viewport
    if id.x >= screen_size.x || id.y >= screen_size.y { return; }

    // Pixel coordinates (centre of pixel, origin at bottom left)
    var fragCoord_standard = vec2f(f32(id.x) + .5, f32(screen_size.y - id.y) - .5);
    var fragCoord = vec2f(0., 0.);

    var col = vec3f(1.,1.,1.);

    // Normalised pixel coordinates (from 0 to 1)
    var uv = fragCoord / vec2f(screen_size);

    var transforms_counter = 0;


    for (var i = 0u; i < arrayLength(&render_queue); i++) {
        switch render_queue[i].command{
            default: {
            }
            case 0u: {
            }
            case 1u: {
            }
            case 2u: {
            }
            case 3u: {
                fragCoord = (vec3f(fragCoord_standard, 1.0) * transforms[transforms_counter]).xy;
                transforms_counter++;
                var d = sdCircle(fragCoord, 100.);

                if d < 0.0 {
                    col = vec3f(0., 0., 0.);
                } else {
                    col = vec3f(1., 1., 1.);
                }
            }
        }
    }
    
    

    // Convert from gamma-encoded to linear colour space
    col = pow(col, vec3f(2.2));

    // Output to screen (linear colour space)
    textureStore(screen, id.xy, vec4f(col, 1.));
}




@group(0) @binding(0) var screen: texture_storage_2d<rgba8unorm,write>;
//@group(0) @binding(1) var samplers: sampler;

@group(1) @binding(0) var<storage> render_queue: array<RenderCommand>;

struct RenderCommand {
    id: u32,
    command: u32,
}

@group(2) @binding(0) var<storage> transforms: array<mat3x3<f32>>;

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}