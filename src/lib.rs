//cd Documents\nils\Programming\rust\gui\penna\dust_renderer

use tao::{
    event::{Event, WindowEvent, KeyEvent, ElementState, self},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window, WindowId}, dpi::{PhysicalSize, LogicalSize},
};
use wgpu::{Device, Queue, SurfaceConfiguration, Surface, Extent3d, Texture, SurfaceTexture, RenderPipeline, BindGroupLayout, ImageCopyTexture, BindGroupEntry};
use std::{collections::HashMap, cell::RefCell, borrow::BorrowMut, ops::DerefMut, num::NonZeroU32};
use std::rc::Rc;

use encase::{
    ShaderType,
    
};

mod wwrapers;

struct RenderQueue {
    label: &'static str,
    commands: Vec<RenderCommand>,
}

#[derive(ShaderType, Clone, Copy, Debug)]
struct RenderCommand {
    id: u32,
    command: u32,

}

#[derive(Debug)]
pub struct Attachments2 { 
    rq: Option<wwrapers::GPUVec<RenderCommand>>,
    rq_group: Option<wwrapers::GroupWrap>,
    target: Option<wwrapers::StorageTextureWrap>,
    target_group: Option<wwrapers::GroupWrap>,
    blit: Option<wwrapers::TextureWrap>,
    blit_group: Option<wwrapers::GroupWrap>,
}
impl Attachments2 {
    pub fn new() -> Self {
        Self {
            rq: None,
            rq_group: None,
            target: None,
            target_group: None,
            blit: None,
            blit_group: None,
            
        }
    }
}


#[derive(Debug)]
pub struct Attachments {
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
    pub fn push_layout(&mut self, layout: wgpu::BindGroupLayout)-> usize {
        self.bind_group_layouts.push(layout);
        self.bind_group_layouts.len() - 1 
    }
    pub fn push_bindgroup(&mut self, group: wgpu::BindGroup)-> usize {
        self.bind_groups.push(group);
        self.bind_groups.len() - 1 
    }

}

pub struct DustMain {
    compute_pipeline: wgpu::ComputePipeline,
    blit_pipeline: wgpu::RenderPipeline,
    attachments: Attachments,
    attachments2: Attachments2,
}
impl DustMain {
    pub fn new(device: &Device, queue: &Queue, config: &SurfaceConfiguration, window_size: glam::UVec2) -> Self {
        let mut attachments = Attachments::new();
        
        //let dimensions = glam::Vec2{x: 1.0, y: 1.0};
        //let dimensions = glam::UVec2{x: 100, y: 100};
        let dimensions = window_size;
        
        //let (key_output, key_blit) = Self::create_target_and_blit(&device, &queue, &dimensions, &mut attachments).expect("bad");
        
        //attachments.target_blit_keys = Some((key_output, key_blit));
        
        let target = wwrapers::StorageTextureWrap::new(device, queue, &dimensions);
        let blit = wwrapers::TextureWrap::new(device, queue, &dimensions);
        let sampler = wwrapers::SamplerWrap::new(device, queue);
        
        let target_group = wwrapers::GroupWrap::new(device, queue, vec![
            (wwrapers::StorageTextureWrap::bind_group_layout_entry(0), target.bind_group_entry(0)),
        ], "label_layout target", "label_group target");
        
        dbg!(&blit);
        let blit_group = wwrapers::GroupWrap::new(device, queue, vec![
            (wwrapers::TextureWrap::bind_group_layout_entry(0), blit.bind_group_entry(0)),
            (wwrapers::SamplerWrap::bind_group_layout_entry(1), sampler.bind_group_entry(1)),
        ], "label_layout blit", "label_group blit");
        
        
        
        let mut attachments2 = Attachments2::new();
        
        
        let rq = wwrapers::GPUVec::<RenderCommand>::new_from(device, queue, "label gpuvec", vec![
            RenderCommand {
            id: 0,
            command: 0,
        },
        RenderCommand {
            id: 1,
            command: 0,
        },
        ]);
        
        let rq_group = wwrapers::GroupWrap::new(
            device, 
            queue, 
            vec![
            (wwrapers::GPUVec::<RenderCommand>::bind_group_layout_entry(0),rq.bind_group_entry(0)),
            ],
            "rq bindgroup layout", "rq bindgroup",
        );
        

        
        
        let compute_module = device.create_shader_module(
            wgpu::ShaderModuleDescriptor { 
                label: Some("compute ShaderModuleDescriptor"), 
                source: wgpu::ShaderSource::Wgsl(include_str!("dust.wgsl").into()),
            }
        );
        dbg!(&attachments.bind_group_layouts);
        
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&target_group.layout, &rq_group.layout],
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
            bind_group_layouts: &[&blit_group.layout],
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
        
                attachments2.rq = Some(rq);
        attachments2.rq_group = Some(rq_group);
        attachments2.target = Some(target);
        attachments2.target_group = Some(target_group);
        attachments2.blit = Some(blit);
        attachments2.blit_group = Some(blit_group);
        
        Self {
            compute_pipeline,
            blit_pipeline: img_pipeline,
            attachments,
            attachments2,
        }
    }
    
    
    
    pub fn setup(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        
        
        
    }
    
    
    pub fn prepare(&mut self, device: &Device, queue: &Queue, ) {
        
    }
    pub fn render_compute(&mut self, mut encoder: wgpu::CommandEncoder,device: &wgpu::Device, queue: &wgpu::Queue, window_size: glam::UVec2) -> wgpu::CommandEncoder{
        
        
        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &self.attachments2.target_group.as_ref().unwrap().bind_group, &[]);
            cpass.set_bind_group(1, &self.attachments2.rq_group.as_ref().unwrap().bind_group, &[]);
            cpass.dispatch_workgroups((window_size.x / 16) + 1, (window_size.y / 16) + 1, 1);
            
        }
        //dbg!(&self.attachments.target.as_ref().unwrap().size());
        encoder.copy_texture_to_texture(
            ImageCopyTexture {
                texture: &self.attachments2.target.as_ref().unwrap().texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            }, 
            ImageCopyTexture {
                texture: &self.attachments2.blit.as_ref().unwrap().texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            }, 
            self.attachments2.target.as_ref().unwrap().texture_size,
        );
        
        encoder
    }
    pub fn render<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>, device: &Device) {
        
        
            rpass.set_pipeline(&self.blit_pipeline); 
            rpass.set_bind_group(0, &self.attachments2.blit_group.as_ref().unwrap().bind_group, &[]);
            rpass.draw(0..5, 0..1); 
    
    }
    pub fn resize(&mut self,device: &Device, queue: &Queue, new_size: glam::UVec2) {
        
        if new_size.x != 0{
            self.attachments2.target.as_mut().unwrap().update(device, queue, &new_size);
            self.attachments2.blit.as_mut().unwrap().update(device, queue, &new_size);
            
            let target = self.attachments2.target.unwrap();
            let blit = self.attachments2.blit.unwrap();
            
            let target_group = wwrapers::GroupWrap::new(device, queue, vec![
                (wwrapers::StorageTextureWrap::bind_group_layout_entry(0), target.bind_group_entry(0)),
            ], "label_layout target", "label_group target");
            
            let blit_group = wwrapers::GroupWrap::new(device, queue, vec![
                (wwrapers::TextureWrap::bind_group_layout_entry(0), blit.bind_group_entry(0)),
                (wwrapers::SamplerWrap::bind_group_layout_entry(1), sampler.bind_group_entry(1)),
            ], "label_layout blit", "label_group blit");
            
            self.attachments2.target_group = Some(target_group);
            self.attachments2.blit_group = Some(blit_group);
        }
        
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
