use std::mem::size_of;
use std::ops::Index;
use wgpu::*;
use encase::{StorageBuffer, ShaderSize, private::WriteInto};

#[derive(Debug)]
pub struct GPUVec<T: Copy> {
    buffer: wgpu::Buffer,
    capacity: usize,
    pub data: Vec<T>,
    label: String,
}

impl<T: Copy + ShaderSize + WriteInto + std::fmt::Debug> GPUVec<T> {
    
    pub fn new_from(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, from: Vec<T> ) -> Self {
        //let staticfrom: &Vec<&T> = &from.iter().map(|i|{i}).collect();
        let mut byte_buffer: Vec<u8> = Vec::new();
        
        let mut storage_buffer = encase::StorageBuffer::new(&mut byte_buffer);
        storage_buffer.write(
            &from
        ).unwrap();
        
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: byte_buffer.len() as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        queue.write_buffer(&buffer, 0, &byte_buffer.as_slice());
        
        
        Self {
            buffer,
            capacity: byte_buffer.len(),
            data: from,
            label: label.into(),
        }
    }
    
    
    /// Updates the underlying gpu buffer with self.data.
    ///
    /// We'd like to write directly to the mapped buffer, but that seemed
    /// tricky with wgpu.
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> bool {
        
        let mut byte_buffer: Vec<u8> = Vec::new();
        
        let mut storage_buffer = encase::StorageBuffer::new(&mut byte_buffer);
        storage_buffer.write(
            &self.data
        ).unwrap();
        
        
        let realloc = byte_buffer.len() > self.capacity;
        if realloc {
            self.capacity = byte_buffer.len();
            self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(self.label.as_str()),
                size: byte_buffer.len() as u64,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        //let sz = self.data.len() * size_of::<T>();
        /*let sz = byte_buffer.len();
        queue.write_buffer(&self.buffer, 0, unsafe {
            std::slice::from_raw_parts_mut(self.data[..].as_ptr() as *mut u8, sz)
        }); */
        
        queue.write_buffer(&self.buffer, 0, &byte_buffer.as_slice());

        println!(":::[GPUVec] <{}> capacity {} byte_buffer {}: {:?}\n {:?}", self.label ,self.capacity, &byte_buffer.len(), &byte_buffer, &self.data);
        //dbg!(&self.data);
        
        realloc
    }
    

    pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE ,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(self.buffer.as_entire_buffer_binding()),
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn replace(&mut self, v: Vec<T>) {
        self.data = v;
    }
}

impl<T: Copy> Index<usize> for GPUVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[derive(Debug)]
pub struct StorageTextureWrap {
    pub texture_size: wgpu::Extent3d,
    pub texture: wgpu::Texture,
    texture_view: wgpu::TextureView,

}
impl StorageTextureWrap {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, dimensions: &glam::UVec2, ) -> Self {
        let texture_size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(
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
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        Self {
            texture_size,
            texture,
            texture_view,
        }
    }
    
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, dimensions: &glam::UVec2, ) {
        let texture_size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(
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
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        self.texture_size = texture_size;
        self.texture = texture;
        self.texture_view = texture_view;
    }
    
    pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE ,
            ty: wgpu::BindingType::StorageTexture { 
                access: wgpu::StorageTextureAccess::WriteOnly, 
                format: wgpu::TextureFormat::Rgba8Unorm, 
                view_dimension: wgpu::TextureViewDimension::D2, 
            },
            count: None,
        }
    }
    
    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(&self.texture_view),
        }
    }
    
}

#[derive(Debug)]
pub struct StorageTextureArrayWrap {
    pub texture_size: wgpu::Extent3d,
    pub texture: wgpu::Texture,
    texture_view: wgpu::TextureView,

}
impl StorageTextureArrayWrap {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, dimensions: &glam::UVec2, ) -> Self {
        let texture_size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(
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
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        Self {
            texture_size,
            texture,
            texture_view,
        }
    }
    
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, dimensions: &glam::UVec2, ) {
        let texture_size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(
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
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        self.texture_size = texture_size;
        self.texture = texture;
        self.texture_view = texture_view;
    }
    
    pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE ,
            ty: wgpu::BindingType::StorageTexture { 
                access: wgpu::StorageTextureAccess::WriteOnly, 
                format: wgpu::TextureFormat::Rgba8Unorm, 
                view_dimension: wgpu::TextureViewDimension::D2, 
            },
            count: None,
        }
    }
    
    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(&self.texture_view),
        }
    }
    
}

#[derive(Debug)]
pub struct TextureWrap {
    pub texture_size: wgpu::Extent3d,
    pub texture: wgpu::Texture,
    texture_view: wgpu::TextureView,

}
impl TextureWrap {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, dimensions: &glam::UVec2, ) -> Self {
        let texture_size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(
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
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        Self {
            texture_size,
            texture,
            texture_view,
        }
    }
    
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, dimensions: &glam::UVec2, ) {
        let texture_size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(
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
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        self.texture_size = texture_size;
        self.texture = texture;
        self.texture_view = texture_view;
    }
    
    pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture { 
                sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                view_dimension: wgpu::TextureViewDimension::D2, 
                multisampled: false, 
            },
            count: None,
        }
    }
    
    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(&self.texture_view),
        }
    }
    
}
#[derive(Debug)]
pub struct SamplerWrap {
    sampler: wgpu::Sampler,

}
impl SamplerWrap {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, ) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });
        
        Self {
            sampler,
        }
    }
    
    pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            // This should match the filterable field of the
            // corresponding Texture entry above.
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        } 
    }
    
    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Sampler(&self.sampler),
        }
    }
    
}

#[derive(Debug)]
pub struct GroupWrap {
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub label_layout: String,
    pub label_group: String,
}
impl GroupWrap {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, entries: Vec<(BindGroupLayoutEntry, BindGroupEntry)>, label_layout: &str, label_group: &str) -> Self {
        let (entries_layout, entries_group): &(Vec<wgpu::BindGroupLayoutEntry>, Vec<wgpu::BindGroupEntry>) = &entries.into_iter().unzip();
        
        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &entries_layout.iter().as_slice(),
                label: Some(label_layout),
        });
        
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &entries_group.iter().as_slice(),
                
                label: Some(label_group),
            }
        );
        
        Self {
            layout,
            bind_group,
            label_layout: label_layout.to_string(),
            label_group: label_group.to_string(),
        }
    }
    
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, entries: Vec<(BindGroupLayoutEntry, BindGroupEntry)>) {
        let (entries_layout, entries_group): &(Vec<wgpu::BindGroupLayoutEntry>, Vec<wgpu::BindGroupEntry>) = &entries.into_iter().unzip();

        self.layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &entries_layout.iter().as_slice(),
            label: Some(self.label_layout.as_str()),
        });
        self.bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.layout,
                entries: &entries_group.iter().as_slice(),
                
                label: Some(self.label_group.as_str()),
            }
        );
    }
    
}