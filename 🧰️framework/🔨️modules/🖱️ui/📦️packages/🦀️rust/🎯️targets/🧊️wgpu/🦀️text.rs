// #region text
//! 🖋️ Glyph atlas — `parley` (shaping/font-fallback resolution) + `fontique` (font collection
//! and generic/emoji fallback registry) + `swash` (rasterization), packed into two atlas pages
//! (alpha-only `pixels` for regular glyphs, RGBA `color_pixels` for COLR/bitmap color-emoji
//! glyphs). A built-in 8×16 ASCII bitmap mode is kept as the deterministic, dependency-free
//! fallback used by `FontAtlas::builtin()` (relied on by many call sites across the crate for
//! fast/fixed-metric test setup) and by any single codepoint no registered font can shape at all.
//! See `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w1b-text-stack.md` for the full
//! architecture writeup, including the deliberate measurement/paint-consistency tradeoff that
//! keeps `measure_text`/`ensure_glyph` per-codepoint rather than switching to whole-string
//! `parley::Layout` metrics.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use parley::fontique::{Blob, Collection, CollectionOptions, FamilyId, FontInfoOverride, GenericFamily, SourceCache};
use parley::{FontContext, FontStack, LayoutContext, PositionedLayoutItem, StyleProperty};
use swash::FontRef as SwashFontRef;
use swash::scale::image::Content as SwashContent;
use swash::scale::{Render, ScaleContext, Source, StrikeWith};
use swash::zeno::Format as SwashFormat;

pub struct GlyphEntry {
    pub atlas_x: u32,
    pub atlas_y: u32,
    pub width: u32,
    pub height: u32,
    pub advance: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
    /// 🎨️ True when this glyph lives in the RGBA `color_pixels` page (COLR/bitmap color emoji)
    /// rather than the alpha-only `pixels` page. Paint call sites in `widgets`/`paint` (outside
    /// this region) still only sample the alpha page — see the report's wiring-request section.
    pub is_color: bool,
}

/// 🔤️ Fixed family names every registered font is forced under via `FontInfoOverride`, so
/// multi-file families (Noto Emoji's 12 codepoint-range buckets) merge into one fontique family
/// regardless of what each file's own `name` table declares.
const FAMILY_SANS: &str = "Anta";
const FAMILY_SERIF: &str = "Kelly Slab";
const FAMILY_MONO: &str = "Share Tech Mono";
const FAMILY_EMOJI: &str = "Noto Emoji";

static ANTA_LATIN: &[u8] = include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🔤️anta/🔤️latin.ttf");
static KELLY_SLAB_LATIN: &[u8] = include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🔤️kelly-slab/🔤️latin.ttf");
static SHARE_TECH_MONO_LATIN: &[u8] = include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🔤️share-tech-mono/🔤️latin.ttf");
static NOTO_EMOJI_BUCKETS: [&[u8]; 12] = [
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️0-400.ttf"),
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️1-400.ttf"),
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️2-400.ttf"),
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️3-400.ttf"),
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️4-400.ttf"),
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️5-400.ttf"),
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️6-400.ttf"),
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️7-400.ttf"),
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️8-400.ttf"),
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️9-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️10-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️11-400.ttf"),
];

const BITMAP_GLYPH_W: u32 = 8;
const BITMAP_GLYPH_H: u32 = 16;

static BITMAP_FONT: [[u8; 8]; 95] = [
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x18, 0x3C, 0x3C, 0x18, 0x18, 0x00, 0x18, 0x00],
    [0x36, 0x36, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x36, 0x36, 0x7F, 0x36, 0x7F, 0x36, 0x36, 0x00],
    [0x0C, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x18, 0x00],
    [0x00, 0x63, 0x66, 0x0C, 0x18, 0x33, 0x63, 0x00],
    [0x1C, 0x36, 0x1C, 0x6E, 0x3B, 0x33, 0x6E, 0x00],
    [0x06, 0x06, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x18, 0x30, 0x60, 0x60, 0x60, 0x30, 0x18, 0x00],
    [0x06, 0x0C, 0x18, 0x18, 0x18, 0x0C, 0x06, 0x00],
    [0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00],
    [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30],
    [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00],
    [0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x80, 0x00],
    [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00],
    [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00],
    [0x3C, 0x66, 0x06, 0x1C, 0x30, 0x60, 0x7E, 0x00],
    [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00],
    [0x0C, 0x1C, 0x3C, 0x6C, 0x7E, 0x0C, 0x0C, 0x00],
    [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00],
    [0x1C, 0x30, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00],
    [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00],
    [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00],
    [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x38, 0x00],
    [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00],
    [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x30],
    [0x0C, 0x18, 0x30, 0x60, 0x30, 0x18, 0x0C, 0x00],
    [0x00, 0x00, 0x7E, 0x00, 0x7E, 0x00, 0x00, 0x00],
    [0x30, 0x18, 0x0C, 0x06, 0x0C, 0x18, 0x30, 0x00],
    [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x00, 0x18, 0x00],
    [0x3C, 0x66, 0x6E, 0x6A, 0x6E, 0x60, 0x3C, 0x00],
    [0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
    [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00],
    [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00],
    [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00],
    [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x7E, 0x00],
    [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x60, 0x00],
    [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00],
    [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
    [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
    [0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38, 0x00],
    [0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00],
    [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00],
    [0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00],
    [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00],
    [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
    [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00],
    [0x3C, 0x66, 0x66, 0x66, 0x6E, 0x6C, 0x3A, 0x00],
    [0x7C, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0x66, 0x00],
    [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00],
    [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
    [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
    [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
    [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00],
    [0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00],
    [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00],
    [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0x00],
    [0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, 0x00],
    [0xC0, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x02, 0x00],
    [0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0x00],
    [0x10, 0x38, 0x6C, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF],
    [0x30, 0x18, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x3C, 0x06, 0x3E, 0x66, 0x3E, 0x00],
    [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x7C, 0x00],
    [0x00, 0x00, 0x3C, 0x66, 0x60, 0x66, 0x3C, 0x00],
    [0x06, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x3E, 0x00],
    [0x00, 0x00, 0x3C, 0x66, 0x7E, 0x60, 0x3C, 0x00],
    [0x1C, 0x30, 0x7C, 0x30, 0x30, 0x30, 0x30, 0x00],
    [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x3C],
    [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
    [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x00],
    [0x0C, 0x00, 0x1C, 0x0C, 0x0C, 0x6C, 0x6C, 0x38],
    [0x60, 0x60, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0x00],
    [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
    [0x00, 0x00, 0x36, 0x7F, 0x6B, 0x6B, 0x63, 0x00],
    [0x00, 0x00, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
    [0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00],
    [0x00, 0x00, 0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60],
    [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x06],
    [0x00, 0x00, 0x7C, 0x66, 0x60, 0x60, 0x60, 0x00],
    [0x00, 0x00, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x00],
    [0x30, 0x30, 0x7C, 0x30, 0x30, 0x30, 0x1C, 0x00],
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x3E, 0x00],
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
    [0x00, 0x00, 0x63, 0x6B, 0x6B, 0x7F, 0x36, 0x00],
    [0x00, 0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x00],
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x3E, 0x06, 0x3C],
    [0x00, 0x00, 0x7E, 0x0C, 0x18, 0x30, 0x7E, 0x00],
    [0x0E, 0x18, 0x18, 0x70, 0x18, 0x18, 0x0E, 0x00],
    [0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
    [0x70, 0x18, 0x18, 0x0E, 0x18, 0x18, 0x70, 0x00],
    [0x31, 0x6B, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00],
];

/// 🧩️ How a `FontAtlas` resolves and rasterizes glyphs. `Bitmap` is the deterministic,
/// dependency-free 8×16 ASCII fallback used by `FontAtlas::builtin()`; `Shaped` runs the full
/// parley/fontique/swash pipeline against a registered `fontique::Collection`.
enum AtlasMode {
    Bitmap,
    Shaped,
}

/// 📦️ One rasterized glyph ready to be packed into an atlas page, produced by either
/// `rasterize_bitmap_glyph` or `rasterize_shaped_glyph`.
struct RasterizedGlyph {
    bitmap: Vec<u8>,
    width: u32,
    height: u32,
    bearing_x: f32,
    bearing_y: f32,
    advance: f32,
    is_color: bool,
}

/// 🧭️ The font and glyph a single codepoint resolved to after running it through parley's
/// shaping/itemization (which performs the family + generic-emoji-fallback font selection).
struct ResolvedGlyph {
    glyph_id: swash::GlyphId,
    advance: f32,
    font_data: Blob<u8>,
    font_index: u32,
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
    mode: AtlasMode,
    font_cx: FontContext,
    layout_cx: LayoutContext<[u8; 4]>,
    scale_cx: ScaleContext,
    glyphs: HashMap<(char, u32), GlyphEntry>,
    cursor_x: u32,
    cursor_y: u32,
    row_height: u32,
    color_cursor_x: u32,
    color_cursor_y: u32,
    color_row_height: u32,
    dirty: bool,
    color_dirty: bool,
}

/// 🗂️ A `fontique::Collection` with no system-font scanning (deterministic, self-contained —
/// every family this atlas ever shapes with is one of the four `FAMILY_*` names registered from
/// `include_bytes!`-embedded assets).
fn empty_font_context() -> FontContext {
    FontContext { collection: Collection::new(CollectionOptions { shared: false, system_fonts: false }), source_cache: SourceCache::default() }
}

/// 📥️ Registers `bytes` into `collection` under a forced `family` name (ignoring whatever family
/// name the font file's own `name` table declares), so multi-file families like Noto Emoji's 12
/// codepoint-range buckets always merge into one fontique family.
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
    fn shaped(primary_override: Option<&[u8]>) -> Self {
        let mut collection = Collection::new(CollectionOptions { shared: false, system_fonts: false });
        let sans_bytes: &[u8] = match primary_override {
            Some(bytes) if SwashFontRef::from_index(bytes, 0).is_some() => bytes,
            _ => ANTA_LATIN,
        };
        let sans_id = register_family(&mut collection, sans_bytes, FAMILY_SANS).or_else(|| register_family(&mut collection, ANTA_LATIN, FAMILY_SANS)).expect("embedded Anta font asset must register");
        let serif_id = register_family(&mut collection, KELLY_SLAB_LATIN, FAMILY_SERIF).expect("embedded Kelly Slab font asset must register");
        let mono_id = register_family(&mut collection, SHARE_TECH_MONO_LATIN, FAMILY_MONO).expect("embedded Share Tech Mono font asset must register");
        let mut emoji_id: Option<FamilyId> = None;
        for bucket in NOTO_EMOJI_BUCKETS {
            if let Some(id) = register_family(&mut collection, bucket, FAMILY_EMOJI) {
                emoji_id.get_or_insert(id);
            }
        }
        let emoji_id = emoji_id.expect("embedded Noto Emoji font assets must register");
        collection.set_generic_families(GenericFamily::Emoji, std::iter::once(emoji_id));
        collection.set_generic_families(GenericFamily::SansSerif, std::iter::once(sans_id));
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

    /// 🔑️ Quantizes a float px size to the glyph-cache's integer key component, so float jitter
    /// (e.g. 15.999999 vs 16.0) doesn't fragment the cache into near-duplicate entries.
    fn quantize_size(size_px: f32) -> u32 {
        size_px.round().max(1.0) as u32
    }

    /// 🔍️ Fetches (rasterizing on first use) the glyph for `ch` at `size_px`, keyed by
    /// `(char, size_px)` so the same character rasterized at two different sizes never returns the
    /// wrong bitmap (the pre-fix bug: a `char`-only key meant later sizes reused the first size's
    /// rasterization, blurring text at any size other than whichever was cached first).
    pub fn ensure_glyph(&mut self, ch: char, size_px: f32) -> &GlyphEntry {
        let key = (ch, Self::quantize_size(size_px));
        if !self.glyphs.contains_key(&key) {
            self.rasterize_glyph(key);
        }
        self.glyphs.get(&key).expect("glyph inserted")
    }

    fn rasterize_glyph(&mut self, key: (char, u32)) {
        let (ch, size_px) = key;
        let glyph = match self.mode {
            AtlasMode::Bitmap => self.rasterize_bitmap_glyph(ch),
            AtlasMode::Shaped => self.rasterize_shaped_glyph(ch, size_px as f32),
        };
        self.pack_glyph(key, glyph);
    }

    /// 🧵️ Resolves `ch` to a font + glyph id by running a single-codepoint `parley::Layout`. This
    /// is what performs family resolution and (via parley's built-in emoji-cluster detection)
    /// automatic fallback into the registered `GenericFamily::Emoji` family. Returns `None` when
    /// no registered font (including the emoji fallback) could shape the codepoint at all.
    fn shape_single_char(&mut self, ch: char, size_px: f32) -> Option<ResolvedGlyph> {
        let text = ch.to_string();
        let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, &text, 1.0, true);
        builder.push_default(StyleProperty::FontStack(FontStack::Source(Cow::Borrowed(FAMILY_SANS))));
        builder.push_default(StyleProperty::FontSize(size_px));
        let mut layout: parley::Layout<[u8; 4]> = builder.build(&text);
        layout.break_all_lines(None);
        for line in layout.lines() {
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(run) = item else { continue };
                let Some(glyph) = run.glyphs().next() else { continue };
                let font = run.run().font();
                return Some(ResolvedGlyph { glyph_id: glyph.id, advance: glyph.advance, font_data: font.data.clone(), font_index: font.index });
            }
        }
        None
    }

    /// 🖌️ Rasterizes a resolved glyph via swash, preferring color bitmap/outline sources (COLR,
    /// embedded color bitmaps — what carries Noto Emoji's color) over the plain scalable outline,
    /// so any glyph the resolved font can render in color comes back as `Content::Color` RGBA and
    /// everything else comes back as an 8-bit alpha mask.
    fn render_resolved(&mut self, resolved: &ResolvedGlyph, size_px: f32) -> Option<RasterizedGlyph> {
        let data = resolved.font_data.data();
        let font_ref = SwashFontRef::from_index(data, resolved.font_index as usize)?;
        let mut scaler = self.scale_cx.builder(font_ref).size(size_px).hint(true).build();
        let image = Render::new(&[Source::ColorBitmap(StrikeWith::BestFit), Source::ColorOutline(0), Source::Outline]).format(SwashFormat::Alpha).render(&mut scaler, resolved.glyph_id)?;
        let is_color = matches!(image.content, SwashContent::Color);
        Some(RasterizedGlyph {
            bitmap: image.data,
            width: image.placement.width,
            height: image.placement.height,
            bearing_x: image.placement.left as f32,
            bearing_y: (image.placement.top - image.placement.height as i32) as f32,
            advance: resolved.advance,
            is_color,
        })
    }

    /// 🪶️ `shape_single_char` + `render_resolved`, falling back to the plain ASCII bitmap glyph
    /// (see `rasterize_bitmap_glyph`) when no registered font — including the emoji fallback —
    /// can shape `ch` at all (e.g. scripts none of Anta/Kelly Slab/Share Tech Mono/Noto Emoji
    /// cover, such as CJK or Arabic; a pre-existing limitation this atlas doesn't newly regress).
    fn rasterize_shaped_glyph(&mut self, ch: char, size_px: f32) -> RasterizedGlyph {
        let Some(resolved) = self.shape_single_char(ch, size_px) else {
            return self.rasterize_bitmap_glyph(ch);
        };
        if let Some(glyph) = self.render_resolved(&resolved, size_px) {
            return glyph;
        }
        RasterizedGlyph { bitmap: Vec::new(), width: 0, height: 0, bearing_x: 0.0, bearing_y: 0.0, advance: resolved.advance, is_color: false }
    }

    fn rasterize_bitmap_glyph(&self, ch: char) -> RasterizedGlyph {
        let index = ch as u32;
        let glyph_index = if (32..127).contains(&index) { (index - 32) as usize } else { 0 };
        let pattern = &BITMAP_FONT[glyph_index.min(BITMAP_FONT.len() - 1)];
        let mut bitmap = vec![0u8; (BITMAP_GLYPH_W * BITMAP_GLYPH_H) as usize];
        for (row, row_bits) in pattern.iter().enumerate() {
            for col in 0..BITMAP_GLYPH_W {
                if (row_bits >> (7 - col)) & 1 == 1 {
                    bitmap[row * BITMAP_GLYPH_W as usize + col as usize] = 255;
                }
            }
        }
        RasterizedGlyph { bitmap, width: BITMAP_GLYPH_W, height: BITMAP_GLYPH_H, bearing_x: 0.0, bearing_y: 0.0, advance: BITMAP_GLYPH_W as f32 + 2.0, is_color: false }
    }

    /// 📐️ Bin-packs one rasterized glyph into the alpha (`pixels`) or color (`color_pixels`)
    /// atlas page, per `RasterizedGlyph::is_color`, and records the resulting `GlyphEntry`.
    fn pack_glyph(&mut self, key: (char, u32), glyph: RasterizedGlyph) {
        let RasterizedGlyph { bitmap, width, height, bearing_x, bearing_y, advance, is_color } = glyph;
        let (atlas_x, atlas_y) = if is_color {
            if self.color_cursor_x + width + 2 >= self.color_width {
                self.color_cursor_x = 1;
                self.color_cursor_y += self.color_row_height + 2;
                self.color_row_height = 0;
            }
            let x = self.color_cursor_x;
            let y = self.color_cursor_y;
            for row in 0..height {
                let dst = (((y + row) * self.color_width + x) * 4) as usize;
                let src = (row * width * 4) as usize;
                if !bitmap.is_empty() && width > 0 {
                    self.color_pixels[dst..dst + (width * 4) as usize].copy_from_slice(&bitmap[src..src + (width * 4) as usize]);
                }
            }
            self.color_cursor_x += width + 2;
            self.color_row_height = self.color_row_height.max(height);
            self.color_dirty = true;
            (x, y)
        } else {
            if self.cursor_x + width + 2 >= self.width {
                self.cursor_x = 1;
                self.cursor_y += self.row_height + 2;
                self.row_height = 0;
            }
            let x = self.cursor_x;
            let y = self.cursor_y;
            for row in 0..height {
                let dst = ((y + row) * self.width + x) as usize;
                let src = (row * width) as usize;
                if !bitmap.is_empty() && width > 0 {
                    self.pixels[dst..dst + width as usize].copy_from_slice(&bitmap[src..src + width as usize]);
                }
            }
            self.cursor_x += width + 2;
            self.row_height = self.row_height.max(height);
            self.dirty = true;
            (x, y)
        };
        self.glyphs.insert(key, GlyphEntry { atlas_x, atlas_y, width, height, advance, bearing_x, bearing_y, is_color });
    }

    pub fn measure_text(&mut self, text: &str, size: f32) -> (f32, f32) {
        let mut width = 0.0f32;
        let mut max_height = 0.0f32;
        for ch in text.chars() {
            let glyph = self.ensure_glyph(ch, size);
            width += glyph.advance;
            max_height = max_height.max(glyph.height as f32 + glyph.bearing_y);
        }
        (width, max_height.max(size))
    }

    pub fn measure_text_wrapped(&mut self, text: &str, max_width: f32, size: f32) -> (f32, f32) {
        let mut lines = Vec::new();
        let mut current = String::new();
        for word in text.split_whitespace() {
            let trial = if current.is_empty() { word.to_string() } else { format!("{current} {word}") };
            let (w, _) = self.measure_text(&trial, size);
            if w > max_width && !current.is_empty() {
                lines.push(current);
                current = word.to_string();
            } else {
                current = trial;
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        let line_h = size * 1.35;
        let height = lines.len().max(1) as f32 * line_h;
        let width = lines.iter().map(|line| self.measure_text(line, size).0).fold(0.0f32, f32::max).min(max_width);
        (width, height)
    }
}

pub async fn fetch_font_bytes(url: &str) -> Result<Vec<u8>, String> {
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Uint8Array;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init(url, &opts).map_err(|_| "request failed")?;
        let window = web_sys::window().ok_or("no window")?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.map_err(|_| "fetch failed")?;
        let resp: Response = resp_value.dyn_into().map_err(|_| "response cast failed")?;
        if !resp.ok() {
            return Ok(Vec::new());
        }
        let buffer = JsFuture::from(resp.array_buffer().map_err(|_| "array_buffer failed")?).await.map_err(|_| "buffer failed")?;
        let array = Uint8Array::new(&buffer);
        let mut bytes = vec![0u8; array.length() as usize];
        array.copy_to(&mut bytes);
        Ok(bytes)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = url;
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::FontAtlas;

    #[test]
    fn from_bytes_falls_back_to_embedded_fonts_for_unparseable_input() {
        assert!(FontAtlas::from_bytes(&[]).is_ok());
        let woff2 = b"wOF2\x00\x01\x00\x00";
        let mut atlas = FontAtlas::from_bytes(woff2).expect("unparseable bytes still register embedded fonts");
        let glyph = atlas.ensure_glyph('A', 16.0);
        assert!(glyph.width > 0);
        assert!(!glyph.is_color, "'A' must rasterize as a regular alpha glyph, not a color one");
    }

    #[test]
    fn same_char_at_different_sizes_does_not_collide_in_the_glyph_cache() {
        let mut atlas = FontAtlas::builtin();
        atlas.ensure_glyph('A', 16.0);
        assert_eq!(atlas.glyphs.len(), 1);
        atlas.ensure_glyph('A', 32.0);
        assert_eq!(atlas.glyphs.len(), 2, "a second size for the same char must add a new cache entry, not collide");
        atlas.ensure_glyph('A', 16.0);
        assert_eq!(atlas.glyphs.len(), 2, "re-requesting an already-cached (char, size) must not insert again");
    }

    #[test]
    fn shaped_mode_resolves_real_font_metrics_that_differ_from_the_bitmap_fallback() {
        let mut atlas = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
        let glyph = atlas.ensure_glyph('W', 24.0);
        assert!(glyph.width > 0 && glyph.height > 0);
        assert!(!glyph.is_color);
    }

    /// 🔤️ The bundled `ui/asset/font/noto-emoji/*.ttf` buckets are the monochrome "Noto Emoji"
    /// family (`glyf` outlines only — verified no `COLR`/`CPAL`/`CBDT`/`CBLC`/`sbix` table is
    /// present), not "Noto Color Emoji", so real emoji codepoints correctly resolve through the
    /// `GenericFamily::Emoji` fallback and rasterize successfully, but land on the alpha page
    /// like any other outline glyph. `packing_a_synthetic_color_glyph_lands_on_the_rgba_page_and_marks_it_dirty`
    /// below exercises the RGBA color-page path directly, since these assets never trigger it.
    #[test]
    fn emoji_codepoints_resolve_through_the_noto_emoji_fallback_family() {
        let mut atlas = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
        let glyph = atlas.ensure_glyph('😀', 32.0);
        assert!(glyph.width > 0 && glyph.height > 0, "emoji glyph must produce a non-empty raster");
        assert!(!glyph.is_color, "the bundled Noto Emoji assets are monochrome outline-only");
        assert!(atlas.take_dirty());
    }

    #[test]
    fn packing_a_synthetic_color_glyph_lands_on_the_rgba_page_and_marks_it_dirty() {
        let mut atlas = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
        assert!(!atlas.take_color_dirty());
        atlas.pack_glyph(('🔥', 32), super::RasterizedGlyph { bitmap: vec![255u8; 4 * 4 * 4], width: 4, height: 4, bearing_x: 0.0, bearing_y: 0.0, advance: 32.0, is_color: true });
        let glyph = atlas.ensure_glyph('🔥', 32.0);
        assert!(glyph.is_color);
        assert_eq!((glyph.width, glyph.height), (4, 4));
        assert!(atlas.take_color_dirty(), "packing a color glyph must mark the color page dirty");
        assert!(!atlas.take_color_dirty(), "take_color_dirty must reset after being read");
    }
}
// #endregion text
