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