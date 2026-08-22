//! @emoji 🎬️ Scene primitives, the stacked-scissor/silhouette-clip/glass-region/overlay-stream
//! builder model, `Scene::finish` (validate → snap → order → batch → hash) and `RenderPacket`.
//!
//! This is the backend-neutral half of the wgpu target's fused `draw.rs`: the display-list model and
//! its CPU algorithms, with every `wgpu::` type either dropped (device/pipeline/buffer plumbing,
//! which belongs to a backend) or replaced by a plain description (`StencilPolicy` in place of
//! `wgpu::StencilState`). A `Scene` never suspends — building one and calling `Scene::finish` is a
//! single sync run-to-completion step (ruling U1).

use crate::resource::{MeshId, ResourceOp, ResourceRegistry, TextureId};
use crate::tessellate;
use bytemuck::{Pod, Zeroable};
use std::collections::{HashMap, HashSet};

//#region 🔖️Scene

//#region 🔖️Primitives

//#region Rect

/// 📐️ A logical-pixel axis-aligned rect. This crate's own plain sync type — not
/// `semio_framework_geometry::Rect`, whose constructors are `async fn` under the U-program's R2 and
/// so cannot be called from this crate's literal-sync functions (ruling U1) without an await point
/// this crate is not allowed to have.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl LayoutRect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

/// 🪟️ A physical-pixel scissor rect. Ported verbatim from the wgpu target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScissorRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl ScissorRect {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn from_rect(rect: LayoutRect) -> Self {
        let x = rect.x.max(0.0).floor() as u32;
        let y = rect.y.max(0.0).floor() as u32;
        let x2 = (rect.x + rect.w.max(0.0)).max(0.0).ceil() as u32;
        let y2 = (rect.y + rect.h.max(0.0)).max(0.0).ceil() as u32;
        Self { x, y, w: x2.saturating_sub(x), h: y2.saturating_sub(y) }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn intersect(&self, other: &Self) -> Self {
        let x0 = self.x.max(other.x);
        let y0 = self.y.max(other.y);
        let x1 = (self.x + self.w).min(other.x + other.w);
        let y1 = (self.y + self.h).min(other.y + other.h);
        Self { x: x0, y: y0, w: x1.saturating_sub(x0), h: y1.saturating_sub(y0) }
    }
}

/// 🪟️ An exact union of non-overlapping scissor rects — the silhouette clip. Ported verbatim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClipRegion {
    pub scissors: Vec<ScissorRect>,
}

impl ClipRegion {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn from_rects(rects: &[LayoutRect]) -> Self {
        let scissors = rects.iter().map(|rect| ScissorRect::from_rect(*rect)).filter(|rect| rect.w > 0 && rect.h > 0).collect();
        Self { scissors }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn intersect(&self, other: &Self) -> Self {
        let scissors = self.scissors.iter().flat_map(|left| other.scissors.iter().map(move |right| left.intersect(right))).filter(|rect| rect.w > 0 && rect.h > 0).collect();
        Self { scissors }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn effective_scissors(&self, scissor: Option<ScissorRect>, width: f32, height: f32) -> Vec<ScissorRect> {
        let viewport = ScissorRect { x: 0, y: 0, w: width.max(0.0) as u32, h: height.max(0.0) as u32 };
        self.scissors.iter().map(|clip| clip.intersect(&viewport)).map(|clip| scissor.map_or(clip, |parent| clip.intersect(&parent))).filter(|clip| clip.w > 0 && clip.h > 0).collect()
    }
}

//#endregion Rect

//#region QuadInstance

/// 🌀️ Clockwise spinning + pulsing loading ring kind (see `QuadInstance::loading_border`).
pub const KIND_SOLID: f32 = 3.0;
pub const KIND_ROUNDED: f32 = 1.0;
pub const KIND_GLYPH: f32 = 2.0;
pub const KIND_TEXTURED: f32 = 4.0;
pub const KIND_RASTER: f32 = 5.0;
pub const KIND_LOADING_BORDER: f32 = 6.0;
pub const KIND_WAITING_BORDER: f32 = 7.0;
pub const KIND_FINISHED_BORDER: f32 = 8.0;
pub const KIND_INTRODUCING_BORDER: f32 = 9.0;

/// 🟥️ A single UI quad instance: byte-identical to the wgpu target's `UiInstance` (rect/color/params/
/// uv_rect, 64 bytes, no padding) so a ported backend can upload this array to the exact same shader
/// unchanged. `params` is `[radius, border, kind, extra]`; `kind` selects the shader branch via the
/// `KIND_*` constants.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct QuadInstance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub params: [f32; 4],
    pub uv_rect: [f32; 4],
}

impl QuadInstance {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn solid(rect: [f32; 4], color: [f32; 4]) -> Self {
        Self { rect, color, params: [0.0, 0.0, KIND_SOLID, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn rounded(rect: [f32; 4], color: [f32; 4], radius: f32, border: f32, border_color: [f32; 4]) -> Self {
        Self { rect, color, params: [radius, border, KIND_ROUNDED, border_color[3]], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    /// 🌀️ Clockwise spinning + pulsing loading ring; the sweep/pulse phase is a shader-side function
    /// of elapsed seconds, so a packet carrying this kind must set `has_animated_primitives`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn loading_border(rect: [f32; 4], color: [f32; 4], radius: f32, border: f32) -> Self {
        Self { rect, color, params: [radius, border, KIND_LOADING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    /// 🌀️ Dashed, slow-spinning + gently pulsing waiting ring — animated, see `loading_border`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn waiting_border(rect: [f32; 4], color: [f32; 4], radius: f32, border: f32) -> Self {
        Self { rect, color, params: [radius, border, KIND_WAITING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    /// ✅️ Solid, static at-bounds ring — no animation, unlike `loading_border`/`waiting_border`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn finished_border(rect: [f32; 4], color: [f32; 4], radius: f32, border: f32) -> Self {
        Self { rect, color, params: [radius, border, KIND_FINISHED_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    /// 💫️ Raised-cosine breathing pulse ring — animated, see `loading_border`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn introducing_border(rect: [f32; 4], color: [f32; 4], radius: f32, border: f32) -> Self {
        Self { rect, color, params: [radius, border, KIND_INTRODUCING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn glyph(rect: [f32; 4], color: [f32; 4], uv_rect: [f32; 4]) -> Self {
        Self { rect, color, params: [0.0, 0.0, KIND_GLYPH, 0.0], uv_rect }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn textured(rect: [f32; 4], uv_rect: [f32; 4], color: [f32; 4]) -> Self {
        Self { rect, color, params: [0.0, 0.0, KIND_TEXTURED, 0.0], uv_rect }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn raster(rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) -> Self {
        Self { rect, color: [1.0, 1.0, 1.0, alpha], params: [0.0, 0.0, KIND_RASTER, 0.0], uv_rect }
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_animated_kind(kind: f32) -> bool {
    kind == KIND_LOADING_BORDER || kind == KIND_WAITING_BORDER || kind == KIND_INTRODUCING_BORDER
}

//#endregion QuadInstance

//#region VectorVertex

/// ✒️ One vertex of a vector triangle-list primitive (thick lines, dashed lines, triangle fans).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct VectorVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

//#endregion VectorVertex

//#region Glass

/// 🧊️ A resolved glass appearance — callers derive this from their own theme/level lookup; this
/// crate has no opinion on how a level maps to alpha/blur/saturate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassStyle {
    pub tint: [f32; 4],
    pub alpha: f32,
    pub blur_px: f32,
    pub saturate: f32,
}

/// 🧊️ One glass backdrop region: a rounded rect blurred/saturated/tinted from the scene content
/// behind it. `SceneBuilder::push_glass` returns this region's index, which `begin_glass_content`
/// tags onto every layer drawn until the matching `end_glass_content`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassRegion {
    pub rect: [f32; 4],
    pub radius: f32,
    pub tint: [f32; 4],
    pub alpha: f32,
    pub blur_px: f32,
    pub saturate: f32,
}

/// 🧊️ The GPU-instance form of a [`GlassRegion`]: `params` is `[radius, alpha, blur_px, saturate]`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct GlassInstance {
    pub rect: [f32; 4],
    pub tint: [f32; 4],
    pub params: [f32; 4],
}

impl From<GlassRegion> for GlassInstance {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(region: GlassRegion) -> Self {
        Self { rect: region.rect, tint: region.tint, params: [region.radius, region.alpha, region.blur_px, region.saturate] }
    }
}

//#endregion Glass

//#region Stencil

/// 🎭️ A plain description of the two stencil policies the wgpu target's `mask_stencil_state`/
/// `content_stencil_state` compiled into `wgpu::StencilState`. A backend maps these onto its own
/// pipeline description; this crate makes the *decision* (which policy a batch needs), never the
/// device-level encoding of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StencilPolicy {
    /// ✏️ Always passes and replaces the stencil value — painting a silhouette mask.
    WriteMask,
    /// 🔒 Passes only where the stencil equals the reference value, and never writes — painting
    /// content that must stay inside a previously written silhouette mask.
    RequireMaskEquality,
}

//#endregion Stencil

//#region Surface

/// 🧊️ One instance of a resident mesh, positioned/tinted/flagged. `model` is a row-major 4x4 matrix.
#[derive(Clone, Copy, Debug)]
pub struct MeshInstance {
    pub model: [f32; 16],
    pub color: [f32; 4],
    pub selected: bool,
    pub hovered: bool,
}

/// 🧊️ One mesh's instances within a [`SurfacePass`].
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

/// ➖️ A batch of 3D line-list vertices within a [`SurfacePass`].
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

/// 🖼️ A batch of textured-mesh instances within a [`SurfacePass`].
#[derive(Clone, Debug, Default)]
pub struct SurfaceTexturedDraw {
    pub instances: Vec<TexturedMeshInstance>,
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
    pub light_dir: [f32; 3],
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

//#region 🔖️SceneBuilder

/// 🗂️ One draw layer: a scissor/clip/glass-membership state plus the primitives painted while that
/// state was active. `foreground_of` is `Some(region_index)` while inside `begin_glass_content(region)
/// .. end_glass_content()`, `None` for backdrop content. Ported from the wgpu target's `DrawLayer`.
#[derive(Default)]
pub struct SceneLayer {
    pub scissor: Option<ScissorRect>,
    pub clip: Option<ClipRegion>,
    pub foreground_of: Option<usize>,
    pub quad_instances: Vec<QuadInstance>,
    pub raster_instances: Vec<(TextureId, QuadInstance)>,
    pub vector_vertices: Vec<VectorVertex>,
    pub overlay_quad_instances: Vec<QuadInstance>,
    pub overlay_vector_vertices: Vec<VectorVertex>,
}

impl SceneLayer {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn is_empty(&self) -> bool {
        self.quad_instances.is_empty() && self.raster_instances.is_empty() && self.vector_vertices.is_empty() && self.overlay_quad_instances.is_empty() && self.overlay_vector_vertices.is_empty()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn merge_from(&mut self, mut other: Self) {
        self.quad_instances.append(&mut other.quad_instances);
        self.raster_instances.append(&mut other.raster_instances);
        self.vector_vertices.append(&mut other.vector_vertices);
        self.overlay_quad_instances.append(&mut other.overlay_quad_instances);
        self.overlay_vector_vertices.append(&mut other.overlay_vector_vertices);
    }
}

/// 🖊️ The pure-data display-list builder: stacked scissors, stacked silhouette clips, glass regions
/// and their foreground content, plus an overlay stream per layer. Ported from the wgpu target's
/// `DrawList` — same push/pop semantics, same layer-splitting-on-state-change behavior — with every
/// `wgpu::` type gone and every method a plain sync `fn`.
pub struct SceneBuilder {
    pub scene_passes: Vec<SurfacePass>,
    pub layers: Vec<SceneLayer>,
    pub glass_regions: Vec<GlassRegion>,
    scissor_stack: Vec<ScissorRect>,
    clip_stack: Vec<ClipRegion>,
    glass_content_stack: Vec<usize>,
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self { scene_passes: Vec::new(), layers: vec![SceneLayer::default()], glass_regions: Vec::new(), scissor_stack: Vec::new(), clip_stack: Vec::new(), glass_content_stack: Vec::new() }
    }
}

impl SceneBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn active_foreground_of(&self) -> Option<usize> {
        self.glass_content_stack.last().copied()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn active_layer(&mut self) -> &mut SceneLayer {
        if self.layers.is_empty() {
            self.layers.push(SceneLayer::default());
        }
        self.layers.last_mut().expect("layers is never empty past the guard above")
    }

    //#region Lifecycle

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn clear(&mut self) {
        self.scene_passes.clear();
        self.layers.clear();
        self.layers.push(SceneLayer::default());
        self.glass_regions.clear();
        self.scissor_stack.clear();
        self.clip_stack.clear();
        self.glass_content_stack.clear();
    }

    //#endregion Lifecycle

    //#region Scissor

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_scissor(&mut self, rect: LayoutRect) {
        let mut scissor = ScissorRect::from_rect(rect);
        if let Some(parent) = self.scissor_stack.last() {
            scissor = parent.intersect(&scissor);
        }
        self.scissor_stack.push(scissor);
        self.layers.push(SceneLayer { scissor: Some(scissor), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..SceneLayer::default() });
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn pop_scissor(&mut self) {
        self.scissor_stack.pop();
        let parent = self.scissor_stack.last().copied();
        self.layers.push(SceneLayer { scissor: parent, clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..SceneLayer::default() });
    }

    //#endregion Scissor

    //#region SilhouetteClip

    /// 🪟️ Clips subsequent draw content to an exact union of non-overlapping rectangles.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn begin_silhouette_clip(&mut self, rects: &[LayoutRect]) {
        let mut clip = ClipRegion::from_rects(rects);
        if let Some(parent) = self.clip_stack.last() {
            clip = parent.intersect(&clip);
        }
        self.clip_stack.push(clip.clone());
        self.layers.push(SceneLayer { scissor: self.scissor_stack.last().copied(), clip: Some(clip), foreground_of: self.active_foreground_of(), ..SceneLayer::default() });
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn end_silhouette_clip(&mut self) {
        self.clip_stack.pop();
        self.layers.push(SceneLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..SceneLayer::default() });
    }

    //#endregion SilhouetteClip

    //#region Surface

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_scene_pass(&mut self, mut pass: SurfacePass) {
        if self.layers.is_empty() {
            self.layers.push(SceneLayer::default());
        }
        let layer_index = self.layers.len() - 1;
        let layer = &self.layers[layer_index];
        pass.layer_index = layer_index;
        pass.quad_watermark = layer.quad_instances.len();
        pass.vector_watermark = layer.vector_vertices.len();
        self.scene_passes.push(pass);
    }

    //#endregion Surface

    //#region Quad

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_solid(&mut self, rect: [f32; 4], color: [f32; 4]) {
        self.active_layer().quad_instances.push(QuadInstance::solid(rect, color));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_rounded(&mut self, rect: [f32; 4], color: [f32; 4], radius: f32) {
        self.active_layer().quad_instances.push(QuadInstance::rounded(rect, color, radius, 0.0, color));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_loading_border(&mut self, rect: [f32; 4], color: [f32; 4], radius: f32, stroke: f32) {
        self.active_layer().quad_instances.push(QuadInstance::loading_border(rect, color, radius, stroke));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_waiting_border(&mut self, rect: [f32; 4], color: [f32; 4], radius: f32, stroke: f32) {
        self.active_layer().quad_instances.push(QuadInstance::waiting_border(rect, color, radius, stroke));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_finished_border(&mut self, rect: [f32; 4], color: [f32; 4], radius: f32, stroke: f32) {
        self.active_layer().quad_instances.push(QuadInstance::finished_border(rect, color, radius, stroke));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_introducing_border(&mut self, rect: [f32; 4], color: [f32; 4], radius: f32, stroke: f32) {
        self.active_layer().quad_instances.push(QuadInstance::introducing_border(rect, color, radius, stroke));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_glyph(&mut self, rect: [f32; 4], color: [f32; 4], uv_rect: [f32; 4]) {
        self.active_layer().quad_instances.push(QuadInstance::glyph(rect, color, uv_rect));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_glyph_overlay(&mut self, rect: [f32; 4], color: [f32; 4], uv_rect: [f32; 4]) {
        self.active_layer().overlay_quad_instances.push(QuadInstance::glyph(rect, color, uv_rect));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_solid_overlay(&mut self, rect: [f32; 4], color: [f32; 4]) {
        self.active_layer().overlay_quad_instances.push(QuadInstance::solid(rect, color));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_textured(&mut self, rect: [f32; 4], uv_rect: [f32; 4], color: [f32; 4]) {
        self.active_layer().quad_instances.push(QuadInstance::textured(rect, uv_rect, color));
    }

    /// 🖼️ Interns `key` into the resource registry (never re-allocating an id for a key seen before)
    /// and appends the instance tagged with that [`TextureId`] — replacing the wgpu target's
    /// `Vec<(String, UiInstance)>`, which cloned a `String` per instance per frame.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_raster_quad(&mut self, resources: &mut ResourceRegistry, key: &str, rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) {
        let texture = resources.intern_texture(key);
        self.active_layer().raster_instances.push((texture, QuadInstance::raster(rect, uv_rect, alpha)));
    }

    //#endregion Quad

    //#region Vector

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: [f32; 4], width: f32) {
        let layer = self.active_layer();
        for position in tessellate::thick_line_positions(x0, y0, x1, y1, width) {
            layer.vector_vertices.push(VectorVertex { position, color });
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_line_overlay(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: [f32; 4], width: f32) {
        let layer = self.active_layer();
        for position in tessellate::thick_line_positions(x0, y0, x1, y1, width) {
            layer.overlay_vector_vertices.push(VectorVertex { position, color });
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_triangle_fan(&mut self, points: &[[f32; 2]], color: [f32; 4]) {
        let layer = self.active_layer();
        for triangle in tessellate::triangle_fan_positions(points) {
            layer.vector_vertices.extend(triangle.map(|position| VectorVertex { position, color }));
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_triangle_fan_overlay(&mut self, points: &[[f32; 2]], color: [f32; 4]) {
        let layer = self.active_layer();
        for triangle in tessellate::triangle_fan_positions(points) {
            layer.overlay_vector_vertices.extend(triangle.map(|position| VectorVertex { position, color }));
        }
    }

    /// 〰️ The wgpu target's `push_dashed_line` called `self.push_line(...)` — an `async fn` — as a
    /// bare statement with no `.await`; the constructed `Future` was immediately dropped and the dash
    /// silently drew nothing. Both `push_line` and this function are plain sync `fn` here, so that
    /// call site cannot exist — see `tessellate::dashed_line_segments`'s doc comment.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_dashed_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: [f32; 4], width: f32, dash: f32, gap: f32) {
        for (sx0, sy0, sx1, sy1) in tessellate::dashed_line_segments(x0, y0, x1, y1, dash, gap) {
            self.push_line(sx0, sy0, sx1, sy1, color, width);
        }
    }

    /// 〰️ Overlay counterpart of [`Self::push_dashed_line`]; the wgpu target's overlay variant had the
    /// same dropped-future bug against `push_line_overlay`, fixed the same way.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_dashed_line_overlay(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: [f32; 4], width: f32, dash: f32, gap: f32) {
        for (sx0, sy0, sx1, sy1) in tessellate::dashed_line_segments(x0, y0, x1, y1, dash, gap) {
            self.push_line_overlay(sx0, sy0, sx1, sy1, color, width);
        }
    }

    //#endregion Vector

    //#region Glass

    /// 🧊️ Appends a glass region rendered with an already-resolved `style` — callers derive `style`
    /// from their own theme/level lookup rather than this method picking one.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push_glass(&mut self, rect: [f32; 4], radius: f32, style: GlassStyle) -> usize {
        let index = self.glass_regions.len();
        self.glass_regions.push(GlassRegion { rect, radius, tint: style.tint, alpha: style.alpha, blur_px: style.blur_px, saturate: style.saturate });
        index
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn begin_glass_content(&mut self, region: usize) {
        self.glass_content_stack.push(region);
        self.layers.push(SceneLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: Some(region), ..SceneLayer::default() });
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn end_glass_content(&mut self) {
        self.glass_content_stack.pop();
        self.layers.push(SceneLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..SceneLayer::default() });
    }

    //#endregion Glass
}

//#endregion 🔖️SceneBuilder

//#region 🔖️Finish

/// 🏷️ The scissor/clip/glass-membership/overlay state a [`DrawBatch`] renders under.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LayerState {
    pub scissor: Option<ScissorRect>,
    pub clip: Option<ClipRegion>,
    pub foreground_of: Option<usize>,
    pub overlay: bool,
}

/// 🎞️ One replayable draw call: a pipeline, the state to render under, an instance range into
/// [`RenderPacket::quad_instances`] or [`RenderPacket::vector_vertices`] (backend picks the array by
/// `pipeline`), and an optional precomputed stencil-mask range (also into `quad_instances`) a backend
/// paints with [`StencilPolicy::WriteMask`] before painting `instance_range` with
/// [`StencilPolicy::RequireMaskEquality`]. A backend replays ranges; it makes no clipping decisions.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawBatch {
    pub pipeline: PipelineKind,
    pub layer_state: LayerState,
    pub instance_range: (u32, u32),
    pub mask_range: Option<(u32, u32)>,
    pub texture: Option<TextureId>,
}

/// ⚠️ Why [`Scene::finish`] refused to build a [`RenderPacket`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneError {
    UnbalancedScissorStack,
    UnbalancedClipStack,
    UnbalancedGlassStack,
    NonFiniteGeometry,
    NegativeRect,
}

/// 🎬️ Per-frame inputs that are not part of the builder's own accumulated state.
pub struct FinishParams {
    pub viewport: [f32; 2],
    pub dpr: f32,
    pub time_seconds_origin: f64,
    pub resource_ops: Vec<ResourceOp>,
}

/// 🎬️ The empty namespace for [`Scene::finish`]'s five-step pipeline.
pub struct Scene;

impl Scene {
    /// 🏁️ validate → snap → order → batch → hash. Consumes `builder`; an equal `content_hash` on two
    /// packets built from equal input lets a host skip submission entirely.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn finish(builder: SceneBuilder, params: FinishParams) -> Result<RenderPacket, SceneError> {
        validate(&builder)?;
        let mut snapped = snap(builder, params.dpr);
        let layers = std::mem::take(&mut snapped.layers);
        let mut scene_passes = std::mem::take(&mut snapped.scene_passes);
        let ordered_layers = order(layers, &mut scene_passes);
        let (quad_instances, vector_vertices, glass_instances, batches) = batch(&ordered_layers, &snapped.glass_regions, params.viewport);
        let has_animated_primitives = quad_instances.iter().any(|quad| is_animated_kind(quad.params[2]));
        let content_hash = content_hash(&quad_instances, &vector_vertices, &glass_instances, &batches);
        Ok(RenderPacket {
            viewport: params.viewport,
            dpr: params.dpr,
            time_seconds_origin: params.time_seconds_origin,
            quad_instances,
            vector_vertices,
            glass_instances,
            batches,
            surface_passes: scene_passes,
            resource_ops: params.resource_ops,
            has_animated_primitives,
            content_hash,
        })
    }
}

//#region Validate

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn validate(builder: &SceneBuilder) -> Result<(), SceneError> {
    if !builder.scissor_stack.is_empty() {
        return Err(SceneError::UnbalancedScissorStack);
    }
    if !builder.clip_stack.is_empty() {
        return Err(SceneError::UnbalancedClipStack);
    }
    if !builder.glass_content_stack.is_empty() {
        return Err(SceneError::UnbalancedGlassStack);
    }
    for layer in &builder.layers {
        for quad in layer.quad_instances.iter().chain(layer.overlay_quad_instances.iter()).chain(layer.raster_instances.iter().map(|(_, quad)| quad)) {
            validate_quad(quad)?;
        }
        for vertex in layer.vector_vertices.iter().chain(layer.overlay_vector_vertices.iter()) {
            if !vertex.position.iter().chain(vertex.color.iter()).all(|value| value.is_finite()) {
                return Err(SceneError::NonFiniteGeometry);
            }
        }
    }
    for region in &builder.glass_regions {
        let finite = region.rect.iter().chain(region.tint.iter()).chain([&region.radius, &region.alpha, &region.blur_px, &region.saturate]).all(|value| value.is_finite());
        if !finite {
            return Err(SceneError::NonFiniteGeometry);
        }
        if region.rect[2] < 0.0 || region.rect[3] < 0.0 {
            return Err(SceneError::NegativeRect);
        }
    }
    Ok(())
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn validate_quad(quad: &QuadInstance) -> Result<(), SceneError> {
    let finite = quad.rect.iter().chain(quad.color.iter()).chain(quad.params.iter()).chain(quad.uv_rect.iter()).all(|value| value.is_finite());
    if !finite {
        return Err(SceneError::NonFiniteGeometry);
    }
    if quad.rect[2] < 0.0 || quad.rect[3] < 0.0 {
        return Err(SceneError::NegativeRect);
    }
    Ok(())
}

//#endregion Validate

//#region Snap

/// 🎯️ Logical→physical pixel rounding. Kills a class of shimmer (an edge that lands on a fractional
/// physical pixel dithers between neighboring pixels as content scrolls) and makes goldens
/// deterministic: two builders with the same logical geometry snap to the exact same physical rects
/// at a given `dpr`, regardless of what produced the fractional logical values upstream.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn snap(mut builder: SceneBuilder, dpr: f32) -> SceneBuilder {
    for layer in &mut builder.layers {
        for quad in layer.quad_instances.iter_mut().chain(layer.overlay_quad_instances.iter_mut()).chain(layer.raster_instances.iter_mut().map(|(_, quad)| quad)) {
            quad.rect = tessellate::snap_rect(quad.rect, dpr);
        }
        for vertex in layer.vector_vertices.iter_mut().chain(layer.overlay_vector_vertices.iter_mut()) {
            vertex.position = tessellate::snap_point(vertex.position, dpr);
        }
    }
    for region in &mut builder.glass_regions {
        region.rect = tessellate::snap_rect(region.rect, dpr);
    }
    builder
}

//#endregion Snap

//#region Order

/// 🗜️ Merges consecutive layers that share identical `(scissor, clip, foreground_of)` state and drops
/// layers left empty by upstream push/pop pairs that never drew anything between them — the wgpu
/// target's `push_scissor`/`pop_scissor`/`begin_silhouette_clip`/`end_silhouette_clip` each start a new
/// layer unconditionally, so returning to an outer clip state that was already active produces an
/// empty boundary layer plus a content layer identical in state to one seen before; dropping the empty
/// one lets the identical-state survivors coalesce into a single batch. A layer referenced by a
/// [`SurfacePass::layer_index`] (an "anchor") is kept even if empty and is never merged with a
/// neighbor in either direction, because its exact byte offset is what `quad_watermark`/
/// `vector_watermark` point into; `scene_passes` has its `layer_index` remapped to the anchor's new
/// position afterward.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn order(layers: Vec<SceneLayer>, scene_passes: &mut [SurfacePass]) -> Vec<SceneLayer> {
    let anchors: HashSet<usize> = scene_passes.iter().map(|pass| pass.layer_index).collect();
    let survivors: Vec<(usize, SceneLayer)> = layers.into_iter().enumerate().filter(|(old_index, layer)| anchors.contains(old_index) || !layer.is_empty()).collect();

    let mut merged: Vec<SceneLayer> = Vec::new();
    let mut last_was_anchor = false;
    let mut anchor_new_index: HashMap<usize, usize> = HashMap::new();

    for (old_index, layer) in survivors {
        let is_anchor = anchors.contains(&old_index);
        let can_merge = !is_anchor && !last_was_anchor && merged.last().is_some_and(|prev: &SceneLayer| prev.scissor == layer.scissor && prev.clip == layer.clip && prev.foreground_of == layer.foreground_of);
        if can_merge {
            merged.last_mut().expect("checked by can_merge").merge_from(layer);
        } else {
            if is_anchor {
                anchor_new_index.insert(old_index, merged.len());
            }
            merged.push(layer);
        }
        last_was_anchor = is_anchor;
    }

    for pass in scene_passes.iter_mut() {
        if let Some(&new_index) = anchor_new_index.get(&pass.layer_index) {
            pass.layer_index = new_index;
        }
    }
    merged
}

//#endregion Order

//#region Batch

struct LayerSpan {
    scissor: Option<ScissorRect>,
    clip: Option<ClipRegion>,
    foreground_of: Option<usize>,
    quad_range: (u32, u32),
    raster_runs: Vec<(TextureId, u32, u32)>,
    vec_range: (u32, u32),
}

/// 📦️ Copies one stream (backdrop-vs-foreground × normal-vs-overlay, selected by the caller) out of
/// `layers` into the flat `quad_instances`/`vector_vertices` arrays, recording each contributing
/// layer's ranges. Ported from the wgpu target's `build_layer_batches`/`build_overlay_layer_batches`,
/// generalized to also chunk `raster_instances` into contiguous same-[`TextureId`] runs (each run
/// needs its own batch — a backend binds one texture at a time).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn collect_spans<'a>(layers: impl Iterator<Item = &'a SceneLayer>, overlay: bool, quad_instances: &mut Vec<QuadInstance>, vector_vertices: &mut Vec<VectorVertex>) -> Vec<LayerSpan> {
    let mut spans = Vec::new();
    for layer in layers {
        let (quads, raster, vertices): (&[QuadInstance], &[(TextureId, QuadInstance)], &[VectorVertex]) =
            if overlay { (&layer.overlay_quad_instances, &[], &layer.overlay_vector_vertices) } else { (&layer.quad_instances, &layer.raster_instances, &layer.vector_vertices) };
        if quads.is_empty() && raster.is_empty() && vertices.is_empty() {
            continue;
        }

        let quad_start = quad_instances.len() as u32;
        quad_instances.extend_from_slice(quads);
        let quad_range = (quad_start, quads.len() as u32);

        let mut raster_runs = Vec::new();
        let mut index = 0;
        while index < raster.len() {
            let texture = raster[index].0;
            let run_start = quad_instances.len() as u32;
            let mut count = 0u32;
            while index < raster.len() && raster[index].0 == texture {
                quad_instances.push(raster[index].1);
                index += 1;
                count += 1;
            }
            raster_runs.push((texture, run_start, count));
        }

        let vec_start = vector_vertices.len() as u32;
        vector_vertices.extend_from_slice(vertices);
        let vec_range = (vec_start, vertices.len() as u32);

        spans.push(LayerSpan { scissor: layer.scissor, clip: layer.clip.clone(), foreground_of: layer.foreground_of, quad_range, raster_runs, vec_range });
    }
    spans
}

/// 🎞️ Builds every [`DrawBatch`] across the four independent streams (backdrop/foreground ×
/// normal/overlay — each renders in its own pass with its own stencil buffer, so each gets its own
/// mask chain) plus one [`PipelineKind::Glass`] batch per glass region, then appends the precomputed
/// stencil-mask quads for every batch straight into `quad_instances` (ported from `build_batch_masks`)
/// so a backend replays `mask_range` with [`StencilPolicy::WriteMask`] and needs no mask geometry of
/// its own.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn batch(layers: &[SceneLayer], glass_regions: &[GlassRegion], viewport: [f32; 2]) -> (Vec<QuadInstance>, Vec<VectorVertex>, Vec<GlassInstance>, Vec<DrawBatch>) {
    let mut quad_instances = Vec::new();
    let mut vector_vertices = Vec::new();
    let mut batches = Vec::new();

    for overlay in [false, true] {
        for want_foreground in [false, true] {
            let spans = collect_spans(layers.iter().filter(|layer| layer.foreground_of.is_some() == want_foreground), overlay, &mut quad_instances, &mut vector_vertices);
            let mut previous_bounds = None;
            for span in spans {
                let (mask_quads, current_bounds) = tessellate::mask_instances(span.scissor, span.clip.as_ref(), previous_bounds, viewport[0], viewport[1]);
                let mask_range = if mask_quads.is_empty() {
                    None
                } else {
                    let start = quad_instances.len() as u32;
                    let count = mask_quads.len() as u32;
                    quad_instances.extend(mask_quads);
                    Some((start, count))
                };
                previous_bounds = current_bounds;

                let layer_state = LayerState { scissor: span.scissor, clip: span.clip.clone(), foreground_of: span.foreground_of, overlay };
                if span.quad_range.1 > 0 {
                    batches.push(DrawBatch { pipeline: PipelineKind::UiQuad, layer_state: layer_state.clone(), instance_range: span.quad_range, mask_range, texture: None });
                }
                for (texture, start, count) in span.raster_runs {
                    batches.push(DrawBatch { pipeline: PipelineKind::UiRasterTextured, layer_state: layer_state.clone(), instance_range: (start, count), mask_range, texture: Some(texture) });
                }
                if span.vec_range.1 > 0 {
                    batches.push(DrawBatch { pipeline: PipelineKind::Vector, layer_state, instance_range: span.vec_range, mask_range, texture: None });
                }
            }
        }
    }

    let glass_instances: Vec<GlassInstance> = glass_regions.iter().copied().map(GlassInstance::from).collect();
    for index in 0..glass_instances.len() {
        batches.push(DrawBatch { pipeline: PipelineKind::Glass, layer_state: LayerState::default(), instance_range: (index as u32, 1), mask_range: None, texture: None });
    }

    (quad_instances, vector_vertices, glass_instances, batches)
}

//#endregion Batch

//#region Hash

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn fnv1a64(bytes: &[u8], mut state: u64) -> u64 {
    for &byte in bytes {
        state ^= u64::from(byte);
        state = state.wrapping_mul(0x0000_0100_0000_01b3);
    }
    state
}

/// 🔢️ A 64-bit content hash over packed instance bytes plus the batch list, so a host can compare two
/// packets' `content_hash` and skip submission entirely when they match.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn content_hash(quad_instances: &[QuadInstance], vector_vertices: &[VectorVertex], glass_instances: &[GlassInstance], batches: &[DrawBatch]) -> u64 {
    let mut state = 0xcbf2_9ce4_8422_2325u64;
    state = fnv1a64(bytemuck::cast_slice(quad_instances), state);
    state = fnv1a64(bytemuck::cast_slice(vector_vertices), state);
    state = fnv1a64(bytemuck::cast_slice(glass_instances), state);
    for draw_batch in batches {
        state = fnv1a64(&(draw_batch.pipeline as u32).to_le_bytes(), state);
        state = fnv1a64(&draw_batch.instance_range.0.to_le_bytes(), state);
        state = fnv1a64(&draw_batch.instance_range.1.to_le_bytes(), state);
        if let Some((start, count)) = draw_batch.mask_range {
            state = fnv1a64(&start.to_le_bytes(), state);
            state = fnv1a64(&count.to_le_bytes(), state);
        }
    }
    state
}

//#endregion Hash

//#endregion 🔖️Finish

//#region 🔖️RenderPacket

/// 📦️ Everything a [`crate::backend`] `GraphicsBackend` needs to paint one frame, and nothing it needs
/// a device to compute. Apply `resource_ops` before replaying `batches`; skip the whole submission
/// when `content_hash` matches the previously submitted packet.
#[derive(Debug)]
pub struct RenderPacket {
    pub viewport: [f32; 2],
    pub dpr: f32,
    pub time_seconds_origin: f64,
    pub quad_instances: Vec<QuadInstance>,
    pub vector_vertices: Vec<VectorVertex>,
    pub glass_instances: Vec<GlassInstance>,
    pub batches: Vec<DrawBatch>,
    pub surface_passes: Vec<SurfacePass>,
    pub resource_ops: Vec<ResourceOp>,
    pub has_animated_primitives: bool,
    pub content_hash: u64,
}

//#endregion 🔖️RenderPacket

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{offset_of, size_of};

    fn finish_params(viewport: [f32; 2], dpr: f32) -> FinishParams {
        FinishParams { viewport, dpr, time_seconds_origin: 0.0, resource_ops: Vec::new() }
    }

    //#region SilhouetteClipTests

    #[test]
    fn scissor_intersects_child() {
        let a = ScissorRect { x: 0, y: 0, w: 100, h: 100 };
        let b = ScissorRect { x: 50, y: 50, w: 100, h: 100 };
        let c = a.intersect(&b);
        assert_eq!(c.w, 50);
        assert_eq!(c.h, 50);
    }

    #[test]
    fn scissor_covers_fractional_rect_edges_without_pixel_seams() {
        assert_eq!(ScissorRect::from_rect(LayoutRect::new(10.25, 20.75, 30.5, 40.5)), ScissorRect { x: 10, y: 20, w: 31, h: 42 });
    }

    #[test]
    fn clip_region_preserves_disjoint_cutouts_and_intersects_scissor() {
        let clip = ClipRegion::from_rects(&[LayoutRect::new(0.0, 0.0, 40.0, 20.0), LayoutRect::new(80.0, 0.0, 20.0, 20.0), LayoutRect::new(0.0, 20.0, 100.0, 80.0)]);
        let scissors = clip.effective_scissors(Some(ScissorRect { x: 10, y: 0, w: 80, h: 100 }), 100.0, 100.0);
        assert_eq!(scissors, vec![ScissorRect { x: 10, y: 0, w: 30, h: 20 }, ScissorRect { x: 80, y: 0, w: 10, h: 20 }, ScissorRect { x: 10, y: 20, w: 80, h: 80 }]);
    }

    #[test]
    fn scene_builder_nests_and_restores_clip_regions() {
        let mut builder = SceneBuilder::default();
        builder.begin_silhouette_clip(&[LayoutRect::new(0.0, 0.0, 100.0, 100.0)]);
        builder.begin_silhouette_clip(&[LayoutRect::new(25.0, 25.0, 100.0, 100.0)]);
        assert_eq!(builder.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.as_slice()), Some([ScissorRect { x: 25, y: 25, w: 75, h: 75 }].as_slice()));
        builder.end_silhouette_clip();
        assert_eq!(builder.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.as_slice()), Some([ScissorRect { x: 0, y: 0, w: 100, h: 100 }].as_slice()));
        builder.end_silhouette_clip();
        assert!(builder.layers.last().is_some_and(|layer| layer.clip.is_none()));
    }

    #[test]
    fn glass_foreground_inherits_active_silhouette_clip() {
        let mut builder = SceneBuilder::default();
        builder.begin_silhouette_clip(&[LayoutRect::new(0.0, 0.0, 40.0, 20.0), LayoutRect::new(0.0, 20.0, 100.0, 80.0)]);
        let style = GlassStyle { tint: [0.1, 0.1, 0.1, 1.0], alpha: 0.5, blur_px: 8.0, saturate: 1.2 };
        let glass = builder.push_glass([0.0, 0.0, 40.0, 20.0], 0.0, style);
        builder.begin_glass_content(glass);
        assert_eq!(builder.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.len()), Some(2));
    }

    #[test]
    fn glass_content_layers_tagged_with_foreground_of() {
        let style = GlassStyle { tint: [0.2, 0.2, 0.2, 1.0], alpha: 0.6, blur_px: 4.0, saturate: 1.0 };
        let mut builder = SceneBuilder::default();
        builder.push_solid([0.0, 0.0, 100.0, 100.0], [0.2, 0.2, 0.2, 1.0]);
        let glass = builder.push_glass([10.0, 10.0, 80.0, 80.0], 8.0, style);
        assert_eq!(glass, 0);
        builder.begin_glass_content(glass);
        builder.push_solid([10.0, 10.0, 80.0, 80.0], [1.0, 0.0, 0.0, 1.0]);
        builder.end_glass_content();
        let backdrop = builder.layers.iter().filter(|layer| layer.foreground_of.is_none()).count();
        let foreground = builder.layers.iter().filter(|layer| layer.foreground_of == Some(glass)).count();
        assert_eq!(backdrop, 2);
        assert_eq!(foreground, 1);
        assert_eq!(builder.layers[1].quad_instances.len(), 1);
    }

    #[test]
    fn glass_scissor_inherits_foreground_tag() {
        let style = GlassStyle { tint: [0.0; 4], alpha: 0.5, blur_px: 8.0, saturate: 1.0 };
        let mut builder = SceneBuilder::default();
        let glass = builder.push_glass([0.0, 0.0, 100.0, 100.0], 8.0, style);
        builder.begin_glass_content(glass);
        builder.push_scissor(LayoutRect::new(10.0, 10.0, 80.0, 80.0));
        builder.push_solid([10.0, 10.0, 80.0, 80.0], [0.0, 1.0, 0.0, 1.0]);
        builder.pop_scissor();
        builder.end_glass_content();
        let scissor_layer = builder.layers.iter().find(|layer| layer.scissor.is_some()).expect("scissor layer");
        assert_eq!(scissor_layer.foreground_of, Some(glass));
    }

    //#endregion SilhouetteClipTests

    #[test]
    fn push_scissor_splits_layers() {
        let mut builder = SceneBuilder::default();
        builder.push_solid([0.0, 0.0, 200.0, 200.0], [1.0, 0.0, 0.0, 1.0]);
        builder.push_scissor(LayoutRect::new(10.0, 10.0, 80.0, 80.0));
        builder.push_solid([10.0, 10.0, 80.0, 80.0], [0.0, 1.0, 0.0, 1.0]);
        builder.pop_scissor();
        assert!(builder.layers.len() >= 3);
    }

    #[test]
    fn scene_pass_records_layer_watermarks() {
        let mut builder = SceneBuilder::default();
        builder.push_solid([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
        builder.push_solid([1.0, 1.0, 8.0, 8.0], [0.0, 1.0, 0.0, 1.0]);
        builder.push_scene_pass(SurfacePass { viewport: [0.0, 0.0, 100.0, 100.0], view_proj: [0.0; 16], light_dir: [0.0, 0.0, 1.0], ..Default::default() });
        builder.push_line(0.0, 0.0, 1.0, 1.0, [0.0, 0.0, 1.0, 1.0], 1.0);
        let pass = &builder.scene_passes[0];
        assert_eq!(pass.layer_index, 0);
        assert_eq!(pass.quad_watermark, 2);
        assert_eq!(pass.vector_watermark, 0);
        assert_eq!(builder.layers[0].quad_instances.len(), 2);
        assert_eq!(builder.layers[0].vector_vertices.len(), 6);
    }

    #[test]
    fn quad_instance_byte_layout_matches_the_wgpu_targets_ui_instance() {
        assert_eq!(size_of::<QuadInstance>(), 64);
        assert_eq!(offset_of!(QuadInstance, rect), 0);
        assert_eq!(offset_of!(QuadInstance, color), 16);
        assert_eq!(offset_of!(QuadInstance, params), 32);
        assert_eq!(offset_of!(QuadInstance, uv_rect), 48);
    }

    fn sample_builder() -> SceneBuilder {
        let mut builder = SceneBuilder::default();
        builder.push_solid([1.0, 2.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
        builder.push_line(0.0, 0.0, 5.0, 5.0, [0.0, 1.0, 0.0, 1.0], 1.0);
        builder
    }

    #[test]
    fn content_hash_is_stable_across_runs_for_identical_input() {
        let a = Scene::finish(sample_builder(), finish_params([200.0, 200.0], 1.0)).expect("finish");
        let b = Scene::finish(sample_builder(), finish_params([200.0, 200.0], 1.0)).expect("finish");
        assert_eq!(a.content_hash, b.content_hash);
    }

    #[test]
    fn content_hash_differs_when_an_instance_byte_changes() {
        let a = Scene::finish(sample_builder(), finish_params([200.0, 200.0], 1.0)).expect("finish");
        let mut changed = SceneBuilder::default();
        changed.push_solid([1.0, 2.0, 10.0, 11.0], [1.0, 0.0, 0.0, 1.0]);
        changed.push_line(0.0, 0.0, 5.0, 5.0, [0.0, 1.0, 0.0, 1.0], 1.0);
        let b = Scene::finish(changed, finish_params([200.0, 200.0], 1.0)).expect("finish");
        assert_ne!(a.content_hash, b.content_hash);
    }

    #[test]
    fn pixel_snapping_is_deterministic_across_common_dprs() {
        for dpr in [1.0, 1.5, 2.0] {
            let a = Scene::finish(sample_builder(), finish_params([200.0, 200.0], dpr)).expect("finish");
            let b = Scene::finish(sample_builder(), finish_params([200.0, 200.0], dpr)).expect("finish");
            assert_eq!(a.quad_instances, b.quad_instances);
        }
    }

    #[test]
    fn finish_rejects_unbalanced_scissor_stack() {
        let mut builder = SceneBuilder::default();
        builder.push_scissor(LayoutRect::new(0.0, 0.0, 10.0, 10.0));
        let result = Scene::finish(builder, finish_params([50.0, 50.0], 1.0));
        assert_eq!(result.unwrap_err(), SceneError::UnbalancedScissorStack);
    }

    #[test]
    fn finish_rejects_non_finite_geometry() {
        let mut builder = SceneBuilder::default();
        builder.push_solid([f32::NAN, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
        let result = Scene::finish(builder, finish_params([50.0, 50.0], 1.0));
        assert_eq!(result.unwrap_err(), SceneError::NonFiniteGeometry);
    }

    #[test]
    fn finish_drops_layers_left_empty_by_a_push_pop_pair_that_drew_nothing() {
        let mut builder = SceneBuilder::default();
        builder.push_scissor(LayoutRect::new(0.0, 0.0, 10.0, 10.0));
        builder.pop_scissor();
        let packet = Scene::finish(builder, finish_params([50.0, 50.0], 1.0)).expect("finish");
        assert!(packet.batches.is_empty());
    }

    #[test]
    fn finish_merges_layers_that_return_to_an_identical_clip_state() {
        let mut builder = SceneBuilder::default();
        builder.push_scissor(LayoutRect::new(0.0, 0.0, 50.0, 50.0));
        builder.push_solid([1.0, 1.0, 2.0, 2.0], [1.0, 0.0, 0.0, 1.0]);
        builder.pop_scissor();
        builder.push_scissor(LayoutRect::new(0.0, 0.0, 50.0, 50.0));
        builder.push_solid([3.0, 3.0, 2.0, 2.0], [0.0, 1.0, 0.0, 1.0]);
        builder.pop_scissor();
        let packet = Scene::finish(builder, finish_params([100.0, 100.0], 1.0)).expect("finish");
        let quad_batches: Vec<&DrawBatch> = packet.batches.iter().filter(|draw_batch| draw_batch.pipeline == PipelineKind::UiQuad).collect();
        assert_eq!(quad_batches.len(), 1);
        assert_eq!(quad_batches[0].instance_range.1, 2);
    }

    #[test]
    fn finish_computes_a_mask_range_for_a_silhouette_clipped_batch() {
        let mut builder = SceneBuilder::default();
        builder.begin_silhouette_clip(&[LayoutRect::new(0.0, 0.0, 40.0, 40.0)]);
        builder.push_solid([1.0, 1.0, 2.0, 2.0], [1.0, 0.0, 0.0, 1.0]);
        builder.end_silhouette_clip();
        let packet = Scene::finish(builder, finish_params([100.0, 100.0], 1.0)).expect("finish");
        let quad_batch = packet.batches.iter().find(|draw_batch| draw_batch.pipeline == PipelineKind::UiQuad).expect("quad batch");
        assert!(quad_batch.mask_range.is_some());
    }

    #[test]
    fn finish_emits_one_glass_batch_per_glass_region_with_a_content_hash_dependent_instance() {
        let style = GlassStyle { tint: [0.3, 0.3, 0.3, 1.0], alpha: 0.4, blur_px: 6.0, saturate: 1.1 };
        let mut builder = SceneBuilder::default();
        builder.push_glass([0.0, 0.0, 40.0, 40.0], 8.0, style);
        let packet = Scene::finish(builder, finish_params([100.0, 100.0], 1.0)).expect("finish");
        assert_eq!(packet.glass_instances.len(), 1);
        assert_eq!(packet.batches.iter().filter(|draw_batch| draw_batch.pipeline == PipelineKind::Glass).count(), 1);
    }

    #[test]
    fn stencil_policies_are_distinct() {
        assert_ne!(StencilPolicy::WriteMask, StencilPolicy::RequireMaskEquality);
    }
}

//#endregion Tests

//#endregion 🔖️Scene
