//cd Documents\nils\Programming\rust\gui\penna\dust_renderer

use tao::{
    event::{Event, WindowEvent, KeyEvent, ElementState, self},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window, WindowId}, dpi::{PhysicalSize, LogicalSize},
  };
  
use env_logger;
use wgpu::{Device, SurfaceConfiguration, Surface};

use std::collections::HashMap;
use std::rc::Rc;


pub struct DustRenderer {
    label: &'static str,
    //depth_buffer,
    plugins: HashMap<&'static str, Rc<dyn RenderPlugin>>,
    
}
impl DustRenderer {
    pub fn new(label: &'static str,) -> Self {
        Self{
            label,
            plugins: HashMap::new().into(),
            
        }
    }
    pub fn add_plugin(&mut self, label: &'static str, plugin: Rc<dyn RenderPlugin> ) {
        self.plugins.insert(label,plugin);
        println!("adding plugin");
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
                    },
                })],
                depth_stencil_attachment: None,
            });
            
            for p in self.plugins.values() {
                p.render(&mut render_pass, device);
            }
        }
        
        
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
    
}


pub trait RenderPlugin {
    fn prepare(&self) {}
    fn render<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>, device: &Device) {}
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
    
}
impl Attachments {
    fn new() -> Self {
        Self {
            
        }
    }
}



pub struct DustMain {
    compute_pipeline: wgpu::ComputePipeline,
}
impl DustMain {
    pub fn new(device: &Device) -> Self {
        
        let module = device.create_shader_module(
            wgpu::ShaderModuleDescriptor { 
                label: Some("compute ShaderModuleDescriptor"), 
                source: wgpu::ShaderSource::Wgsl(include_str!("dust.wgsl").into()),
            }
        );
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
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
    fn prepare(&self) {
    
    }
    fn render<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>, device: &Device) {
        let input_f = &[1.0f32, 2.0f32];
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.compute_pipeline);
            
            cpass.dispatch_workgroups(input_f.len() as u32, 1, 1);
        }
        
    }
}



pub async fn setup(window: Window) -> (Option<Window>, PhysicalSize<u32>, wgpu::Instance, wgpu::Device, wgpu::Surface, wgpu::SurfaceConfiguration, wgpu::Queue, ){
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

