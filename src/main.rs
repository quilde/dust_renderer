use dust_renderer::{DustMain, setup, resize_window};

use tao::{
    event::{Event, WindowEvent, KeyEvent, ElementState, self},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window, WindowId}, dpi::{PhysicalSize, LogicalSize},
  };
  

use wgpu::{Device, SurfaceConfiguration, Surface};

use std::collections::HashMap;
use std::rc::Rc;



fn main() {
    pollster::block_on(run());
}

pub async fn run() {
    
    
    
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
    
    let (mut window, physical_size, instance, device, mut surface, mut surface_configuration, queue, ) = setup(window_beforemove.unwrap()).await;
    
    let size = glam::UVec2 { x: physical_size.width, y: physical_size.height };
    
    //let mut dust = DustRenderer::new("dust label DustRenderer");
    
    let mut dust_main = DustMain::new(&device, &queue, &surface_configuration, size);
    dust_main.setup(&device, &queue);
    //dust.add_plugin("dust_main plugin", Rc::new(dust_main) );
    //dust.prepare(&device, &queue, &surface_configuration);
    //env_logger::init();
    
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
                    resize_window(physical_size, &mut surface_configuration, &device, &mut surface);
                    dust_main.resize(glam::UVec2 { x: physical_size.width, y: physical_size.height });
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
                    dust_main.prepare(&device, &queue);
                    let size1 = &window.as_ref().unwrap().inner_size();
                    let size =  glam::UVec2 { x: size1.width, y: size1.height };
                    let result = render(&device,&queue, "label", &surface, &surface_configuration, &dust_main, &size);
                    
                    match result {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => resize_window(physical_size, &mut surface_configuration, &device, &mut surface),
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

pub fn render(
    
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    surface: &wgpu::Surface,
    config: &wgpu::SurfaceConfiguration,
    dust_main: &DustMain,
    size: &glam::UVec2,
    ) -> Result<(), wgpu::SurfaceError> {
    let output = surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });
    
    let mut encoder = dust_main.render_compute(encoder,&device, &queue, *size);
    
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
        
        dust_main.render(&mut render_pass, &device);
        
    }
    
    
    queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}