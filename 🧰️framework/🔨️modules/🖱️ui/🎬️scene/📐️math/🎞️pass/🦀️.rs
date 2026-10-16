//! 🎞️ Scene draw descriptors: materials, lighting, shadow pass, procedural grid, line and textured draws, and frustum culling.
//!
//! Carved out of the former `semio-framework-ui-scene` math module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

use semio_framework_geometry::{Mat4, Vec3};
use semio_framework_ui_scene_algebra::{mat4_look_at_m, mat4_orthographic_m, vec3_from_array_m, vec3_new_m, Mat4Math, Vec3Math};
use semio_framework_ui_scene_mesh::Instance3d;
use serde::{Deserialize, Serialize};

//#region ScenePass
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SceneShadowRole3d {
    pub casts: bool,
    pub receives: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SceneDraw3d {
    pub mesh_key: String,
    pub mesh_version: u64,
    pub instances: Vec<Instance3d>,
    pub shadow_role: SceneShadowRole3d,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SceneMaterialKind3d {
    Standard,
    Painted { texture_key: String },
    Celebration { stops: [[f32; 4]; 3], angle: f32 },
    Authored(SceneAuthoredMaterial3d),
}

impl SceneMaterialKind3d {
    pub fn texture_key(&self) -> Option<&str> {
        match self {
            Self::Painted { texture_key } => Some(texture_key),
            Self::Authored(material) => material.base_color_texture.as_deref(),
            Self::Standard | Self::Celebration { .. } => None,
        }
    }

    pub fn texture_keys(&self) -> impl Iterator<Item=&str> {
        let keys=match self {
            Self::Painted { texture_key } => [Some(texture_key.as_str()),None,None,None,None],
            Self::Authored(material) => [material.base_color_texture.as_deref(),material.metallic_roughness_texture.as_deref(),material.normal_texture.as_deref(),material.occlusion_texture.as_deref(),material.emissive_texture.as_deref()],
            _ => [None;5],
        };
        keys.into_iter().flatten()
    }

    pub fn texture_key_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Painted { texture_key } => Some(texture_key),
            Self::Authored(material) => [&mut material.base_color_texture,&mut material.metallic_roughness_texture,&mut material.normal_texture,&mut material.occlusion_texture,&mut material.emissive_texture].into_iter().flatten().find(|key| !key.is_empty() || key.capacity() > 0),
            Self::Standard | Self::Celebration { .. } => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneMaterialAlpha3d {
    Opaque,
    Mask,
    Blend,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneTextureWrap3d {
    ClampToEdge,
    Repeat,
    MirrorRepeat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneTextureFilter3d {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneTextureSampler3d {
    pub wrap_u: SceneTextureWrap3d,
    pub wrap_v: SceneTextureWrap3d,
    pub mag_filter: SceneTextureFilter3d,
    pub min_filter: SceneTextureFilter3d,
}

impl Default for SceneTextureSampler3d {
    fn default() -> Self {
        Self { wrap_u: SceneTextureWrap3d::Repeat, wrap_v: SceneTextureWrap3d::Repeat, mag_filter: SceneTextureFilter3d::Linear, min_filter: SceneTextureFilter3d::Linear }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SceneAuthoredMaterial3d {
    pub base_color: [f32; 4],
    pub emissive: [f32; 3],
    pub metalness: f32,
    pub roughness: f32,
    pub alpha: SceneMaterialAlpha3d,
    pub alpha_cutoff: f32,
    pub double_sided: bool,
    pub preserve_vertex_color: bool,
    pub base_color_texture: Option<String>,
    pub texture_sampler: SceneTextureSampler3d,
    pub metallic_roughness_texture: Option<String>,
    pub normal_texture: Option<String>,
    pub occlusion_texture: Option<String>,
    pub emissive_texture: Option<String>,
    pub additional_texture_samplers: [SceneTextureSampler3d; 4],
    pub normal_scale:[f32;2],
    pub occlusion_strength:f32,
}

#[derive(Clone, Debug)]
pub struct SceneMaterialDraw3d {
    pub mesh_key: String,
    pub mesh_version: u64,
    pub first_index: u32,
    pub index_count: u32,
    pub instances: Vec<Instance3d>,
    pub material: SceneMaterialKind3d,
    pub translucent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneLighting3d {
    pub ambient_color: [f32; 3],
    pub ambient_intensity: f32,
    pub sun_color: [f32; 3],
    pub sun_intensity: f32,
    pub sun_enabled: bool,
}

impl Default for SceneLighting3d {
    fn default() -> Self {
        Self { ambient_color: [1.0; 3], ambient_intensity: 1.15, sun_color: [1.0; 3], sun_intensity: 0.85, sun_enabled: false }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneMaterial3d {
    pub metalness: f32,
    pub roughness: f32,
    pub emissive: [f32; 3],
    pub emissive_intensity: f32,
}

impl Default for SceneMaterial3d {
    fn default() -> Self {
        Self { metalness: 0.0, roughness: 1.0, emissive: [0.0; 3], emissive_intensity: 0.0 }
    }
}

pub const WORLD_SHADOW_LIGHT_DISTANCE: f32 = 120.0;
pub const WORLD_SHADOW_HALF_EXTENT: f32 = 5.0;
pub const WORLD_SHADOW_NEAR: f32 = 0.5;
pub const WORLD_SHADOW_FAR: f32 = 500.0;
pub const WORLD_SHADOW_MAP_SIZE: u32 = 512;
pub const ICON_SHADOW_MAP_SIZE: u32 = 1024;

fn directional_shadow_view_projection_m(light_dir: [f32; 3]) -> Mat4 {
    let mut direction = vec3_from_array_m(light_dir);
    if direction.length_m() <= 1e-6 {
        direction = vec3_new_m(0.4, 0.6, 0.8);
    }
    direction = direction.normalize_m();
    let up = if direction.cross_m(vec3_new_m(0.0, 1.0, 0.0)).length_m() > 1e-4 { vec3_new_m(0.0, 1.0, 0.0) } else { vec3_new_m(0.0, 0.0, 1.0) };
    mat4_orthographic_m(WORLD_SHADOW_HALF_EXTENT, WORLD_SHADOW_HALF_EXTENT, WORLD_SHADOW_NEAR, WORLD_SHADOW_FAR).mul_m(mat4_look_at_m(direction.scale_m(WORLD_SHADOW_LIGHT_DISTANCE), Vec3::ZERO, up))
}

/// 🌑️ React's configured directional light shadow camera: three.js' default symmetric
/// orthographic frustum, placed at the sun and aimed at the origin with its default +Y up axis.
pub fn directional_shadow_view_projection(light_dir: [f32; 3]) -> [f32; 16] {
    directional_shadow_view_projection_m(light_dir).to_cols_array_m()
}

/// 🌑️ The directional light's own culling planes, independent of the color camera.
pub fn directional_shadow_frustum_planes(light_dir: [f32; 3]) -> [FrustumPlane; 6] {
    frustum_planes(directional_shadow_view_projection_m(light_dir))
}

/// 🧪️ Three's `DirectionalLightShadow.matrix` oracle: XY clip coordinates biased into
/// texture space while Z stays in the renderer's shared WebGPU `[0, 1]` depth convention.
pub fn directional_shadow_texture_matrix(light_dir: [f32; 3]) -> [f32; 16] {
    let clip = directional_shadow_view_projection_m(light_dir);
    Mat4 { cols: [[0.5, 0.0, 0.0, 0.0], [0.0, 0.5, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.5, 0.5, 0.0, 1.0]] }.mul_m(clip).to_cols_array_m()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneShadow3d {
    pub enabled: bool,
    pub map_size: u32,
    pub view_proj: [f32; 16],
}

impl Default for SceneShadow3d {
    fn default() -> Self {
        Self { enabled: false, map_size: WORLD_SHADOW_MAP_SIZE, view_proj: Mat4::identity().to_cols_array_m() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ScenePass3d {
    pub viewport: [f32; 4],
    pub viewport_mask: SceneViewportMask3d,
    pub render_profile: SceneRenderProfile3d,
    pub clear_color: Option<[f32; 4]>,
    pub view_proj: [f32; 16],
    pub camera_position: [f32; 3],
    pub light_dir: [f32; 3],
    pub lighting: SceneLighting3d,
    pub neutral_material: SceneMaterial3d,
    pub shadow: SceneShadow3d,
    /// ✂️ The section plane `[nx, ny, nz, constant]` (a fragment is discarded where `n · p + constant < 0`, the three.js clipping-plane convention) applied to every instance whose
    /// [`SceneInstanceMaterial3d::section`] clips; `None` clips nothing.
    pub section_clip: Option<[f32; 4]>,
    pub procedural_grid: Option<ProceduralGrid3d>,
    pub shadow_draws: Vec<SceneDraw3d>,
    pub draws: Vec<SceneDraw3d>,
    pub line_draws: Vec<LineDraw3d>,
    pub translucent_draws: Vec<SceneDraw3d>,
    pub material_draws: Vec<SceneMaterialDraw3d>,
    pub textured_draws: Vec<TexturedDraw3d>,
    pub layer_index: usize,
    pub ui_watermark: usize,
    pub vector_watermark: usize,
    pub curvilinear: Option<SceneCurvilinear3d>,
}

/// 🎨️ Lighting/output profile owned by one accepted 3D scene pass.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SceneRenderProfile3d {
    #[default]
    RasterPbr,
    SvgFlatLit,
}

/// 🎭️ Shape that admits a completed 3D scene inside its physical viewport.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SceneViewportMask3d {
    #[default]
    Rectangle,
    Ellipse,
}

/// 🐟️ Image-space remap owned by one accepted 3D scene pass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneCurvilinear3d {
    pub fov_radians: f32,
    pub strength: f32,
}
//#endregion ScenePass

//#region ProceduralGrid
pub const PROCEDURAL_GRID_CELL_THICKNESS: f32 = 0.6;
pub const PROCEDURAL_GRID_SECTION_THICKNESS: f32 = 0.0;
pub const PROCEDURAL_GRID_FADE_STRENGTH: f32 = 1.5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProceduralGrid3d {
    pub plane_z: f32,
    pub camera_plane_projection: [f32; 3],
    pub cell_size: f32,
    pub fade_distance: f32,
    pub cell_color: [f32; 3],
}
//#endregion ProceduralGrid

//#region LineDraw
#[derive(Clone, Copy, Debug)]
pub struct LineVertex3d {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

#[derive(Clone, Debug, Default)]
pub struct LineDraw3d {
    pub vertices: Vec<LineVertex3d>,
}

#[derive(Clone, Debug)]
pub struct TexturedInstance3d {
    pub texture_key: String,
    pub model: Mat4,
    pub background: [f32; 4],
    pub appearance: [f32; 4],
}

#[derive(Clone, Debug, Default)]
pub struct TexturedDraw3d {
    pub instances: Vec<TexturedInstance3d>,
}
//#endregion LineDraw

//#region Culling
#[derive(Clone, Copy, Debug)]
pub struct FrustumPlane {
    pub normal: Vec3,
    pub distance: f32,
}

pub fn frustum_planes(view_proj: Mat4) -> [FrustumPlane; 6] {
    let m = view_proj.cols;
    let rows = [
        [m[0][0] + m[0][3], m[1][0] + m[1][3], m[2][0] + m[2][3], m[3][0] + m[3][3]],
        [m[0][3] - m[0][0], m[1][3] - m[1][0], m[2][3] - m[2][0], m[3][3] - m[3][0]],
        [m[0][3] + m[0][1], m[1][3] + m[1][1], m[2][3] + m[2][1], m[3][3] + m[3][1]],
        [m[0][3] - m[0][1], m[1][3] - m[1][1], m[2][3] - m[2][1], m[3][3] - m[3][1]],
        [m[0][3] + m[0][2], m[1][3] + m[1][2], m[2][3] + m[2][2], m[3][3] + m[3][2]],
        [m[0][3] - m[0][2], m[1][3] - m[1][2], m[2][3] - m[2][2], m[3][3] - m[3][2]],
    ];
    let mut planes = [FrustumPlane { normal: Vec3::ZERO, distance: 0.0 }; 6];
    for (index, row) in rows.into_iter().enumerate() {
        let normal = vec3_new_m(row[0], row[1], row[2]);
        let length = normal.length_m();
        if length > 1e-8 {
            planes[index] = FrustumPlane { normal: normal.scale_m(1.0 / length), distance: row[3] / length };
        }
    }
    planes
}

pub fn transform_aabb(model: Mat4, min: [f32; 3], max: [f32; 3]) -> ([f32; 3], [f32; 3]) {
    let corners = [[min[0], min[1], min[2]], [max[0], min[1], min[2]], [min[0], max[1], min[2]], [max[0], max[1], min[2]], [min[0], min[1], max[2]], [max[0], min[1], max[2]], [min[0], max[1], max[2]], [max[0], max[1], max[2]]];
    let mut out_min = [f32::INFINITY; 3];
    let mut out_max = [f32::NEG_INFINITY; 3];
    for corner in corners {
        let world = model.transform_point_m(vec3_from_array_m(corner)).to_array_m();
        for axis in 0..3 {
            out_min[axis] = out_min[axis].min(world[axis]);
            out_max[axis] = out_max[axis].max(world[axis]);
        }
    }
    (out_min, out_max)
}

pub fn aabb_intersects_frustum(planes: &[FrustumPlane; 6], min: [f32; 3], max: [f32; 3]) -> bool {
    for plane in planes {
        let positive = vec3_new_m(if plane.normal.x >= 0.0 { max[0] } else { min[0] }, if plane.normal.y >= 0.0 { max[1] } else { min[1] }, if plane.normal.z >= 0.0 { max[2] } else { min[2] });
        if plane.normal.dot_m(positive) + plane.distance < 0.0 {
            return false;
        }
    }
    true
}
//#endregion Culling

#[cfg(test)]
#[path = "../../🧪️tests/🔬️pass-unit/🦀️.rs"]
mod tests;
