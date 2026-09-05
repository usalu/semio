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

/// 🌐️ Mirrors `WORLD3D_SHADER`'s `Globals { view_proj: mat4x4<f32>, light_dir: vec4<f32> }`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct World3dGlobals {
    pub view_proj: [f32; 16],
    pub light_dir: [f32; 4],
}

/// 🧊️ Mirrors `WORLD3D_SHADER`'s per-instance `InstanceInput` (`model0..3`/`color`/`flags`), built
/// from [`ui_render::MeshInstance`]'s row-major `model`/`color`/`selected`/`hovered`.
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
            flags: [if instance.selected { 1.0 } else { 0.0 }, if instance.hovered { 1.0 } else { 0.0 }, 0.0, 0.0],
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
mod tests {
    use super::*;

    #[test]
    fn ui_globals_is_sixteen_bytes() {
        assert_eq!(std::mem::size_of::<UiGlobals>(), 16);
    }

    #[test]
    fn blur_globals_is_thirty_two_bytes() {
        assert_eq!(std::mem::size_of::<BlurGlobals>(), 32);
    }

    #[test]
    fn world3d_globals_is_eighty_bytes() {
        assert_eq!(std::mem::size_of::<World3dGlobals>(), 80);
    }

    #[test]
    fn world3d_gpu_instance_is_ninety_six_bytes() {
        assert_eq!(std::mem::size_of::<World3dGpuInstance>(), 96);
    }

    #[test]
    fn from_instance_packs_row_major_model_and_flags() {
        let mut model = [0.0f32; 16];
        for (index, value) in model.iter_mut().enumerate() {
            *value = index as f32;
        }
        let instance = ui_render::MeshInstance { model, color: [1.0, 0.0, 0.0, 1.0], selected: true, hovered: false };
        let gpu = World3dGpuInstance::from_instance(&instance);
        assert_eq!(gpu.model0, [0.0, 1.0, 2.0, 3.0]);
        assert_eq!(gpu.model3, [12.0, 13.0, 14.0, 15.0]);
        assert_eq!(gpu.flags, [1.0, 0.0, 0.0, 0.0]);
    }
}

//#endregion Tests
