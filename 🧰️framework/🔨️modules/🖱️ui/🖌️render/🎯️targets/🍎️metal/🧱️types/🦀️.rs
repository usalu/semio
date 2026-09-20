//! @emoji 🧱️ GPU-layout mirrors this crate needs that `ui_render::scene` does not already export as
//! `#[repr(C)]`/`Pod` — `ui_render::{QuadInstance, VectorVertex, GlassInstance}` are byte-identical to
//! their Metal buffer layout already and are used directly via `bytemuck::cast_slice`; only the
//! world3d instance/vertex/uniform shapes need a Metal-side GPU form, built from
//! `ui_render::scene::{MeshInstance, LineVertex3}` (which are not `Pod` — `MeshInstance` carries
//! `bool` fields no GPU buffer can hold directly).

use bytemuck::{Pod, Zeroable};
use ui_render::{LineVertex3, MeshInstance};

//#region 🔖️Types

//#region 🌐️World3d

/// 🌐️ Byte-identical to `WorldGlobals` in `✨️msl.rs`'s world3d MSL. 240 bytes;
/// `WORLD_GLOBALS_SLOT_SIZE` below is the buffer-offset
/// stride a caller must round up to when packing several of these into one ring buffer, mirroring the
/// wgpu target's `WORLD_GLOBALS_SLOT_SIZE = 256` (Metal constant-buffer offset alignment is
/// device-dependent; 256 is the safe universal bound on every Apple GPU generation).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct WorldGlobalsGpu {
    pub view_proj: [f32; 16],
    pub shadow_view_proj: [f32; 16],
    pub camera_position: [f32; 4],
    pub light_dir: [f32; 4],
    pub ambient: [f32; 4],
    pub sun: [f32; 4],
    pub material: [f32; 4],
    pub material_emissive: [f32; 4],
    pub shadow: [f32; 4],
}

impl WorldGlobalsGpu {
    pub fn from_pass(pass: &ui_render::SurfacePass) -> Self {
        Self {
            view_proj: pass.view_proj,
            shadow_view_proj: pass.shadow.view_proj,
            camera_position: [pass.camera_position[0], pass.camera_position[1], pass.camera_position[2], 0.0],
            light_dir: [pass.light_dir[0], pass.light_dir[1], pass.light_dir[2], 0.0],
            ambient: [pass.lighting.ambient_color[0], pass.lighting.ambient_color[1], pass.lighting.ambient_color[2], pass.lighting.ambient_intensity],
            sun: [pass.lighting.sun_color[0], pass.lighting.sun_color[1], pass.lighting.sun_color[2], pass.lighting.sun_intensity],
            material: [pass.neutral_material.metalness, pass.neutral_material.roughness, pass.neutral_material.emissive_intensity, if pass.lighting.sun_enabled { 1.0 } else { 0.0 }],
            material_emissive: [pass.neutral_material.emissive[0], pass.neutral_material.emissive[1], pass.neutral_material.emissive[2], 0.0],
            shadow: [if pass.shadow.enabled { 1.0 } else { 0.0 }, pass.shadow.opacity, pass.shadow.softness, 0.0],
        }
    }
}

/// 📏️ Byte stride between consecutive `WorldGlobalsGpu` slots inside the ring buffer — see
/// `WorldGlobalsGpu`'s docstring for why 256 rather than `size_of::<WorldGlobalsGpu>()` (240).
pub const WORLD_GLOBALS_SLOT_SIZE: u64 = 256;

/// 🧊️ Byte-identical to `WorldMeshVertexIn`'s per-instance attributes (locations 3..8) in
/// `✨️msl.rs`'s world3d mesh MSL: four `float4` model-matrix rows, `color`, `flags`. Mirrors the wgpu
/// target's `World3dGpuInstance`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct World3dGpuInstance {
    pub model0: [f32; 4],
    pub model1: [f32; 4],
    pub model2: [f32; 4],
    pub model3: [f32; 4],
    pub color: [f32; 4],
    pub flags: [f32; 4],
}

impl From<&MeshInstance> for World3dGpuInstance {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(instance: &MeshInstance) -> Self {
        let m = instance.model;
        Self {
            model0: [m[0], m[1], m[2], m[3]],
            model1: [m[4], m[5], m[6], m[7]],
            model2: [m[8], m[9], m[10], m[11]],
            model3: [m[12], m[13], m[14], m[15]],
            color: instance.color,
            flags: [if instance.preserve_vertex_color { 1.0 } else { 0.0 }, instance.emissive_intensity, instance.metalness, instance.roughness],
        }
    }
}

/// ➖️ Byte-identical to `WorldLineVertexIn` in `✨️msl.rs`'s world3d lines MSL.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct WorldLineGpuVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl From<&LineVertex3> for WorldLineGpuVertex {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(vertex: &LineVertex3) -> Self {
        Self { position: vertex.position, color: vertex.color }
    }
}

/// 🧊️ Byte-identical to `WorldMeshVertexIn`'s per-vertex attributes (locations 0..1).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct World3dGpuVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

//#endregion 🌐️World3d

//#region 📐️QuadVertex

/// 📐️ The shared unit-quad corner buffer (two CCW triangles) every 2D pipeline's `buffer(0)` binds —
/// byte-identical to the wgpu target's `quad_vertex_buffer` contents.
pub const UNIT_QUAD_CORNERS: [[f32; 2]; 6] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

//#endregion 📐️QuadVertex

//#region 🌫️BlurGlobals

/// 🌫️ The tiny per-draw uniform `✨️msl.rs`'s blur downsample fragment function reads its source mip
/// from — see that file's header for why this replaces the wgpu target's per-mip texture view.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct BlurMipGpu {
    pub src_mip: u32,
}

//#endregion 🌫️BlurGlobals

//#endregion 🔖️Types
