use std::{mem, num::NonZeroU64};

use glam::{Mat3A, Mat4};
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, Device,
    ShaderStage,
};

use crate::ModeData;

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct PerObjectData {
    model_view: Mat4,
    model_view_proj: Mat4,
    inv_trans_model_view: Mat3A,
    // Unused in shader
    material_idx: u32,
}

unsafe impl bytemuck::Pod for PerObjectData {}
unsafe impl bytemuck::Zeroable for PerObjectData {}

pub struct ShaderInterfaces {
    pub samplers_bgl: BindGroupLayout,
    pub culled_object_bgl: BindGroupLayout,
    pub material_bgl: ModeData<BindGroupLayout, BindGroupLayout>,
    pub texture_bgl: ModeData<BindGroupLayout, BindGroupLayout>,
    pub uniform_bgl: BindGroupLayout,
}

impl ShaderInterfaces {
    pub fn new(device: &Device, max_texture_count: ModeData<(), usize>) -> Self {
        let samplers_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("sampler bgl"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::Sampler {
                        filtering: true,
                        comparison: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
            ],
        });

        let culled_object_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("culled object bgl"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(mem::size_of::<PerObjectData>() as _),
                },
                count: None,
            }],
        });

        // RESUME HERE

        Self {
            samplers_bgl,
            culled_object_bgl,
            material_bgl,
            texture_bgl,
            uniform_bgl,
        }
    }
}
