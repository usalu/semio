//! 🅰️ Glyph entry and text weight: the packed `GlyphEntry`, the text weight and the faux medium and bold offsets. See [`super`].

use super::*;

// #region text
/// 🔠️ One packed glyph. `atlas_*`/`width`/`height` are ATLAS TEXELS — device pixels, because the
/// atlas is rasterised at `size_px * raster_scale` so text stays crisp on a HiDPI surface.
/// `advance`/`bearing_*` and [`Self::logical_width`]/[`Self::logical_height`] are LOGICAL (CSS)
/// pixels, the only unit a layout or paint call site is ever allowed to see. See the ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g report for the whole unit model.
#[derive(Clone, Copy, Debug)]
pub struct GlyphEntry {
    pub atlas_x: u32,
    pub atlas_y: u32,
    pub width: u32,
    pub height: u32,
    pub advance: f32,
    /// 📏️ The advance per logical pixel of font size, from the face's own units — what [`FontAtlas::ensure_glyph_for`] scales to
    /// the EXACT requested size, so no layout ever sees the raster grid.
    pub advance_em: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
    /// 📐️ The atlas raster scale this glyph was rasterised at — the divisor that turns its texel
    /// extent back into the logical extent a draw-list quad must use.
    pub raster_scale: f32,
    /// 🎨️ True when this glyph lives in the RGBA `color_pixels` page (COLR/bitmap color emoji)
    /// rather than the alpha-only `pixels` page. Paint call sites in `widgets`/`paint` (outside
    /// this region) still only sample the alpha page — see the report's wiring-request section.
    pub is_color: bool,
}

impl GlyphEntry {
    /// 📏️ Quad width in LOGICAL pixels — what every `push_glyph` call site must use, so a glyph
    /// rasterised at 2× covers the same CSS box it does at 1×.
    pub fn logical_width(&self) -> f32 {
        self.width as f32 / self.raster_scale.max(f32::MIN_POSITIVE)
    }

    /// 📏️ Quad height in LOGICAL pixels — the vertical twin of [`Self::logical_width`].
    pub fn logical_height(&self) -> f32 {
        self.height as f32 / self.raster_scale.max(f32::MIN_POSITIVE)
    }
}

//#region 🅰️Weight
/// 🅰️ The three weights the UI asks for — default, `font-medium`, and `font-semibold`.
///
/// ⚠️ No bold Latin face ships in this repo: `🖼️assets/🔤️fonts/{🚀️anta,🧱️kelly-slab,⌨️share-tech-mono}`
/// each carry `📖️regular` only (Anta and Share Tech Mono are single-weight upstream), and the only
/// bold/semibold outlines on disk are Noto **Emoji**'s. [`TextWeight::Semibold`] is therefore
/// SYNTHETIC — a second strike offset along x by [`faux_bold_offset`], the faux bold every renderer
/// without a real face falls back to. It keeps the glyph ADVANCES, and so the line box and every wrap
/// point, byte-identical to the regular face — which is what makes it a safe weight swap at React's
/// own size instead of the size swap this target used to make.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextWeight {
    #[default]
    Regular,
    Medium,
    Semibold,
}

impl TextWeight {
    /// 🅰️ React's own mapping: `emphasize` is `font-semibold`, everything else the default face.
    pub const fn of(emphasize: bool) -> Self {
        if emphasize {
            TextWeight::Semibold
        } else {
            TextWeight::Regular
        }
    }

    /// 🅰️ How many strikes one glyph costs at this weight — the number a per-glyph reservation must
    /// price before the retained paint arm can carry a real weight swap.
    pub const fn strikes(self) -> usize {
        match self {
            TextWeight::Regular => 1,
            TextWeight::Medium | TextWeight::Semibold => 2,
        }
    }

    pub fn synthetic_offset(self, size_px: f32) -> Option<f32> {
        match self {
            TextWeight::Regular => None,
            TextWeight::Medium => Some(faux_medium_offset(size_px)),
            TextWeight::Semibold => Some(faux_bold_offset(size_px)),
        }
    }
}

fn faux_medium_offset(size_px: f32) -> f32 {
    if !size_px.is_finite() || size_px <= 0.0 {
        return 0.0;
    }
    (size_px * FAUX_MEDIUM_EM_RATIO).max(FAUX_MEDIUM_MIN_PX)
}

/// 🅰️ The x offset of a synthetic semibold's second strike, in LOGICAL pixels. CSS `font-weight: 600`
/// against `400` thickens a stem by roughly 4 % of the em, which is the ratio this reproduces — so it
/// scales with the font size rather than being a fixed pixel smear, with a floor so the strike still
/// shows at the smallest ramp step.
pub fn faux_bold_offset(size_px: f32) -> f32 {
    if !size_px.is_finite() || size_px <= 0.0 {
        return 0.0;
    }
    (size_px * FAUX_BOLD_EM_RATIO).max(FAUX_BOLD_MIN_PX)
}

/// 🅰️ The em fraction a synthetic semibold offsets by.
const FAUX_BOLD_EM_RATIO: f32 = 0.04;

/// 🅰️ The floor below which a synthetic semibold would not show at all.
const FAUX_BOLD_MIN_PX: f32 = 0.34;

const FAUX_MEDIUM_EM_RATIO: f32 = 0.02;

const FAUX_MEDIUM_MIN_PX: f32 = 0.17;
//#endregion 🅰️Weight
// #endregion text
