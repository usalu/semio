//! 🗂️ Font atlas state and setup: the atlas mode, the rasterized and resolved glyphs, the `FontAtlas`, its builtin and font-backed construction, dirty flags, shaping and the raster scale. See [`super`].

use super::*;

// #region text
/// 🧩️ How a `FontAtlas` resolves and rasterizes glyphs. `Bitmap` is the deterministic,
/// dependency-free 8×16 ASCII fallback used by `FontAtlas::builtin()`; `Shaped` runs the full
/// parley/fontique/swash pipeline against a registered `fontique::Collection`.
pub(super) enum AtlasMode {
    Bitmap,
    Shaped,
}

/// 📦️ One rasterized glyph ready to be packed into an atlas page, produced by either
/// `rasterize_bitmap_glyph` or `rasterize_shaped_glyph`.
pub(super) struct RasterizedGlyph {
    pub(super) bitmap: Vec<u8>,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) bearing_x: f32,
    pub(super) bearing_y: f32,
    pub(super) advance: f32,
    /// 📐️ Device pixels per logical pixel THIS raster used — the shaped path honours the atlas
    /// scale, the fixed 8×16 ASCII bitmap fallback cannot and always reports `1.0`.
    pub(super) raster_scale: f32,
    pub(super) is_color: bool,
}

/// 🧭️ The font and glyph a single codepoint resolved to after running it through parley's
/// shaping/itemization (which performs the family + generic-emoji-fallback font selection).
pub(super) struct ResolvedGlyph {
    pub(super) glyph_id: swash::GlyphId,
    pub(super) advance: f32,
    pub(super) font_data: Blob<u8>,
    pub(super) font_index: u32,
}

pub struct FontAtlas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    /// 🌈️ RGBA emoji atlas page (2048×2048×4 in `Shaped` mode, empty in `Bitmap` mode since that
    /// mode never produces color glyphs).
    pub color_width: u32,
    pub color_height: u32,
    pub color_pixels: Vec<u8>,
    pub(super) mode: AtlasMode,
    pub(super) font_cx: FontContext,
    pub(super) layout_cx: LayoutContext<[u8; 4]>,
    pub(super) scale_cx: ScaleContext,
    pub(super) glyphs: HashMap<(TextFace, char, u32), GlyphEntry>,
    /// 📏️ Per EXACT device size (`f32` bits), the grid glyph with its advance at that size — derived, bounded by
    /// [`SIZED_GLYPH_CAPACITY`], rebuilt from `glyphs` on demand.
    pub(super) sized: HashMap<(TextFace, char, u32), GlyphEntry>,
    /// 🤝️ Pair kerning per face and scalar pair, in em — [`PairKerning::em`]'s answers, bounded by [`KERNING_PAIR_CAPACITY`].
    pub(super) kerning: HashMap<(TextFace, char, char), f32>,
    pub(super) pair_kerning: PairKerning,
    /// 📐️ Device pixels per logical pixel the atlas rasterises at — the ONLY place the surface
    /// scale factor reaches the text stack. Everything a caller passes in or reads back stays
    /// logical; see [`Self::set_raster_scale`].
    pub(super) raster_scale: f32,
    pub(super) cursor_x: u32,
    pub(super) cursor_y: u32,
    pub(super) row_height: u32,
    pub(super) color_cursor_x: u32,
    pub(super) color_cursor_y: u32,
    pub(super) color_row_height: u32,
    pub(super) dirty: bool,
    pub(super) color_dirty: bool,
}

/// 🗂️ Shapes only the embedded catalog faces and their explicitly registered emoji subsets.
fn empty_font_context() -> FontContext {
    FontContext { collection: Collection::new(CollectionOptions { shared: false, system_fonts: false }), source_cache: SourceCache::default() }
}

/// 📥️ Registers an original embedded font under its explicit catalog or private subset identity.
fn register_family(collection: &mut Collection, bytes: &[u8], family: &'static str) -> Option<FamilyId> {
    let over = FontInfoOverride { family_name: Some(family), ..Default::default() };
    collection.register_fonts(Blob::new(Arc::new(bytes.to_vec())), Some(over)).into_iter().next().map(|(id, _)| id)
}

impl FontAtlas {
    pub fn builtin() -> Self {
        Self {
            width: 2048,
            height: 2048,
            pixels: vec![0; 2048 * 2048],
            color_width: 0,
            color_height: 0,
            color_pixels: Vec::new(),
            mode: AtlasMode::Bitmap,
            font_cx: empty_font_context(),
            layout_cx: LayoutContext::new(),
            scale_cx: ScaleContext::new(),
            glyphs: HashMap::new(),
            sized: HashMap::new(),
            kerning: HashMap::new(),
            pair_kerning: PairKerning::default(),
            raster_scale: 1.0,
            cursor_x: 1,
            cursor_y: 1,
            row_height: 0,
            color_cursor_x: 1,
            color_cursor_y: 1,
            color_row_height: 0,
            dirty: false,
            color_dirty: false,
        }
    }

    pub fn take_dirty(&mut self) -> bool {
        let dirty = self.dirty;
        self.dirty = false;
        dirty
    }

    /// 🌈️ Same contract as `take_dirty` for the RGBA emoji atlas page (`color_pixels`). Wiring
    /// this into an actual GPU upload is a `gpu`-region call site left as a wiring request — see
    /// `GpuContext::upload_font_atlas`/`upload_emoji_atlas` in the report.
    pub fn take_color_dirty(&mut self) -> bool {
        let dirty = self.color_dirty;
        self.color_dirty = false;
        dirty
    }

    /// 🏗️ Builds a fully self-contained `Shaped`-mode atlas: registers the embedded Anta, Kelly
    /// Slab and Share Tech Mono families plus all 12 Noto Emoji fallback buckets, then wires
    /// `GenericFamily::Emoji`/`SansSerif`/`Serif`/`Monospace` to them. `primary_override`, when it
    /// parses as a real font, replaces the embedded Anta bytes as the `FAMILY_SANS` source (this
    /// is how `from_bytes` keeps honoring whatever bytes the host fetched) — falling back to the
    /// embedded copy keeps this constructor infallible even for garbage/empty-ish input.
    pub(super) fn shaped(primary_override: Option<&[u8]>) -> Self {
        let mut collection = Collection::new(CollectionOptions { shared: false, system_fonts: false });
        let sans_bytes: &[u8] = match primary_override {
            Some(bytes) if SwashFontRef::from_index(bytes, 0).is_some() => bytes,
            _ => ANTA_LATIN,
        };
        let sans_id = register_family(&mut collection, sans_bytes, FAMILY_SANS).or_else(|| register_family(&mut collection, ANTA_LATIN, FAMILY_SANS)).expect("embedded Anta font asset must register");
        let serif_id = register_family(&mut collection, KELLY_SLAB_LATIN, FAMILY_SERIF).expect("embedded Kelly Slab font asset must register");
        let mono_id = register_family(&mut collection, SHARE_TECH_MONO_LATIN, FAMILY_MONO).expect("embedded Share Tech Mono font asset must register");
        let emoji_ids=NOTO_EMOJI_BUCKETS.into_iter().zip(EMOJI_BUCKET_FAMILIES).filter_map(|(bytes,family)|register_family(&mut collection,bytes,family)).collect::<Vec<_>>();
        assert!(!emoji_ids.is_empty(),"embedded Noto Emoji sources must register");
        collection.set_generic_families(GenericFamily::Emoji,emoji_ids.into_iter());        collection.set_generic_families(GenericFamily::SansSerif, std::iter::once(sans_id));
        collection.set_generic_families(GenericFamily::Serif, std::iter::once(serif_id));
        collection.set_generic_families(GenericFamily::Monospace, std::iter::once(mono_id));
        Self {
            width: 2048,
            height: 2048,
            pixels: vec![0; 2048 * 2048],
            color_width: 2048,
            color_height: 2048,
            color_pixels: vec![0; 2048 * 2048 * 4],
            mode: AtlasMode::Shaped,
            font_cx: FontContext { collection, source_cache: SourceCache::default() },
            layout_cx: LayoutContext::new(),
            scale_cx: ScaleContext::new(),
            glyphs: HashMap::new(),
            sized: HashMap::new(),
            kerning: HashMap::new(),
            pair_kerning: PairKerning::default(),
            raster_scale: 1.0,
            cursor_x: 1,
            cursor_y: 1,
            row_height: 0,
            color_cursor_x: 1,
            color_cursor_y: 1,
            color_row_height: 0,
            dirty: false,
            color_dirty: false,
        }
    }

    /// 🔤️ The atlas every REAL host boots with: full `Shaped` mode over the embedded Anta /
    /// Kelly Slab / Share Tech Mono / Noto Emoji families, with no host-fetched override. Anta is
    /// exactly the face React's `--font-sans` names first
    /// (`🎨️styling/🎨️palette/🎨️.css`'s `--font-sans: Anta, …`), so a host that cannot fetch a font
    /// file still shapes the UI font at the reference's own advances instead of falling back to the
    /// fixed-pitch 8×16 ASCII debug bitmap [`Self::builtin`] carries — which paints every chrome
    /// label monospace at a flat 10 px advance and is why a frame worker booted from
    /// `from_bytes(&[])` could never match a measured chip width.
    pub fn shaped_default() -> Self {
        Self::shaped(None)
    }

    /// 🔡️ `bytes` empty ⇒ deterministic `builtin()` bitmap mode (unchanged contract). Any
    /// non-empty input — including bytes that fail to parse as a font — now resolves into full
    /// `Shaped` mode (registering the embedded Anta/Kelly Slab/Share Tech Mono/Noto Emoji
    /// families regardless), which is a strict improvement over the old fontdue-only pipeline's
    /// "garbage bytes ⇒ crude ASCII boxes" behavior.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.is_empty() {
            return Ok(Self::builtin());
        }
        Ok(Self::shaped(Some(bytes)))
    }

    /// 📐️ The surface scale factor the atlas rasterises at. Callers keep speaking logical pixels;
    /// only the swash raster size and the packed texel extents move. Changing it drops every cached
    /// glyph and rewinds both pages, so the next frame re-rasterises at the new density and the
    /// dirty flags force a full re-upload — this is what a window moved between a Retina and a
    /// non-Retina display must do. A no-op when the scale is unchanged or not a positive finite.
    pub fn set_raster_scale(&mut self, raster_scale: f32) {
        if !raster_scale.is_finite() || raster_scale <= 0.0 || (raster_scale - self.raster_scale).abs() < f32::EPSILON {
            return;
        }
        self.raster_scale = raster_scale;
        self.glyphs.clear();
        self.sized.clear();
        self.pixels.iter_mut().for_each(|texel| *texel = 0);
        self.color_pixels.iter_mut().for_each(|texel| *texel = 0);
        self.cursor_x = 1;
        self.cursor_y = 1;
        self.row_height = 0;
        self.color_cursor_x = 1;
        self.color_cursor_y = 1;
        self.color_row_height = 0;
        self.dirty = true;
        self.color_dirty = !self.color_pixels.is_empty();
    }

    /// 📐️ The scale the atlas currently rasterises at — device pixels per logical pixel.
    pub fn raster_scale(&self) -> f32 {
        self.raster_scale
    }

    /// 🔑️ The glyph-cache key of a DEVICE size: its position on the quarter-pixel raster grid (`RASTER_GRID_STEPS_PER_PX`
    /// steps per pixel), so float jitter never fragments the cache and a raster is never more than an eighth of a pixel
    /// off the size it paints. Only rasters snap: advances stay exact ([`GlyphEntry::advance_em`]).
    pub(super) fn quantize_size(size_px: f32) -> u32 {
        (size_px * RASTER_GRID_STEPS_PER_PX).round().max(1.0) as u32
    }

    /// 📐️ The device size one grid key rasterises at.
    pub(super) fn grid_px(key: u32) -> f32 {
        key as f32 / RASTER_GRID_STEPS_PER_PX
    }

}
// #endregion text
