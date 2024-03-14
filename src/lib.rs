use crate::render_element::Node;

use wgpu::{Device, ImageCopyTexture, Queue, Surface, SurfaceConfiguration};

use encase::ShaderType;

mod render_element;
mod wburrito;

pub struct RenderQueue {
    label: &'static str,
    pub commands: Vec<RenderCommand>,
}
impl RenderQueue {
    fn new(label: &'static str) -> Self {
        Self {
            label,
            commands: Vec::new(),
        }
    }
}

#[derive(ShaderType, Clone, Copy, Debug)]
pub struct RenderCommand {
    id: u32,
    command: u32,
}

#[derive(Debug)]
pub struct Attachments {
    rq: wburrito::GPUVec<RenderCommand>,
    rq_group: wburrito::GroupWrap,
    target: wburrito::StorageTextureWrap,
    target_group: wburrito::GroupWrap,
    blit: wburrito::TextureWrap,
    blit_group: wburrito::GroupWrap,
    sampler: wburrito::SamplerWrap,
    pub transforms: wburrito::GPUVec<glam::Mat3>,
    streams_group: wburrito::GroupWrap,
}
impl Attachments {}

pub struct DustMain {
    compute_pipeline: wgpu::ComputePipeline,
    blit_pipeline: wgpu::RenderPipeline,

    pub attachments: Attachments,
}
impl DustMain {
    pub fn new(
        device: &Device,
        queue: &Queue,
        config: &SurfaceConfiguration,
        window_size: glam::UVec2,
    ) -> Self {
        let dimensions = window_size;

        let target = wburrito::StorageTextureWrap::new(device, queue, &dimensions);
        let blit = wburrito::TextureWrap::new(device, queue, &dimensions);
        let sampler = wburrito::SamplerWrap::new(device, queue);

        let target_group = wburrito::GroupWrap::new(
            device,
            queue,
            vec![(
                wburrito::StorageTextureWrap::bind_group_layout_entry(0),
                target.bind_group_entry(0),
            )],
            "label_layout target",
            "label_group target",
        );

        dbg!(&blit);
        let blit_group = wburrito::GroupWrap::new(
            device,
            queue,
            vec![
                (
                    wburrito::TextureWrap::bind_group_layout_entry(0),
                    blit.bind_group_entry(0),
                ),
                (
                    wburrito::SamplerWrap::bind_group_layout_entry(1),
                    sampler.bind_group_entry(1),
                ),
            ],
            "label_layout blit",
            "label_group blit",
        );

        let rq = wburrito::GPUVec::<RenderCommand>::new_from(
            device,
            queue,
            "label gpuvec",
            vec![
                RenderCommand { id: 0, command: 0 },
                RenderCommand { id: 1, command: 0 },
            ],
        );

        let rq_group = wburrito::GroupWrap::new(
            device,
            queue,
            vec![(
                wburrito::GPUVec::<RenderCommand>::bind_group_layout_entry(0),
                rq.bind_group_entry(0),
            )],
            "rq bindgroup layout",
            "rq bindgroup",
        );

        let transforms = wburrito::GPUVec::<glam::Mat3>::new_from(
            device,
            queue,
            "transforms",
            vec![glam::Mat3::from_cols_array(&[
                1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ])],
        );

        let streams_group = wburrito::GroupWrap::new(
            device,
            queue,
            vec![(
                wburrito::GPUVec::<glam::Mat3>::bind_group_layout_entry(0),
                transforms.bind_group_entry(0),
            )],
            "label_layout",
            "label_group",
        );

        let compute_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute ShaderModuleDescriptor"),
            source: wgpu::ShaderSource::Wgsl(include_str!("dust.wgsl").into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &target_group.layout,
                &rq_group.layout,
                &streams_group.layout,
            ],
            push_constant_ranges: &[],
        });

        let desc = wgpu::ComputePipelineDescriptor {
            label: Some("ComputePipelineDescriptor"),
            layout: Some(&layout),
            module: &compute_module,
            entry_point: "main_image",
        };
        let compute_pipeline = device.create_compute_pipeline(&desc);

        let blending = Some(wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent::OVER,
        });

        let module_image = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("paint image"),
            source: wgpu::ShaderSource::Wgsl(include_str!("paint_image.wgsl").into()),
        });
        let layout_img = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout paint image"),
            bind_group_layouts: &[&blit_group.layout],
            push_constant_ranges: &[],
        });
        let blit_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline paint image"),
            layout: Some(&layout_img),
            vertex: wgpu::VertexState {
                module: &module_image,
                entry_point: "vtx_main", // 1.
                buffers: &[],            // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &module_image,
                entry_point: "frag_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: blending,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: None,                  //Some(wgpu::Face::Back),
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

        let attachments = Attachments {
            rq,
            rq_group,
            target,
            blit,
            target_group,
            blit_group,
            sampler,
            transforms,
            streams_group,
        };

        Self {
            compute_pipeline,
            blit_pipeline,
            attachments,
        }
    }

    pub fn setup(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {}

    pub fn test(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.attachments
            .transforms
            .push(glam::Mat3::from_cols_array(&[
                1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ]));
    }

    pub fn prepare_render(
        &mut self,
        device: &Device,
        queue: &Queue,
        node: &mut render_element::Node,
    ) {
        let (v, t) = derive_buffers_from_node(node);

        let mut transforms = &mut self.attachments.transforms;
        transforms.replace(t);
        let realloc_transforms = transforms.update(device, queue);
        if realloc_transforms {
            self.attachments.streams_group.update(
                device,
                queue,
                vec![(
                    wburrito::GPUVec::<glam::Mat3>::bind_group_layout_entry(0),
                    transforms.bind_group_entry(0),
                )],
            );
        }

        let mut rq = &mut self.attachments.rq;
        rq.replace(v);
        let realloc_rq = rq.update(device, queue);
        if realloc_rq {
            self.attachments.rq_group.update(
                device,
                queue,
                vec![(
                    wburrito::GPUVec::<RenderCommand>::bind_group_layout_entry(0),
                    rq.bind_group_entry(0),
                )],
            );
        }
    }

    pub fn render_compute(
        &mut self,
        mut encoder: wgpu::CommandEncoder,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        window_size: glam::UVec2,
    ) -> wgpu::CommandEncoder {
        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &self.attachments.target_group.bind_group, &[]);
            cpass.set_bind_group(1, &self.attachments.rq_group.bind_group, &[]);
            cpass.set_bind_group(2, &self.attachments.streams_group.bind_group, &[]);
            cpass.dispatch_workgroups((window_size.x / 16) + 1, (window_size.y / 16) + 1, 1);
        }

        encoder.copy_texture_to_texture(
            ImageCopyTexture {
                texture: &self.attachments.target.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            ImageCopyTexture {
                texture: &self.attachments.blit.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            self.attachments.target.texture_size,
        );

        encoder
    }
    pub fn render<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>, device: &Device) {
        rpass.set_pipeline(&self.blit_pipeline);
        rpass.set_bind_group(0, &self.attachments.blit_group.bind_group, &[]);
        rpass.draw(0..5, 0..1);
    }
    pub fn resize(&mut self, device: &Device, queue: &Queue, new_size: glam::UVec2) {
        let target = &mut self.attachments.target;
        let blit = &mut self.attachments.blit;
        let sampler = &mut self.attachments.sampler;
        if new_size.x != 0 && new_size.y != 0 {
            target.update(device, queue, &new_size);
            blit.update(device, queue, &new_size);

            let target_group = wburrito::GroupWrap::new(
                device,
                queue,
                vec![(
                    wburrito::StorageTextureWrap::bind_group_layout_entry(0),
                    target.bind_group_entry(0),
                )],
                "label_layout target",
                "label_group target",
            );

            let blit_group = wburrito::GroupWrap::new(
                device,
                queue,
                vec![
                    (
                        wburrito::TextureWrap::bind_group_layout_entry(0),
                        blit.bind_group_entry(0),
                    ),
                    (
                        wburrito::SamplerWrap::bind_group_layout_entry(1),
                        sampler.bind_group_entry(1),
                    ),
                ],
                "label_layout blit",
                "label_group blit",
            );

            self.attachments.target_group = target_group;
            self.attachments.blit_group = blit_group;
        }
    }
}

pub async fn setup<
    W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
>(
    windows: Vec<&W>,
    sizes: Vec<glam::UVec2>,
) -> (
    Vec<&W>,
    Vec<wgpu::Surface>,
    Vec<wgpu::SurfaceConfiguration>,
    Vec<wgpu::Device>,
    Vec<wgpu::Queue>,
) {
    //let size = window.inner_size();

    let instance_desc = wgpu::InstanceDescriptor::default();
    let instance = wgpu::Instance::new(instance_desc);

    let mut surfaces = Vec::new();
    let mut configs = Vec::new();
    let mut devices = Vec::new();
    let mut queues = Vec::new();

    for i in 0..windows.len() {
        // Safety
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&windows[i]) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
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
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: sizes[i].x,
            height: sizes[i].y,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        surfaces.push(surface);
        devices.push(device);
        queues.push(queue);
        configs.push(config);
    }

    (windows, surfaces, configs, devices, queues)
}

pub async fn setup_single<
    W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
>(
    window: &W,
    size: glam::UVec2,
) -> (
    wgpu::Device,
    wgpu::Queue,
    wgpu::Surface,
    wgpu::SurfaceConfiguration,
) {
    let instance_desc = wgpu::InstanceDescriptor::default();
    let instance = wgpu::Instance::new(instance_desc);
    // Safety
    // The surface needs to live as long as the window that created it.
    // State owns the window so this should be safe.
    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
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
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.x,
        height: size.y,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    (device, queue, surface, config)
}

pub fn resize_window(
    new_size: tao::dpi::PhysicalSize<u32>,
    surface_configuration: &mut SurfaceConfiguration,
    device: &Device,
    surface: &mut Surface,
) {
    if new_size.width > 0 && new_size.height > 0 {
        //self.size = new_size;
        surface_configuration.width = new_size.width;
        surface_configuration.height = new_size.height;
        surface.configure(device, surface_configuration);
    }
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

pub fn test_op() -> render_element::Node {
    render_element::Node::Blend {
        layers: vec![
            render_element::Node::Circle {
                radius: 10.0,
                transform: glam::Mat3::from_cols_array(&[
                    1.0, 0.0, -100.0, 0.0, 1.0, -100.0, 0.0, 0.0, 1.0,
                ]),
            },
            render_element::Node::Circle {
                radius: 10.0,
                transform: glam::Mat3::from_cols_array(&[
                    1.0, 0.0, -200.0, 0.0, 1.0, -200.0, 0.0, 0.0, 1.0,
                ]),
            },
        ],
    }
}

fn derive_buffers_from_node(node: &mut Node) -> (Vec<RenderCommand>, Vec<glam::Mat3>) {
    let mut v = Vec::<RenderCommand>::new();
    let mut t = Vec::<glam::Mat3>::new();
    match node {
        render_element::Node::Blend { layers } => {
            for l in layers.iter_mut() {
                let (mut a, mut b) = derive_buffers_from_node(l);
                v.append(&mut a);
                t.append(&mut b);
            }

            v.push(RenderCommand { id: 0, command: 1 });
        }
        render_element::Node::Circle { radius, transform } => {
            v.push(RenderCommand { id: 0, command: 2 });

            t.push(*transform);
        }
    }

    return (v, t);
}
