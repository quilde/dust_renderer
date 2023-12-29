//cd Documents\nils\Programming\rust\gui\penna\dust_renderer

use tao::{
    event::{Event, WindowEvent, KeyEvent, ElementState, self},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window, WindowId}, dpi::{PhysicalSize, LogicalSize},
  };
  

use wgpu::{Device, Queue, SurfaceConfiguration, Surface, Extent3d};

use std::collections::HashMap;
use std::rc::Rc;



pub struct DustRenderer {
    label: &'static str,
    //depth_buffer,
    plugins: HashMap<&'static str, Rc<dyn RenderPlugin>>,
    attachments: Attachments,
}
impl DustRenderer {
    pub fn new(label: &'static str,) -> Self {
        Self{
            label,
            plugins: HashMap::new().into(),
            attachments: Attachments::new(),
        }
    }
    pub fn prepare(&mut self, device: &Device, queue: &wgpu::Queue) {
        self.attachments.prepare(&device, &queue);
    }
    pub fn add_plugin(&mut self, label: &'static str, plugin: Rc<dyn RenderPlugin> ) {
        self.plugins.insert(label,plugin);
        println!("adding plugin");
    }
    pub fn bindgroups(&self) -> &Vec<wgpu::BindGroupLayout>{
        &self.attachments.bind_group_layouts
    }
    pub fn render(
    &mut self,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    surface: &wgpu::Surface,
    config: &wgpu::SurfaceConfiguration,) -> Result<(), wgpu::SurfaceError> {
        let output = surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        
        
        for p in self.plugins.values() {
            //let p: &dyn RenderPlugin = *p.borrow();
            p.prepare();
        }
        
        
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 0.5,
                        }),
                        store: true,
                        //store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                //timestamp_writes: None,
                //occlusion_query_set: None,
            });
            
            for p in self.plugins.values() {
                p.render(&mut render_pass, device, &self.attachments.bind_groups);
            }
        }
        
        
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
    
}


pub trait RenderPlugin {
    
    fn prepare(&self) {}
    fn render<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>, device: &Device, bind_groups: &Vec<wgpu::BindGroup>) {}
}

struct RenderQueue {
    commands: Vec<RenderCommand>,
}
struct RenderCommand {
    label: &'static str,
    command: u32,
    indices: Vec<u32>,
}



struct Attachments {
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
            textures: Vec::new(),
            texture_dimensions: Vec::new(),
            texture_views: Vec::new(),
            samplers: Vec::new(),
            bind_group_layouts: Vec::new(),
            bind_groups: Vec::new(),
        }
    }
    pub fn prepare(&mut self, device: &Device, queue: &Queue) {
        //let dimensions = glam::Vec2{x: 1.0, y: 1.0};
        let dimensions = glam::UVec2{x: 1, y: 1};
        let texture_size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };
        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                
                size: texture_size,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
                view_formats: &[],
            }
        );
        let mut data: Vec<u8> = vec![];
        for _ in 0..dimensions.x {
            for _ in 0..dimensions.y {
                data.push(0);
            }
        }
        let diffuse_rgba = data.as_slice();
        
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            diffuse_rgba,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.x),
                rows_per_image: Some(dimensions.y),
            },
            texture_size,
        );
        let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
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
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        
        self.textures.push(diffuse_texture);
        self.texture_dimensions.push(texture_size);
        self.texture_views.push(diffuse_texture_view);
        self.samplers.push(diffuse_sampler);
        self.bind_group_layouts.push(texture_bind_group_layout);
        self.bind_groups.push(diffuse_bind_group);
    }

}



pub struct DustMain {
    compute_pipeline: wgpu::ComputePipeline,
}
impl DustMain {
    pub fn new(device: &Device, bind_groups: &Vec<wgpu::BindGroupLayout>) -> Self {
        
        let module = device.create_shader_module(
            wgpu::ShaderModuleDescriptor { 
                label: Some("compute ShaderModuleDescriptor"), 
                source: wgpu::ShaderSource::Wgsl(include_str!("dust.wgsl").into()),
            }
        );
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_groups[0]],
            push_constant_ranges: &[],
        });
        
        let desc = wgpu::ComputePipelineDescriptor { 
            label: Some("ComputePipelineDescriptor"), 
            layout: Some(&layout), 
            module: &module, 
            entry_point: "main_image",
        };
        let compute_pipeline = device.create_compute_pipeline(&desc);
        Self {
            compute_pipeline,
        }
    }
}
impl RenderPlugin for DustMain {
    
    
    fn prepare(&self, ) {
    
    }
    fn render<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>, device: &Device, bind_groups: &Vec<wgpu::BindGroup>) {
        let input_f = &[1.0f32, 2.0f32];
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &bind_groups[0], &[]);
            cpass.dispatch_workgroups(input_f.len() as u32, 1, 1);
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

