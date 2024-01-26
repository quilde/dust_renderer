struct RenderElementTree {
    label: &'static str,
    render_elements: HashMap<u64, RenderElement>,
    attachments: Attachments,
}
impl RenderElementTree {
    fn new(label: &'static str) -> Self {
        Self {
            label,
            render_elements: HashMap::<u64, RenderElement>::new(),
            attachments: Attachments::new(),
        }
    }
    fn insert(&mut self, element: RenderElement) -> Result<u64, String> {
        for i in 0..u64::MAX {
            match self.render_elements.get(&i) {
                Some(r) => {
                        // go on
                    },
                None => {
                    self.render_elements.insert(i, element);
                    return Ok(i);
                },
        
            }
        }
        let label = self.label;
        let message = format!("failed to insert a RenderElement into a RenderElementTree label: {label}\n probable cause: keys up to u64::MAX taken" );
        Err(message)
        
    }
}

struct RenderElement {
    
}
impl RenderElement {
    
}


fn insert_render_element(&mut self, render_element: RenderElement) -> u64 {
        return self.tree.insert(render_element).unwrap();
    }
    
    



/*
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
        */
        
        

pub fn create_target_and_blit(device: &wgpu::Device, queue: &wgpu::Queue, dimensions: &glam::UVec2, attachments: &mut Attachments, ) -> Option<(usize, usize)> {
        
        let texture_size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };
        
        let target_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                
                size: texture_size,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
                label: Some("target_texture"),
                view_formats: &[],
            }
        );
        let target_texture_view = target_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let target_bindgroup_layout =
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
            
        let target_bindgroup = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &target_bindgroup_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&target_texture_view),
                    },
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        
        
        
        
        let blit_texture = device.create_texture(
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
        let blit_texture_view = blit_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let blit_sampler: wgpu::Sampler;
        
        if attachments.target_blit_keys == None{
            blit_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });
            
            attachments.samplers.push(blit_sampler);
        }
        
        let blit_bindgroup_layout =
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
            
        let blit_bindgroup = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &blit_bindgroup_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&blit_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&attachments.samplers[0]),
                    } 
                ],
                label: Some("paint_bind_group"),
            }
        );
                
        attachments.target = Some(target_texture);
        attachments.blit = Some(blit_texture);
        
        if attachments.texture_dimensions.len() == 0 {
            attachments.texture_dimensions.push(texture_size);
        } else {
            attachments.texture_dimensions[0] = texture_size;
        }
        
        if attachments.texture_views.len() == 0 {
            attachments.texture_views.push(target_texture_view);
            attachments.texture_views.push(blit_texture_view);
        } else {
            attachments.texture_views[0] = target_texture_view;
            attachments.texture_views[1] = blit_texture_view;
        }
        
        if attachments.bind_groups.len() == 0 {
            attachments.bind_groups.push(target_bindgroup);
            attachments.bind_groups.push(blit_bindgroup);
        } else {
            attachments.bind_groups[0] = target_bindgroup;
            attachments.bind_groups[1] = blit_bindgroup;
        }
        
        match attachments.target_blit_keys {
            Some(a) => {
                attachments.bind_group_layouts[a.0] = target_bindgroup_layout;
                attachments.bind_group_layouts[a.1] = blit_bindgroup_layout;
                //dbg!(attachments);
                return  None;
            },
            None => {
                let key_output = attachments.push_layout(target_bindgroup_layout);
                let key_paint = attachments.push_layout(blit_bindgroup_layout);
                //dbg!(attachments);
                return Some((key_output, key_paint));
            },
        }        
    }
    



        let rq_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wwrapers::GPUVec::<RenderCommand>::bind_group_layout_entry(0),
                ],
                label: Some("rq bindgroup layout"),
            });
        
        let rq_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &rq_layout,
                entries: &[
                    rq.bind_group_entry(0),
                ],
                label: Some("diffuse_bind_group"),
            }
        );


        
        let rq_key = attachments.push_layout(rq_layout);
        let rq_group_key = attachments.push_bindgroup(rq_bind_group);
        
bind_group_layouts: &[&attachments.bind_group_layouts[key_output], &attachments.bind_group_layouts[rq_key]],


if new_size.x != 0{
            let _ = Self::create_target_and_blit(
                device, 
                queue, 
                &new_size, 
                &mut self.attachments, 
            );
            //dbg!(new_size);
        }


let mut attachments = &mut self.attachments;
self.attachments.prepare(&device, &queue);