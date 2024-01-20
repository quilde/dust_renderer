use std::mem::size_of;
use std::ops::Index;
use wgpu::*;
use encase::{StorageBuffer, ShaderSize, private::WriteInto};

pub struct GPUVec<T: Copy> {
    buffer: wgpu::Buffer,
    capacity: usize,
    data: Vec<T>,
    label: String,
}

impl<T: Copy + ShaderSize + WriteInto> GPUVec<T> {
    
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
            capacity: 1,
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
            self.capacity = byte_buffer.len().next_power_of_two();
            self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(self.label.as_str()),
                size: byte_buffer.len() as u64,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        //let sz = self.data.len() * size_of::<T>();
        /*queue.write_buffer(&self.buffer, 0, unsafe {
            std::slice::from_raw_parts_mut(self.data[..].as_ptr() as *mut u8, sz)
        }); */
        
        queue.write_buffer(&self.buffer, 0, &byte_buffer.as_slice());
        
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
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.buffer,
                offset: 0,
                size: None,
            }),
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
}

impl<T: Copy> Index<usize> for GPUVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}