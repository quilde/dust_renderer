use dust_renderer::{resize_window, setup, DustMain};

use tao::{
    dpi::{LogicalSize, PhysicalSize},
    event::{self, ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

use wgpu::{Device, Surface, SurfaceConfiguration};

use std::collections::HashMap;
use std::rc::Rc;

use glam::UVec2;

fn main() {
    pollster::block_on(run());
}

pub async fn run() {
    let event_loop = EventLoop::new();
    let window_beforemove = Some(
        WindowBuilder::new()
            .with_decorations(true)
            .with_inner_size(tao::dpi::LogicalSize::new(300.0, 300.0))
            .with_min_inner_size(tao::dpi::LogicalSize::new(200.0, 200.0))
            .with_transparent(true)
            .build(&event_loop)
            .unwrap(),
    );
    let window_beforemove2 = Some(
        WindowBuilder::new()
            .with_decorations(true)
            .with_inner_size(tao::dpi::LogicalSize::new(600.0, 600.0))
            .with_min_inner_size(tao::dpi::LogicalSize::new(200.0, 200.0))
            .with_transparent(true)
            .build(&event_loop)
            .unwrap(),
    );
    
    let size1 = glam::UVec2{
        x: window_beforemove.unwrap().inner_size().width,
        y: window_beforemove.unwrap().inner_size().height,
    };
    
    let size2 = glam::UVec2{
        x: window_beforemove2.unwrap().inner_size().width,
        y: window_beforemove2.unwrap().inner_size().height,
    };

    let (mut windows, mut surfaces, mut surface_configurations, devices, queues) =
        setup(vec![&window_beforemove.unwrap(),&window_beforemove2.unwrap(),], 
            vec![
            size1,
            size2,
                ]).await;

    //let mut dust = DustRenderer::new("dust label DustRenderer");

    let mut dust_main1 = DustMain::new(&devices[0], &queues[0], &surface_configurations[0], size1);
    dust_main1.setup(&devices[0], &queues[0]);
    
    let mut dust_main2 = DustMain::new(&devices[1], &queues[1], &surface_configurations[1], size2);
    dust_main2.setup(&devices[1], &queues[1]);
    //dust.add_plugin("dust_main plugin", Rc::new(dust_main) );
    //dust.prepare(&device, &queue, &surface_configuration);
    //env_logger::init();
    let mut now = std::time::Instant::now();

    event_loop.run(move |event_main, _, control_flow| {
        //2: _
        *control_flow = ControlFlow::Wait;

        //println!("{event_main:?}");
        match event_main {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                // drop the window to fire the `Destroyed` event
                windows = None;
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
                resize_window(
                    physical_size,
                    &mut surface_configuration,
                    &device,
                    &mut surface,
                );
                dust_main.resize(
                    &device,
                    &queue,
                    glam::UVec2 {
                        x: physical_size.width,
                        y: physical_size.height,
                    },
                );
            }
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
                window_id: _,
                ..
            } => {}
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        device_id,
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                ..
                            },
                        is_synthetic,
                        ..
                    },
                window_id,
                ..
            } => {}
            Event::DeviceEvent {
                device_id, event, ..
            } => {
                //println!("device event!!!!");
            }
            Event::MainEventsCleared => {
                //if let w = window { //Some(w)}
                //windows[&window_id].request_redraw();
                if let Some(w) = &windows {
                    windows.as_ref().unwrap().request_redraw();
                }
            }
            Event::RedrawRequested(window_id) => {
                println!("redrawing!\n");
                dust_main.prepare(&device, &queue);
                let size1 = &windows.as_ref().unwrap().inner_size();
                let size = glam::UVec2 {
                    x: size1.width,
                    y: size1.height,
                };
                let result = render(
                    &device,
                    &queue,
                    "label",
                    &surface,
                    &surface_configuration,
                    &mut dust_main,
                    &size,
                );

                match result {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => resize_window(
                        physical_size,
                        &mut surface_configuration,
                        &device,
                        &mut surface,
                    ),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }

                let fps = if now.elapsed().as_secs() != 0 {
                    1 / now.elapsed().as_secs()
                } else {
                    u64::MIN
                };
                print!("{}ms  {}   ", now.elapsed().as_millis(), fps);
                now = std::time::Instant::now();
            }

            _ => (),
        }
    });
}

