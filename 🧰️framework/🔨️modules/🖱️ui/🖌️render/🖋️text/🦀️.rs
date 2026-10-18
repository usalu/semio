//! 🖋️ CPU-side text: font registration/fallback, a shaped-layout cache and the glyph atlas.
//!
//! Ported from `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📝️text/🦀️.rs` (604 lines,
//! read-only — that file lives inside the externally-red `wgpu-engine` feature and is not required to
//! compile). That file's own imports and shaping/rasterization call sequence (parley `RangedBuilder` →
//! `Layout` → `PositionedLayoutItem::GlyphRun` → swash `Render`) were already correct against the real
//! parley 0.5.0/swash 0.2.x/fontique 0.5.0 APIs (verified against the vendored crate sources, not
//! guessed) — the repo-wide asyncify pass had only sprinkled `async fn`/`.await` onto plain CPU-bound
//! functions with no suspension point, which is what this port strips. Everything else here is a
//! genuine architectural change, not a mechanical strip: whole-string shaping via `parley::Layout`
//! replaces the old per-character single-glyph `parley::Layout` build (real shaping, bidi and UAX#14
//! line breaking instead of a whitespace-split hack); `Measurement::{Ready,Pending,Failed}` (this
//! crate's own type, [`Measurement`]) replaces the old `(f32, f32)` tuple; atlas uploads
//! go through [`ResourceRegistry`] instead of raw `pixels`/`take_dirty` fields; cursor
//! movement and selection geometry are new, built on parley's own `Cursor`/`Selection`/`Cluster` API
//! (grapheme-cluster-safe by construction — see the `🔖️Measure` region docstring); and `fetch_font_bytes`
//! is gone (host I/O does not belong in this crate — see `FontSource` in `🔖️Font`).
//!
//! No `parley::`/`swash::`/`fontique::`/`peniko::` type appears in any `pub` signature in this file —
//! every such type is either wrapped (`FontHandle`, `GlyphEntry`, `ShapedGlyph`) or converted at the
//! boundary (`kurbo::Rect` → `[f32; 4]` in `selection_geometry`). This is what lets a future shaping
//! engine swap without touching a single call site outside this file.
//!
//! No font is compiled into this crate: the four built-in families arrive at run time as a
//! [`TextFontSet`] the host builds from the `🖼️assets/🔤️fonts` catalog (native hosts read it with
//! `TextFontFiles`; `wasm32` hosts fetch each [`TextFontRole::asset_path`] themselves).

use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

use parley::fontique::{Blob, Collection, CollectionOptions, FamilyId, FontInfoOverride, GenericFamily, SourceCache};
use parley::{Affinity, Alignment, AlignmentOptions, Cursor, FontContext, FontFamily, FontStack, FontWeight, Layout, LayoutContext, LineHeight, PositionedLayoutItem, Selection, StyleProperty};
use swash::scale::image::Content as SwashContent;
use swash::scale::{Render, ScaleContext, Source, StrikeWith};
use swash::zeno::Format as SwashFormat;
use swash::FontRef as SwashFontRef;

use semio_framework_ui_render_packet::{AtlasId, ResourceRegistry};

#[path = "🔤️font-roles-and-files/🦀️.rs"]
mod font_roles_and_files;
use font_roles_and_files::{FAMILY_MONO, FAMILY_SANS, FAMILY_SERIF};
pub use font_roles_and_files::{FontFamilyChoice, TextFontError, TextFontRole, TextFontSet};
#[cfg(not(target_arch="wasm32"))]
pub use font_roles_and_files::{TextFontFiles, TextFontReadError};

#[path = "🅰️text-styles-and-text-system/🦀️.rs"]
mod text_styles_and_text_system;
pub use text_styles_and_text_system::{
    FontDependencyId, FontFetch, FontHandle, FontSource, FontStatus, TextAlignment, TextRunStyle,
    TextStyle, TextSystem,
};

#[path = "🧶️shaping-and-glyph-atlas/🦀️.rs"]
mod shaping_and_glyph_atlas;
use shaping_and_glyph_atlas::{AtlasPage, GlyphCacheKey, ShapeCacheKey};
pub use shaping_and_glyph_atlas::{GlyphEntry, ShapedGlyph, ShapedText};

#[path = "📏️measurement-and-utf16/🦀️.rs"]
mod measurement_and_utf16;
pub use measurement_and_utf16::{utf16_to_utf8, utf8_to_utf16, Measurement};

//#region Tests

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../🧪️tests/🔬️text-unit/🦀️.rs"]
mod tests;

//#endregion Tests
