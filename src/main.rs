use tao::{
    event::{Event, WindowEvent, KeyEvent, ElementState, self},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window, WindowId}, dpi::{PhysicalSize, LogicalSize},
  };
  
use env_logger;
use wgpu::{Device, SurfaceConfiguration, Surface};

use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::{Borrow};

fn main() {
    pollster::block_on(run());
}

pub async fn run() {
    
    let mut dust = DustRenderer::new("penna label");
    let mut dust_main = DustMain::new();
    dust.add_plugin("text", Rc::new(dust_main) );
    
    let event_loop = EventLoop::new();
    let mut window_beforemove = Some(
        WindowBuilder::new()
        .with_decorations(true)
        .with_inner_size(tao::dpi::LogicalSize::new(300.0, 300.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(200.0, 200.0))
        .with_transparent(true)
        .build(&event_loop)
        .unwrap()
    );
    
    
    
    let (mut window, PhysicalSize, instance, device, mut surface, mut surface_configuration, queue, ) = setup(window_beforemove.unwrap()).await;
    
    let size = window.as_ref().unwrap().inner_size();
    
    env_logger::init();
    
    event_loop.run(move |event_main, _, control_flow| { //2: _
        *control_flow = ControlFlow::Wait;
        println!("{event_main:?}");
            match event_main {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                    ..
                
                } => {
                    // drop the window to fire the `Destroyed` event
                    window = None;
                  }
                Event::WindowEvent {
                    event: WindowEvent::Destroyed,
                    window_id: _,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                  }
                Event::WindowEvent {
                    event: WindowEvent::Resized(physical_size),
                    window_id: _,
                    ..
                } => {
                    
                  }
                Event::WindowEvent {
                    event: WindowEvent::ScaleFactorChanged{new_inner_size,..},
                    window_id: _,
                    ..
                } => {
                    
                  }
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { device_id, event: KeyEvent{state: ElementState::Pressed,..}, is_synthetic, .. },
                    window_id,
                    ..
                } => {
                    
                  }
                Event::DeviceEvent { device_id, event, .. } => {
                    println!("device event!!!!");
                }
                Event::MainEventsCleared => {
                    //if let w = window { //Some(w)}
                      //windows[&window_id].request_redraw();
                    
                  }
                Event::RedrawRequested(window_id)   => {
                    println!("\nredrawing!\n");
                    
                    let result = dust.render(&device,&queue, "label", &surface, &surface_configuration);
                    
                    match result {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => resize_window(size, &mut surface_configuration, &device, &mut surface),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    } 
                  }
                       
                  _ => (),
            }
            
        });    
    
}



pub struct DustRenderer {
    label: &'static str,
    //depth_buffer,
    plugins: HashMap<&'static str, Rc<dyn RenderPlugin>>,
    tree: RenderElementTree,
}
impl DustRenderer {
    fn new(label: &'static str,) -> Self {
        Self{
            label,
            plugins: HashMap::new().into(),
            tree: RenderElementTree::new()
        }
    }
    fn add_plugin(&mut self, label: &'static str, plugin: Rc<dyn RenderPlugin> ) {
        self.plugins.insert(label,plugin);
        println!("adding plugin");
    }
    fn render(
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
                p.render(&mut render_pass);
            }
        }
        
        
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}

trait RenderPlugin {
    fn prepare(&self) {}
    fn render<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>) {}
}

struct RenderQueue {
    commands: Vec<RenderCommand>,
}
struct RenderCommand {
    label: &'static str,
    command: u32,
    indices: Vec<u32>,
}


struct RenderElement {
    
}
impl RenderElement {
    
}
struct Attachments {
    
}
impl Attachments {
    fn new() -> Self {
        Self {
            
        }
    }
}

struct RenderElementTree {
    render_elements: HashMap<u64, RenderElement>,
    attachments: Attachments,
}
impl RenderElementTree {
    fn new() -> Self {
        Self {
            render_elements: HashMap::<u64, RenderElement>::new(),
            attachments: Attachments::new(),
        }
    }
}


struct DustMain {
    
}
impl DustMain {
    fn new() -> Self {
        Self {
            
        }
    }
}
impl RenderPlugin for DustMain {
    fn prepare(&self) {
    
    }
    fn render<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>) {
        
    }
}



async fn setup(window: Window) -> (Option<Window>, PhysicalSize<u32>, wgpu::Instance, wgpu::Device, wgpu::Surface, wgpu::SurfaceConfiguration, wgpu::Queue, ){
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

fn resize_window(new_size: tao::dpi::PhysicalSize<u32>, surface_configuration: &mut SurfaceConfiguration, device: &Device, surface: &mut Surface) {
    if new_size.width > 0 && new_size.height > 0 {
        //self.size = new_size;
        surface_configuration.width = new_size.width;
        surface_configuration.height = new_size.height;
        surface.configure(device, surface_configuration);
    }
}

