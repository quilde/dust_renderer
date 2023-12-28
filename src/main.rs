use dust_renderer::{DustRenderer, DustMain, setup, resize_window};

use tao::{
    event::{Event, WindowEvent, KeyEvent, ElementState, self},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window, WindowId}, dpi::{PhysicalSize, LogicalSize},
  };
  
use env_logger;
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
    
    let size = window.as_ref().unwrap().inner_size();
    
    let mut dust = DustRenderer::new("dust label DustRenderer");
    dust.prepare(&device, &queue);
    let mut dust_main = DustMain::new(&device, dust.bindgroups());
    dust.add_plugin("text", Rc::new(dust_main) );
    
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