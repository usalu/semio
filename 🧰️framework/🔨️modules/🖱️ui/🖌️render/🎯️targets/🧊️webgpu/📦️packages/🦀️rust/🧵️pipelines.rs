//! @emoji 🧵️ Builds every `wgpu::RenderPipeline` this backend needs from `ui_render`'s canonical
//! [`ui_render::PipelineSpec`] + WGSL (`ui_render::ALL_SHADERS`), so the shader source is shared with
//! the other three hand-written backends instead of duplicated here (ticket brief, `backend-webgpu`).
//! The handful of `wgpu::BindGroupLayout`s are still hand-built (not derived from `BindGroupSpec`
//! generically): `BindGroupSpec` carries no `PartialEq`/`Hash`, and — more importantly — a `wgpu::
//! BindGroup` must be created against the *exact* `BindGroupLayout` object its pipeline's `Pipeline
//! Layout` was built from for `set_bind_group` to validate, so the five distinct layouts this family
//! actually uses (`ui_globals`, `world_globals`, `blur`, `scene_sample`, and glass's own group 1 which
//! reuses `scene_sample`) are built once and shared, exactly mirroring `draw.rs`'s `UiPipelines::new`.

use crate::gpu_types::{binding_type, blend_state, color_writes, cull_mode, depth_stencil_state, primitive_topology, step_mode, vertex_format};
use crate::gpu_uniforms::{BlurGlobals, UiGlobals};
use ui_render::{PipelineSpec, BLUR_FAMILY, GLASS_FAMILY, UI_FAMILY, VECTOR_FAMILY, WORLD3D_FAMILY};
use wgpu::util::DeviceExt;

//#region 🔖️Pipelines

//#region 🧱️Build

/// 🏗️ Builds one `wgpu::RenderPipeline` from `spec`, `module` (vertex+fragment share one WGSL module
/// per `ShaderVariant`) and an already-built `layout`. Every field on `spec` translates through
/// `crate::gpu_types` — nothing here reads WGSL or guesses a layout.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn build_pipeline(device: &wgpu::Device, module: &wgpu::ShaderModule, spec: &PipelineSpec, layout: &wgpu::PipelineLayout, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
    let buffers: Vec<Vec<wgpu::VertexAttribute>> =
        spec.vertex_buffers.iter().map(|buffer| buffer.attributes.iter().map(|attribute| wgpu::VertexAttribute { offset: attribute.offset, shader_location: attribute.shader_location, format: vertex_format(attribute.format) }).collect()).collect();
    let vertex_buffers: Vec<wgpu::VertexBufferLayout> = spec.vertex_buffers.iter().zip(buffers.iter()).map(|(buffer, attributes)| wgpu::VertexBufferLayout { array_stride: buffer.stride, step_mode: step_mode(buffer.step_mode), attributes }).collect();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(spec.label),
        layout: Some(layout),
        vertex: wgpu::VertexState { module, entry_point: Some(spec.vertex_entry), buffers: &vertex_buffers, compilation_options: Default::default() },
        fragment: Some(wgpu::FragmentState {
            module,
            entry_point: Some(spec.fragment_entry),
            targets: &[Some(wgpu::ColorTargetState { format: surface_format, blend: blend_state(spec.blend), write_mask: color_writes(spec.color_write) })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState { topology: primitive_topology(spec.topology), cull_mode: cull_mode(spec.cull_mode), ..Default::default() },
        depth_stencil: spec.depth_stencil.as_ref().map(depth_stencil_state),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn shader_module(device: &wgpu::Device, label: &str, wgsl: &str) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some(label), source: wgpu::ShaderSource::Wgsl(wgsl.into()) })
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn bind_group_layout(device: &wgpu::Device, label: &str, entries: &[ui_render::BindGroupEntrySpec]) -> wgpu::BindGroupLayout {
    let entries: Vec<wgpu::BindGroupLayoutEntry> = entries
        .iter()
        .map(|entry| {
            let visibility = match (entry.visibility.vertex, entry.visibility.fragment) {
                (true, true) => wgpu::ShaderStages::VERTEX_FRAGMENT,
                (false, true) => wgpu::ShaderStages::FRAGMENT,
                (true, false) => wgpu::ShaderStages::VERTEX,
                (false, false) => wgpu::ShaderStages::NONE,
            };
            wgpu::BindGroupLayoutEntry { binding: entry.binding, visibility, ty: binding_type(entry.kind), count: None }
        })
        .collect();
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some(label), entries: &entries })
}

//#endregion 🧱️Build

//#region 📦️Pipelines

/// 📦️ Every pipeline, bind group layout, shared vertex buffer and globals uniform this backend needs
/// for one surface format — one instance lives on [`crate::backend::WebGpuBackend`] and is rebuilt
/// whenever [`ui_render::GraphicsBackend::recover`] follows a device loss.
pub(crate) struct Pipelines {
    pub mask_pipeline: wgpu::RenderPipeline,
    pub content_pipeline: wgpu::RenderPipeline,
    pub vector_pipeline: wgpu::RenderPipeline,
    pub world_opaque_pipeline: wgpu::RenderPipeline,
    pub world_translucent_pipeline: wgpu::RenderPipeline,
    pub world_line_pipeline: wgpu::RenderPipeline,
    pub blur_downsample_pipeline: wgpu::RenderPipeline,
    pub scene_blit_pipeline: wgpu::RenderPipeline,
    pub glass_pipeline: wgpu::RenderPipeline,

    pub ui_globals_layout: wgpu::BindGroupLayout,
    pub world_globals_layout: wgpu::BindGroupLayout,
    pub blur_layout: wgpu::BindGroupLayout,
    pub scene_sample_layout: wgpu::BindGroupLayout,

    pub quad_vertex_buffer: wgpu::Buffer,
    pub globals_buffer: wgpu::Buffer,
    pub blur_globals_buffer: wgpu::Buffer,
}

impl Pipelines {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        let ui_globals_layout = bind_group_layout(device, "ui_globals_layout", UI_FAMILY.variants[0].pipelines[0].bind_groups[0].entries);
        let world_globals_layout = bind_group_layout(device, "world3d_globals_layout", WORLD3D_FAMILY.variants[0].pipelines[0].bind_groups[0].entries);
        let blur_layout = bind_group_layout(device, "blur_downsample_layout", BLUR_FAMILY.variants[0].pipelines[0].bind_groups[0].entries);
        let scene_sample_layout = bind_group_layout(device, "scene_sample_layout", BLUR_FAMILY.variants[1].pipelines[0].bind_groups[0].entries);

        let ui_module = shader_module(device, "ui_shader", UI_FAMILY.variants[0].wgsl);
        let vector_module = shader_module(device, "vector_shader", VECTOR_FAMILY.variants[0].wgsl);
        let world_module = shader_module(device, "world3d_shader", WORLD3D_FAMILY.variants[0].wgsl);
        let world_lines_module = shader_module(device, "world3d_lines_shader", WORLD3D_FAMILY.variants[1].wgsl);
        let blur_module = shader_module(device, "blur_downsample_shader", BLUR_FAMILY.variants[0].wgsl);
        let scene_blit_module = shader_module(device, "scene_blit_shader", BLUR_FAMILY.variants[1].wgsl);
        let glass_module = shader_module(device, "glass_shader", GLASS_FAMILY.variants[0].wgsl);

        let ui_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("ui_pipeline_layout"), bind_group_layouts: &[&ui_globals_layout], push_constant_ranges: &[] });
        let mask_pipeline = build_pipeline(device, &ui_module, &UI_FAMILY.variants[0].pipelines[0], &ui_layout, surface_format);
        let content_pipeline = build_pipeline(device, &ui_module, &UI_FAMILY.variants[0].pipelines[1], &ui_layout, surface_format);
        let vector_pipeline = build_pipeline(device, &vector_module, &VECTOR_FAMILY.variants[0].pipelines[0], &ui_layout, surface_format);

        let world_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("world3d_pipeline_layout"), bind_group_layouts: &[&world_globals_layout], push_constant_ranges: &[] });
        let world_opaque_pipeline = build_pipeline(device, &world_module, &WORLD3D_FAMILY.variants[0].pipelines[0], &world_layout, surface_format);
        let world_translucent_pipeline = build_pipeline(device, &world_module, &WORLD3D_FAMILY.variants[0].pipelines[1], &world_layout, surface_format);
        let world_line_pipeline = build_pipeline(device, &world_lines_module, &WORLD3D_FAMILY.variants[1].pipelines[0], &world_layout, surface_format);

        let blur_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("blur_downsample_pipeline_layout"), bind_group_layouts: &[&blur_layout], push_constant_ranges: &[] });
        let blur_downsample_pipeline = build_pipeline(device, &blur_module, &BLUR_FAMILY.variants[0].pipelines[0], &blur_pipeline_layout, surface_format);

        let scene_blit_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("scene_blit_pipeline_layout"), bind_group_layouts: &[&scene_sample_layout], push_constant_ranges: &[] });
        let scene_blit_pipeline = build_pipeline(device, &scene_blit_module, &BLUR_FAMILY.variants[1].pipelines[0], &scene_blit_layout, surface_format);

        let glass_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("glass_pipeline_layout"), bind_group_layouts: &[&ui_globals_layout, &scene_sample_layout], push_constant_ranges: &[] });
        let glass_pipeline = build_pipeline(device, &glass_module, &GLASS_FAMILY.variants[0].pipelines[0], &glass_layout, surface_format);

        let quad_vertices: &[f32] = &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui_quad_vertices"), contents: bytemuck::cast_slice(quad_vertices), usage: wgpu::BufferUsages::VERTEX });
        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ui_globals"),
            contents: bytemuck::bytes_of(&UiGlobals { screen_size: [1.0, 1.0], _pad: [0.0, 0.0] }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let blur_globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("blur_globals"), contents: bytemuck::bytes_of(&BlurGlobals::default()), usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST });

        Self {
            mask_pipeline,
            content_pipeline,
            vector_pipeline,
            world_opaque_pipeline,
            world_translucent_pipeline,
            world_line_pipeline,
            blur_downsample_pipeline,
            scene_blit_pipeline,
            glass_pipeline,
            ui_globals_layout,
            world_globals_layout,
            blur_layout,
            scene_sample_layout,
            quad_vertex_buffer,
            globals_buffer,
            blur_globals_buffer,
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn update_globals(&self, queue: &wgpu::Queue, width: f32, height: f32, time_seconds: f32) {
        queue.write_buffer(&self.globals_buffer, 0, bytemuck::bytes_of(&UiGlobals { screen_size: [width, height], _pad: [time_seconds, 0.0] }));
    }
}

//#endregion 📦️Pipelines

//#endregion 🔖️Pipelines
