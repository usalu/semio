//! @emoji 🌐️ World3d mesh/lines encoding: the dynamic-offset globals ring plus per-`SurfacePass`
//! instance/line upload and draw replay. Mirrors the wgpu target's `WorldGlobalsRing`/
//! `prepare_world_passes`/`draw_world_pass_at` (`🎯️targets/🧊️wgpu/🦀️draw.rs`), minus the textured-mesh
//! variant — `shader_contract.rs` itself documents `WORLD3D_TEXTURED_PIPELINE` as **inferred, not
//! wired to any real pipeline construction** in the reference implementation (no `draw.rs` call site
//! ever built it), so this backend does not add a first Metal call site for dead contract surface.
//!
//! **No dynamic-offset bind group.** wgpu bakes a `has_dynamic_offset: true` uniform binding into the
//! bind group layout and supplies the byte offset at `set_bind_group` time. Metal has no such
//! distinction: `setVertexBuffer_offset_atIndex`/`setFragmentBuffer_offset_atIndex` always take a
//! plain byte offset into whatever buffer is bound, so the ring here is exactly one `MTLBuffer` sized
//! `WORLD_GLOBALS_SLOT_SIZE * capacity_slots`, written once per frame and re-bound per pass by offset
//! — no bind-group-layout ceremony at all.
//!
//! **Known limitation — no cross-2D/3D layer interleaving.** The wgpu target interleaves a
//! `SurfacePass`'s draws between the 2D UI/vector content painted before/after it *within the same
//! scissor layer*, using `SurfacePass::layer_index` plus `quad_watermark`/`vector_watermark` to find
//! the exact split point in `DrawList::layers`. `ui_render::scene::DrawBatch` — the only per-batch
//! data a `GraphicsBackend` actually receives — carries no `layer_index` and no watermark: `Scene::finish`'s
//! `order()` step remaps `SurfacePass::layer_index` to a position in its own internal (merged, not
//! exposed) layer list, so by the time a `RenderPacket` reaches this crate that index names nothing a
//! backend can look up. This is a real gap in the current contract (`ui_render::scene`, not in this
//! crate's `OWNS` list), not a shortcut taken here — see `📓️terra-backend-metal-report.md`'s
//! deviations for the exact upstream fix this needs (either put `layer_index` on `DrawBatch` itself,
//! or replay pass draws inline as its own batch kind). Until that lands, this backend renders every
//! `SurfacePass` for a frame as one group, after the backdrop 2D content and before backdrop overlay
//! content, rather than interleaved layer-by-layer.

use crate::frame_buffers::FrameBuffers;
use crate::pipelines::Pipelines;
use crate::resources::GpuResources;
use crate::types::{WorldGlobalsGpu, World3dGpuInstance, WorldLineGpuVertex, WORLD_GLOBALS_SLOT_SIZE};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::{MTLBuffer, MTLDevice, MTLIndexType, MTLPrimitiveType, MTLRenderCommandEncoder, MTLResourceOptions};
use ui_render::SurfacePass;

//#region 🔖️World3d

type Device = ProtocolObject<dyn MTLDevice>;
type MetalBuffer = ProtocolObject<dyn MTLBuffer>;
type Encoder = ProtocolObject<dyn MTLRenderCommandEncoder>;

/// 🌐️ The globals ring buffer every `SurfacePass` writes its `view_proj`/`light_dir` into at a
/// `WORLD_GLOBALS_SLOT_SIZE`-strided offset. Mirrors `WorldGlobalsRing`.
pub struct WorldGlobalsRing {
    buffer: Option<Retained<MetalBuffer>>,
    capacity_slots: u32,
}

impl Default for WorldGlobalsRing {
    fn default() -> Self {
        Self { buffer: None, capacity_slots: 0 }
    }
}

impl WorldGlobalsRing {
    /// 📏️ Grows (recreating, never shrinking) to hold at least `slots` slots.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn ensure_slots(&mut self, device: &Device, slots: u32) {
        if slots <= self.capacity_slots && self.buffer.is_some() {
            return;
        }
        let capacity_slots = slots.max(1).next_power_of_two().max(4);
        let size = WORLD_GLOBALS_SLOT_SIZE * capacity_slots as u64;
        self.buffer = device.newBufferWithLength_options(size as _, MTLResourceOptions::StorageModeShared);
        self.capacity_slots = capacity_slots;
    }

    /// ✍️ Writes every pass's globals into its slot via the `Shared`-storage `contents()` pointer.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn write_passes(&self, globals: &[WorldGlobalsGpu]) {
        let Some(buffer) = self.buffer.as_deref() else { return };
        // 🔓️ SAFETY: `ensure_slots` was called with `globals.len()` immediately before this, so every
        // slot offset `index * WORLD_GLOBALS_SLOT_SIZE` plus `size_of::<WorldGlobalsGpu>()` fits
        // within `buffer.length()`; `contents()` is CPU-visible for the buffer's whole lifetime under
        // `Shared` storage.
        unsafe {
            let base = buffer.contents().as_ptr().cast::<u8>();
            for (index, slot) in globals.iter().enumerate() {
                let destination = base.add(index * WORLD_GLOBALS_SLOT_SIZE as usize);
                std::ptr::copy_nonoverlapping((slot as *const WorldGlobalsGpu).cast::<u8>(), destination, std::mem::size_of::<WorldGlobalsGpu>());
            }
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn buffer(&self) -> Option<&MetalBuffer> {
        self.buffer.as_deref()
    }
}

/// 🌐️ Uploads every `SurfacePass`'s globals/instances/lines for one frame and returns the ring plus
/// the flat instance/line buffers, ready for `encode_passes` to replay. Mirrors
/// `UiPipelines::upload_world_passes`/`prepare_world_passes`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn upload_world_passes(device: &Device, ring: &mut WorldGlobalsRing, frame_buffers: &mut FrameBuffers, passes: &[SurfacePass]) {
    if passes.is_empty() {
        return;
    }
    ring.ensure_slots(device, passes.len() as u32);
    let globals: Vec<WorldGlobalsGpu> = passes.iter().map(|pass| WorldGlobalsGpu { view_proj: pass.view_proj, light_dir: [pass.light_dir[0], pass.light_dir[1], pass.light_dir[2], 0.0] }).collect();
    ring.write_passes(&globals);

    let mut instances: Vec<World3dGpuInstance> = Vec::new();
    for pass in passes {
        for draw in pass.draws.iter().chain(pass.translucent_draws.iter()) {
            instances.extend(draw.instances.iter().map(World3dGpuInstance::from));
        }
    }
    frame_buffers.world_instances.upload(device, bytemuck::cast_slice(&instances));

    let mut lines: Vec<WorldLineGpuVertex> = Vec::new();
    for pass in passes {
        for line_draw in &pass.line_draws {
            lines.extend(line_draw.vertices.iter().map(WorldLineGpuVertex::from));
        }
    }
    frame_buffers.world_lines.upload(device, bytemuck::cast_slice(&lines));
}

/// 🎬️ Replays every `SurfacePass`'s opaque/translucent mesh draws and line draws against `encoder`,
/// restoring the encoder's viewport/scissor/pipeline/depth-stencil-state to the caller's 2D defaults
/// afterward so the next `UiQuad`/`Vector` batch does not inherit world3d state. `screen_size` is the
/// full render-target size in physical pixels (the "restore" viewport/scissor).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[allow(clippy::too_many_arguments)]
pub fn encode_passes(encoder: &Encoder, pipelines: &Pipelines, resources: &GpuResources, ring: &WorldGlobalsRing, frame_buffers: &FrameBuffers, passes: &[SurfacePass], screen_width: f32, screen_height: f32) {
    if passes.is_empty() {
        return;
    }
    let Some(globals_buffer) = ring.buffer() else { return };
    let instance_buffer = frame_buffers.world_instances.buffer();
    let line_buffer = frame_buffers.world_lines.buffer();
    let instance_stride = std::mem::size_of::<World3dGpuInstance>();
    let line_stride = std::mem::size_of::<WorldLineGpuVertex>();

    let mut instance_cursor: usize = 0;
    let mut line_cursor: usize = 0;

    for (slot, pass) in passes.iter().enumerate() {
        let globals_offset = slot * WORLD_GLOBALS_SLOT_SIZE as usize;
        let viewport = pass.viewport;
        if viewport[2] <= 0.0 || viewport[3] <= 0.0 {
            continue;
        }
        encoder.setViewport(objc2_metal::MTLViewport { originX: viewport[0] as f64, originY: viewport[1] as f64, width: viewport[2] as f64, height: viewport[3] as f64, znear: 0.0, zfar: 1.0 });
        encoder.setScissorRect(objc2_metal::MTLScissorRect { x: viewport[0].max(0.0) as usize, y: viewport[1].max(0.0) as usize, width: viewport[2].max(0.0) as usize, height: viewport[3].max(0.0) as usize });

        encoder.setRenderPipelineState(&pipelines.world3d_mesh_opaque);
        encoder.setDepthStencilState(Some(&pipelines.world3d_opaque_depth_stencil));
        if let Some(instances) = instance_buffer {
            unsafe { encoder.setVertexBuffer_offset_atIndex(Some(globals_buffer), globals_offset, 2) };
            unsafe { encoder.setFragmentBuffer_offset_atIndex(Some(globals_buffer), globals_offset, 2) };
            for draw in &pass.draws {
                instance_cursor = draw_mesh(encoder, resources, draw.mesh, instances, instance_cursor, draw.instances.len(), instance_stride);
            }
        }

        if pass.line_draws.iter().any(|line| !line.vertices.is_empty()) {
            if let Some(lines) = line_buffer {
                encoder.setRenderPipelineState(&pipelines.world3d_line);
                encoder.setDepthStencilState(Some(&pipelines.world3d_translucent_depth_stencil));
                unsafe { encoder.setVertexBuffer_offset_atIndex(Some(globals_buffer), globals_offset, 2) };
                for line_draw in &pass.line_draws {
                    let count = line_draw.vertices.len();
                    if count == 0 {
                        continue;
                    }
                    unsafe {
                        encoder.setVertexBuffer_offset_atIndex(Some(lines), line_cursor * line_stride, 0);
                        encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Line, 0, count);
                    }
                    line_cursor += count;
                }
            }
        }

        if !pass.translucent_draws.is_empty() {
            if let Some(instances) = instance_buffer {
                encoder.setRenderPipelineState(&pipelines.world3d_mesh_translucent);
                encoder.setDepthStencilState(Some(&pipelines.world3d_translucent_depth_stencil));
                encoder.setCullMode(objc2_metal::MTLCullMode::Back);
                encoder.setDepthBias_slopeScale_clamp(-2.0, -1.0, 0.0);
                unsafe { encoder.setVertexBuffer_offset_atIndex(Some(globals_buffer), globals_offset, 2) };
                unsafe { encoder.setFragmentBuffer_offset_atIndex(Some(globals_buffer), globals_offset, 2) };
                for draw in &pass.translucent_draws {
                    instance_cursor = draw_mesh(encoder, resources, draw.mesh, instances, instance_cursor, draw.instances.len(), instance_stride);
                }
                encoder.setCullMode(objc2_metal::MTLCullMode::None);
                encoder.setDepthBias_slopeScale_clamp(0.0, 0.0, 0.0);
            }
        }
    }

    encoder.setViewport(objc2_metal::MTLViewport { originX: 0.0, originY: 0.0, width: screen_width as f64, height: screen_height as f64, znear: 0.0, zfar: 1.0 });
    encoder.setScissorRect(objc2_metal::MTLScissorRect { x: 0, y: 0, width: screen_width as usize, height: screen_height as usize });
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn draw_mesh(encoder: &Encoder, resources: &GpuResources, mesh: ui_render::MeshId, instance_buffer: &MetalBuffer, instance_cursor: usize, instance_count: usize, instance_stride: usize) -> usize {
    if instance_count == 0 {
        return instance_cursor;
    }
    let next_cursor = instance_cursor + instance_count;
    let Some(mesh_buffers) = resources.mesh(mesh) else { return next_cursor };
    let byte_offset = instance_cursor * instance_stride;
    unsafe {
        encoder.setVertexBuffer_offset_atIndex(Some(&mesh_buffers.vertex_buffer), 0, 0);
        encoder.setVertexBuffer_offset_atIndex(Some(instance_buffer), byte_offset, 1);
        encoder.drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount(
            MTLPrimitiveType::Triangle,
            mesh_buffers.index_count as usize,
            MTLIndexType::UInt32,
            &mesh_buffers.index_buffer,
            0,
            instance_count,
        );
    }
    next_cursor
}

//#endregion 🔖️World3d
