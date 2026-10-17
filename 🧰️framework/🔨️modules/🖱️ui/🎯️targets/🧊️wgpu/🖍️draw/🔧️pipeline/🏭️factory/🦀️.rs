//! 🏭️ Actual pipeline, atlas, target and bind-group construction owns every prepared render resource.
use semio_framework_ui_wgpu_device as gpu_device;
use semio_framework_ui_draw_contract::*;
use bytemuck::{Pod,Zeroable};
use std::mem::size_of;
use gpu_device::util::DeviceExt;
mod kernel_3d_scene{pub use semio_framework_ui_scene_algebra::*;pub use semio_framework_ui_scene_camera::*;pub use semio_framework_ui_scene_mesh::*;pub use semio_framework_ui_scene_pass::*;}
use kernel_3d_scene::{Mat4Math,ProceduralGrid3d,ScenePass3d,SceneRenderProfile3d,PROCEDURAL_GRID_CELL_THICKNESS,PROCEDURAL_GRID_FADE_STRENGTH};
use semio_framework_ui_prepared as prepared;
use semio_framework_ui_raster_owner as raster_ownership;
use semio_framework_ui_theme as theme;
use semio_framework_ui_theme::{geometry,Theme};
#[cfg(test)]
use semio_framework_ui_theme::Rgba;
use semio_framework_ui_wgpu_mesh::*;
use semio_framework_ui_render_prepared_shaders::*;
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
impl PipelineResources {
    pub fn new(device: &gpu_device::Device, queue: &gpu_device::Queue, format: gpu_device::TextureFormat) -> Self {
        let world_encoded_format = format.remove_srgb_suffix();
        let globals_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("ui_globals_layout"),
            entries: &[
                gpu_device::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: gpu_device::ShaderStages::VERTEX | gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Buffer { ty: gpu_device::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry { binding: 2, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering), count: None },
                gpu_device::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry { binding: 4, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering), count: None },
            ],
        });

        let glyph_bind_group_layout = globals_bind_group_layout.clone();
        let _ = glyph_bind_group_layout;

        let ui_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("ui_shader"), source: gpu_device::ShaderSource::Wgsl(UI_SHADER.into()) });
        let vector_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("vector_shader"), source: gpu_device::ShaderSource::Wgsl(VECTOR_SHADER.into()) });
        let world_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_SHADER.into()) });
        let world_lines_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_lines_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_LINES_SHADER.into()) });

        let depth_state = Some(gpu_device::DepthStencilState {
            format: gpu_device::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: Some(true),
            depth_compare: Some(gpu_device::CompareFunction::Less),
            stencil: content_stencil_state(),
            bias: gpu_device::DepthBiasState::default(),
        });
        let overlay_depth_state = Some(gpu_device::DepthStencilState {
            format: gpu_device::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: Some(false),
            depth_compare: Some(gpu_device::CompareFunction::Always),
            stencil: content_stencil_state(),
            bias: gpu_device::DepthBiasState::default(),
        });

        let quad_vertices: &[f32] = &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        let quad_vertex_buffer = device.create_buffer_init(&gpu_device::util::BufferInitDescriptor { label: Some("ui_quad_vertices"), contents: bytemuck::cast_slice(quad_vertices), usage: gpu_device::BufferUsages::VERTEX });

        let globals_buffer = device.create_buffer_init(&gpu_device::util::BufferInitDescriptor {
            label: Some("ui_globals"),
            contents: bytemuck::bytes_of(&UiGlobals { screen_size: [1.0, 1.0], _pad: [0.0, 0.0] }),
            usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST,
        });

        let glyph_texture = device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("glyph_atlas"),
            size: gpu_device::Extent3d { width: ICON_ATLAS_TEXTURE_SIZE, height: ICON_ATLAS_TEXTURE_SIZE, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: gpu_device::TextureFormat::R8Unorm,
            usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let glyph_view = glyph_texture.create_view(&gpu_device::TextureViewDescriptor::default());
        let glyph_sampler = device.create_sampler(&gpu_device::SamplerDescriptor { label: Some("glyph_sampler"), mag_filter: gpu_device::FilterMode::Linear, min_filter: gpu_device::FilterMode::Linear, ..Default::default() });
        let icon_texture = device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("icon_atlas"),
            size: gpu_device::Extent3d { width: ICON_ATLAS_TEXTURE_SIZE, height: ICON_ATLAS_TEXTURE_SIZE, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: gpu_device::TextureFormat::Rgba8UnormSrgb,
            usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let icon_view = icon_texture.create_view(&gpu_device::TextureViewDescriptor::default());
        let icon_sampler = device.create_sampler(&gpu_device::SamplerDescriptor { label: Some("icon_sampler"), mag_filter: gpu_device::FilterMode::Linear, min_filter: gpu_device::FilterMode::Linear, ..Default::default() });
        let glyph_bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("ui_bind_group"),
            layout: &globals_bind_group_layout,
            entries: &[
                gpu_device::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::TextureView(&glyph_view) },
                gpu_device::BindGroupEntry { binding: 2, resource: gpu_device::BindingResource::Sampler(&glyph_sampler) },
                gpu_device::BindGroupEntry { binding: 3, resource: gpu_device::BindingResource::TextureView(&icon_view) },
                gpu_device::BindGroupEntry { binding: 4, resource: gpu_device::BindingResource::Sampler(&icon_sampler) },
            ],
        });
        let ui_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("ui_pipeline_layout"), bind_group_layouts: &[Some(&globals_bind_group_layout)], immediate_size: 0 });
        #[cfg(any(test,feature="testkit"))]
        let mask_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("silhouette_mask_pipeline"),
            layout: Some(&ui_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &ui_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    gpu_device::VertexBufferLayout { array_stride: 8, step_mode: gpu_device::VertexStepMode::Vertex, attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x2 }] },
                    gpu_device::VertexBufferLayout {
                        array_stride: size_of::<UiInstance>() as gpu_device::BufferAddress,
                        step_mode: gpu_device::VertexStepMode::Instance,
                        attributes: &[
                            gpu_device::VertexAttribute { offset: 0, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 16, shader_location: 2, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 32, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 48, shader_location: 4, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 64, shader_location: 5, format: gpu_device::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState { module: &ui_shader, entry_point: Some("fs_main"), targets: &[Some(gpu_device::ColorTargetState { format, blend: None, write_mask: gpu_device::ColorWrites::empty() })], compilation_options: Default::default() }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: Some(false),
                depth_compare: Some(gpu_device::CompareFunction::Always),
                stencil: mask_stencil_state(),
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let ui_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("ui_pipeline"),
            layout: Some(&ui_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &ui_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    gpu_device::VertexBufferLayout { array_stride: 8, step_mode: gpu_device::VertexStepMode::Vertex, attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x2 }] },
                    gpu_device::VertexBufferLayout {
                        array_stride: size_of::<UiInstance>() as gpu_device::BufferAddress,
                        step_mode: gpu_device::VertexStepMode::Instance,
                        attributes: &[
                            gpu_device::VertexAttribute { offset: 0, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 16, shader_location: 2, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 32, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 48, shader_location: 4, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 64, shader_location: 5, format: gpu_device::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &ui_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: overlay_depth_state.clone(),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let vector_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("vector_pipeline_layout"), bind_group_layouts: &[Some(&globals_bind_group_layout)], immediate_size: 0 });
        let vector_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("vector_pipeline"),
            layout: Some(&vector_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &vector_shader,
                entry_point: Some("vs_main"),
                buffers: &[gpu_device::VertexBufferLayout {
                    array_stride: size_of::<VectorVertex>() as gpu_device::BufferAddress,
                    step_mode: gpu_device::VertexStepMode::Vertex,
                    attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x2 }, gpu_device::VertexAttribute { offset: 8, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 }],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &vector_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: overlay_depth_state,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let world_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("world3d_globals_layout"),
            entries: &[gpu_device::BindGroupLayoutEntry {
                binding: 0,
                visibility: gpu_device::ShaderStages::VERTEX | gpu_device::ShaderStages::FRAGMENT,
                ty: gpu_device::BindingType::Buffer { ty: gpu_device::BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: std::num::NonZeroU64::new(size_of::<World3dGlobals>() as u64) },
                count: None,
            }],
        });

        let world_globals_ring = WorldGlobalsRing::new(device, &world_bind_group_layout, 8);

        let world_shadow_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("world3d_shadow_layout"),
            entries: &[
                gpu_device::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Depth, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry { binding: 1, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Comparison), count: None },
            ],
        });
        let world_shadow_target = WorldShadowTarget::new(device, &world_shadow_bind_group_layout, 1);

        let world_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout)], immediate_size: 0 });
        let world_mesh_pipeline_layout =
            device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_mesh_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout), Some(&world_shadow_bind_group_layout)], immediate_size: 0 });
        let world_vertex_layout = || gpu_device::VertexBufferLayout {
            array_stride: size_of::<World3dVertex>() as gpu_device::BufferAddress,
            step_mode: gpu_device::VertexStepMode::Vertex,
            attributes: &[
                gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 },
                gpu_device::VertexAttribute { offset: 12, shader_location: 1, format: gpu_device::VertexFormat::Float32x3 },
                gpu_device::VertexAttribute { offset: 24, shader_location: 2, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 40, shader_location: 9, format: gpu_device::VertexFormat::Float32x2 },
            ],
        };
        let world_instance_layout = || gpu_device::VertexBufferLayout {
            array_stride: size_of::<World3dGpuInstance>() as gpu_device::BufferAddress,
            step_mode: gpu_device::VertexStepMode::Instance,
            attributes: &[
                gpu_device::VertexAttribute { offset: 0, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 16, shader_location: 4, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 32, shader_location: 5, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 48, shader_location: 6, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 64, shader_location: 7, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 80, shader_location: 8, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 96, shader_location: 10, format: gpu_device::VertexFormat::Float32x4 },
            ],
        };
        let world_surface_vertex_layout=||gpu_device::VertexBufferLayout {array_stride:size_of::<World3dVertex>() as gpu_device::BufferAddress,step_mode:gpu_device::VertexStepMode::Vertex,attributes:&[
            gpu_device::VertexAttribute{offset:0,shader_location:0,format:gpu_device::VertexFormat::Float32x3},gpu_device::VertexAttribute{offset:12,shader_location:1,format:gpu_device::VertexFormat::Float32x3},gpu_device::VertexAttribute{offset:24,shader_location:2,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:40,shader_location:9,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:56,shader_location:11,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:72,shader_location:12,format:gpu_device::VertexFormat::Float32x2},gpu_device::VertexAttribute{offset:80,shader_location:13,format:gpu_device::VertexFormat::Float32x4},]};
        let world_surface_instance_layout=||gpu_device::VertexBufferLayout{array_stride:size_of::<World3dGpuInstance>() as gpu_device::BufferAddress,step_mode:gpu_device::VertexStepMode::Instance,attributes:&[
            gpu_device::VertexAttribute{offset:0,shader_location:3,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:16,shader_location:4,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:32,shader_location:5,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:48,shader_location:6,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:64,shader_location:7,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:80,shader_location:8,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:96,shader_location:10,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:112,shader_location:15,format:gpu_device::VertexFormat::Float32x4},]};
        let world_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: depth_state,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let translucent_depth_state = Some(gpu_device::DepthStencilState {
            format: gpu_device::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: Some(false),
            depth_compare: Some(gpu_device::CompareFunction::LessEqual),
            stencil: content_stencil_state(),
            bias: gpu_device::DepthBiasState { constant: -2, slope_scale: -1.0, clamp: 0.0 },
        });
        let world_line_depth_state = Some(gpu_device::DepthStencilState {
            format: gpu_device::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: Some(false),
            depth_compare: Some(gpu_device::CompareFunction::LessEqual),
            stencil: content_stencil_state(),
            bias: gpu_device::DepthBiasState::default(),
        });
        let world_pipeline_translucent = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_pipeline_translucent"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: translucent_depth_state,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let section_toggle_face = gpu_device::StencilFaceState { compare: gpu_device::CompareFunction::Equal, fail_op: gpu_device::StencilOperation::Keep, depth_fail_op: gpu_device::StencilOperation::Invert, pass_op: gpu_device::StencilOperation::Invert };
        let world_section_stencil_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_section_stencil_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_section_stencil"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: None, write_mask: gpu_device::ColorWrites::empty() })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: Some(false),
                depth_compare: Some(gpu_device::CompareFunction::Always),
                stencil: gpu_device::StencilState { front: section_toggle_face, back: section_toggle_face, read_mask: !SECTION_STENCIL_BIT & 0xff, write_mask: SECTION_STENCIL_BIT },
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let section_cap_face = gpu_device::StencilFaceState { compare: gpu_device::CompareFunction::Equal, fail_op: gpu_device::StencilOperation::Keep, depth_fail_op: gpu_device::StencilOperation::Keep, pass_op: gpu_device::StencilOperation::Keep };
        let world_section_cap_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_section_cap_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_section_cap"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: Some(true),
                depth_compare: Some(gpu_device::CompareFunction::LessEqual),
                stencil: gpu_device::StencilState { front: section_cap_face, back: section_cap_face, read_mask: 0xff, write_mask: 0x00 },
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_shadow_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_shadow_pipeline"),
            layout: Some(&world_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_shadow"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: None,
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(gpu_device::CompareFunction::LessEqual),
                stencil: gpu_device::StencilState::default(),
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_line_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_line_pipeline"),
            layout: Some(&world_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &world_lines_shader,
                entry_point: Some("vs_main"),
                buffers: &[gpu_device::VertexBufferLayout {
                    array_stride: size_of::<WorldLineGpuVertex>() as gpu_device::BufferAddress,
                    step_mode: gpu_device::VertexStepMode::Vertex,
                    attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 }, gpu_device::VertexAttribute { offset: 12, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 }],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &world_lines_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { topology: gpu_device::PrimitiveTopology::LineList, ..Default::default() },
            depth_stencil: world_line_depth_state,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let blur_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("blur_downsample_shader"), source: gpu_device::ShaderSource::Wgsl(BLUR_DOWNSAMPLE_SHADER.into()) });
        let scene_blit_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("scene_blit_shader"), source: gpu_device::ShaderSource::Wgsl(SCENE_BLIT_SHADER.into()) });
        let glass_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("glass_shader"), source: gpu_device::ShaderSource::Wgsl(GLASS_SHADER.into()) });
        let world_postprocess_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world_postprocess_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_POSTPROCESS_SHADER.into()) });

        let blur_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("blur_downsample_layout"),
            entries: &[
                gpu_device::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Buffer { ty: gpu_device::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(size_of::<BlurGlobals>() as u64) },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry { binding: 2, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering), count: None },
            ],
        });

        let postprocess_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("world_postprocess_layout"),
            entries: &[
                gpu_device::BindGroupLayoutEntry { binding: 0, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Buffer { ty: gpu_device::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(size_of::<WorldPostprocessGlobals>() as u64) }, count: None },
                gpu_device::BindGroupLayoutEntry { binding: 1, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false }, count: None },
                gpu_device::BindGroupLayoutEntry { binding: 2, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering), count: None },
                gpu_device::BindGroupLayoutEntry { binding: 3, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false }, count: None },
            ],
        });

        let scene_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("scene_sample_layout"),
            entries: &[
                gpu_device::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry { binding: 1, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering), count: None },
            ],
        });

        let authored_entries: Vec<_> = (0..10).map(|binding| gpu_device::BindGroupLayoutEntry { binding, visibility: gpu_device::ShaderStages::FRAGMENT, ty: if binding % 2 == 0 { gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false } } else { gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering) }, count: None }).collect();
        let world_authored_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor { label: Some("world3d_authored_surface_layout"), entries: &authored_entries });
        let authored_white = device.create_texture(&gpu_device::TextureDescriptor { label: Some("world3d_authored_white"), size: gpu_device::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: gpu_device::TextureDimension::D2, format: gpu_device::TextureFormat::Rgba8Unorm, usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST, view_formats: &[] });
        queue.write_texture(gpu_device::TexelCopyTextureInfo { texture: &authored_white, mip_level: 0, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All }, &[255;4], gpu_device::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4), rows_per_image: Some(1) }, gpu_device::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 });
        let world_authored_white = authored_white.create_view(&Default::default());
        let world_authored_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_authored_shader"), source: gpu_device::ShaderSource::Wgsl(world3d_authored_shader().into()) });
        let world_authored_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_authored_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout), Some(&world_shadow_bind_group_layout), Some(&world_authored_bind_group_layout)], immediate_size: 0 });
        let world_painted_source = world3d_painted_shader();
        let world_painted_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_painted_shader"), source: gpu_device::ShaderSource::Wgsl(world_painted_source.into()) });
        let world_celebration_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_celebration_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_CELEBRATION_SHADER.into()) });
        let world_painted_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor {
            label: Some("world3d_painted_pipeline_layout"),
            bind_group_layouts: &[Some(&world_bind_group_layout), Some(&world_shadow_bind_group_layout), Some(&scene_bind_group_layout)],
            immediate_size: 0,
        });
        let world_celebration_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_celebration_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout)], immediate_size: 0 });
        let celebration_instance_layout = || gpu_device::VertexBufferLayout {
            array_stride: size_of::<World3dCelebrationGpuInstance>() as gpu_device::BufferAddress,
            step_mode: gpu_device::VertexStepMode::Instance,
            attributes: &[
                gpu_device::VertexAttribute { offset: 0, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 16, shader_location: 4, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 32, shader_location: 5, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 48, shader_location: 6, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 64, shader_location: 7, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 80, shader_location: 8, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 96, shader_location: 10, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 112, shader_location: 11, format: gpu_device::VertexFormat::Float32x4 },
            ],
        };
        let material_depth = |write| gpu_device::DepthStencilState {
            format: gpu_device::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: Some(write),
            depth_compare: Some(gpu_device::CompareFunction::LessEqual),
            stencil: content_stencil_state(),
            bias: if write { gpu_device::DepthBiasState::default() } else { gpu_device::DepthBiasState { constant: -2, slope_scale: -1.0, clamp: 0.0 } },
        };
        let world_standard_translucent_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_standard_translucent_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_front_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_front_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_double_translucent_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_double_translucent_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(false)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_painted_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_painted_pipeline"),
            layout: Some(&world_painted_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_painted_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_painted_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_painted_double_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_painted_double_pipeline"),
            layout: Some(&world_authored_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_authored_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_painted_double_translucent_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_painted_double_translucent_pipeline"),
            layout: Some(&world_authored_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_authored_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(false)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_painted_front_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_painted_front_pipeline"),
            layout: Some(&world_authored_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_authored_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_painted_front_translucent_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_painted_front_translucent_pipeline"),
            layout: Some(&world_authored_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_authored_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: Some(material_depth(false)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_painted_pipeline_translucent = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_painted_pipeline_translucent"),
            layout: Some(&world_painted_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_painted_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_painted_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(false)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_celebration_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_celebration_pipeline"),
            layout: Some(&world_celebration_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_celebration_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), celebration_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_celebration_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_celebration_pipeline_translucent = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_celebration_pipeline_translucent"),
            layout: Some(&world_celebration_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_celebration_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), celebration_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_celebration_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(false)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });

        let world_textured_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_textured_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_TEXTURED_SHADER.into()) });
        let world_grid_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_grid_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_GRID_SHADER.into()) });
        let world_plane_vertex_buffer = device.create_buffer_init(&gpu_device::util::BufferInitDescriptor { label: Some("world3d_plane_vertices"), contents: bytemuck::cast_slice(WORLD_PLANE_VERTICES), usage: gpu_device::BufferUsages::VERTEX });
        let world_plane_sampler = device.create_sampler(&gpu_device::SamplerDescriptor {
            label: Some("world3d_plane_sampler"),
            address_mode_u: gpu_device::AddressMode::ClampToEdge,
            address_mode_v: gpu_device::AddressMode::ClampToEdge,
            address_mode_w: gpu_device::AddressMode::ClampToEdge,
            mag_filter: gpu_device::FilterMode::Linear,
            min_filter: gpu_device::FilterMode::Linear,
            mipmap_filter: gpu_device::MipmapFilterMode::Nearest,
            ..Default::default()
        });
        let mut world_material_samplers = Vec::with_capacity(108);
        for wrap_u in [
            crate::kernel_3d_scene::SceneTextureWrap3d::ClampToEdge,
            crate::kernel_3d_scene::SceneTextureWrap3d::Repeat,
            crate::kernel_3d_scene::SceneTextureWrap3d::MirrorRepeat,
        ] {
            for wrap_v in [
                crate::kernel_3d_scene::SceneTextureWrap3d::ClampToEdge,
                crate::kernel_3d_scene::SceneTextureWrap3d::Repeat,
                crate::kernel_3d_scene::SceneTextureWrap3d::MirrorRepeat,
            ] {
                for mag_filter in [crate::kernel_3d_scene::SceneTextureFilter3d::Nearest, crate::kernel_3d_scene::SceneTextureFilter3d::Linear] {
                    for min_filter in [crate::kernel_3d_scene::SceneTextureFilter3d::Nearest, crate::kernel_3d_scene::SceneTextureFilter3d::Linear,crate::kernel_3d_scene::SceneTextureFilter3d::NearestMipmapNearest,crate::kernel_3d_scene::SceneTextureFilter3d::LinearMipmapNearest,crate::kernel_3d_scene::SceneTextureFilter3d::NearestMipmapLinear,crate::kernel_3d_scene::SceneTextureFilter3d::LinearMipmapLinear] {
                        let descriptor = crate::kernel_3d_scene::SceneTextureSampler3d { wrap_u, wrap_v, mag_filter, min_filter };
                        let address = |mode| match mode {
                            crate::kernel_3d_scene::SceneTextureWrap3d::ClampToEdge => gpu_device::AddressMode::ClampToEdge,
                            crate::kernel_3d_scene::SceneTextureWrap3d::Repeat => gpu_device::AddressMode::Repeat,
                            crate::kernel_3d_scene::SceneTextureWrap3d::MirrorRepeat => gpu_device::AddressMode::MirrorRepeat,
                        };
                        let filter = |mode| match mode {
                            crate::kernel_3d_scene::SceneTextureFilter3d::Nearest | crate::kernel_3d_scene::SceneTextureFilter3d::NearestMipmapNearest | crate::kernel_3d_scene::SceneTextureFilter3d::NearestMipmapLinear => gpu_device::FilterMode::Nearest,
                            _ => gpu_device::FilterMode::Linear,
                        };
                        let sampler = device.create_sampler(&gpu_device::SamplerDescriptor {
                            label: Some("world3d_material_sampler"),
                            address_mode_u: address(wrap_u),
                            address_mode_v: address(wrap_v),
                            address_mode_w: gpu_device::AddressMode::Repeat,
                            mag_filter: filter(mag_filter),
                            min_filter: filter(min_filter),
                            mipmap_filter:if matches!(min_filter,crate::kernel_3d_scene::SceneTextureFilter3d::NearestMipmapLinear|crate::kernel_3d_scene::SceneTextureFilter3d::LinearMipmapLinear){gpu_device::MipmapFilterMode::Linear}else{gpu_device::MipmapFilterMode::Nearest},
                            lod_max_clamp:if matches!(min_filter,crate::kernel_3d_scene::SceneTextureFilter3d::Nearest|crate::kernel_3d_scene::SceneTextureFilter3d::Linear){0.0}else{32.0},
                            ..Default::default()
                        });
                        world_material_samplers.push((descriptor, sampler));
                    }
                }
            }
        }
        let world_textured_pipeline_layout =
            device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_textured_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout), Some(&scene_bind_group_layout)], immediate_size: 0 });
        let world_textured_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_textured_pipeline"),
            layout: Some(&world_textured_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &world_textured_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    gpu_device::VertexBufferLayout {
                        array_stride: 20,
                        step_mode: gpu_device::VertexStepMode::Vertex,
                        attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 }, gpu_device::VertexAttribute { offset: 12, shader_location: 1, format: gpu_device::VertexFormat::Float32x2 }],
                    },
                    gpu_device::VertexBufferLayout {
                        array_stride: size_of::<World3dTexturedGpuInstance>() as gpu_device::BufferAddress,
                        step_mode: gpu_device::VertexStepMode::Instance,
                        attributes: &[
                            gpu_device::VertexAttribute { offset: 0, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 16, shader_location: 4, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 32, shader_location: 5, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 48, shader_location: 6, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 64, shader_location: 7, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 80, shader_location: 8, format: gpu_device::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &world_textured_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: Some(false),
                depth_compare: Some(gpu_device::CompareFunction::LessEqual),
                stencil: content_stencil_state(),
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_grid_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("world3d_grid_layout"),
            entries: &[gpu_device::BindGroupLayoutEntry {
                binding: 0,
                visibility: gpu_device::ShaderStages::VERTEX | gpu_device::ShaderStages::FRAGMENT,
                ty: gpu_device::BindingType::Buffer { ty: gpu_device::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(size_of::<World3dGridUniforms>() as u64) },
                count: None,
            }],
        });
        let world_grid_uniform_buffer =
            device.create_buffer_init(&gpu_device::util::BufferInitDescriptor { label: Some("world3d_grid_uniforms"), contents: bytemuck::bytes_of(&World3dGridUniforms::zeroed()), usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST });
        let world_grid_bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("world3d_grid_bind_group"),
            layout: &world_grid_bind_group_layout,
            entries: &[gpu_device::BindGroupEntry { binding: 0, resource: world_grid_uniform_buffer.as_entire_binding() }],
        });
        let world_grid_pipeline_layout =
            device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_grid_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout), Some(&world_grid_bind_group_layout)], immediate_size: 0 });
        let world_grid_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_grid_pipeline"),
            layout: Some(&world_grid_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &world_grid_shader,
                entry_point: Some("vs_main"),
                buffers: &[gpu_device::VertexBufferLayout { array_stride: 20, step_mode: gpu_device::VertexStepMode::Vertex, attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 }] }],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &world_grid_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: Some(false),
                depth_compare: Some(gpu_device::CompareFunction::LessEqual),
                stencil: content_stencil_state(),
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let blur_globals_buffer =
            device.create_buffer_init(&gpu_device::util::BufferInitDescriptor { label: Some("blur_globals"), contents: bytemuck::bytes_of(&BlurGlobals { src_mip: 0.0, _pad: [0.0; 7] }), usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST });
        let postprocess_globals_buffer = device.create_buffer_init(&gpu_device::util::BufferInitDescriptor {
            label: Some("world_postprocess_globals"),
            contents: bytemuck::bytes_of(&WorldPostprocessGlobals::zeroed()),
            usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST,
        });

        let world_postprocess_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world_postprocess_pipeline_layout"), bind_group_layouts: &[Some(&postprocess_bind_group_layout)], immediate_size: 0 });
        let world_postprocess_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world_postprocess_pipeline"),
            layout: Some(&world_postprocess_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_postprocess_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState { module: &world_postprocess_shader, entry_point: Some("fs_main"), targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: None,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let blur_downsample_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("blur_downsample_pipeline_layout"), bind_group_layouts: &[Some(&blur_bind_group_layout)], immediate_size: 0 });
        let blur_downsample_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("blur_downsample_pipeline"),
            layout: Some(&blur_downsample_pipeline_layout),
            vertex: gpu_device::VertexState { module: &blur_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &blur_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: None,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let scene_blit_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("scene_blit_pipeline_layout"), bind_group_layouts: &[Some(&scene_bind_group_layout)], immediate_size: 0 });
        let scene_blit_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("scene_blit_pipeline"),
            layout: Some(&scene_blit_pipeline_layout),
            vertex: gpu_device::VertexState { module: &scene_blit_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &scene_blit_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: None,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let glass_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("glass_pipeline_layout"), bind_group_layouts: &[Some(&globals_bind_group_layout), Some(&scene_bind_group_layout)], immediate_size: 0 });
        let glass_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("glass_pipeline"),
            layout: Some(&glass_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &glass_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    gpu_device::VertexBufferLayout { array_stride: 8, step_mode: gpu_device::VertexStepMode::Vertex, attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x2 }] },
                    gpu_device::VertexBufferLayout {
                        array_stride: size_of::<GlassInstance>() as gpu_device::BufferAddress,
                        step_mode: gpu_device::VertexStepMode::Instance,
                        attributes: &[
                            gpu_device::VertexAttribute { offset: 0, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 16, shader_location: 2, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 32, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &glass_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: None,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let _ = queue;
        Self {
            #[cfg(any(test,feature="testkit"))]
            mask_pipeline,
            ui_pipeline,
            vector_pipeline,
            world_pipeline,
            world_pipeline_translucent,
            world_section_stencil_pipeline,
            world_section_cap_pipeline,
            world_authored_front_pipeline,
            world_authored_double_translucent_pipeline,
            world_standard_translucent_pipeline,
            world_painted_pipeline,
            world_painted_pipeline_translucent,
            world_authored_painted_double_pipeline,
            world_authored_painted_double_translucent_pipeline,
            world_authored_painted_front_pipeline,
            world_authored_painted_front_translucent_pipeline,
            world_celebration_pipeline,
            world_celebration_pipeline_translucent,
            world_shadow_pipeline,
            world_line_pipeline,
            world_grid_pipeline,
            world_textured_pipeline,
            blur_downsample_pipeline,
            scene_blit_pipeline,
            glass_pipeline,
            world_postprocess_pipeline,
            quad_vertex_buffer,
            world_plane_vertex_buffer,
            world_plane_sampler,
            world_material_samplers,
            world_grid_uniform_buffer,
            world_grid_bind_group,
            globals_buffer,
            blur_globals_buffer,
            postprocess_globals_buffer,
            world_globals_ring,
            world_bind_group_layout,
            world_shadow_bind_group_layout,
            world_shadow_target,
            blur_bind_group_layout,
            postprocess_bind_group_layout,
            scene_bind_group_layout,
            world_authored_bind_group_layout,
            world_authored_white,
            glyph_texture,
            glyph_sampler,
            icon_texture,
            icon_sampler,
            glyph_bind_group,
            bind_group_layout: globals_bind_group_layout,
            surface_scale: 1.0,
        }
    }

}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
