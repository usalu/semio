//! 🔲️ Rect, quad, glass and stencil primitives: the layout, scissor and clip rects, the quad instance kinds, the vector vertex, the glass style, region and instance and the stencil policy. See [`super`].

use super::*;

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
#[cfg(any(test, feature = "backend-testing"))]
pub(super) fn is_animated_kind(kind: f32) -> bool {
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
//#endregion 🔖️Primitives
//#endregion 🔖️Scene
