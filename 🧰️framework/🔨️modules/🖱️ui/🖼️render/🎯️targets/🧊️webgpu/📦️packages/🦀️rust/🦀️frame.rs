//! @emoji 🎞️ Replays one [`ui_render::RenderPacket`]: buckets `packet.batches` by
//! [`crate::surface_state::ScenePhase`], renders backdrop content/overlay into the offscreen
//! [`crate::scene_target::SceneColorTarget`], runs the blur chain, blits + composites glass onto the
//! swapchain, then renders foreground content/overlay directly onto it. Ported from
//! `🎯️targets/🧊️wgpu/🦀️draw.rs`'s `render_scene_content`/`composite_to_swapchain` — see this
//! packet's report for exactly which structural decisions changed and why.

use crate::buffers::{FrameBuffers, WorldGlobalsRing};
use crate::gpu_uniforms::{BlurGlobals, World3dGlobals, World3dGpuInstance, WorldLineGpuVertex};
use crate::pipelines::Pipelines;
use crate::resources::GpuResources;
use crate::scene_target::{SceneColorTarget, SCENE_MIP_LEVELS};
use crate::surface_state::{classify_batch_phase, ScenePhase};
use ui_render::{BackendError, DrawBatch, FrameStats, PipelineKind, QuadInstance, RenderPacket, ResourceKind, SurfacePass};

//#region 🔖️Frame

//#region 📦️BatchBuckets

#[derive(Default)]
struct BatchBuckets<'a> {
    backdrop_content: Vec<&'a DrawBatch>,
    backdrop_overlay: Vec<&'a DrawBatch>,
    foreground_content: Vec<&'a DrawBatch>,
    foreground_overlay: Vec<&'a DrawBatch>,
    glass: Vec<&'a DrawBatch>,
}

/// 🪣️ Splits `packet.batches` into the four scene-phase buckets plus a fifth for
/// `PipelineKind::Glass` batches, which carry `LayerState::default()` (no meaningful phase) and
/// composite in their own dedicated step between the blur chain and foreground content.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn bucket_batches(packet: &RenderPacket) -> BatchBuckets<'_> {
    let mut buckets = BatchBuckets::default();
    for batch in &packet.batches {
        if batch.pipeline == PipelineKind::Glass {
            buckets.glass.push(batch);
            continue;
        }
        match classify_batch_phase(&batch.layer_state) {
            ScenePhase::BackdropContent => buckets.backdrop_content.push(batch),
            ScenePhase::BackdropOverlay => buckets.backdrop_overlay.push(batch),
            ScenePhase::ForegroundContent => buckets.foreground_content.push(batch),
            ScenePhase::ForegroundOverlay => buckets.foreground_overlay.push(batch),
        }
    }
    buckets
}

//#endregion 📦️BatchBuckets

//#region 🎭️Mask

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn draw_silhouette_mask(pass: &mut wgpu::RenderPass<'_>, pipelines: &Pipelines, resources: &GpuResources, quad_buffer: &wgpu::BufferSlice<'_>, start: u32, count: u32, width: f32, height: f32) {
    if count == 0 {
        pass.set_stencil_reference(1);
        return;
    }
    pass.set_scissor_rect(0, 0, width as u32, height as u32);
    pass.set_pipeline(&pipelines.mask_pipeline);
    pass.set_bind_group(0, resources.content_bind_group(), &[]);
    pass.set_vertex_buffer(0, pipelines.quad_vertex_buffer.slice(..));
    pass.set_vertex_buffer(1, *quad_buffer);
    pass.set_stencil_reference(0);
    pass.draw(0..6, start..start + 1);
    if count > 1 {
        pass.set_stencil_reference(1);
        pass.draw(0..6, start + 1..start + count);
    }
    pass.set_stencil_reference(1);
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn apply_mask(pass: &mut wgpu::RenderPass<'_>, pipelines: &Pipelines, resources: &GpuResources, quad_buffer: Option<&wgpu::BufferSlice<'_>>, mask_range: Option<(u32, u32)>, width: f32, height: f32) {
    match (mask_range, quad_buffer) {
        (Some((start, count)), Some(buffer)) => draw_silhouette_mask(pass, pipelines, resources, buffer, start, count, width, height),
        _ => pass.set_stencil_reference(1),
    }
}

//#endregion 🎭️Mask

//#region 🎞️BatchReplay

/// 🎞️ Replays one [`DrawBatch`] verbatim — the pipeline/bind-group/vertex-buffer choice is entirely
/// determined by `batch.pipeline`/`batch.texture`; no ordering, batching or clipping decision is made
/// here (ticket `backend.rs` invariant).
#[allow(clippy::too_many_arguments, reason = "one arg per GPU resource this replay step needs; see draw.rs's own identical allow")]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn replay_batch(
    pass: &mut wgpu::RenderPass<'_>,
    pipelines: &Pipelines,
    resources: &GpuResources,
    quad_buffer: Option<&wgpu::BufferSlice<'_>>,
    vector_buffer: Option<&wgpu::BufferSlice<'_>>,
    glass_buffer: Option<&wgpu::BufferSlice<'_>>,
    scene_sample_bind_group: &wgpu::BindGroup,
    batch: &DrawBatch,
    width: f32,
    height: f32,
) -> Result<(), BackendError> {
    apply_mask(pass, pipelines, resources, quad_buffer, batch.mask_range, width, height);
    let (start, count) = batch.instance_range;
    match batch.pipeline {
        PipelineKind::UiQuad => {
            let Some(buffer) = quad_buffer else { return Ok(()) };
            pass.set_pipeline(&pipelines.content_pipeline);
            pass.set_bind_group(0, resources.content_bind_group(), &[]);
            pass.set_vertex_buffer(0, pipelines.quad_vertex_buffer.slice(..));
            pass.set_vertex_buffer(1, *buffer);
            pass.draw(0..6, start..start + count);
        }
        PipelineKind::UiRasterTextured => {
            let Some(buffer) = quad_buffer else { return Ok(()) };
            let texture = batch.texture.ok_or(BackendError::UnknownResource(ResourceKind::Texture))?;
            let bind_group = resources.raster_bind_group(texture).ok_or(BackendError::UnknownResource(ResourceKind::Texture))?;
            pass.set_pipeline(&pipelines.content_pipeline);
            pass.set_bind_group(0, bind_group, &[]);
            pass.set_vertex_buffer(0, pipelines.quad_vertex_buffer.slice(..));
            pass.set_vertex_buffer(1, *buffer);
            pass.draw(0..6, start..start + count);
        }
        PipelineKind::Vector => {
            let Some(buffer) = vector_buffer else { return Ok(()) };
            pass.set_pipeline(&pipelines.vector_pipeline);
            pass.set_bind_group(0, resources.content_bind_group(), &[]);
            pass.set_vertex_buffer(0, *buffer);
            pass.draw(start..start + count, 0..1);
        }
        PipelineKind::Glass => {
            let Some(buffer) = glass_buffer else { return Ok(()) };
            pass.set_pipeline(&pipelines.glass_pipeline);
            pass.set_bind_group(0, resources.content_bind_group(), &[]);
            pass.set_bind_group(1, scene_sample_bind_group, &[]);
            pass.set_vertex_buffer(0, pipelines.quad_vertex_buffer.slice(..));
            pass.set_vertex_buffer(1, *buffer);
            pass.draw(0..6, start..start + count);
        }
        PipelineKind::BlurMipChain | PipelineKind::SceneBlit | PipelineKind::StencilMask | PipelineKind::World3dMesh | PipelineKind::World3dLines | PipelineKind::World3dTextured => {}
    }
    pass.set_scissor_rect(0, 0, width as u32, height as u32);
    Ok(())
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn replay_batches(
    pass: &mut wgpu::RenderPass<'_>,
    pipelines: &Pipelines,
    resources: &GpuResources,
    quad_buffer: Option<&wgpu::BufferSlice<'_>>,
    vector_buffer: Option<&wgpu::BufferSlice<'_>>,
    glass_buffer: Option<&wgpu::BufferSlice<'_>>,
    scene_sample_bind_group: &wgpu::BindGroup,
    batches: &[&DrawBatch],
    width: f32,
    height: f32,
) -> Result<(), BackendError> {
    for batch in batches {
        replay_batch(pass, pipelines, resources, quad_buffer, vector_buffer, glass_buffer, scene_sample_bind_group, batch, width, height)?;
    }
    Ok(())
}

//#endregion 🎞️BatchReplay

//#region 🌐️World3d

struct PreparedWorldPass {
    globals: World3dGlobals,
    viewport: [f32; 4],
    opaque: Vec<(ui_render::MeshId, u32, u32)>,
    translucent: Vec<(ui_render::MeshId, u32, u32)>,
    line_range: (u32, u32),
}

/// 🌐️ Flattens every `packet.surface_passes` into upload-ready GPU arrays. **Decision, flagged in the
/// report**: unlike `draw.rs` (which interleaves each `SurfacePass` between its originating 2D layer's
/// `quad_watermark`/`vector_watermark`), all 3D content here renders as one block after the backdrop's
/// 2D content — `RenderPacket` drops the `ordered_layers` a backend would need to recover the exact
/// interleave point (see `📓️terra-backend-webgpu-report.md`). Each pass still gets its own silhouette
/// mask scoped to its own `viewport` rect (`mask_quads`), so 3D content is at least clipped to its own
/// declared rectangle rather than whatever the last 2D batch's stencil state happened to leave behind.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn gather_world_passes(surface_passes: &[SurfacePass], width: f32, height: f32) -> (Vec<PreparedWorldPass>, Vec<World3dGpuInstance>, Vec<WorldLineGpuVertex>, Vec<QuadInstance>) {
    let mut prepared = Vec::new();
    let mut instances = Vec::new();
    let mut lines = Vec::new();
    let mut mask_quads = Vec::new();
    let white = [1.0, 1.0, 1.0, 1.0];
    let full_screen = [0.0, 0.0, width.max(1.0), height.max(1.0)];
    for pass in surface_passes {
        let opaque: Vec<(ui_render::MeshId, u32, u32)> = pass
            .draws
            .iter()
            .filter(|draw| !draw.instances.is_empty())
            .map(|draw| {
                let offset = instances.len() as u32;
                instances.extend(draw.instances.iter().map(World3dGpuInstance::from_instance));
                (draw.mesh, offset, draw.instances.len() as u32)
            })
            .collect();
        let translucent: Vec<(ui_render::MeshId, u32, u32)> = pass
            .translucent_draws
            .iter()
            .filter(|draw| !draw.instances.is_empty())
            .map(|draw| {
                let offset = instances.len() as u32;
                instances.extend(draw.instances.iter().map(World3dGpuInstance::from_instance));
                (draw.mesh, offset, draw.instances.len() as u32)
            })
            .collect();
        let line_start = lines.len() as u32;
        for line_draw in &pass.line_draws {
            lines.extend(line_draw.vertices.iter().map(WorldLineGpuVertex::from));
        }
        let line_range = (line_start, lines.len() as u32 - line_start);
        mask_quads.push(QuadInstance::solid(full_screen, white));
        mask_quads.push(QuadInstance::solid(pass.viewport, white));
        prepared.push(PreparedWorldPass {
            globals: World3dGlobals { view_proj: pass.view_proj, light_dir: [pass.light_dir[0], pass.light_dir[1], pass.light_dir[2], 0.0] },
            viewport: pass.viewport,
            opaque,
            translucent,
            line_range,
        });
    }
    (prepared, instances, lines, mask_quads)
}

#[allow(clippy::too_many_arguments, reason = "one arg per GPU resource this replay step needs")]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn replay_world_passes(
    pass: &mut wgpu::RenderPass<'_>,
    pipelines: &Pipelines,
    resources: &GpuResources,
    world_ring: &WorldGlobalsRing,
    prepared: &[PreparedWorldPass],
    mask_buffer: Option<&wgpu::BufferSlice<'_>>,
    instance_buffer: Option<wgpu::BufferSlice<'_>>,
    line_buffer: Option<wgpu::BufferSlice<'_>>,
    width: f32,
    height: f32,
) -> Result<(), BackendError> {
    let Some(instance_buffer) = instance_buffer else { return Ok(()) };
    for (slot, scene) in prepared.iter().enumerate() {
        if let Some(mask_buffer) = mask_buffer {
            draw_silhouette_mask(pass, pipelines, resources, mask_buffer, slot as u32 * 2, 2, width, height);
        }
        pass.set_viewport(scene.viewport[0], scene.viewport[1], scene.viewport[2].max(1.0), scene.viewport[3].max(1.0), 0.0, 1.0);
        pass.set_bind_group(0, world_ring.bind_group(), &[world_ring.offset_for_slot(slot as u32)]);
        pass.set_pipeline(&pipelines.world_opaque_pipeline);
        for (mesh, offset, count) in &scene.opaque {
            draw_mesh_range(pass, resources, *mesh, instance_buffer, *offset, *count)?;
        }
        if scene.line_range.1 > 0 {
            if let Some(line_buffer) = line_buffer {
                pass.set_pipeline(&pipelines.world_line_pipeline);
                let stride = std::mem::size_of::<WorldLineGpuVertex>() as u64;
                let byte_offset = scene.line_range.0 as u64 * stride;
                pass.set_vertex_buffer(0, line_buffer.slice(byte_offset..byte_offset + scene.line_range.1 as u64 * stride));
                pass.draw(0..scene.line_range.1, 0..1);
            }
        }
        if !scene.translucent.is_empty() {
            pass.set_pipeline(&pipelines.world_translucent_pipeline);
            pass.set_bind_group(0, world_ring.bind_group(), &[world_ring.offset_for_slot(slot as u32)]);
            for (mesh, offset, count) in &scene.translucent {
                draw_mesh_range(pass, resources, *mesh, instance_buffer, *offset, *count)?;
            }
        }
    }
    pass.set_viewport(0.0, 0.0, width, height, 0.0, 1.0);
    pass.set_scissor_rect(0, 0, width as u32, height as u32);
    Ok(())
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn draw_mesh_range(pass: &mut wgpu::RenderPass<'_>, resources: &GpuResources, mesh: ui_render::MeshId, instance_buffer: wgpu::BufferSlice<'_>, offset: u32, count: u32) -> Result<(), BackendError> {
    let gpu_mesh = resources.mesh(mesh).ok_or(BackendError::UnknownResource(ResourceKind::Mesh))?;
    let stride = std::mem::size_of::<World3dGpuInstance>() as u64;
    let byte_offset = offset as u64 * stride;
    pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
    pass.set_vertex_buffer(1, instance_buffer.slice(byte_offset..byte_offset + count as u64 * stride));
    pass.set_index_buffer(gpu_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    pass.draw_indexed(0..gpu_mesh.index_count, 0, 0..count);
    Ok(())
}

//#endregion 🌐️World3d

//#region ✅️Validate

/// ✅️ Mirrors `NullBackend::validate_known_resources` — every id a packet references must already be
/// resident, or `render` fails cleanly with [`BackendError::UnknownResource`] instead of a GPU-side
/// panic on a missing bind group / mesh buffer.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn validate_known_resources(resources: &GpuResources, packet: &RenderPacket) -> Result<(), BackendError> {
    for batch in &packet.batches {
        if let Some(texture) = batch.texture {
            if !resources.has_texture(texture) {
                return Err(BackendError::UnknownResource(ResourceKind::Texture));
            }
        }
    }
    for pass in &packet.surface_passes {
        for draw in pass.draws.iter().chain(pass.translucent_draws.iter()) {
            if !resources.has_mesh(draw.mesh) {
                return Err(BackendError::UnknownResource(ResourceKind::Mesh));
            }
        }
    }
    Ok(())
}

//#endregion ✅️Validate

//#region 🌫️BlurAndComposite

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn run_blur_chain(device: &wgpu::Device, queue: &wgpu::Queue, pipelines: &Pipelines, scene: &SceneColorTarget) {
    for mip in 1..SCENE_MIP_LEVELS {
        let src_mip = mip - 1;
        queue.write_buffer(&pipelines.blur_globals_buffer, 0, bytemuck::bytes_of(&BlurGlobals { src_mip: 0.0, ..Default::default() }));
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blur_downsample_bind_group"),
            layout: &pipelines.blur_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: pipelines.blur_globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(scene.blur_scratch_mip_view(src_mip)) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(scene.sampler()) },
            ],
        });
        let mut copy_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("blur_copy_encoder") });
        scene.copy_mip_to_blur_scratch(&mut copy_encoder, src_mip);
        queue.submit(Some(copy_encoder.finish()));
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("blur_downsample_encoder") });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("blur_downsample_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: scene.mip_view(mip), resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&pipelines.blur_downsample_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..6, 0..1);
        drop(pass);
        queue.submit(Some(encoder.finish()));
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn blit_scene_to_swapchain(device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, pipelines: &Pipelines, scene: &SceneColorTarget, clear_color: wgpu::Color) {
    let scene_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("scene_blit_bind_group"),
        layout: &pipelines.scene_sample_layout,
        entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(scene.sample_view()) }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(scene.sampler()) }],
    });
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("scene_blit_pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear_color), store: wgpu::StoreOp::Store }, depth_slice: None })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    });
    pass.set_pipeline(&pipelines.scene_blit_pipeline);
    pass.set_bind_group(0, &scene_bind_group, &[]);
    pass.draw(0..6, 0..1);
}

//#endregion 🌫️BlurAndComposite

//#region 🎬️Render

pub(crate) const CLEAR_COLOR: wgpu::Color = wgpu::Color { r: 0.05, g: 0.05, b: 0.06, a: 1.0 };

fn depth_stencil_attachment(view: &wgpu::TextureView, depth_load: wgpu::LoadOp<f32>) -> wgpu::RenderPassDepthStencilAttachment<'_> {
    wgpu::RenderPassDepthStencilAttachment { view, depth_ops: Some(wgpu::Operations { load: depth_load, store: wgpu::StoreOp::Store }), stencil_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0), store: wgpu::StoreOp::Store }) }
}

/// 🎬️ This crate's entire implementation of [`ui_render::GraphicsBackend::render`]'s encode step
/// (acquire/present live in `crate::backend`). Returns the instance/draw-call counts for
/// [`ui_render::FrameStats`].
#[allow(clippy::too_many_arguments, reason = "one arg per GPU resource a frame touches; see draw.rs's own identical allow")]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn render(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    view: &wgpu::TextureView,
    depth_view: &wgpu::TextureView,
    scene: &SceneColorTarget,
    pipelines: &Pipelines,
    resources: &GpuResources,
    frame_buffers: &mut FrameBuffers,
    world_ring: &mut WorldGlobalsRing,
    packet: &RenderPacket,
    physical_width: f32,
    physical_height: f32,
    time_seconds: f32,
) -> Result<FrameStats, BackendError> {
    validate_known_resources(resources, packet)?;
    // 🖼️ `packet.viewport` is deliberately *logical*-pixel (see `ui_render::backend::PhysicalSize`'s
    // own docstring) — the geometry inside `packet.quad_instances` etc. was already dpr-snapped to
    // physical pixels by `Scene::finish`, so the screen-size uniform and every scissor/viewport call
    // here must use the caller's known *physical* surface size, never `packet.viewport` directly.
    let width = physical_width.max(1.0);
    let height = physical_height.max(1.0);
    pipelines.update_globals(queue, width, height, time_seconds);

    let buckets = bucket_batches(packet);
    let quad_buffer = frame_buffers.quad_instances.upload(device, queue, &packet.quad_instances, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "quad_instances");
    let vector_buffer = frame_buffers.vector_vertices.upload(device, queue, &packet.vector_vertices, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "vector_vertices");
    let glass_buffer = frame_buffers.glass_instances.upload(device, queue, &packet.glass_instances, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_instances");

    let (prepared_world, world_instances, world_lines, world_masks) = gather_world_passes(&packet.surface_passes, width, height);
    if !prepared_world.is_empty() {
        world_ring.ensure_slots(device, &pipelines.world_globals_layout, prepared_world.len() as u32);
        let globals: Vec<World3dGlobals> = prepared_world.iter().map(|pass| pass.globals).collect();
        world_ring.write_passes(queue, &globals);
    }
    let world_instance_buffer = frame_buffers.world_instances.upload(device, queue, &world_instances, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "world3d_instances");
    let world_line_buffer = frame_buffers.world_lines.upload(device, queue, &world_lines, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "world3d_lines");
    let world_mask_buffer = frame_buffers.world_masks.upload(device, queue, &world_masks, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "world3d_masks");

    let scene_sample_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("glass_scene_bind_group"),
        layout: &pipelines.scene_sample_layout,
        entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(scene.sample_view()) }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(scene.sampler()) }],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("webgpu_backend_frame") });

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scene_backdrop_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: scene.mip_view(0), resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(CLEAR_COLOR), store: wgpu::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0), store: wgpu::StoreOp::Store }),
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        replay_batches(&mut pass, pipelines, resources, quad_buffer.as_ref(), vector_buffer.as_ref(), glass_buffer.as_ref(), &scene_sample_bind_group, &buckets.backdrop_content, width, height)?;
        replay_world_passes(&mut pass, pipelines, resources, world_ring, &prepared_world, world_mask_buffer.as_ref(), world_instance_buffer, world_line_buffer, width, height)?;
    }

    if !buckets.backdrop_overlay.is_empty() {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scene_overlay_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: scene.mip_view(0), resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(depth_stencil_attachment(depth_view, wgpu::LoadOp::Load)),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        replay_batches(&mut pass, pipelines, resources, quad_buffer.as_ref(), vector_buffer.as_ref(), glass_buffer.as_ref(), &scene_sample_bind_group, &buckets.backdrop_overlay, width, height)?;
    }

    queue.submit(Some(encoder.finish()));

    run_blur_chain(device, queue, pipelines, scene);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("webgpu_backend_composite") });
    blit_scene_to_swapchain(device, &mut encoder, view, pipelines, scene, CLEAR_COLOR);

    if !buckets.glass.is_empty() {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("glass_composite_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        replay_batches(&mut pass, pipelines, resources, quad_buffer.as_ref(), vector_buffer.as_ref(), glass_buffer.as_ref(), &scene_sample_bind_group, &buckets.glass, width, height)?;
    }

    if !buckets.foreground_content.is_empty() {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("foreground_content_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(depth_stencil_attachment(depth_view, wgpu::LoadOp::Load)),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        replay_batches(&mut pass, pipelines, resources, quad_buffer.as_ref(), vector_buffer.as_ref(), glass_buffer.as_ref(), &scene_sample_bind_group, &buckets.foreground_content, width, height)?;
    }

    if !buckets.foreground_overlay.is_empty() {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("foreground_overlay_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(depth_stencil_attachment(depth_view, wgpu::LoadOp::Load)),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        replay_batches(&mut pass, pipelines, resources, quad_buffer.as_ref(), vector_buffer.as_ref(), glass_buffer.as_ref(), &scene_sample_bind_group, &buckets.foreground_overlay, width, height)?;
    }

    queue.submit(Some(encoder.finish()));

    let draw_call_count = packet.batches.len() as u32 + prepared_world.iter().map(|pass| pass.opaque.len() + pass.translucent.len()).sum::<usize>() as u32;
    let instance_count = packet.quad_instances.len() as u32 + packet.vector_vertices.len() as u32 + packet.glass_instances.len() as u32 + world_instances.len() as u32;
    Ok(FrameStats { encode_duration_seconds: 0.0, submit_duration_seconds: 0.0, present_duration_seconds: 0.0, draw_call_count, instance_count })
}

//#endregion 🎬️Render

//#endregion 🔖️Frame

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;
    use ui_render::{FinishParams, Scene, SceneBuilder};

    #[test]
    fn empty_packet_buckets_into_no_batches() {
        let builder = SceneBuilder::default();
        let packet = Scene::finish(builder, FinishParams { viewport: [100.0, 100.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
        let buckets = bucket_batches(&packet);
        assert!(buckets.backdrop_content.is_empty());
        assert!(buckets.glass.is_empty());
    }

    #[test]
    fn solid_quad_batches_into_backdrop_content() {
        let mut builder = SceneBuilder::default();
        builder.push_solid([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
        let packet = Scene::finish(builder, FinishParams { viewport: [100.0, 100.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
        let buckets = bucket_batches(&packet);
        assert_eq!(buckets.backdrop_content.len(), 1);
        assert!(buckets.backdrop_overlay.is_empty());
        assert!(buckets.foreground_content.is_empty());
    }

    #[test]
    fn overlay_quad_batches_into_backdrop_overlay() {
        let mut builder = SceneBuilder::default();
        builder.push_solid_overlay([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
        let packet = Scene::finish(builder, FinishParams { viewport: [100.0, 100.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
        let buckets = bucket_batches(&packet);
        assert!(buckets.backdrop_content.is_empty());
        assert_eq!(buckets.backdrop_overlay.len(), 1);
    }

    #[test]
    fn glass_region_batches_into_its_own_bucket_never_a_scene_phase() {
        let mut builder = SceneBuilder::default();
        builder.push_glass([0.0, 0.0, 40.0, 40.0], 8.0, ui_render::GlassStyle { tint: [1.0, 1.0, 1.0, 1.0], alpha: 0.5, blur_px: 12.0, saturate: 1.0 });
        let packet = Scene::finish(builder, FinishParams { viewport: [100.0, 100.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
        let buckets = bucket_batches(&packet);
        assert_eq!(buckets.glass.len(), 1);
        assert!(buckets.backdrop_content.is_empty());
    }

    #[test]
    fn gather_world_passes_is_empty_for_no_surface_passes() {
        let (prepared, instances, lines, masks) = gather_world_passes(&[], 800.0, 600.0);
        assert!(prepared.is_empty());
        assert!(instances.is_empty());
        assert!(lines.is_empty());
        assert!(masks.is_empty());
    }

    #[test]
    fn gather_world_passes_emits_two_mask_quads_per_pass() {
        let pass = SurfacePass { viewport: [0.0, 0.0, 200.0, 150.0], ..SurfacePass::default() };
        let (prepared, _, _, masks) = gather_world_passes(std::slice::from_ref(&pass), 800.0, 600.0);
        assert_eq!(prepared.len(), 1);
        assert_eq!(masks.len(), 2);
        assert_eq!(masks[0].rect, [0.0, 0.0, 800.0, 600.0]);
        assert_eq!(masks[1].rect, [0.0, 0.0, 200.0, 150.0]);
    }
}

//#endregion Tests
