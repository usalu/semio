//! 🌈️ Scene instance materials: the colour source, section role, instance material, `Instance3d` and component source. See [`super`].

use super::*;

//#region Mesh
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SceneColorSource3d {
    SemanticNeutral,
    #[default]
    Authored,
    Environment,
}

impl SceneColorSource3d {
    pub const fn snapshot_flag(self) -> u16 {
        self as u16
    }

    pub const fn from_snapshot_flag(flag: u16) -> Self {
        match flag {
            0 => Self::SemanticNeutral,
            2 => Self::Environment,
            _ => Self::Authored,
        }
    }
}

/// 🎟️ How one instance takes part in the pass's section plane ([`ScenePass3d::section_clip`]). `Clipped` fragments on the removed side of the plane are discarded;
/// `StencilToggle` is the cap's parity pass over a clipped solid (it writes only the stencil's section bit, never colour or depth);
/// `Cap` is the flat plane that is drawn where that bit is set and the cut therefore exposes a solid's inside.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SceneSectionRole3d {
    #[default]
    None,
    Clipped,
    StencilToggle,
    Cap,
}

impl SceneSectionRole3d {
    /// ✂️ Whether the instance's fragments are tested against the section plane.
    pub fn clips(self) -> bool {
        matches!(self, Self::Clipped | Self::StencilToggle)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneInstanceMaterial3d {
    pub color_source: SceneColorSource3d,
    pub preserve_vertex_color: bool,
    pub emissive_intensity: f32,
    pub metalness: f32,
    pub roughness: f32,
    pub section: SceneSectionRole3d,
}

impl Default for SceneInstanceMaterial3d {
    fn default() -> Self {
        Self { color_source: SceneColorSource3d::default(), preserve_vertex_color: false, emissive_intensity: 0.0, metalness: 0.0, roughness: 1.0, section: SceneSectionRole3d::None }
    }
}

#[derive(Clone, Debug)]
pub struct Instance3d {
    pub component_source: Option<ComponentSource3d>,
    pub id: String,
    pub model: Mat4,
    pub color: [f32; 4],
    pub selected: bool,
    pub hovered: bool,
    pub material: SceneInstanceMaterial3d,
}

/// 🧬️ The exact analytic source identity carried by one scene instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentSource3d {
    pub(super) handle: [u8; 64],
    pub(super) revision: [u8; 64],
}

impl ComponentSource3d {
    pub fn new(handle: &str, revision: &str) -> Option<Self> {
        let valid = |value: &str| value.len() == 64 && value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'));
        if !valid(handle) || !valid(revision) { return None; }
        Some(Self { handle: handle.as_bytes().try_into().ok()?, revision: revision.as_bytes().try_into().ok()? })
    }

    pub fn handle(&self) -> &str { std::str::from_utf8(&self.handle).expect("validated component source handle") }

    pub fn revision(&self) -> &str { std::str::from_utf8(&self.revision).expect("validated component source revision") }
}

impl Instance3d {
    pub fn model_from_trs(position: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> Mat4 {
        mat4_translation_m(vec3_from_array_m(position)).mul_m(mat4_from_quat_m(rotation[0], rotation[1], rotation[2], rotation[3])).mul_m(mat4_scale_vec_m(vec3_from_array_m(scale)))
    }
}
//#endregion Mesh
