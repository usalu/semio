//! 🎯️ Gpu targets and pipeline resources: the scene color, composite and shadow targets, the world globals ring, the vertex records and the `PipelineResources` state with the stencil states. See [`super`].

use super::*;

pub const WORLD_GLOBALS_SLOT_SIZE:u64=256;

pub const SCENE_MIP_LEVELS:u32=5;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct WorldLineGpuVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct BlurGlobals {
    pub src_mip: f32,
    pub _pad: [f32; 7],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WorldPostprocessGlobals {
    pub viewport: [f32; 4],
    pub surface_size: [f32; 2],
    pub fov_strength: [f32; 2],
    pub modes: [u32; 4],
    pub clear_color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GlassInstance {
    pub rect: [f32; 4],
    pub tint: [f32; 4],
    pub params: [f32; 4],
}

pub struct SceneColorTarget {
    pub texture: gpu_device::Texture,
    pub blur_scratch: gpu_device::Texture,
    pub blur_scratch_mip_views: Vec<gpu_device::TextureView>,
    pub world_encoded_view: gpu_device::TextureView,
    pub sample_view: gpu_device::TextureView,
    pub mip_views: Vec<gpu_device::TextureView>,
    pub sampler: gpu_device::Sampler,
    pub width: u32,
    pub height: u32,
}

impl SceneColorTarget {
    pub fn ensure(device: &gpu_device::Device, target: &mut Option<Self>, width: u32, height: u32, format: gpu_device::TextureFormat) {
        let width = width.max(1);
        let height = height.max(1);
        if let Some(existing) = target {
            if existing.width == width && existing.height == height {
                return;
            }
        }
        let world_encoded_format = format.remove_srgb_suffix();
        let texture = device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("scene_color"),
            size: gpu_device::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: SCENE_MIP_LEVELS,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: world_encoded_format,
            usage: gpu_device::TextureUsages::RENDER_ATTACHMENT | gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_SRC,
            view_formats: &[format, world_encoded_format],
        });
        let blur_scratch = device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("scene_blur_scratch"),
            size: gpu_device::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: SCENE_MIP_LEVELS,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: world_encoded_format,
            usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST,
            view_formats: &[format, world_encoded_format],
        });
        let blur_scratch_mip_views = (0..SCENE_MIP_LEVELS)
            .map(|level| {
                blur_scratch.create_view(&gpu_device::TextureViewDescriptor {
                    label: Some(&format!("scene_blur_scratch_mip_{level}")),
                    format: Some(format),
                    dimension: Some(gpu_device::TextureViewDimension::D2),
                    base_mip_level: level,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let sample_view = texture.create_view(&gpu_device::TextureViewDescriptor {
            label: Some("scene_color_sample"),
            format: Some(format),
            dimension: Some(gpu_device::TextureViewDimension::D2),
            base_mip_level: 0,
            mip_level_count: Some(SCENE_MIP_LEVELS),
            ..Default::default()
        });
        let world_encoded_view = texture.create_view(&gpu_device::TextureViewDescriptor {
            label: Some("scene_color_world_encoded"),
            format: Some(world_encoded_format),
            dimension: Some(gpu_device::TextureViewDimension::D2),
            base_mip_level: 0,
            mip_level_count: Some(1),
            ..Default::default()
        });
        let mip_views = (0..SCENE_MIP_LEVELS)
            .map(|level| {
                texture.create_view(&gpu_device::TextureViewDescriptor {
                    label: Some(&format!("scene_color_mip_{level}")),
                    format: Some(format),
                    dimension: Some(gpu_device::TextureViewDimension::D2),
                    base_mip_level: level,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let sampler =
            device.create_sampler(&gpu_device::SamplerDescriptor { label: Some("scene_color_sampler"), mag_filter: gpu_device::FilterMode::Linear, min_filter: gpu_device::FilterMode::Linear, mipmap_filter: gpu_device::MipmapFilterMode::Linear, ..Default::default() });
        *target = Some(Self { texture, blur_scratch, blur_scratch_mip_views, world_encoded_view, sample_view, mip_views, sampler, width, height });
    }

    pub fn mip_view(&self, level: u32) -> &gpu_device::TextureView {
        &self.mip_views[level as usize]
    }

    pub fn sample_view(&self) -> &gpu_device::TextureView {
        &self.sample_view
    }

    pub fn world_encoded_view(&self) -> &gpu_device::TextureView {
        &self.world_encoded_view
    }

    pub fn sampler(&self) -> &gpu_device::Sampler {
        &self.sampler
    }

    pub fn blur_scratch_mip_view(&self, level: u32) -> &gpu_device::TextureView {
        &self.blur_scratch_mip_views[level as usize]
    }

    pub fn mip_extent(&self, level: u32) -> gpu_device::Extent3d {
        gpu_device::Extent3d { width: (self.width >> level).max(1), height: (self.height >> level).max(1), depth_or_array_layers: 1 }
    }

    pub fn copy_mip_to_blur_scratch(&self, encoder: &mut gpu_device::CommandEncoder, src_mip: u32) {
        let extent = self.mip_extent(src_mip);
        encoder.copy_texture_to_texture(
            gpu_device::TexelCopyTextureInfo { texture: &self.texture, mip_level: src_mip, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All },
            gpu_device::TexelCopyTextureInfo { texture: &self.blur_scratch, mip_level: src_mip, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All },
            extent,
        );
    }
}

/// 🖼️ The surface-sized colour target a prepared frame composites into BEFORE it ever touches the
/// swapchain — the scene blit lands here, every glass region lands here, and only the terminal
/// present step copies it onto the acquired surface texture.
///
/// 🩸️ A swapchain texture is a SINGLE-TASK resource in a browser: WebGPU expires the canvas's
/// current texture at the end of the task that obtained it, and presents whatever it held at that
/// moment. The prepared present ladder is stepped across many host tasks, so acquiring in one task
/// and compositing in a later one presented the untouched (opaque black) texture and threw the
/// composite away — measured 6/6 on `26/09/17/WGPU-RENDERER-REACT-PARITY`: every black boot had its
/// acquire and its composite in different host pumps, every painted boot had them in the same one.
/// One extra full-surface blit buys an acquire→present span that cannot straddle a task.
pub struct PreparedCompositeTarget {
    pub texture: gpu_device::Texture,
    pub view: gpu_device::TextureView,
    pub world_encoded_view: gpu_device::TextureView,
    pub postprocess_scene_scratch: gpu_device::Texture,
    pub postprocess_scene_scratch_view: gpu_device::TextureView,
    pub postprocess_background_scratch: Option<(gpu_device::Texture, gpu_device::TextureView)>,
    pub world_encoded_format: gpu_device::TextureFormat,
    pub sampler: gpu_device::Sampler,
    pub width: u32,
    pub height: u32,
}

impl PreparedCompositeTarget {
    pub fn ensure(device: &gpu_device::Device, target: &mut Option<Self>, width: u32, height: u32, format: gpu_device::TextureFormat) {
        let width = width.max(1);
        let height = height.max(1);
        if let Some(existing) = target {
            if existing.width == width && existing.height == height {
                return;
            }
        }
        let world_encoded_format = format.remove_srgb_suffix();
        let texture = device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("prepared_composite_color"),
            size: gpu_device::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: world_encoded_format,
            usage: gpu_device::TextureUsages::RENDER_ATTACHMENT | gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_SRC,
            view_formats: &[format, world_encoded_format],
        });
        let view = texture.create_view(&gpu_device::TextureViewDescriptor { label: Some("prepared_composite_color_view"), format: Some(format), dimension: Some(gpu_device::TextureViewDimension::D2), ..Default::default() });
        let world_encoded_view = texture.create_view(&gpu_device::TextureViewDescriptor { label: Some("prepared_composite_world_encoded_view"), format: Some(world_encoded_format), dimension: Some(gpu_device::TextureViewDimension::D2), ..Default::default() });
        let postprocess_scene_scratch = device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("prepared_postprocess_scene_scratch"),
            size: gpu_device::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: world_encoded_format,
            usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST,
            view_formats: &[world_encoded_format],
        });
        let postprocess_scene_scratch_view = postprocess_scene_scratch.create_view(&gpu_device::TextureViewDescriptor { label: Some("prepared_postprocess_scene_scratch_view"), format: Some(world_encoded_format), dimension: Some(gpu_device::TextureViewDimension::D2), ..Default::default() });
        let sampler = device.create_sampler(&gpu_device::SamplerDescriptor { label: Some("prepared_composite_sampler"), mag_filter: gpu_device::FilterMode::Linear, min_filter: gpu_device::FilterMode::Linear, ..Default::default() });
        *target = Some(Self { texture, view, world_encoded_view, postprocess_scene_scratch, postprocess_scene_scratch_view, postprocess_background_scratch: None, world_encoded_format, sampler, width, height });
    }

    pub fn view(&self) -> &gpu_device::TextureView {
        &self.view
    }

    pub fn world_encoded_view(&self) -> &gpu_device::TextureView {
        &self.world_encoded_view
    }

    pub fn copy_to_readback(&self, encoder: &mut gpu_device::CommandEncoder, buffer: &gpu_device::Buffer, bytes_per_row: u32) {
        encoder.copy_texture_to_buffer(
            gpu_device::TexelCopyTextureInfo { texture: &self.texture, mip_level: 0, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All },
            gpu_device::TexelCopyBufferInfo { buffer, layout: gpu_device::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bytes_per_row), rows_per_image: Some(self.height) } },
            gpu_device::Extent3d { width: self.width, height: self.height, depth_or_array_layers: 1 },
        );
    }

    pub fn copy_to_postprocess_scene_scratch(&self, encoder: &mut gpu_device::CommandEncoder) {
        encoder.copy_texture_to_texture(
            gpu_device::TexelCopyTextureInfo { texture: &self.texture, mip_level: 0, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All },
            gpu_device::TexelCopyTextureInfo { texture: &self.postprocess_scene_scratch, mip_level: 0, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All },
            gpu_device::Extent3d { width: self.width, height: self.height, depth_or_array_layers: 1 },
        );
    }

    pub fn ensure_postprocess_background_scratch(&mut self, device: &gpu_device::Device) {
        if self.postprocess_background_scratch.is_some() {
            return;
        }
        let texture = device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("prepared_postprocess_background_scratch"),
            size: gpu_device::Extent3d { width: self.width, height: self.height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: self.world_encoded_format,
            usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST,
            view_formats: &[self.world_encoded_format],
        });
        let view = texture.create_view(&gpu_device::TextureViewDescriptor { label: Some("prepared_postprocess_background_scratch_view"), format: Some(self.world_encoded_format), dimension: Some(gpu_device::TextureViewDimension::D2), ..Default::default() });
        self.postprocess_background_scratch = Some((texture, view));
    }

    pub fn copy_to_postprocess_background_scratch(&mut self, device: &gpu_device::Device, encoder: &mut gpu_device::CommandEncoder) {
        self.ensure_postprocess_background_scratch(device);
        let texture = &self.postprocess_background_scratch.as_ref().expect("background scratch was ensured").0;
        encoder.copy_texture_to_texture(
            gpu_device::TexelCopyTextureInfo { texture: &self.texture, mip_level: 0, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All },
            gpu_device::TexelCopyTextureInfo { texture, mip_level: 0, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All },
            gpu_device::Extent3d { width: self.width, height: self.height, depth_or_array_layers: 1 },
        );
    }

    pub fn postprocess_background_view(&self) -> Option<&gpu_device::TextureView> {
        self.postprocess_background_scratch.as_ref().map(|(_, view)| view)
    }
}

pub struct WorldGlobalsRing {
    pub buffer: gpu_device::Buffer,
    pub bind_group: gpu_device::BindGroup,
    pub slot_stride: u32,
    pub capacity_slots: u32,
}

impl WorldGlobalsRing {
    pub fn new(device: &gpu_device::Device, layout: &gpu_device::BindGroupLayout, initial_slots: u32) -> Self {
        let slot_stride = WORLD_GLOBALS_SLOT_SIZE as u32;
        let capacity_slots = initial_slots.max(1);
        let buffer =
            device.create_buffer(&gpu_device::BufferDescriptor { label: Some("world3d_globals_ring"), size: slot_stride as u64 * capacity_slots as u64, usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST, mapped_at_creation: false });
        let bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("world3d_bind_group"),
            layout,
            entries: &[gpu_device::BindGroupEntry { binding: 0, resource: gpu_device::BindingResource::Buffer(gpu_device::BufferBinding { buffer: &buffer, offset: 0, size: std::num::NonZeroU64::new(size_of::<World3dGlobals>() as u64) }) }],
        });
        Self { buffer, bind_group, slot_stride, capacity_slots }
    }

    pub fn ensure_slots(&mut self, device: &gpu_device::Device, layout: &gpu_device::BindGroupLayout, slots: u32) {
        if slots <= self.capacity_slots {
            return;
        }
        self.capacity_slots = slots.next_power_of_two().max(4);
        self.buffer = device.create_buffer(&gpu_device::BufferDescriptor {
            label: Some("world3d_globals_ring"),
            size: self.slot_stride as u64 * self.capacity_slots as u64,
            usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("world3d_bind_group"),
            layout,
            entries: &[gpu_device::BindGroupEntry { binding: 0, resource: gpu_device::BindingResource::Buffer(gpu_device::BufferBinding { buffer: &self.buffer, offset: 0, size: std::num::NonZeroU64::new(size_of::<World3dGlobals>() as u64) }) }],
        });
    }

    pub fn write_passes(&self, queue: &gpu_device::Queue, passes: &[World3dGlobals]) {
        for (index, globals) in passes.iter().enumerate() {
            let offset = (index as u64) * self.slot_stride as u64;
            queue.write_buffer(&self.buffer, offset, bytemuck::bytes_of(globals));
        }
    }

    pub fn offset_for_slot(&self, slot: u32) -> u32 {
        slot * self.slot_stride
    }
}

pub struct WorldShadowTarget {
    pub _texture: gpu_device::Texture,
    pub view: gpu_device::TextureView,
    pub _sampler: gpu_device::Sampler,
    pub bind_group: gpu_device::BindGroup,
    pub size: u32,
}

impl WorldShadowTarget {
    pub fn new(device: &gpu_device::Device, layout: &gpu_device::BindGroupLayout, size: u32) -> Self {
        let texture = device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("world3d_shadow_map"),
            size: gpu_device::Extent3d { width: size, height: size, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: gpu_device::TextureFormat::Depth32Float,
            usage: gpu_device::TextureUsages::RENDER_ATTACHMENT | gpu_device::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&gpu_device::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&gpu_device::SamplerDescriptor {
            label: Some("world3d_shadow_sampler"),
            address_mode_u: gpu_device::AddressMode::ClampToEdge,
            address_mode_v: gpu_device::AddressMode::ClampToEdge,
            mag_filter: gpu_device::FilterMode::Linear,
            min_filter: gpu_device::FilterMode::Linear,
            compare: Some(gpu_device::CompareFunction::LessEqual),
            ..Default::default()
        });
        let bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("world3d_shadow_bind_group"),
            layout,
            entries: &[gpu_device::BindGroupEntry { binding: 0, resource: gpu_device::BindingResource::TextureView(&view) }, gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::Sampler(&sampler) }],
        });
        Self { _texture: texture, view, _sampler: sampler, bind_group, size }
    }
}

pub struct PipelineResources {
    #[cfg(any(test,feature="testkit"))]
    pub mask_pipeline: gpu_device::RenderPipeline,
    pub ui_pipeline: gpu_device::RenderPipeline,
    pub vector_pipeline: gpu_device::RenderPipeline,
    pub world_pipeline: gpu_device::RenderPipeline,
    pub world_pipeline_translucent: gpu_device::RenderPipeline,
    pub world_section_stencil_pipeline: gpu_device::RenderPipeline,
    pub world_section_cap_pipeline: gpu_device::RenderPipeline,
    pub world_authored_front_pipeline: gpu_device::RenderPipeline,
    pub world_authored_double_translucent_pipeline: gpu_device::RenderPipeline,
    pub world_standard_translucent_pipeline: gpu_device::RenderPipeline,
    pub world_painted_pipeline: gpu_device::RenderPipeline,
    pub world_painted_pipeline_translucent: gpu_device::RenderPipeline,
    pub world_authored_painted_double_pipeline: gpu_device::RenderPipeline,
    pub world_authored_painted_double_translucent_pipeline: gpu_device::RenderPipeline,
    pub world_authored_painted_front_pipeline: gpu_device::RenderPipeline,
    pub world_authored_painted_front_translucent_pipeline: gpu_device::RenderPipeline,
    pub world_celebration_pipeline: gpu_device::RenderPipeline,
    pub world_celebration_pipeline_translucent: gpu_device::RenderPipeline,
    pub world_shadow_pipeline: gpu_device::RenderPipeline,
    pub world_line_pipeline: gpu_device::RenderPipeline,
    pub world_grid_pipeline: gpu_device::RenderPipeline,
    pub world_textured_pipeline: gpu_device::RenderPipeline,
    pub blur_downsample_pipeline: gpu_device::RenderPipeline,
    pub scene_blit_pipeline: gpu_device::RenderPipeline,
    pub glass_pipeline: gpu_device::RenderPipeline,
    pub world_postprocess_pipeline: gpu_device::RenderPipeline,
    pub quad_vertex_buffer: gpu_device::Buffer,
    pub world_plane_vertex_buffer: gpu_device::Buffer,
    pub world_plane_sampler: gpu_device::Sampler,
    pub world_material_samplers: Vec<(crate::kernel_3d_scene::SceneTextureSampler3d, gpu_device::Sampler)>,
    pub world_grid_uniform_buffer: gpu_device::Buffer,
    pub world_grid_bind_group: gpu_device::BindGroup,
    pub globals_buffer: gpu_device::Buffer,
    pub blur_globals_buffer: gpu_device::Buffer,
    pub postprocess_globals_buffer: gpu_device::Buffer,
    pub world_globals_ring: WorldGlobalsRing,
    pub world_bind_group_layout: gpu_device::BindGroupLayout,
    pub world_shadow_bind_group_layout: gpu_device::BindGroupLayout,
    pub world_shadow_target: WorldShadowTarget,
    pub blur_bind_group_layout: gpu_device::BindGroupLayout,
    pub postprocess_bind_group_layout: gpu_device::BindGroupLayout,
    pub scene_bind_group_layout: gpu_device::BindGroupLayout,
    pub world_authored_bind_group_layout: gpu_device::BindGroupLayout,
    pub world_authored_white: gpu_device::TextureView,
    pub glyph_texture: gpu_device::Texture,
    pub glyph_sampler: gpu_device::Sampler,
    pub icon_texture: gpu_device::Texture,
    pub icon_sampler: gpu_device::Sampler,
    pub glyph_bind_group: gpu_device::BindGroup,
    pub bind_group_layout: gpu_device::BindGroupLayout,
    /// 📐️ Device pixels per logical pixel for the surface these pipelines encode into. Draw lists,
    /// scissors and scene viewports are all authored in LOGICAL pixels; this is the only factor that
    /// turns them into the physical rectangles `wgpu` demands. The projection itself needs no
    /// multiplication — `update_globals` is handed the logical extent, so the divide does the work.
    pub surface_scale: f32,
}

pub fn content_stencil_state() -> gpu_device::StencilState {
    let face = gpu_device::StencilFaceState { compare: gpu_device::CompareFunction::Equal, fail_op: gpu_device::StencilOperation::Keep, depth_fail_op: gpu_device::StencilOperation::Keep, pass_op: gpu_device::StencilOperation::Keep };
    gpu_device::StencilState { front: face, back: face, read_mask: 0xff, write_mask: 0x00 }
}

#[cfg(any(test,feature="testkit"))]
pub fn mask_stencil_state() -> gpu_device::StencilState {
    let face = gpu_device::StencilFaceState { compare: gpu_device::CompareFunction::Always, fail_op: gpu_device::StencilOperation::Replace, depth_fail_op: gpu_device::StencilOperation::Replace, pass_op: gpu_device::StencilOperation::Replace };
    gpu_device::StencilState { front: face, back: face, read_mask: 0xff, write_mask: 0xff }
}
