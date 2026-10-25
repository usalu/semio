//! 🎭️ Surface draws and pipeline kinds: the mesh, line and textured draws, the lighting, material and shadow, the surface pass and the pipeline kind. See [`super`].

use super::*;

//#region 🔖️Scene
//#region 🔖️Primitives
//#region Surface
/// 🧊️ One instance of a resident mesh with its resolved static material policy. `model` is a
/// row-major 4x4 matrix.
#[derive(Clone, Copy, Debug)]
pub struct MeshInstance {
    pub model: [f32; 16],
    pub color: [f32; 4],
    pub preserve_vertex_color: bool,
    pub emissive_intensity: f32,
    pub metalness: f32,
    pub roughness: f32,
}

/// 🕸️ One mesh's instances within a [`SurfacePass`].
#[derive(Clone, Debug)]
pub struct SurfaceMeshDraw {
    pub mesh: MeshId,
    pub instances: Vec<MeshInstance>,
}

/// ➖️ One 3D line-list vertex.
#[derive(Clone, Copy, Debug)]
pub struct LineVertex3 {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

/// 📏️ A batch of 3D line-list vertices within a [`SurfacePass`].
#[derive(Clone, Debug, Default)]
pub struct SurfaceLineDraw {
    pub vertices: Vec<LineVertex3>,
}

/// 🖼️ One textured-mesh instance within a [`SurfacePass`].
#[derive(Clone, Debug)]
pub struct TexturedMeshInstance {
    pub texture: TextureId,
    pub model: [f32; 16],
    pub tint: [f32; 4],
}

/// 🎨️ A batch of textured-mesh instances within a [`SurfacePass`].
#[derive(Clone, Debug, Default)]
pub struct SurfaceTexturedDraw {
    pub instances: Vec<TexturedMeshInstance>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceLighting {
    pub ambient_color: [f32; 3],
    pub ambient_intensity: f32,
    pub sun_color: [f32; 3],
    pub sun_intensity: f32,
    pub sun_enabled: bool,
}

impl Default for SurfaceLighting {
    fn default() -> Self {
        Self { ambient_color: [1.0; 3], ambient_intensity: 1.15, sun_color: [1.0; 3], sun_intensity: 0.85, sun_enabled: false }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceMaterial {
    pub metalness: f32,
    pub roughness: f32,
    pub emissive: [f32; 3],
    pub emissive_intensity: f32,
}

impl Default for SurfaceMaterial {
    fn default() -> Self {
        Self { metalness: 0.0, roughness: 1.0, emissive: [0.0; 3], emissive_intensity: 0.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceShadow {
    pub enabled: bool,
    pub opacity: f32,
    pub softness: f32,
    pub view_proj: [f32; 16],
}

impl Default for SurfaceShadow {
    fn default() -> Self {
        Self { enabled: false, opacity: 1.0, softness: 1.0, view_proj: [0.0; 16] }
    }
}

/// 🌐️ One 3D world pass anchored to a specific 2D layer (`layer_index`), with watermarks recording
/// how many quad/vector instances that layer held when the pass was pushed — a backend interleaves
/// this pass's draws between the 2D content before and after that point within the same layer. Ported
/// from the wgpu target's `ScenePass3d`, with `String` mesh/texture keys replaced by
/// [`crate::resource::MeshId`]/[`crate::resource::TextureId`] and no dependency on the sibling
/// `kernel_3d_scene` module — that module's own constructors are `async fn` under R2 and this crate's
/// functions cannot await (ruling U1), and this crate has no dependency on it in `Cargo.toml`.
#[derive(Clone, Debug, Default)]
pub struct SurfacePass {
    pub viewport: [f32; 4],
    pub view_proj: [f32; 16],
    pub camera_position: [f32; 3],
    pub light_dir: [f32; 3],
    pub lighting: SurfaceLighting,
    pub neutral_material: SurfaceMaterial,
    pub shadow: SurfaceShadow,
    pub draws: Vec<SurfaceMeshDraw>,
    pub translucent_draws: Vec<SurfaceMeshDraw>,
    pub line_draws: Vec<SurfaceLineDraw>,
    pub textured_draws: Vec<SurfaceTexturedDraw>,
    pub layer_index: usize,
    pub quad_watermark: usize,
    pub vector_watermark: usize,
}
//#endregion Surface

//#region Pipeline
/// 🧵️ Names the shader pipeline families a backend must provide. Defined here (packet `render-scene`)
/// because `shader_contract.rs` (packet `shader-repair`) had not defined it yet at the time this
/// packet landed — see this packet's report for the hand-off note.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PipelineKind {
    UiQuad,
    UiRasterTextured,
    Vector,
    Glass,
    BlurMipChain,
    SceneBlit,
    StencilMask,
    World3dMesh,
    World3dLines,
    World3dTextured,
}
//#endregion Pipeline
//#endregion 🔖️Primitives
//#endregion 🔖️Scene
