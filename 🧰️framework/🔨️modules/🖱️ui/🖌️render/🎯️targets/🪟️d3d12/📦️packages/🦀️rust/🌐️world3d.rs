//! @emoji 🌐️ World3d mesh/lines encoding: packing every `SurfacePass`'s globals into the shared
//! 256-byte-strided root-CBV ring, uploading instance/line data, and replaying opaque/translucent
//! mesh + line draws. Mirrors the wgpu target's `WorldGlobalsRing`/`prepare_world_passes`/
//! `draw_world_pass_at` (`🎯️targets/🧊️wgpu/🦀️draw.rs`) and the Metal backend's `🌐️world3d.rs`, minus
//! the textured-mesh variant — `shader_contract.rs` itself documents `WORLD3D_TEXTURED_PIPELINE` as
//! **inferred, not wired to any real pipeline construction** in the reference implementation (no
//! `draw.rs` call site ever built it), so this backend does not add a first D3D12 call site for dead
//! contract surface either (same call the Metal backend made).
//!
//! **No separate "ring buffer" type, unlike Metal.** Metal needs `WorldGlobalsRing` because Metal's
//! vertex/fragment buffer binding takes a *byte offset* into a buffer at bind time
//! (`setVertexBuffer_offset_atIndex`) with no concept of "the buffer's own base address" exposed to
//! the caller. D3D12's root CBV (`SetGraphicsRootConstantBufferView`) takes a raw GPU virtual address
//! directly — `buffer.GetGPUVirtualAddress() + slot * WORLD_GLOBALS_SLOT_SIZE` *is* the bind value, no
//! wrapper object needed — so this file reuses `📬️frame_buffers.rs::FrameBuffers::world_globals`
//! (a plain `GrowBuffer`) instead of inventing a second buffer type.
//!
//! **Known limitation — no cross-2D/3D layer interleaving — identical to the Metal backend's finding,
//! not re-derived here.** `ui_render::scene::DrawBatch` carries no `layer_index`/watermark a backend
//! could use to interleave a `SurfacePass`'s draws between the 2D content immediately before/after it
//! within one scissor layer (`Scene::finish`'s `order()` step remaps `SurfacePass::layer_index` into
//! an internal, unexposed layer list). This is a gap in `ui_render::scene` (packet `render-scene`), not
//! in this crate's `OWNS` list. `🪟️backend.rs::encode_scene_pass` renders every `SurfacePass` for a
//! frame as one group, after backdrop-normal 2D content and before backdrop-overlay 2D content —
//! exactly where the Metal backend put it, for the same reason.

use crate::frame_buffers::FrameBuffers;
use crate::pipelines::Pipelines;
use crate::resources::GpuResources;
use crate::types::{World3dGpuInstance, WorldLineGpuVertex, WORLD_GLOBALS_SLOT_SIZE};
use ui_render::SurfacePass;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct3D::{D3D_PRIMITIVE_TOPOLOGY_LINELIST, D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST};
use windows::Win32::Graphics::Direct3D12::*;

//#region 🔖️World3d

type Device = ID3D12Device;
type CommandList = ID3D12GraphicsCommandList;

/// 📦️ Packs every pass's `view_proj`/`light_dir` into a `WORLD_GLOBALS_SLOT_SIZE`-strided byte buffer
/// and uploads it, then flattens every pass's mesh instances / line vertices into the frame's shared
/// instance/line buffers. Mirrors `UiPipelines::upload_world_passes`/`prepare_world_passes`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn upload_world_passes(device: &Device, frame_buffers: &mut FrameBuffers, passes: &[SurfacePass]) {
    if passes.is_empty() {
        return;
    }
    let mut globals_bytes = vec![0u8; passes.len() * WORLD_GLOBALS_SLOT_SIZE as usize];
    for (index, pass) in passes.iter().enumerate() {
        let globals = crate::types::WorldGlobalsGpu { view_proj: pass.view_proj, light_dir: [pass.light_dir[0], pass.light_dir[1], pass.light_dir[2], 0.0] };
        let source: &[u8] = bytemuck::bytes_of(&globals);
        let start = index * WORLD_GLOBALS_SLOT_SIZE as usize;
        globals_bytes[start..start + source.len()].copy_from_slice(source);
    }
    frame_buffers.world_globals.upload(device, &globals_bytes);

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

/// 🎬️ Replays every `SurfacePass`'s opaque/translucent mesh draws and line draws against `list`,
/// restoring the IA topology to `D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST` afterward so the next `UiQuad`/
/// `Vector` batch does not inherit the line list topology. Cull mode and depth bias need **no**
/// encoder-side push/pop the way Metal's `encode_passes` does — both are baked into
/// `world3d_translucent`'s PSO directly (see `🏗️pipelines.rs`'s header).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn encode_passes(list: &CommandList, pipelines: &Pipelines, resources: &GpuResources, frame_buffers: &FrameBuffers, passes: &[SurfacePass], screen_width: f32, screen_height: f32) {
    if passes.is_empty() {
        return;
    }
    let Some(globals_buffer) = frame_buffers.world_globals.gpu_address() else { return };
    let instance_buffer = frame_buffers.world_instances.buffer();
    let line_buffer = frame_buffers.world_lines.buffer();
    let instance_stride = std::mem::size_of::<World3dGpuInstance>() as u64;
    let line_stride = std::mem::size_of::<WorldLineGpuVertex>() as u32;

    let mut instance_cursor: u64 = 0;
    let mut line_cursor: u32 = 0;

    for (slot, pass) in passes.iter().enumerate() {
        let globals_address = globals_buffer + (slot as u64) * WORLD_GLOBALS_SLOT_SIZE;
        let viewport = pass.viewport;
        if viewport[2] <= 0.0 || viewport[3] <= 0.0 {
            continue;
        }
        unsafe {
            list.RSSetViewports(&[D3D12_VIEWPORT { TopLeftX: viewport[0], TopLeftY: viewport[1], Width: viewport[2], Height: viewport[3], MinDepth: 0.0, MaxDepth: 1.0 }]);
            list.RSSetScissorRects(&[RECT { left: viewport[0].max(0.0) as i32, top: viewport[1].max(0.0) as i32, right: (viewport[0] + viewport[2]).max(0.0) as i32, bottom: (viewport[1] + viewport[3]).max(0.0) as i32 }]);
        }

        unsafe {
            list.SetPipelineState(&pipelines.world3d_opaque);
            list.SetGraphicsRootConstantBufferView(0, globals_address);
        }
        if let Some(instances) = instance_buffer {
            for draw in &pass.draws {
                instance_cursor = draw_mesh(list, resources, draw.mesh, instances, instance_cursor, draw.instances.len() as u64, instance_stride);
            }
        }

        if pass.line_draws.iter().any(|line| !line.vertices.is_empty()) {
            if let Some(lines) = line_buffer {
                unsafe {
                    list.SetPipelineState(&pipelines.world3d_line);
                    list.SetGraphicsRootConstantBufferView(0, globals_address);
                    list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_LINELIST);
                }
                for line_draw in &pass.line_draws {
                    let count = line_draw.vertices.len() as u32;
                    if count == 0 {
                        continue;
                    }
                    let view = D3D12_VERTEX_BUFFER_VIEW { BufferLocation: unsafe { lines.GetGPUVirtualAddress() } + (line_cursor as u64) * (line_stride as u64), SizeInBytes: count * line_stride, StrideInBytes: line_stride };
                    unsafe {
                        list.IASetVertexBuffers(0, Some(&[view]));
                        list.DrawInstanced(count, 1, 0, 0);
                    }
                    line_cursor += count;
                }
                unsafe { list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST) };
            }
        }

        if !pass.translucent_draws.is_empty() {
            if let Some(instances) = instance_buffer {
                unsafe {
                    list.SetPipelineState(&pipelines.world3d_translucent);
                    list.SetGraphicsRootConstantBufferView(0, globals_address);
                }
                for draw in &pass.translucent_draws {
                    instance_cursor = draw_mesh(list, resources, draw.mesh, instances, instance_cursor, draw.instances.len() as u64, instance_stride);
                }
            }
        }
    }

    unsafe {
        list.RSSetViewports(&[D3D12_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: screen_width, Height: screen_height, MinDepth: 0.0, MaxDepth: 1.0 }]);
        list.RSSetScissorRects(&[RECT { left: 0, top: 0, right: screen_width as i32, bottom: screen_height as i32 }]);
        list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn draw_mesh(list: &CommandList, resources: &GpuResources, mesh: ui_render::MeshId, instance_buffer: &ID3D12Resource, instance_cursor: u64, instance_count: u64, instance_stride: u64) -> u64 {
    if instance_count == 0 {
        return instance_cursor;
    }
    let next_cursor = instance_cursor + instance_count;
    let Some(mesh_buffers) = resources.mesh(mesh) else { return next_cursor };
    let byte_offset = instance_cursor * instance_stride;
    let instance_view = D3D12_VERTEX_BUFFER_VIEW { BufferLocation: unsafe { instance_buffer.GetGPUVirtualAddress() } + byte_offset, SizeInBytes: (instance_count * instance_stride) as u32, StrideInBytes: instance_stride as u32 };
    unsafe {
        list.IASetVertexBuffers(0, Some(&[mesh_buffers.vertex_buffer_view, instance_view]));
        list.IASetIndexBuffer(Some(&mesh_buffers.index_buffer_view));
        list.DrawIndexedInstanced(mesh_buffers.index_count, instance_count as u32, 0, 0, 0);
    }
    next_cursor
}

//#endregion 🔖️World3d
