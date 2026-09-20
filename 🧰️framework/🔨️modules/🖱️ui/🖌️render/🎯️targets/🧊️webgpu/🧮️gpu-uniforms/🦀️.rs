//! @emoji 🧮️ Byte-exact GPU uniform/instance layouts for the canonical WGSL's `Globals`/`BlurGlobals`
//! structs and the world3d instance/line-vertex shapes. `ui_render::shader_contract` deliberately
//! carries no Rust mirror of a WGSL `struct` — only [`ui_render::PipelineSpec`] metadata — so these
//! belong to this backend, the one crate that actually builds buffers against them. Ported field-for-
//! field from `🎯️targets/🧊️wgpu/🦀️draw.rs`'s `UiGlobals`/`BlurGlobals`/`World3dGlobals`/
//! `World3dGpuInstance`/`WorldLineGpuVertex`.

use bytemuck::{Pod, Zeroable};

//#region 🔖️GpuUniforms

/// 🌐️ Mirrors `UI_SHADER`'s `Globals { screen_size: vec2<f32>, _pad: vec2<f32> }`; `_pad.x` doubles
/// as elapsed seconds, driving the animated border kinds.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct UiGlobals {
    pub screen_size: [f32; 2],
    pub _pad: [f32; 2],
}

/// 🌫️ Mirrors `BLUR_DOWNSAMPLE_SHADER`'s `BlurGlobals { src_mip: f32, _pad: vec3<f32> }`, padded to a
/// 32-byte uniform (7 trailing floats, matching the source's over-padding).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub(crate) struct BlurGlobals {
    pub src_mip: f32,
    pub _pad: [f32; 7],
}

/// 🌐️ Mirrors `WORLD3D_SHADER`'s 240-byte per-pass environment and shadow globals.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct World3dGlobals {
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

impl World3dGlobals {
    pub(crate) fn from_pass(pass: &ui_render::SurfacePass) -> Self {
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

/// 🧊️ Mirrors `WORLD3D_SHADER`'s per-instance `InstanceInput` (`model0..3`/`color`/`flags`), built
/// from [`ui_render::MeshInstance`]'s row-major model, color, and resolved material policy.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct World3dGpuInstance {
    pub model0: [f32; 4],
    pub model1: [f32; 4],
    pub model2: [f32; 4],
    pub model3: [f32; 4],
    pub color: [f32; 4],
    pub flags: [f32; 4],
}

impl World3dGpuInstance {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn from_instance(instance: &ui_render::MeshInstance) -> Self {
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

/// ➖️ Mirrors `WORLD3D_LINES_SHADER`'s per-vertex `VertexInput` (`position`/`color`), built from
/// [`ui_render::LineVertex3`].
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct WorldLineGpuVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl From<&ui_render::LineVertex3> for WorldLineGpuVertex {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(vertex: &ui_render::LineVertex3) -> Self {
        Self { position: vertex.position, color: vertex.color }
    }
}

//#endregion 🔖️GpuUniforms

//#region Tests

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️webgpu-gpu-uniforms-unit/🦀️.rs"]
mod tests;

//#endregion Tests
