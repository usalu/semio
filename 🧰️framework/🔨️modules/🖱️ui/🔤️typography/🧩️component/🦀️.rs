// #region text
//! 🖋️ Glyph atlas — `parley` (shaping/font-fallback resolution) + `fontique` (font collection
//! and generic/emoji fallback registry) + `swash` (rasterization), packed into two atlas pages
//! (alpha-only `pixels` for regular glyphs, RGBA `color_pixels` for COLR/bitmap color-emoji
//! glyphs). A built-in 8×16 ASCII bitmap mode is kept as the deterministic, dependency-free
//! fallback used by `FontAtlas::builtin()` (relied on by many call sites across the crate for
//! fast/fixed-metric test setup) and by any single codepoint no registered font can shape at all.
//! See `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w1b-text-stack.md` for the full
//! architecture writeup, including the deliberate measurement/paint-consistency tradeoff that
//! keeps `measure_text`/`ensure_glyph` per-codepoint rather than switching to whole-string
//! `parley::Layout` metrics.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use parley::fontique::{Blob, Collection, CollectionOptions, FamilyId, FontInfoOverride, GenericFamily, SourceCache};
use parley::{FontContext, FontStack, LayoutContext, PositionedLayoutItem, StyleProperty};
use swash::scale::image::Content as SwashContent;
use swash::scale::{Render, ScaleContext, Source, StrikeWith};
#[cfg(test)]
use swash::shape::ShapeContext;
use swash::zeno::Format as SwashFormat;
use swash::zeno::{Cap as ZenoCap, Join as ZenoJoin, Mask as ZenoMask, Stroke as ZenoStroke, Style as ZenoStyle, Transform as ZenoTransform};
use swash::FontRef as SwashFontRef;
use super::font_pair_io::{BorrowedFontPairSource,FontPairQuery};

#[path = "🅰️glyph-entry-and-text-weight/🦀️.rs"]
mod glyph_entry_and_text_weight;
pub use glyph_entry_and_text_weight::{faux_bold_offset, GlyphEntry, TextWeight};

#[path = "🔠️text-faces-and-kerning/🦀️.rs"]
mod text_faces_and_kerning;
use text_faces_and_kerning::{
    BITMAP_GLYPH_H, BITMAP_GLYPH_W, EMOJI_BUCKET_FAMILIES, EMOJI_BUCKET_STACK, FAMILY_MONO,
    FAMILY_SANS, FAMILY_SERIF, KERNING_PAIR_CAPACITY, RASTER_GRID_STEPS_PER_PX,
    SIZED_GLYPH_CAPACITY,
};
pub use text_faces_and_kerning::{font_advance_em, PairKerning, TextFace, NOTO_EMOJI_BUCKETS};
#[cfg(test)]
use text_faces_and_kerning::authored_face_bytes;

#[path = "✂️line-breaking/🦀️.rs"]
mod line_breaking;
pub use line_breaking::{
    is_break_opportunity, is_wrap_space, may_break_between, unbreakable_run_end,
    LINE_BREAK_FIT_EPSILON,
};

#[path = "🔣️bitmap-and-symbol-faces/🦀️.rs"]
mod bitmap_and_symbol_faces;
use bitmap_and_symbol_faces::{
    is_zero_width_format_char, symbol_face_path, BITMAP_FONT, SYMBOL_EM_UNITS,
    SYMBOL_SIDE_BEARING_UNITS, SYMBOL_STROKE_UNITS,
};

#[path = "🗂️font-atlas-state-and-setup/🦀️.rs"]
mod font_atlas_state_and_setup;
use font_atlas_state_and_setup::{AtlasMode, RasterizedGlyph, ResolvedGlyph};
pub use font_atlas_state_and_setup::FontAtlas;

#[path = "🖼️glyph-rasterization/🦀️.rs"]
mod glyph_rasterization;

#[path = "📏️glyph-packing-measure-and-kerning/🦀️.rs"]
mod glyph_packing_measure_and_kerning;

#[path = "🧵️text-wrapping-and-font-fetch/🦀️.rs"]
mod text_wrapping_and_font_fetch;
pub use text_wrapping_and_font_fetch::fetch_font_bytes;

pub use semio_framework_font_anta_latin_regular::OUTLINE as ANTA_LATIN;
pub use semio_framework_font_kelly_slab_latin_regular::OUTLINE as KELLY_SLAB_LATIN;
pub use semio_framework_font_share_tech_mono_latin_regular::OUTLINE as SHARE_TECH_MONO_LATIN;

pub use semio_framework_ui_theme::metrics::line_height;

#[cfg(any(test, feature = "testkit"))]
#[path = "../../🧪️testing/🔤️text-reference/🦀️.rs"]
pub mod text_reference;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs"]
mod tests;
// #endregion text
