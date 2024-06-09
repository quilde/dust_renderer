use dust_renderer::{resize_window, setup, setup_single, test_op, DustMain};

use image::{GenericImageView, EncodableLayout};
use tao::{
    dpi::{LogicalSize, PhysicalSize},
    event::{self, ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

use wgpu::{Device, Extent3d, Surface, SurfaceConfiguration};

use std::{borrow::BorrowMut, collections::HashMap};
use std::rc::Rc;

mod render_element;

fn main() {
    pollster::block_on(run());
}

pub async fn run() {
    let event_loop = EventLoop::new();
    let mut window = Some(
        WindowBuilder::new()
            .with_decorations(true)
            .with_inner_size(tao::dpi::LogicalSize::new(300.0, 300.0))
            .with_min_inner_size(tao::dpi::LogicalSize::new(200.0, 200.0))
            .with_transparent(false)
            .build(&event_loop)
            .unwrap(),
    );

    let size = glam::UVec2 {
        x: window.as_ref().unwrap().inner_size().width,
        y: window.as_ref().unwrap().inner_size().height,
    };
    let (
        device,
        queue,
        mut surface,
        mut surface_configuration,
        
        
    ) = setup_single(window.as_ref().unwrap(), size).await;

    //let mut dust = DustRenderer::new("dust label DustRenderer");

    let mut dust_main = DustMain::new(&device, &queue, &surface_configuration, size);
    dust_main.setup(&device, &queue);
    //let image = std::fs::read("2023-08-15_episode_39_visual-development_the_awekening_of_the_komona_tree.jpg").unwrap();
    let image = image::io::Reader::open(r"C:\Users\ensel\Documents\nils\Programming\rust\gui\renderer\src\2023-08-15_episode_39_visual-development_the_awekening_of_the_komona_tree.jpg").unwrap().decode().unwrap();
    
    let texture_size = Extent3d {
        width: image.dimensions().0,
        height: image.dimensions().1,
        depth_or_array_layers: 1,
    };
    dust_main.allocate_image(&device, &queue, image.to_rgba8().as_bytes().to_vec(), texture_size);
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
                if let Some(w) = &window {
                    window.as_ref().unwrap().request_redraw();
                }
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
                if let Some(w) = &window {
                    window.as_ref().unwrap().request_redraw();
                }
            }
            Event::RedrawRequested(window_id) => {
                println!("redrawing!\n");
                dust_main.prepare_render(&device, &queue, test_op().borrow_mut());

                //dust_main.attachments.transforms.clear();
                //dust_main.test(&device, &queue);
                //dust_main.test(&device, &queue);
                //dust_main.attachments.transforms.update(&device, &queue);
                let size1 = &window.as_ref().unwrap().inner_size();
                let size = glam::UVec2 {
                    x: size1.width,
                    y: size1.height,
                };
                let result = dust_renderer::render(
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
                    Err(wgpu::SurfaceError::Lost) => dust_renderer::resize_window(
                        window.as_ref().unwrap().inner_size(),
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

pub fn render(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    surface: &wgpu::Surface,
    config: &wgpu::SurfaceConfiguration,
    dust_main: &mut DustMain,
    size: &glam::UVec2,
) -> Result<(), wgpu::SurfaceError> {
    let output = surface.get_current_texture()?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    let mut encoder = dust_main.render_compute(encoder, &device, &queue, *size);

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
