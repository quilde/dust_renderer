# Wgsl

## Type [n] is invalid

thread 'main' panicked at ....cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.17.2\src\backend\direct.rs:3056:5:
wgpu error: Validation Error

Caused by:
    In Device::create_shader_module
      note: label = `compute ShaderModuleDescriptor`

Shader validation error:


    Type [2] '' is invalid
    Base type [1] for the array is invalid

you have two times the same binding e.g.
@group(0) @binding(0) var<storage, read_write> images: array<texture_2d<f32>>;
@group(0) @binding(0) var<storage, read> samplers: array<sampler>;
                   ^

## Operation '' can't work with [n0] and [n1]
    Entry point main_image at Compute is invalid
    Expression [116] is invalid
    Operation Divide can't work with [114] and [115]

probably wrong types: write 5.0 not 5

## expected '<', found '>'
    thread 'main' panicked at ...\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.17.2\src\backend\direct.rs:3056:5:
wgpu error: Validation Error

Caused by:
    In Device::create_shader_module
      note: label = `compute ShaderModuleDescriptor`

Shader 'compute ShaderModuleDescriptor' parsing error: expected '<', found '>'
   ┌─ wgsl:46:60
   │
46 │ @group(2) @binding(0) var<storage> transforms: array<mat3x3>;
   │                                                            ^ expected '<'


    expected '<', found '>'


note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\dust_renderer.exe` (exit code: 101)

array<mat3x3<f32>>

# git

## exit git log
To exit git log, type “q” or “z”.



# Cargo 

## Features

[dependencies.rand]
version = "0.7.2"
features = ["small_rng"]


https://crates.io/crates/naga_oil

# "Why is a trait not implemented for a type that clearly has it implemented?"


```
T, &T, and &mut T are all different types; and that means that &mut &mut T is likewise a different type. Traits are not automatically implemented for references to a type. If you wish to implement a trait for either of the references, you need to write it out explicitly.
```
https://stackoverflow.com/questions/44928882/why-do-i-get-the-error-the-trait-foo-is-not-implemented-for-mut-t-even-th

not the solution in this case though.

https://stackoverflow.com/questions/44437123/why-is-a-trait-not-implemented-for-a-type-that-clearly-has-it-implemented

```
The general problem is that the code has multiple versions of the crate, each providing a different version of the traits. The fact that Rust allows this is a good thing, but the error messages around it are confusing.

Your crate implements Serialize from version A but the library is using version B in a public interface. These traits are not compatible, so when you pass your type implementing Serialize@A to the function requiring Serialize@B, the compiler stops you.

While your example is about different traits, it's also possible for this to occur for types which have been re-exported from a crate.

cargo tree, available starting in Rust 1.44 is highly useful to verify this is your problem. It shows all of your dependencies and their versions. It even has a -d flag to show duplicate dependencies! That mode isn't shown here, but is highly useful.
```
crates-lsp apparently checks for the lowest common denominator and displays the highest version that doesn't break stuff. 

example cargo tree: 

dust_renderer v0.1.0 (...rust\gui\penna\dust_renderer)
├── encase v0.6.1
│   ├── const_panic v0.2.8
│   ├── encase_derive v0.6.1 (proc-macro)
│   │   └── encase_derive_impl v0.6.1
│   │       ├── proc-macro2 v1.0.71
│   │       │   └── unicode-ident v1.0.12
│   │       ├── quote v1.0.33
│   │       │   └── proc-macro2 v1.0.71 (*)
│   │       └── syn v2.0.42
│   │           ├── proc-macro2 v1.0.71 (*)
│   │           ├── quote v1.0.33 (*)
│   │           └── unicode-ident v1.0.12
│   ├── glam v0.24.2
│   └── thiserror v1.0.51
│       └── thiserror-impl v1.0.51 (proc-macro)
│           ├── proc-macro2 v1.0.71 (*)
│           ├── quote v1.0.33 (*)
│           └── syn v2.0.42 (*)
├── glam v0.25.0
├── kurbo v0.10.4
│   ├── arrayvec v0.7.4
│   └── smallvec v1.11.2
├── log v0.4.20
├── pollster v0.3.0
├── tao v0.24.0
│   ├── bitflags v1.3.2
│   ├── crossbeam-channel v0.5.10
│   │   ├── cfg-if v1.0.0
│   │   └── crossbeam-utils v0.8.18
│   │       └── cfg-if v1.0.0
│   ├── image v0.24.7
│   │   ├── bytemuck v1.14.0
│   │   ├── byteorder v1.5.0
│   │   ├── color_quant v1.1.0
│   │   ├── num-rational v0.4.1
│   │   │   ├── num-integer v0.1.45
│   │   │   │   └── num-traits v0.2.17
│   │   │   │       [build-dependencies]
│   │   │   │       └── autocfg v1.1.0
│   │   │   │   [build-dependencies]
│   │   │   │   └── autocfg v1.1.0
│   │   │   └── num-traits v0.2.17 (*)
│   │   │   [build-dependencies]
│   │   │   └── autocfg v1.1.0
│   │   └── num-traits v0.2.17 (*)
│   ├── instant v0.1.12
│   │   └── cfg-if v1.0.0
│   ├── lazy_static v1.4.0
│   ├── libc v0.2.151
│   ├── log v0.4.20
│   ├── once_cell v1.19.0
│   ├── parking_lot v0.12.1
│   │   ├── lock_api v0.4.11
│   │   │   └── scopeguard v1.2.0
│   │   │   [build-dependencies]
│   │   │   └── autocfg v1.1.0
│   │   └── parking_lot_core v0.9.9
│   │       ├── cfg-if v1.0.0
│   │       ├── smallvec v1.11.2
│   │       └── windows-targets v0.48.5
│   │           └── windows_x86_64_msvc v0.48.5
│   ├── raw-window-handle v0.6.0
│   ├── unicode-segmentation v1.10.1
│   ├── url v2.5.0
│   │   ├── form_urlencoded v1.2.1
│   │   │   └── percent-encoding v2.3.1
│   │   ├── idna v0.5.0
│   │   │   ├── unicode-bidi v0.3.14
│   │   │   └── unicode-normalization v0.1.22
│   │   │       └── tinyvec v1.6.0
│   │   │           └── tinyvec_macros v0.1.1
│   │   └── percent-encoding v2.3.1
│   ├── windows v0.52.0
│   │   ├── windows-core v0.52.0
│   │   │   └── windows-targets v0.52.0
│   │   │       └── windows_x86_64_msvc v0.52.0
│   │   ├── windows-implement v0.52.0 (proc-macro)
│   │   │   ├── proc-macro2 v1.0.71 (*)
│   │   │   ├── quote v1.0.33 (*)
│   │   │   └── syn v2.0.42 (*)
│   │   ├── windows-interface v0.52.0 (proc-macro)
│   │   │   ├── proc-macro2 v1.0.71 (*)
│   │   │   ├── quote v1.0.33 (*)
│   │   │   └── syn v2.0.42 (*)
│   │   └── windows-targets v0.52.0 (*)
│   ├── windows-implement v0.52.0 (proc-macro) (*)
│   └── windows-version v0.1.0
│       └── windows-targets v0.52.0 (*)
│   [build-dependencies]
│   └── cc v1.0.83
└── wgpu v0.18.0
    ├── arrayvec v0.7.4
    ├── cfg-if v1.0.0
    ├── flume v0.11.0
    │   ├── futures-core v0.3.30
    │   ├── futures-sink v0.3.30
    │   ├── nanorand v0.7.0
    │   │   └── getrandom v0.2.11
    │   │       └── cfg-if v1.0.0
    │   └── spin v0.9.8
    │       └── lock_api v0.4.11 (*)
    ├── log v0.4.20
    ├── parking_lot v0.12.1 (*)
    ├── profiling v1.0.13
    ├── raw-window-handle v0.5.2
    ├── smallvec v1.11.2
    ├── static_assertions v1.1.0
    ├── wgpu-core v0.18.1
    │   ├── arrayvec v0.7.4
    │   ├── bit-vec v0.6.3
    │   ├── bitflags v2.4.1
    │   ├── codespan-reporting v0.11.1
    │   │   ├── termcolor v1.4.0
    │   │   │   └── winapi-util v0.1.6
    │   │   │       └── winapi v0.3.9
    │   │   └── unicode-width v0.1.11
    │   ├── log v0.4.20
    │   ├── naga v0.14.2
    │   │   ├── bit-set v0.5.3
    │   │   │   └── bit-vec v0.6.3
    │   │   ├── bitflags v2.4.1
    │   │   ├── codespan-reporting v0.11.1 (*)
    │   │   ├── hexf-parse v0.2.1
    │   │   ├── indexmap v2.1.0
    │   │   │   ├── equivalent v1.0.1
    │   │   │   └── hashbrown v0.14.3
    │   │   │       ├── ahash v0.8.6
    │   │   │       │   ├── cfg-if v1.0.0
    │   │   │       │   ├── once_cell v1.19.0
    │   │   │       │   └── zerocopy v0.7.32
    │   │   │       │   [build-dependencies]
    │   │   │       │   └── version_check v0.9.4
    │   │   │       └── allocator-api2 v0.2.16
    │   │   ├── log v0.4.20
    │   │   ├── num-traits v0.2.17 (*)
    │   │   ├── rustc-hash v1.1.0
    │   │   ├── spirv v0.2.0+1.5.4
    │   │   │   ├── bitflags v1.3.2
    │   │   │   └── num-traits v0.2.17 (*)
    │   │   ├── termcolor v1.4.0 (*)
    │   │   ├── thiserror v1.0.51 (*)
    │   │   └── unicode-xid v0.2.4
    │   ├── parking_lot v0.12.1 (*)
    │   ├── profiling v1.0.13
    │   ├── raw-window-handle v0.5.2
    │   ├── rustc-hash v1.1.0
    │   ├── smallvec v1.11.2
    │   ├── thiserror v1.0.51 (*)
    │   ├── wgpu-hal v0.18.1
    │   │   ├── arrayvec v0.7.4
    │   │   ├── ash v0.37.3+1.3.251
    │   │   │   └── libloading v0.7.4
    │   │   │       └── winapi v0.3.9
    │   │   ├── bit-set v0.5.3 (*)
    │   │   ├── bitflags v2.4.1
    │   │   ├── d3d12 v0.7.0
    │   │   │   ├── bitflags v2.4.1
    │   │   │   ├── libloading v0.8.1
    │   │   │   │   └── windows-sys v0.48.0
    │   │   │   │       └── windows-targets v0.48.5 (*)
    │   │   │   └── winapi v0.3.9
    │   │   ├── glow v0.13.0
    │   │   ├── glutin_wgl_sys v0.5.0
    │   │   │   [build-dependencies]
    │   │   │   └── gl_generator v0.14.0
    │   │   │       ├── khronos_api v3.1.0
    │   │   │       ├── log v0.4.20
    │   │   │       └── xml-rs v0.8.19
    │   │   ├── gpu-alloc v0.6.0
    │   │   │   ├── bitflags v2.4.1
    │   │   │   └── gpu-alloc-types v0.3.0
    │   │   │       └── bitflags v2.4.1
    │   │   ├── gpu-allocator v0.23.0
    │   │   │   ├── backtrace v0.3.69
    │   │   │   │   ├── cfg-if v1.0.0
    │   │   │   │   └── rustc-demangle v0.1.23
    │   │   │   │   [build-dependencies]
    │   │   │   │   └── cc v1.0.83
    │   │   │   ├── log v0.4.20
    │   │   │   ├── presser v0.3.1
    │   │   │   ├── thiserror v1.0.51 (*)
    │   │   │   ├── winapi v0.3.9
    │   │   │   └── windows v0.51.1
    │   │   │       ├── windows-core v0.51.1
    │   │   │       │   └── windows-targets v0.48.5 (*)
    │   │   │       └── windows-targets v0.48.5 (*)
    │   │   ├── gpu-descriptor v0.2.4
    │   │   │   ├── bitflags v2.4.1
    │   │   │   ├── gpu-descriptor-types v0.1.2
    │   │   │   │   └── bitflags v2.4.1
    │   │   │   └── hashbrown v0.14.3 (*)
    │   │   ├── hassle-rs v0.10.0
    │   │   │   ├── bitflags v1.3.2
    │   │   │   ├── com-rs v0.2.1
    │   │   │   ├── libloading v0.7.4 (*)
    │   │   │   ├── thiserror v1.0.51 (*)
    │   │   │   ├── widestring v1.0.2
    │   │   │   └── winapi v0.3.9
    │   │   ├── khronos-egl v6.0.0
    │   │   │   ├── libc v0.2.151
    │   │   │   └── libloading v0.8.1 (*)
    │   │   ├── libloading v0.8.1 (*)
    │   │   ├── log v0.4.20
    │   │   ├── naga v0.14.2 (*)
    │   │   ├── once_cell v1.19.0
    │   │   ├── parking_lot v0.12.1 (*)
    │   │   ├── profiling v1.0.13
    │   │   ├── range-alloc v0.1.3
    │   │   ├── raw-window-handle v0.5.2
    │   │   ├── renderdoc-sys v1.0.0
    │   │   ├── rustc-hash v1.1.0
    │   │   ├── smallvec v1.11.2
    │   │   ├── thiserror v1.0.51 (*)
    │   │   ├── wgpu-types v0.18.0
    │   │   │   └── bitflags v2.4.1
    │   │   └── winapi v0.3.9
    │   └── wgpu-types v0.18.0 (*)
    ├── wgpu-hal v0.18.1 (*)
    └── wgpu-types v0.18.0 (*)


# wgpu
## pipeline layout, contains a bind group layout at index n which is incompatible with the bind group layout associated with the bind group at n
thread 'main' panicked at ....cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.17.2\src\backend\direct.rs:3056:5:
wgpu error: Validation Error

Caused by:
    In a ComputePass
      note: encoder = `Render Encoder`
    In a dispatch command, indirect:false
      note: compute pipeline = `ComputePipelineDescriptor`
    The pipeline layout, associated with the current compute pipeline, contains a bind group layout at index 2 which is incompatible with the bind group layout associated with the bind group at 2


note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\dust_renderer.exe` (exit code: 101)

you forgot to set the bindgroup
eg 
```rust
cpass.set_bind_group(
                2,
                &self.attachments2.streams_group.as_ref().unwrap().bind_group,
                &[],
            );
            
```

## panicked at 'Error in Queue::write_texture: copy of 0..n would end up overrunning the bounds of the Source buffer of size m'

https://www.reddit.com/r/bevy/comments/y64bk5/error_in_queuewrite_texture/?rdt=43488

it could be that your data is rgb when you have TextureFormat something with rgba

```rust
image.to_rgba8()``` if you use the image crate