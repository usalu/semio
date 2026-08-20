//! @emoji 🖋️ CPU-side text: font registration/fallback, a shaped-layout cache and the glyph atlas.
//!
//! Ported from `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️text.rs` (604 lines,
//! read-only — that file lives inside the externally-red `wgpu-engine` feature and is not required to
//! compile). That file's own imports and shaping/rasterization call sequence (parley `RangedBuilder` →
//! `Layout` → `PositionedLayoutItem::GlyphRun` → swash `Render`) were already correct against the real
//! parley 0.5.0/swash 0.2.x/fontique 0.5.0 APIs (verified against the vendored crate sources, not
//! guessed) — the repo-wide asyncify pass had only sprinkled `async fn`/`.await` onto plain CPU-bound
//! functions with no suspension point, which is what this port strips. Everything else here is a
//! genuine architectural change, not a mechanical strip: whole-string shaping via `parley::Layout`
//! replaces the old per-character single-glyph `parley::Layout` build (real shaping, bidi and UAX#14
//! line breaking instead of a whitespace-split hack); `Measurement::{Ready,Pending,Failed}` (this
//! crate's own type, `crate::layout::Measurement`) replaces the old `(f32, f32)` tuple; atlas uploads
//! go through `crate::resource::ResourceRegistry` instead of raw `pixels`/`take_dirty` fields; cursor
//! movement and selection geometry are new, built on parley's own `Cursor`/`Selection`/`Cluster` API
//! (grapheme-cluster-safe by construction — see the `🔖️Measure` region docstring); and `fetch_font_bytes`
//! is gone (host I/O does not belong in this crate — see `FontSource` in `🔖️Font`).
//!
//! No `parley::`/`swash::`/`fontique::`/`peniko::` type appears in any `pub` signature in this file —
//! every such type is either wrapped (`FontHandle`, `GlyphEntry`, `ShapedGlyph`) or converted at the
//! boundary (`kurbo::Rect` → `[f32; 4]` in `selection_geometry`). This is what lets a future shaping
//! engine swap without touching a single call site outside this file.

use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

use parley::fontique::{Blob, Collection, CollectionOptions, FamilyId, FontInfoOverride, GenericFamily, SourceCache};
use parley::{Affinity, Cursor, FontContext, FontFamily, FontStack, Layout, LayoutContext, PositionedLayoutItem, Selection, StyleProperty};
use swash::scale::image::Content as SwashContent;
use swash::scale::{Render, ScaleContext, Source, StrikeWith};
use swash::zeno::Format as SwashFormat;
use swash::FontRef as SwashFontRef;

use crate::layout::Measurement;
use crate::resource::{AtlasId, ResourceRegistry};

//#region 🔖️Font

/// 🔤️ Fixed family names every built-in font is forced under via `FontInfoOverride`, so multi-file
/// families (Noto Emoji's 12 codepoint-range buckets) merge into one fontique family regardless of
/// what each file's own `name` table declares. Ported verbatim from the wgpu-old target.
const FAMILY_SANS: &str = "Anta";
const FAMILY_SERIF: &str = "Kelly Slab";
const FAMILY_MONO: &str = "Share Tech Mono";
const FAMILY_EMOJI: &str = "Noto Emoji";

static ANTA_LATIN: &[u8] = include_bytes!("../../../../🖼️assets/🔤️fonts/🔤️anta/🔤️latin.ttf");
static KELLY_SLAB_LATIN: &[u8] = include_bytes!("../../../../🖼️assets/🔤️fonts/🔤️kelly-slab/🔤️latin.ttf");
static SHARE_TECH_MONO_LATIN: &[u8] = include_bytes!("../../../../🖼️assets/🔤️fonts/🔤️share-tech-mono/🔤️latin.ttf");
static NOTO_EMOJI_BUCKETS: [&[u8]; 12] = [
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️0-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️1-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️2-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️3-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️4-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️5-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️6-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️7-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️8-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️9-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️10-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️11-400.ttf"),
];

/// 🧭️ Which family a [`TextStyle`] resolves to. `Custom` names a host-provided font requested via
/// [`TextSystem::request_font`] — never a `parley`/`fontique` type, so a caller never needs those
/// crates in scope just to pick a style.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontFamilyChoice {
    SansSerif,
    Serif,
    Monospace,
    Custom(FontDependencyId),
}

/// 🖋️ How to shape a run: family choice plus pixel size. Deliberately minimal — weight/style/features
/// are product-policy the caller resolves before reaching this crate, same posture `layout.rs` takes
/// on `Theme`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextStyle {
    pub family: FontFamilyChoice,
    pub size_px: f32,
}

/// 🔗️ A stable handle into [`TextSystem`]'s internal font table, carried by [`ShapedGlyph`] instead of
/// the raw `parley::Font`/`swash::FontRef` a real shape resolved to. Interning key is
/// `(fontique Blob::id(), font_index)` — `peniko`/`linebender_resource_handle`'s `Blob` already carries
/// a monotonic unique `id()` per allocation, so no content hashing or pointer arithmetic is needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontHandle(u32);

struct FontSlot {
    data: Blob<u8>,
    index: u32,
}

/// 🕓️ A host-requested font that has not necessarily loaded yet. `request_font` allocates one
/// synchronously (never blocking on the actual bytes); [`TextSystem::provide_font_bytes`] resolves it
/// once a host's [`FontSource`] impl finishes fetching.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontDependencyId(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontStatus {
    Pending,
    Ready,
    Failed,
}

struct CustomFontSlot {
    key: String,
    status: FontStatus,
}

/// 🌐️ The seam a future `ui-host` packet implements. Fetching font bytes is host I/O (network/disk) —
/// never this crate's concern under ruling U1, which is exactly why this trait has no async method:
/// `poll_font` is a synchronous, non-blocking status check the host calls from its own executor's poll
/// loop (or completion callback), and once it observes `FontFetch::Ready` it hands the bytes to
/// [`TextSystem::provide_font_bytes`] — a plain sync call, never awaited, never blocking a frame.
/// Generic per U3 (never `dyn FontSource`): a host picks its own transport/executor type and
/// monomorphizes over it, so this crate never names one. Unused within this crate itself — it exists
/// to be implemented outside it; see this packet's report for the registrar-request to wire it into
/// `ui-host` once that packet exists.
pub trait FontSource {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn poll_font(&mut self, key: &str) -> FontFetch;
}

/// 🌐️ One host font fetch's current state, as `FontSource::poll_font` reports it.
#[derive(Clone, Debug, PartialEq)]
pub enum FontFetch {
    Pending,
    Ready(Vec<u8>),
    Failed,
}

/// 🖋️ Font registration/fallback, a shaped-layout cache and the glyph atlas — the whole CPU text
/// pipeline for one window/surface. Every method is a plain sync `fn` (ruling U1): shaping and
/// rasterizing have no suspension point, so making them `async` would buy nothing and would cost the
/// frame its run-to-completion guarantee. A still-loading host font never blocks a call here — it
/// surfaces as [`Measurement::Pending`] with a placeholder metric (see `🔖️Measure`).
pub struct TextSystem {
    font_cx: FontContext,
    layout_cx: LayoutContext<[u8; 4]>,
    scale_cx: ScaleContext,

    custom_fonts: HashMap<String, FontDependencyId>,
    custom_font_slots: Vec<CustomFontSlot>,
    ready_dependencies: Vec<FontDependencyId>,

    font_table: Vec<FontSlot>,
    font_identity: HashMap<(u64, u32), FontHandle>,

    shape_cache: HashMap<ShapeCacheKey, ShapedText>,
    shape_hits: u32,
    shape_misses: u32,

    alpha_page: AtlasPage,
    color_page: AtlasPage,
    glyph_cache: HashMap<GlyphCacheKey, GlyphEntry>,
}

impl Default for TextSystem {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn default() -> Self {
        Self::new()
    }
}

impl TextSystem {
    /// 🏗️ Registers the four embedded families (Anta/Kelly Slab/Share Tech Mono/Noto Emoji) and wires
    /// `GenericFamily::{SansSerif,Serif,Monospace,Emoji}` to them — deterministic and dependency-free
    /// (no network, no system-font scan: `CollectionOptions { system_fonts: false, .. }`), same
    /// guarantee the wgpu-old target's `FontAtlas::builtin()`/`shaped()` merge gave. Infallible: the
    /// embedded assets are checked into the repo, so registration failing here is a build-time asset
    /// defect, not a runtime condition — hence the `expect`s.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        let mut collection = Collection::new(CollectionOptions { shared: false, system_fonts: false });
        let sans_id = Self::register_embedded(&mut collection, ANTA_LATIN, FAMILY_SANS).expect("embedded Anta font asset must register");
        let serif_id = Self::register_embedded(&mut collection, KELLY_SLAB_LATIN, FAMILY_SERIF).expect("embedded Kelly Slab font asset must register");
        let mono_id = Self::register_embedded(&mut collection, SHARE_TECH_MONO_LATIN, FAMILY_MONO).expect("embedded Share Tech Mono font asset must register");
        let mut emoji_id: Option<FamilyId> = None;
        for bucket in NOTO_EMOJI_BUCKETS {
            if let Some(id) = Self::register_embedded(&mut collection, bucket, FAMILY_EMOJI) {
                emoji_id.get_or_insert(id);
            }
        }
        let emoji_id = emoji_id.expect("embedded Noto Emoji font assets must register");
        collection.set_generic_families(GenericFamily::Emoji, std::iter::once(emoji_id));
        collection.set_generic_families(GenericFamily::SansSerif, std::iter::once(sans_id));
        collection.set_generic_families(GenericFamily::Serif, std::iter::once(serif_id));
        collection.set_generic_families(GenericFamily::Monospace, std::iter::once(mono_id));
        Self {
            font_cx: FontContext { collection, source_cache: SourceCache::default() },
            layout_cx: LayoutContext::new(),
            scale_cx: ScaleContext::new(),
            custom_fonts: HashMap::new(),
            custom_font_slots: Vec::new(),
            ready_dependencies: Vec::new(),
            font_table: Vec::new(),
            font_identity: HashMap::new(),
            shape_cache: HashMap::new(),
            shape_hits: 0,
            shape_misses: 0,
            alpha_page: AtlasPage::new("ui-text-glyph-atlas-alpha", 1),
            color_page: AtlasPage::new("ui-text-glyph-atlas-color", 4),
            glyph_cache: HashMap::new(),
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn register_embedded(collection: &mut Collection, bytes: &[u8], family: &'static str) -> Option<FamilyId> {
        let over = FontInfoOverride { family_name: Some(family), width: None, style: None, weight: None, axes: None };
        collection.register_fonts(Blob::new(Arc::new(bytes.to_vec())), Some(over)).into_iter().next().map(|(id, _)| id)
    }

    /// 🔗️ Interns `(font, glyph_id)`'s font identity to a stable [`FontHandle`], allocating on first
    /// sight. `Blob::id()` is already a unique monotonic id per allocation, so `(id, index)` is a
    /// correct identity key without hashing font bytes or comparing pointers.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn intern_font(&mut self, data: Blob<u8>, index: u32) -> FontHandle {
        let key = (data.id(), index);
        if let Some(handle) = self.font_identity.get(&key) {
            return *handle;
        }
        let handle = FontHandle(self.font_table.len() as u32);
        self.font_table.push(FontSlot { data, index });
        self.font_identity.insert(key, handle);
        handle
    }

    /// 🕓️ Idempotent: requesting the same `key` twice returns the same [`FontDependencyId`], same
    /// interning idiom `ResourceRegistry::intern_texture` uses for texture keys. Never blocks — the
    /// actual bytes arrive later via [`Self::provide_font_bytes`], driven by a host's [`FontSource`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn request_font(&mut self, key: &str) -> FontDependencyId {
        if let Some(&id) = self.custom_fonts.get(key) {
            return id;
        }
        let id = FontDependencyId(self.custom_font_slots.len() as u32);
        self.custom_font_slots.push(CustomFontSlot { key: key.to_string(), status: FontStatus::Pending });
        self.custom_fonts.insert(key.to_string(), id);
        id
    }

    /// 📥️ Registers `bytes` under `id`'s requested key and marks it `Ready`, forcing the family name
    /// exactly like [`Self::register_embedded`]. Returns `false` (no-op) for an unknown id, an already
    /// -resolved id, or empty/unparseable bytes (marked `Failed` in the last case) — never panics on a
    /// bad host response. On success, `id` is queued for [`Self::take_ready_dependencies`] so whoever
    /// owns this window's `FrameScheduler` can invalidate it with `InvalidationReason::RESOURCE_READY`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn provide_font_bytes(&mut self, id: FontDependencyId, bytes: &[u8]) -> bool {
        let Some(slot) = self.custom_font_slots.get(id.0 as usize) else { return false };
        if slot.status == FontStatus::Ready || bytes.is_empty() {
            if bytes.is_empty() {
                if let Some(slot) = self.custom_font_slots.get_mut(id.0 as usize) {
                    slot.status = FontStatus::Failed;
                }
            }
            return false;
        }
        let key = slot.key.clone();
        let over = FontInfoOverride { family_name: Some(&key), width: None, style: None, weight: None, axes: None };
        let registered = self.font_cx.collection.register_fonts(Blob::new(Arc::new(bytes.to_vec())), Some(over));
        let Some(slot) = self.custom_font_slots.get_mut(id.0 as usize) else { return false };
        if registered.is_empty() {
            slot.status = FontStatus::Failed;
            return false;
        }
        slot.status = FontStatus::Ready;
        self.ready_dependencies.push(id);
        true
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn font_status(&self, id: FontDependencyId) -> FontStatus {
        self.custom_font_slots.get(id.0 as usize).map_or(FontStatus::Failed, |slot| slot.status)
    }

    /// 🔔️ Drains the dependencies that resolved since the last drain — the invalidation half of
    /// `Measurement::Pending`'s contract (`layout.rs`'s own docstring: "whoever completes that
    /// dependency ... is responsible for invalidating the window once it lands"). Same take-and-reset
    /// idiom as `ResourceRegistry::drain_ops`/the old `FontAtlas::take_dirty`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn take_ready_dependencies(&mut self) -> Vec<FontDependencyId> {
        std::mem::take(&mut self.ready_dependencies)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn family_ready(&self, family: FontFamilyChoice) -> bool {
        match family {
            FontFamilyChoice::Custom(id) => matches!(self.custom_font_slots.get(id.0 as usize).map(|slot| slot.status), Some(FontStatus::Ready)),
            _ => true,
        }
    }

    /// 🧭️ Resolves a [`TextStyle`] to the `FontStack` a shape actually uses. A still-pending or failed
    /// custom font falls back to `FAMILY_SANS` here — this is the paint-phase fallback; the
    /// layout-phase `Measurement::Pending` short-circuit in `🔖️Measure` is what actually keeps a
    /// pending font from being measured with the wrong metrics.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn font_stack_for(&self, family: FontFamilyChoice) -> FontStack<'static> {
        match family {
            FontFamilyChoice::SansSerif => FontStack::Source(Cow::Borrowed(FAMILY_SANS)),
            FontFamilyChoice::Serif => FontStack::Source(Cow::Borrowed(FAMILY_SERIF)),
            FontFamilyChoice::Monospace => FontStack::Source(Cow::Borrowed(FAMILY_MONO)),
            FontFamilyChoice::Custom(id) => match self.custom_font_slots.get(id.0 as usize) {
                Some(slot) if slot.status == FontStatus::Ready => FontStack::List(Cow::Owned(vec![FontFamily::Named(Cow::Owned(slot.key.clone())), FontFamily::Generic(GenericFamily::SansSerif)])),
                _ => FontStack::Source(Cow::Borrowed(FAMILY_SANS)),
            },
        }
    }
}

//#endregion 🔖️Font

//#region 🔖️Shape

/// 🔑️ Cache key for [`TextSystem::shape`]. `size_bits`/`f32::to_bits` because `f32` is neither `Eq`
/// nor `Hash` — same trick `layout.rs`'s callers use for float-keyed measure caching elsewhere in this
/// program.
#[derive(Clone, PartialEq, Eq, Hash)]
struct ShapeCacheKey {
    text: String,
    family: FontFamilyChoice,
    size_bits: u32,
}

/// 🅰️ One positioned glyph from a shaped run. `font`/`glyph_id` identify what to rasterize (via
/// [`TextSystem::ensure_glyph`]); `x`/`y` are already fully positioned within the whole shape's
/// coordinate space (parley bakes line offset and baseline into `positioned_glyphs`) — a painter adds
/// only the run's own origin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShapedGlyph {
    pub font: FontHandle,
    pub glyph_id: u16,
    pub x: f32,
    pub y: f32,
    pub advance: f32,
}

/// 📐️ The result of shaping one run: positioned glyphs plus the aggregate metrics `layout.rs`'s
/// `Measurement::Ready` needs. `is_rtl` reflects the Unicode Bidi Algorithm's resolved paragraph
/// direction — parley resolves this automatically (no explicit direction override exists in its public
/// API in this version; see this packet's report).
#[derive(Clone, Debug, PartialEq)]
pub struct ShapedText {
    pub glyphs: Vec<ShapedGlyph>,
    pub width: f32,
    pub height: f32,
    pub ascent: f32,
    pub descent: f32,
    pub is_rtl: bool,
}

impl TextSystem {
    /// 🧱️ Builds a real `parley::Layout` for `text`/`style`, breaking lines unwrapped (`max_width:
    /// None`) or at `max_width`. Private — every public shaping/measuring/cursor/selection method below
    /// funnels through this one builder instead of five copies of the same `RangedBuilder` setup.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn layout_for(&mut self, text: &str, style: &TextStyle, max_width: Option<f32>) -> Layout<[u8; 4]> {
        let stack = self.font_stack_for(style.family);
        let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, text, 1.0, true);
        builder.push_default(StyleProperty::FontStack(stack));
        builder.push_default(StyleProperty::FontSize(style.size_px));
        let mut layout: Layout<[u8; 4]> = builder.build(text);
        layout.break_all_lines(max_width);
        layout
    }

    /// 🪶️ Shapes `text`/`style` (unwrapped) and caches the result, keyed by `(text, family, size)` —
    /// re-shaping the same run is a cache hit, never redone work. `Measurement`'s cache (see
    /// `🔖️Measure::measure`) is this same cache: `measure` calls `shape` internally, so "measure the
    /// same string twice" and "shape the same string twice" are the identical cache, not two.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn shape(&mut self, text: &str, style: &TextStyle) -> &ShapedText {
        let key = ShapeCacheKey { text: text.to_string(), family: style.family, size_bits: style.size_px.to_bits() };
        if self.shape_cache.contains_key(&key) {
            self.shape_hits += 1;
        } else {
            let shaped = self.shape_uncached(text, style);
            self.shape_cache.insert(key.clone(), shaped);
            self.shape_misses += 1;
        }
        self.shape_cache.get(&key).expect("shape cache entry just ensured present")
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn shape_uncached(&mut self, text: &str, style: &TextStyle) -> ShapedText {
        let layout = self.layout_for(text, style, None);
        let width = layout.width();
        let height = layout.height();
        let is_rtl = layout.is_rtl();
        let mut ascent = 0.0f32;
        let mut descent = 0.0f32;
        let mut glyphs = Vec::new();
        for line in layout.lines() {
            let metrics = line.metrics();
            ascent = ascent.max(metrics.ascent);
            descent = descent.max(metrics.descent);
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(run) = item else { continue };
                let font = run.run().font();
                let font_data = font.data.clone();
                let font_index = font.index;
                let handle = self.intern_font(font_data, font_index);
                for glyph in run.positioned_glyphs() {
                    glyphs.push(ShapedGlyph { font: handle, glyph_id: glyph.id, x: glyph.x, y: glyph.y, advance: glyph.advance });
                }
            }
        }
        ShapedText { glyphs, width, height, ascent, descent, is_rtl }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn shape_cache_hits(&self) -> u32 {
        self.shape_hits
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn shape_cache_misses(&self) -> u32 {
        self.shape_misses
    }
}

//#endregion 🔖️Shape

//#region 🔖️Atlas

/// 🅰️ One rasterized glyph's atlas placement: origin, size and bearings a painter needs to position the
/// glyph quad, plus which page it landed on. UV coordinates are deliberately NOT stored here — call
/// [`Self::uv_rect`] with the atlas's *current* dimensions at paint time. This is what makes atlas
/// growth (see [`AtlasPage::place`]) safe without invalidating any previously returned `GlyphEntry`:
/// frames are rebuilt from scratch every frame (this crate's whole element model — see `element.rs`'s
/// own docstring), so paint always recomputes UV against whatever the atlas's dimensions are *this*
/// frame, never a value cached from an earlier one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlyphEntry {
    pub atlas_x: u32,
    pub atlas_y: u32,
    pub width: u32,
    pub height: u32,
    pub bearing_x: f32,
    pub bearing_y: f32,
    pub is_color: bool,
}

impl GlyphEntry {
    /// 📐️ `[u0, v0, u1, v1]` normalized against `atlas_width`/`atlas_height` — pass
    /// [`TextSystem::atlas_dimensions`]/[`TextSystem::color_atlas_dimensions`] (matching
    /// [`Self::is_color`]) here, read fresh every paint call, never cached.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn uv_rect(&self, atlas_width: u32, atlas_height: u32) -> [f32; 4] {
        let w = atlas_width as f32;
        let h = atlas_height as f32;
        [self.atlas_x as f32 / w, self.atlas_y as f32 / h, (self.atlas_x + self.width) as f32 / w, (self.atlas_y + self.height) as f32 / h]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GlyphCacheKey {
    font: FontHandle,
    glyph_id: u16,
    size_bucket: u32,
}

struct RasterizedGlyph {
    bitmap: Vec<u8>,
    width: u32,
    height: u32,
    bearing_x: f32,
    bearing_y: f32,
    is_color: bool,
}

impl RasterizedGlyph {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn empty() -> Self {
        Self { bitmap: Vec::new(), width: 0, height: 0, bearing_x: 0.0, bearing_y: 0.0, is_color: false }
    }
}

/// 🗺️ One bin-packed atlas image (the alpha page or the RGBA color-emoji page). Row-cursor packer,
/// ported from the wgpu-old target's `FontAtlas::pack_glyph`. Growth is height-only doubling: `width`
/// (and therefore every existing glyph's `atlas_x`/row byte stride) never changes, so `Vec::resize`
/// simply appends zeroed rows — no re-blit, and every previously packed glyph's `(atlas_x, atlas_y)`
/// stays valid. See [`Self::queue_upload`] for why growth still needs a fresh [`AtlasId`] anyway.
struct AtlasPage {
    key: &'static str,
    channels: u32,
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    cursor_x: u32,
    cursor_y: u32,
    row_height: u32,
    generation: u32,
    id: Option<AtlasId>,
}

impl AtlasPage {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn new(key: &'static str, channels: u32) -> Self {
        let width = 2048;
        let height = 2048;
        Self { key, channels, width, height, pixels: vec![0; (width * height * channels) as usize], cursor_x: 1, cursor_y: 1, row_height: 0, generation: 0, id: None }
    }

    /// 📐️ Bin-packs `w`×`h` into the current row, wrapping and growing as needed, blits `bitmap`, and
    /// returns the placement. Never panics on a pathological glyph wider than the whole page — worst
    /// case it silently clips via [`Self::blit`]'s own bounds checks, which real UI glyph sizes never
    /// approach.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn place(&mut self, w: u32, h: u32, bitmap: &[u8]) -> (u32, u32) {
        if self.cursor_x + w + 2 > self.width {
            self.cursor_x = 1;
            self.cursor_y += self.row_height + 2;
            self.row_height = 0;
        }
        if self.cursor_y + h + 2 > self.height {
            self.grow();
        }
        let x = self.cursor_x;
        let y = self.cursor_y;
        self.blit(x, y, w, h, bitmap);
        self.cursor_x += w + 2;
        self.row_height = self.row_height.max(h);
        (x, y)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn blit(&mut self, x: u32, y: u32, w: u32, h: u32, bitmap: &[u8]) {
        if bitmap.is_empty() || w == 0 {
            return;
        }
        let stride = (self.width * self.channels) as usize;
        let row_bytes = (w * self.channels) as usize;
        for row in 0..h {
            let dst = (y + row) as usize * stride + x as usize * self.channels as usize;
            let src = row as usize * row_bytes;
            if dst + row_bytes <= self.pixels.len() && src + row_bytes <= bitmap.len() {
                self.pixels[dst..dst + row_bytes].copy_from_slice(&bitmap[src..src + row_bytes]);
            }
        }
    }

    /// 📏️ Height-only doubling — see this struct's own docstring for why this never invalidates an
    /// already-returned [`GlyphEntry`]'s coordinates.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn grow(&mut self) {
        let stride = (self.width * self.channels) as usize;
        self.height *= 2;
        self.pixels.resize(self.height as usize * stride, 0);
        self.generation += 1;
    }

    /// 📤️ Queues the whole current page as one `ResourceOp::UploadAtlas` via `resources`, versioning
    /// the interned key by `generation`. **This is the repack-invalidation answer the packet brief
    /// asks for:** `ResourceRegistry::request_atlas_upload` only re-queues an upload for an id that is
    /// not yet `Resident` (`resource.rs`'s own coalescing rule) — so if a backend already marked this
    /// page's *previous*-generation id `Resident` before a growth event, a same-key re-request would
    /// silently no-op and the grown page would never reach the device. Versioning the key by
    /// `generation` sidesteps that gap entirely without touching `resource.rs` (out of this packet's
    /// OWNS list): a grown page always resolves to a brand-new `AtlasId` the registry has never seen,
    /// so the upload is unconditionally queued. The stale previous-generation id is simply abandoned —
    /// nothing in this crate retains it (`Self::id` is overwritten below), and no frame references it
    /// past the one that triggered the growth, since frames are rebuilt from scratch every frame.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn queue_upload(&mut self, resources: &mut ResourceRegistry) {
        let key = format!("{}:{}", self.key, self.generation);
        self.id = Some(resources.request_atlas_upload(&key, self.width, self.height, self.pixels.clone()));
    }
}

impl TextSystem {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn quantize_size(size_px: f32) -> u32 {
        size_px.round().max(1.0) as u32
    }

    /// 🔍️ Fetches (rasterizing and bin-packing on first use) `(font, glyph_id)`'s atlas placement at
    /// `size_px`. A cache hit touches `resources` not at all — no `ResourceOp` is queued for an
    /// already-packed glyph, which is what keeps "a second identical glyph produces none" true.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn ensure_glyph(&mut self, resources: &mut ResourceRegistry, font: FontHandle, glyph_id: u16, size_px: f32) -> GlyphEntry {
        let key = GlyphCacheKey { font, glyph_id, size_bucket: Self::quantize_size(size_px) };
        if let Some(entry) = self.glyph_cache.get(&key) {
            return *entry;
        }
        let rasterized = self.rasterize(font, glyph_id, size_px);
        let page = if rasterized.is_color { &mut self.color_page } else { &mut self.alpha_page };
        let (atlas_x, atlas_y) = page.place(rasterized.width, rasterized.height, &rasterized.bitmap);
        let entry = GlyphEntry { atlas_x, atlas_y, width: rasterized.width, height: rasterized.height, bearing_x: rasterized.bearing_x, bearing_y: rasterized.bearing_y, is_color: rasterized.is_color };
        page.queue_upload(resources);
        self.glyph_cache.insert(key, entry);
        entry
    }

    /// 🖌️ Rasterizes via swash, preferring color bitmap/outline sources (COLR, embedded color bitmaps)
    /// over the plain scalable outline, so any glyph the resolved font can render in color comes back
    /// `Content::Color`; everything else is an 8-bit alpha mask. Ported verbatim from the wgpu-old
    /// target's `render_resolved`. Never panics on an unresolvable font/glyph — returns
    /// [`RasterizedGlyph::empty`], which still caches (so a broken glyph is not re-attempted every call).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn rasterize(&mut self, font: FontHandle, glyph_id: u16, size_px: f32) -> RasterizedGlyph {
        let Some(slot) = self.font_table.get(font.0 as usize) else { return RasterizedGlyph::empty() };
        let data = slot.data.data();
        let Some(font_ref) = SwashFontRef::from_index(data, slot.index as usize) else { return RasterizedGlyph::empty() };
        let mut scaler = self.scale_cx.builder(font_ref).size(size_px).hint(true).build();
        let Some(image) = Render::new(&[Source::ColorBitmap(StrikeWith::BestFit), Source::ColorOutline(0), Source::Outline]).format(SwashFormat::Alpha).render(&mut scaler, glyph_id) else {
            return RasterizedGlyph::empty();
        };
        let is_color = matches!(image.content, SwashContent::Color);
        RasterizedGlyph { bitmap: image.data, width: image.placement.width, height: image.placement.height, bearing_x: image.placement.left as f32, bearing_y: (image.placement.top - image.placement.height as i32) as f32, is_color }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn atlas_id(&self) -> Option<AtlasId> {
        self.alpha_page.id
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn color_atlas_id(&self) -> Option<AtlasId> {
        self.color_page.id
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn atlas_dimensions(&self) -> (u32, u32) {
        (self.alpha_page.width, self.alpha_page.height)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn color_atlas_dimensions(&self) -> (u32, u32) {
        (self.color_page.width, self.color_page.height)
    }
}

//#endregion 🔖️Atlas

//#region 🔖️Measure

/// 🧮️ `measure`/`measure_wrapped`/`wrap` are the layout-phase surface `layout.rs`'s `MeasureFn` calls
/// into. `next_grapheme`/`previous_grapheme`/`selection_geometry` are the input/edit-phase surface —
/// all three funnel through `parley::Cursor`/`Selection`/`Cluster`, which are grapheme-cluster-safe by
/// construction: per swash's own module docs, a shaping cluster is "equivalent to Unicode grapheme
/// clusters" for the vast majority of scripts, and a complex-script cluster that spans multiple
/// graphemes is still a *safe* (never mid-grapheme) place to land, matching what a real text editor
/// does for those scripts anyway. This is what lets this crate ship grapheme-aware cursor movement
/// without adding a `unicode-segmentation` dependency this packet is forbidden from introducing.
impl TextSystem {
    /// 📏️ Unwrapped intrinsic size. Short-circuits to `Measurement::Pending` without shaping at all
    /// when `style.family` names a [`FontDependencyId`] that has not resolved yet — the placeholder is
    /// a crude `chars × size_px × 0.6` estimate, deliberately not a real shape with the wrong font.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn measure(&mut self, text: &str, style: &TextStyle) -> Measurement {
        if !self.family_ready(style.family) {
            return Self::placeholder_measurement(text, style.size_px);
        }
        let shaped = self.shape(text, style);
        Measurement::Ready { width: shaped.width, height: shaped.height }
    }

    /// 📏️ Wrapped bounding size at `max_width`. Uncached (unlike [`Self::measure`]/[`Self::shape`]) —
    /// wrapping is keyed on a float `max_width` a caller typically varies continuously during resize,
    /// which would fragment a cache rather than serve it.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn measure_wrapped(&mut self, text: &str, style: &TextStyle, max_width: f32) -> Measurement {
        if !self.family_ready(style.family) {
            return Self::placeholder_measurement(text, style.size_px);
        }
        let layout = self.layout_for(text, style, Some(max_width));
        Measurement::Ready { width: layout.width(), height: layout.height() }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn placeholder_measurement(text: &str, size_px: f32) -> Measurement {
        let placeholder_width = text.chars().count() as f32 * size_px * 0.6;
        let placeholder_height = size_px * 1.35;
        Measurement::Pending { placeholder_width, placeholder_height }
    }

    /// ✂️ Line byte-ranges at `max_width`, via parley's own UAX#14 line breaking — never mid-cluster,
    /// unlike the wgpu-old target's whitespace-split `wrap_text`. Returns byte ranges into `text`
    /// rather than owned `String` copies (the old API's shape): a caller that wants owned lines slices
    /// `text` itself, and a caller building glyph runs per line avoids a copy entirely.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn wrap(&mut self, text: &str, style: &TextStyle, max_width: f32) -> Vec<Range<usize>> {
        let layout = self.layout_for(text, style, Some(max_width));
        let mut lines: Vec<Range<usize>> = layout.lines().map(|line| line.text_range()).collect();
        if lines.is_empty() {
            lines.push(0..0);
        }
        lines
    }

    /// ➡️ The next cursor-safe byte offset after `byte_index`, in visual (bidi-correct) order —
    /// `parley::Cursor::next_visual`, which is what arrow-key movement should use in mixed-direction
    /// text. Returns `byte_index` itself at the end of `text` (parley's own `Cursor` clamps rather than
    /// panicking).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn next_grapheme(&mut self, text: &str, style: &TextStyle, byte_index: usize) -> usize {
        let layout = self.layout_for(text, style, None);
        Cursor::from_byte_index(&layout, byte_index, Affinity::Downstream).next_visual(&layout).index()
    }

    /// ⬅️ The previous cursor-safe byte offset before `byte_index` — see [`Self::next_grapheme`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn previous_grapheme(&mut self, text: &str, style: &TextStyle, byte_index: usize) -> usize {
        let layout = self.layout_for(text, style, None);
        Cursor::from_byte_index(&layout, byte_index, Affinity::Downstream).previous_visual(&layout).index()
    }

    /// 🖍️ Highlight rectangles for `range`, multi-line-safe, via `parley::Selection::geometry`. Returns
    /// `[x, y, width, height]` — `peniko::kurbo::Rect` never crosses this file's public boundary.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn selection_geometry(&mut self, text: &str, style: &TextStyle, range: Range<usize>) -> Vec<[f32; 4]> {
        let layout = self.layout_for(text, style, None);
        let anchor = Cursor::from_byte_index(&layout, range.start, Affinity::Downstream);
        let focus = Cursor::from_byte_index(&layout, range.end, Affinity::Downstream);
        Selection::new(anchor, focus).geometry(&layout).into_iter().map(|(rect, _line)| [rect.x0 as f32, rect.y0 as f32, (rect.x1 - rect.x0) as f32, (rect.y1 - rect.y0) as f32]).collect()
    }
}

/// 🔡️ UTF-8 byte index → UTF-16 code unit index, for platform (IME/accessibility) boundaries. `text`
/// must be sliced at `byte_index`, a char boundary — every caller in this crate reaches one only via
/// `next_grapheme`/`previous_grapheme`/`Cluster::text_range`, all of which only ever produce char
/// boundaries, so the `unwrap_or_else` fallback (treat an invalid index as "end of string") is a
/// defensive floor, not an expected path.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn utf8_to_utf16(text: &str, byte_index: usize) -> u32 {
    match text.get(..byte_index) {
        Some(prefix) => prefix.encode_utf16().count() as u32,
        None => text.encode_utf16().count() as u32,
    }
}

/// 🔡️ UTF-16 code unit index → UTF-8 byte index — the inverse of [`utf8_to_utf16`]. A non-BMP
/// character (e.g. most emoji) is 2 UTF-16 units and 1 `char`; `ch.len_utf16()` accounts for that so
/// the round trip lands exactly on the character's start, never mid-surrogate-pair.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn utf16_to_utf8(text: &str, utf16_index: u32) -> usize {
    let mut utf16_count = 0u32;
    for (byte_index, ch) in text.char_indices() {
        if utf16_count >= utf16_index {
            return byte_index;
        }
        utf16_count += ch.len_utf16() as u32;
    }
    text.len()
}

//#endregion 🔖️Measure

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ResourceOp;

    #[test]
    fn measuring_ascii_run_is_stable_and_matches_shaped_advance() {
        let mut ts = TextSystem::new();
        let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
        let first = ts.measure("Hello", &style);
        let second = ts.measure("Hello", &style);
        assert_eq!(first, second, "measuring the same run twice must be stable");
        let Measurement::Ready { width, .. } = first else { panic!("expected a Ready measurement for a built-in font") };
        let shaped = ts.shape("Hello", &style);
        let summed_advance: f32 = shaped.glyphs.iter().map(|glyph| glyph.advance).sum();
        assert!((width - summed_advance).abs() < 0.01, "measured width ({width}) must match the sum of shaped glyph advances ({summed_advance})");
    }

    #[test]
    fn wrapping_breaks_at_max_width_and_never_mid_char() {
        let mut ts = TextSystem::new();
        let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
        let text = "one two three four five six seven eight";
        let lines = ts.wrap(text, &style, 100.0);
        assert!(lines.len() > 1, "long text at a narrow max_width must wrap onto multiple lines");
        for range in &lines {
            assert!(text.is_char_boundary(range.start), "line start {} must be a char boundary", range.start);
            assert!(text.is_char_boundary(range.end), "line end {} must be a char boundary", range.end);
        }
        let non_whitespace_in: usize = text.chars().filter(|ch| !ch.is_whitespace()).count();
        let non_whitespace_out: usize = lines.iter().map(|range| text[range.clone()].chars().filter(|ch| !ch.is_whitespace()).count()).sum();
        assert_eq!(non_whitespace_in, non_whitespace_out, "wrapping must not drop or duplicate characters");
    }

    #[test]
    fn measuring_with_an_unloaded_custom_font_yields_pending_with_a_placeholder() {
        let mut ts = TextSystem::new();
        let dependency = ts.request_font("brand-display");
        assert_eq!(ts.font_status(dependency), FontStatus::Pending);
        let style = TextStyle { family: FontFamilyChoice::Custom(dependency), size_px: 20.0 };
        let pending = ts.measure("Loading", &style);
        let Measurement::Pending { placeholder_width, placeholder_height } = pending else { panic!("expected Pending for an unresolved font dependency, got {pending:?}") };
        assert!(placeholder_width > 0.0 && placeholder_height > 0.0, "a Pending measurement must still carry a usable placeholder size");
        assert!(ts.provide_font_bytes(dependency, ANTA_LATIN), "providing real bytes for a pending dependency must succeed");
        assert_eq!(ts.font_status(dependency), FontStatus::Ready);
        assert_eq!(ts.take_ready_dependencies(), vec![dependency], "a newly resolved dependency must surface exactly once for invalidation");
        assert!(ts.take_ready_dependencies().is_empty(), "take_ready_dependencies must drain, not repeat, past dependencies");
        let resolved = ts.measure("Loading", &style);
        assert!(matches!(resolved, Measurement::Ready { .. }), "once the font lands, measurement must resolve to Ready, got {resolved:?}");
    }

    #[test]
    fn measuring_the_same_string_twice_serves_the_second_call_from_the_shape_cache() {
        let mut ts = TextSystem::new();
        let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
        ts.measure("cached", &style);
        assert_eq!(ts.shape_cache_misses(), 1);
        assert_eq!(ts.shape_cache_hits(), 0);
        ts.measure("cached", &style);
        assert_eq!(ts.shape_cache_hits(), 1, "the second identical measurement must hit the cache, not reshape");
        assert_eq!(ts.shape_cache_misses(), 1, "a cache hit must not increment the miss counter");
    }

    #[test]
    fn atlas_insertion_queues_exactly_one_upload_and_a_repeat_glyph_queues_none() {
        let mut ts = TextSystem::new();
        let mut resources = ResourceRegistry::default();
        let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
        let glyph = ts.shape("A", &style).glyphs[0];
        ts.ensure_glyph(&mut resources, glyph.font, glyph.glyph_id, style.size_px);
        let ops = resources.drain_ops();
        assert_eq!(ops.len(), 1, "packing a new glyph must queue exactly one op");
        assert!(matches!(ops[0], ResourceOp::UploadAtlas { .. }), "the queued op must be an UploadAtlas");
        ts.ensure_glyph(&mut resources, glyph.font, glyph.glyph_id, style.size_px);
        assert!(resources.drain_ops().is_empty(), "re-requesting an already-cached glyph must not queue another upload");
    }

    #[test]
    fn utf8_utf16_index_conversion_round_trips_across_a_non_bmp_emoji() {
        let text = "a😀b";
        let byte_index_after_emoji = "a😀".len();
        let utf16_index = utf8_to_utf16(text, byte_index_after_emoji);
        assert_eq!(utf16_index, 3, "'a' is 1 UTF-16 unit, the non-BMP emoji is a 2-unit surrogate pair");
        let round_tripped = utf16_to_utf8(text, utf16_index);
        assert_eq!(round_tripped, byte_index_after_emoji, "forward-then-back conversion must land exactly on the original byte index");
    }

    #[test]
    fn cursor_movement_never_lands_inside_the_emoji_grapheme_cluster() {
        let mut ts = TextSystem::new();
        let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
        let text = "a😀b";
        let after_a = ts.next_grapheme(text, &style, 0);
        assert_eq!(after_a, 1, "moving past 'a' must land right after it");
        let after_emoji = ts.next_grapheme(text, &style, after_a);
        assert_eq!(after_emoji, 1 + "😀".len(), "moving past the emoji must skip its whole grapheme cluster, never landing mid-codepoint");
        let back_to_after_a = ts.previous_grapheme(text, &style, after_emoji);
        assert_eq!(back_to_after_a, after_a, "moving backward across the emoji must land exactly where moving forward started");
    }

    #[test]
    fn selection_geometry_covers_the_requested_range_with_at_least_one_rect() {
        let mut ts = TextSystem::new();
        let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
        let rects = ts.selection_geometry("Hello", &style, 0..5);
        assert!(!rects.is_empty(), "a non-empty selection range must produce at least one highlight rect");
        for rect in &rects {
            assert!(rect[2] > 0.0 && rect[3] > 0.0, "each selection rect must have positive width/height, got {rect:?}");
        }
    }
}

//#endregion Tests
