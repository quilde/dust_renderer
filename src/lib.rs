//cd Documents\nils\Programming\rust\gui\penna\dust_renderer

use tao::{
    event::{Event, WindowEvent, KeyEvent, ElementState, self},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window, WindowId}, dpi::{PhysicalSize, LogicalSize},
};
use wgpu::{Device, Queue, SurfaceConfiguration, Surface, Extent3d, Texture, SurfaceTexture, RenderPipeline, BindGroupLayout, ImageCopyTexture};
use std::{collections::HashMap, cell::RefCell, borrow::BorrowMut, ops::DerefMut, num::NonZeroU32};
use std::rc::Rc;

use encase::{
    ShaderType,
    
};

struct RenderQueue {
    label: &'static str,
    commands: Vec<RenderCommand>,
}

#[derive(ShaderType)]
struct RenderCommand {
    id: u32,
    command: u32,

}

#[derive(Debug)]
struct Attachments {
    target: Option<wgpu::Texture>,
    blit: Option<wgpu::Texture>,
    target_blit_keys: Option<(usize,usize)>,
    textures: Vec<wgpu::Texture>,
    texture_dimensions: Vec<wgpu::Extent3d>,
    texture_views: Vec<wgpu::TextureView>,
    samplers: Vec<wgpu::Sampler>,
    bind_group_layouts: Vec<wgpu::BindGroupLayout>,
    bind_groups: Vec<wgpu::BindGroup>,
    
}
impl Attachments {
    pub fn new() -> Self {
        Self {
            target: None,
            blit: None,
            target_blit_keys: None,
            textures: Vec::new(),
            texture_dimensions: Vec::new(),
            texture_views: Vec::new(),
            samplers: Vec::new(),
            bind_group_layouts: Vec::new(),
            bind_groups: Vec::new(),
            
        }
    }
    pub fn prepare(&mut self, device: &Device, queue: &Queue) {
        
    }
    pub fn bind_groups(&self) -> &Vec<wgpu::BindGroupLayout>{
        &self.bind_group_layouts
    }
    pub fn push_layouts(&mut self, layout: BindGroupLayout)-> usize {
        self.bind_group_layouts.push(layout);
        self.bind_group_layouts.len() - 1 
    }

}

pub struct DustMain {
    compute_pipeline: wgpu::ComputePipeline,
    img_pipeline: wgpu::RenderPipeline,
    attachments: Attachments,
}
impl DustMain {
    pub fn new(device: &Device, queue: &Queue, config: &SurfaceConfiguration, window_size: glam::UVec2) -> Self {
        let mut attachments = Attachments::new();
        
        //let dimensions = glam::Vec2{x: 1.0, y: 1.0};
        //let dimensions = glam::UVec2{x: 100, y: 100};
        let dimensions = window_size;
        
        let (key_output, key_paint) = Self::create_target_and_blit(&device, &queue, &dimensions, &mut attachments).expect("bad");
        
        attachments.target_blit_keys = Some((key_output, key_paint));
        
        let mut byte_buffer: Vec<u8> = Vec::new();
        
        let mut buffer = encase::StorageBuffer::new(&mut byte_buffer);
        
        buffer.write(&RenderCommand {
            id: 0,
            command: 0,
        }).unwrap();
        
        let rq_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE ,
                        ty: wgpu::BindingType::Buffer { 
                            ty: wgpu::BufferBindingType::Storage { 
                                read_only: true 
                            }, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        },
                        count: None,
                    },
                ],
                label: Some("rq bindgroup layout"),
            });
        
        let rq_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("BufferDescriptor rq_buffer"),
                size: byte_buffer.len() as u64,
                usage: wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            }
        );
        let rq_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &rq_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                            wgpu::BufferBinding {
                                buffer: &rq_buffer,
                                offset: 0,
                                size:  None,
                            }
                        ),
                    },
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        
        let compute_module = device.create_shader_module(
            wgpu::ShaderModuleDescriptor { 
                label: Some("compute ShaderModuleDescriptor"), 
                source: wgpu::ShaderSource::Wgsl(include_str!("dust.wgsl").into()),
            }
        );
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&attachments.bind_group_layouts[key_output]],
            push_constant_ranges: &[],
        });
        
        let desc = wgpu::ComputePipelineDescriptor { 
            label: Some("ComputePipelineDescriptor"), 
            layout: Some(&layout), 
            module: &compute_module, 
            entry_point: "main_image",
        };
        let compute_pipeline = device.create_compute_pipeline(&desc);
        

        
        let blending = 
            Some(wgpu::BlendState{
                color: wgpu::BlendComponent{
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,},
                alpha: wgpu::BlendComponent::OVER
            });
        
        let module_image = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("paint image"),
            source: wgpu::ShaderSource::Wgsl(include_str!("paint_image.wgsl").into()),
        });
        let layout_img =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout paint image"),
            bind_group_layouts: &[&attachments.bind_group_layouts[key_paint]],
            push_constant_ranges: &[],
        });
        let img_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline paint image"),
            layout: Some(&layout_img),
            vertex: wgpu::VertexState {
                module: &module_image,
                entry_point: "vtx_main", // 1.
                buffers: &[
                    
                ], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &module_image,
                entry_point: "frag_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: blending,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: None, //Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState::default(), /*{
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },*/
            multiview: None, // 5.
        });
        
        
        
        
        Self {
            compute_pipeline,
            img_pipeline,
            attachments,
        }
    }
    
    pub fn create_target_and_blit(device: &wgpu::Device, queue: &wgpu::Queue, dimensions: &glam::UVec2, attachments: &mut Attachments, ) -> Option<(usize, usize)> {
        
        
        let texture_size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };
        let output_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                
                size: texture_size,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
                label: Some("diffuse_texture"),
                view_formats: &[],
            }
        );
        let mut data: Vec<u8> = vec![];
        
        fill_image(&mut data, &dimensions);
        let output_texture_data = data.as_slice();

        
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &output_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            output_texture_data,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.x ),
                rows_per_image: Some(dimensions.y ),
            },
            texture_size,
        );
        let diffuse_texture_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());

        
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE ,
                        ty: wgpu::BindingType::StorageTexture { 
                            access: wgpu::StorageTextureAccess::WriteOnly, 
                            format: wgpu::TextureFormat::Rgba8Unorm, 
                            view_dimension: wgpu::TextureViewDimension::D2, 
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
            
        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                    },
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        
                
        attachments.target = Some(output_texture);
        //attachments.textures.push(output_texture);
        attachments.texture_dimensions.pop();
        attachments.texture_dimensions.push(texture_size);
        
        attachments.texture_views.pop();
        attachments.texture_views.pop();
        attachments.bind_groups.pop();
        attachments.bind_groups.pop();
        
        
        attachments.texture_views.push(diffuse_texture_view);
        //self.samplers.push(diffuse_sampler);
        
        
        attachments.bind_groups.push(diffuse_bind_group);
        
        
        let paint_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                
                size: texture_size,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST ,
                label: Some("paint texture"),
                view_formats: &[],
            }
        );
        
        let paint_texture_data = data.as_slice();
        
        
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &paint_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            paint_texture_data,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.x ),
                rows_per_image: Some(dimensions.y ),
            },
            texture_size,
        );
        let paint_texture_view = paint_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let paint_sampler: wgpu::Sampler;
        
        if attachments.target_blit_keys == None{
        paint_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        
        attachments.samplers.push(paint_sampler);
        }
        
        
        let paint_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture { 
                            sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                            view_dimension: wgpu::TextureViewDimension::D2, 
                            multisampled: false, 
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    }, 
                ],
                label: Some("paint_texture_bind_group_layout"),
            });
            
        let paint_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &paint_texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&paint_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&attachments.samplers[0]),
                    } 
                ],
                label: Some("paint_bind_group"),
            }
        );
        
                
        attachments.blit = Some(paint_texture);
        //attachments.textures.push(paint_texture);
        attachments.texture_views.push(paint_texture_view);
        
        
        
        
        attachments.bind_groups.push(paint_bind_group);
        
        match attachments.target_blit_keys {
            Some(a) => {
                attachments.bind_group_layouts[a.0] = texture_bind_group_layout;
                attachments.bind_group_layouts[a.1] = paint_texture_bind_group_layout;
                dbg!(attachments);
                return  None;
            },
            None => {
                let key_output = attachments.push_layouts(texture_bind_group_layout);
                let key_paint = attachments.push_layouts(paint_texture_bind_group_layout);
                dbg!(attachments);
                return Some((key_output, key_paint));
            },
        }
        
        
    }
    
    pub fn setup(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut attachments = &mut self.attachments;
        
        
    }
    
    
    pub fn prepare(&mut self, device: &Device, queue: &Queue, ) {
        self.attachments.prepare(&device, &queue);
    }
    pub fn render_compute(&self, mut encoder: wgpu::CommandEncoder,device: &wgpu::Device, queue: &wgpu::Queue, window_size: glam::UVec2) -> wgpu::CommandEncoder{
        
        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &self.attachments.bind_groups[0], &[]);
            cpass.dispatch_workgroups((window_size.x / 16) + 1, (window_size.y / 16) + 1, 1);
            
        }
        dbg!(&self.attachments.target.as_ref().unwrap().size());
        encoder.copy_texture_to_texture(
            ImageCopyTexture {
                texture: &self.attachments.target.as_ref().unwrap(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            }, 
            ImageCopyTexture {
                texture: &self.attachments.blit.as_ref().unwrap(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            }, 
            self.attachments.texture_dimensions[0]
        );
        
        encoder
    }
    pub fn render<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>, device: &Device) {
        let attachments = &self.attachments;
        
            rpass.set_pipeline(&self.img_pipeline); 
            rpass.set_bind_group(0, &attachments.bind_groups[1], &[]);
            rpass.draw(0..5, 0..1); 
    
    }
    pub fn resize(&mut self,device: &Device, queue: &Queue, new_size: glam::UVec2) {
        let _ = Self::create_target_and_blit(
            device, 
            queue, 
            &new_size, 
            &mut self.attachments, 
        );
        dbg!(new_size);
    }
}





pub async fn setup(window: tao::window::Window) -> (Option<Window>, PhysicalSize<u32>, wgpu::Instance, wgpu::Device, wgpu::Surface, wgpu::SurfaceConfiguration, wgpu::Queue, ){
    let size = window.inner_size();
        
    let instance_desc = wgpu::InstanceDescriptor::default();
    let instance = wgpu::Instance::new(instance_desc);
    // Safety
    // The surface needs to live as long as the window that created it.
    // State owns the window so this should be safe.
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    
    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ).await
        .unwrap();
    
    let (device, queue) = adapter
    .request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
        },
        None,
        //trace_dir.ok().as_ref().map(std::path::Path::new),
    )
    .await
    .expect("Unable to find a suitable GPU adapter!");
    
    let surface_caps = surface.get_capabilities(&adapter);
    // Shader code in this tutorial assumes an sRGB surface texture. Using a different
    // one will result all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f| f.is_srgb())            
        .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);
    
    (Some(window), size, instance, device, surface, config, queue )
}

pub fn resize_window(new_size: tao::dpi::PhysicalSize<u32>, surface_configuration: &mut SurfaceConfiguration, device: &Device, surface: &mut Surface) {
    if new_size.width > 0 && new_size.height > 0 {
        //self.size = new_size;
        surface_configuration.width = new_size.width;
        surface_configuration.height = new_size.height;
        surface.configure(device, surface_configuration);
    }
}

fn fill_image(data: &mut Vec<u8>, dimensions: &glam::UVec2) {
    for _ in 0..dimensions.x {
            for _ in 0..(dimensions.y * 8) {
                data.push(0);
            }
        }
}