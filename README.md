# Dust Renderer 
*a 2d sdf-based renderer written in rust using wgpu*
## Status
== project on ice. ==
When you are learning wgpu you might find [[src/dumb_errors.md]] helpful
### features implemented:
![vscode with dust renderer window open, rendering two black circles](https://github.com/quilde/dust_renderer/blob/main/src/rendering3.PNG)

- able to draw circles
- image fills kindof (very broken)
- all sorts of behind the scenes stuff, 
- safe wrappers around storage buffers, texture atlas etc. You can find them inthe wburrito module. The storage buffer wrappers use a version of [vger's GPUVec](https://github.com/audulus/vger-rs/blob/main/src/gpu_vec.rs) that has been modified to use [encase](https://crates.io/crates/encase) to ensure correct memory layout. the texture atlas version uses [guillotiere](https://crates.io/crates/guillotiere)


## Project goals
- create 2d renderer
    - shapes ⚫(✅)
    - paths (likely similar to vger or like described in the text rendering articles)
    - images 
    - blur 
    - encoding as a tree ✅
    - text rendering probably using one of these techniques:
        - [GPU text rendering with vector textures](https://wdobbie.com/post/gpu-text-rendering-with-vector-textures/)
        - [Easy Scalable Text Rendering on the GPU](https://medium.com/@evanwallace/easy-scalable-text-rendering-on-the-gpu-c3f4d782c5ac)
        - [GPU Font Rendering](https://github.com/GreenLightning/gpu-font-rendering)
- learn about rendering ✅✅✅
- finding ways to improve experience with wgpu and to help newcomers (✅)

## Why compute shaders if we are going for a simple sdf-approach ?
Dust Renderer was going for what Raph Levien calls the "shader toy approach", meaning the threads don't communicate, they just compute their color from the underlying data.

The dust renderer uses Signed Distance Fields. (I'm not going to be the ten thousandth person gushing about how good they are and showing cool gifs. You can find all of that online. they're pretty cool though)

- we don't need any vertices
- => compute shader is basically fragment shader that fills the whole screen
- encoding the scene as a tree is only possible with storage buffers and one draw call. we need encoding as a tree to allow for more complex clipping and blurring. 

- being able to pipe the resulting texture into other renderers like with bevy_vello

I see now that it could have been possible to do it in the traditional pipeline, except for the last point.

## Why I don't plan on continuing it

I might return to this in the future if I see more value in doing this and have the ressources to do it. 

- graphics programming is hard. To get to this point, it took me over half a year. I learned a lot, for sure, but it is very little what you get for a lot of hard work and bug fixing 
- Vello is now released and pretty much usable
- I have a lot to do

