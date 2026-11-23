//! 🧱️ Instance records and clip: the draw kind constants, the glass region, the ui instance, the vector vertex, the scissor rect and the scanline clip region. See [`super`].

use super::*;

// #region draw_types
pub const KIND_SOLID: f32 = 3.0;

pub const KIND_ROUNDED: f32 = 1.0;

pub const KIND_GLYPH: f32 = 2.0;

pub const KIND_TEXTURED: f32 = 4.0;

pub const KIND_RASTER: f32 = 5.0;

/// ↗️ A glyph with its screen-space horizontal and vertical axes carried by the instance.
pub const KIND_AFFINE_GLYPH: f32 = 10.0;

/// 🖼️ An image with its screen-space horizontal and vertical axes carried by the instance.
pub const KIND_AFFINE_RASTER: f32 = 11.0;

/// 🌀️ Clockwise spinning + pulsing loading ring (see `UiInstance::loading_border` and `UI_SHADER`'s `kind == 6` branch).
pub const KIND_LOADING_BORDER: f32 = 6.0;

/// ⏸️ Dashed, slow-spinning + gently pulsing waiting ring (see `UiInstance::waiting_border` and `UI_SHADER`'s `kind == 7` branch).
pub const KIND_WAITING_BORDER: f32 = 7.0;

/// ✅️ Solid, static at-bounds ring for `UiStatus::Finished` (see `UiInstance::finished_border` and `UI_SHADER`'s `kind == 8` branch) — no motion, distinguishing it from loading/waiting.
pub const KIND_FINISHED_BORDER: f32 = 8.0;

/// 💫️ Opacity pulse `1 -> 0.7 -> 1` ring (`introduced-border-pulse`) for `UiState::Introducing` (see `UiInstance::introducing_border` and `UI_SHADER`'s `kind == 9` branch) — the single shared implementation of the introduction-tour pulse, driven by `globals._pad.x`.
pub const KIND_INTRODUCING_BORDER: f32 = 9.0;

#[derive(Clone, Copy, Debug)]
pub struct GlassRegion {
    pub layer_index: usize,
    pub rect: [f32; 4],
    pub radius: f32,
    pub tint: Rgba,
    pub alpha: f32,
    pub blur_px: f32,
    pub saturate: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UiInstance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub params: [f32; 4],
    pub uv_rect: [f32; 4],
    pub clip_ellipse: [f32; 4],
}

impl UiInstance {
    pub fn solid(rect: [f32; 4], color: Rgba) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [0.0, 0.0, KIND_SOLID, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0], clip_ellipse: [0.0; 4] }
    }

    pub fn rounded(rect: [f32; 4], color: Rgba, radius: f32, border: f32, border_color: Rgba) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_ROUNDED, border_color.a], uv_rect: [0.0, 0.0, 1.0, 1.0], clip_ellipse: [0.0; 4] }
    }

    /// ⭕️ Clockwise spinning + pulsing loading ring in `color`; the sweep and pulse phase come from `globals._pad.x` (elapsed seconds) in `UI_SHADER`.
    pub fn loading_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_LOADING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0], clip_ellipse: [0.0; 4] }
    }

    /// ⏳️ Dashed, slow-spinning + gently pulsing waiting ring in `color`; the sweep and pulse phase come from `globals._pad.x` (elapsed seconds) in `UI_SHADER`.
    pub fn waiting_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_WAITING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0], clip_ellipse: [0.0; 4] }
    }

    /// 🏁️ Solid, static at-bounds ring for `UiStatus::Finished` in `color` — no animation.
    pub fn finished_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_FINISHED_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0], clip_ellipse: [0.0; 4] }
    }

    /// 🌅️ Raised-cosine breathing pulse ring for `UiState::Introducing` in `color`; phase comes from `globals._pad.x` in `UI_SHADER`.
    pub fn introducing_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_INTRODUCING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0], clip_ellipse: [0.0; 4] }
    }

    pub fn glyph(rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [0.0, 0.0, KIND_GLYPH, 0.0], uv_rect, clip_ellipse: [0.0; 4] }
    }

    pub fn textured(rect: [f32; 4], uv_rect: [f32; 4], color: Rgba) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [0.0, 0.0, KIND_TEXTURED, 0.0], uv_rect, clip_ellipse: [0.0; 4] }
    }

    pub fn raster(rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) -> Self {
        Self { rect, color: [1.0, 1.0, 1.0, alpha], params: [0.0, 0.0, KIND_RASTER, 0.0], uv_rect, clip_ellipse: [0.0; 4] }
    }

    /// 🔤️ Preserves the glyph's authored rotation, reflection, shear and scale.
    pub fn glyph_affine(rect:[f32;4],color:Rgba,uv_rect:[f32;4],matrix:[f32;6])->Self {
        Self::glyph(rect,color,uv_rect).affine(matrix,KIND_AFFINE_GLYPH)
    }

    /// 📷️ Preserves the sampled image's authored rotation, reflection, shear and scale.
    pub fn raster_affine(rect:[f32;4],uv_rect:[f32;4],alpha:f32,matrix:[f32;6])->Self {
        Self::raster(rect,uv_rect,alpha).affine(matrix,KIND_AFFINE_RASTER)
    }

    fn affine(mut self,matrix:[f32;6],kind:f32)->Self {
        let [x,y,width,height]=self.rect;
        let [a,b,c,d,e,f]=matrix;
        self.rect=[a*x+c*y+e,b*x+d*y+f,a*width,b*width];
        self.params=[c*height,d*height,kind,0.0];
        self
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct VectorVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

//#region Clip
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScissorRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl ScissorRect {
    pub fn from_rect(rect: semio_ui_theme::geometry::Rect, _screen_h: f32) -> Self {
        let x = rect.x.max(0.0).floor() as u32;
        let y = rect.y.max(0.0).floor() as u32;
        let x2 = (rect.x + rect.w.max(0.0)).max(0.0).ceil() as u32;
        let y2 = (rect.y + rect.h.max(0.0)).max(0.0).ceil() as u32;
        Self { x, y, w: x2.saturating_sub(x), h: y2.saturating_sub(y) }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        let x0 = self.x.max(other.x);
        let y0 = self.y.max(other.y);
        let x1 = (self.x + self.w).min(other.x + other.w);
        let y1 = (self.y + self.h).min(other.y + other.h);
        Self { x: x0, y: y0, w: x1.saturating_sub(x0), h: y1.saturating_sub(y0) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClipRegion {
    pub scissors: PagedVec<ScissorRect>,
}

impl ClipRegion {
    pub(super) fn intersect(&self, other: &Self) -> Self {
        let scissors = self.scissors.iter().flat_map(|left| other.scissors.iter().map(move |right| left.intersect(right))).filter(|rect| rect.w > 0 && rect.h > 0).collect();
        Self { scissors }
    }
}
//#endregion Clip
// #endregion draw_types
