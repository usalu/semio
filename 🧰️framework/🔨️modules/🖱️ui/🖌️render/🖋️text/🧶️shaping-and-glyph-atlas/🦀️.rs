//! 🧶️ Shaping and glyph atlas: the shape cache key, the shaped glyphs and text, the `TextSystem` shaping, the glyph entries and the atlas pages with glyph rasterization. See [`super`].

use super::*;

//#region 🔖️Shape
/// 🔑️ Cache key for [`TextSystem::shape`]. `size_bits`/`f32::to_bits` because `f32` is neither `Eq`
/// nor `Hash` — same trick `layout.rs`'s callers use for float-keyed measure caching elsewhere in this
/// program.
#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) struct ShapeCacheKey {
    pub(super) text: String,
    pub(super) family: FontFamilyChoice,
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
    pub(super) fn layout_for(&mut self, text: &str, style: &TextStyle, max_width: Option<f32>) -> Layout<[u8; 4]> {
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
    pub fn shape(&mut self, text: &str, style: &TextStyle) -> &ShapedText {
        let key = ShapeCacheKey { text: text.to_string(), family: style.family, size_bits: style.size_px.to_bits() };
        if self.shape_cache.contains_key(&key) {
            self.shape_hits += 1;
        } else {
            let layout = self.layout_for(text, style, None);
            let shaped = self.extract_shaped(&layout);
            self.shape_cache.insert(key.clone(), shaped);
            self.shape_misses += 1;
        }
        self.shape_cache.get(&key).expect("shape cache entry just ensured present")
    }

    /// 🧵️ Reads glyphs/metrics out of an already-built (and already line-broken/aligned) `Layout` — the
    /// shared tail of [`Self::shape`] and [`Self::shape_paragraph`], so wrapping/alignment support never
    /// duplicates the glyph-extraction loop.
    fn extract_shaped(&mut self, layout: &Layout<[u8; 4]>) -> ShapedText {
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

    pub fn shape_cache_hits(&self) -> u32 {
        self.shape_hits
    }

    pub fn shape_cache_misses(&self) -> u32 {
        self.shape_misses
    }

    /// 📝️ Shapes a wrapped, aligned paragraph — [`Self::shape`]/[`Self::layout_for`]'s minimal
    /// unwrapped run plus weight/line-height/letter-spacing/alignment, for a caller (e.g. a page-layout
    /// text frame) that needs real paragraph typesetting rather than a single measured run. Uncached
    /// like [`Self::measure_wrapped`] — `max_width` and `style`'s extra fields vary too continuously
    /// (interactive resize, live paragraph editing) to key a cache on productively. Falls back to a
    /// glyph-less placeholder [`ShapedText`] while `style.base.family` names an unresolved
    /// [`FontDependencyId`], same short-circuit [`Self::measure`] takes.
    pub fn shape_paragraph(&mut self, text: &str, style: &TextRunStyle, max_width: f32) -> ShapedText {
        if !self.family_ready(style.base.family) {
            let Measurement::Pending { placeholder_width, placeholder_height } = Self::placeholder_measurement(text, style.base.size_px) else { unreachable!("placeholder_measurement always returns Pending") };
            return ShapedText { glyphs: Vec::new(), width: placeholder_width, height: placeholder_height, ascent: 0.0, descent: 0.0, is_rtl: false };
        }
        let stack = self.font_stack_for(style.base.family);
        let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, text, 1.0, true);
        builder.push_default(StyleProperty::FontStack(stack));
        builder.push_default(StyleProperty::FontSize(style.base.size_px));
        builder.push_default(StyleProperty::FontWeight(FontWeight::new(style.weight)));
        builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(style.line_height_relative)));
        builder.push_default(StyleProperty::LetterSpacing(style.letter_spacing_px));
        let mut layout: Layout<[u8; 4]> = builder.build(text);
        layout.break_all_lines(Some(max_width));
        layout.align(Some(max_width), Self::to_parley_alignment(style.alignment), AlignmentOptions::default());
        self.extract_shaped(&layout)
    }

    fn to_parley_alignment(alignment: TextAlignment) -> Alignment {
        match alignment {
            TextAlignment::Left => Alignment::Left,
            TextAlignment::Middle => Alignment::Middle,
            TextAlignment::Right => Alignment::Right,
            TextAlignment::Justified => Alignment::Justified,
        }
    }
}
//#endregion 🔖️Shape

//#region 🔖️Atlas
/// 📋️ One rasterized glyph's atlas placement: origin, size and bearings a painter needs to position the
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
    /// ➗️ `[u0, v0, u1, v1]` normalized against `atlas_width`/`atlas_height` — pass
    /// [`TextSystem::atlas_dimensions`]/[`TextSystem::color_atlas_dimensions`] (matching
    /// [`Self::is_color`]) here, read fresh every paint call, never cached.
    pub fn uv_rect(&self, atlas_width: u32, atlas_height: u32) -> [f32; 4] {
        let w = atlas_width as f32;
        let h = atlas_height as f32;
        [self.atlas_x as f32 / w, self.atlas_y as f32 / h, (self.atlas_x + self.width) as f32 / w, (self.atlas_y + self.height) as f32 / h]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct GlyphCacheKey {
    pub(super) font: FontHandle,
    pub(super) glyph_id: u16,
    size_bucket: u32,
}

struct RasterizedGlyph {
    bitmap: Vec<u8>,
    pub(super) width: u32,
    pub(super) height: u32,
    bearing_x: f32,
    bearing_y: f32,
    is_color: bool,
}

impl RasterizedGlyph {
    fn empty() -> Self {
        Self { bitmap: Vec::new(), width: 0, height: 0, bearing_x: 0.0, bearing_y: 0.0, is_color: false }
    }
}

/// 🗺️ One bin-packed atlas image (the alpha page or the RGBA color-emoji page). Row-cursor packer,
/// ported from the wgpu-old target's `FontAtlas::pack_glyph`. Growth is height-only doubling: `width`
/// (and therefore every existing glyph's `atlas_x`/row byte stride) never changes, so `Vec::resize`
/// simply appends zeroed rows — no re-blit, and every previously packed glyph's `(atlas_x, atlas_y)`
/// stays valid. See [`Self::queue_upload`] for why growth still needs a fresh [`AtlasId`] anyway.
pub(super) struct AtlasPage {
    pub(super) key: &'static str,
    channels: u32,
    pub(super) width: u32,
    pub(super) height: u32,
    pixels: Vec<u8>,
    cursor_x: u32,
    cursor_y: u32,
    row_height: u32,
    generation: u32,
    pub(super) id: Option<AtlasId>,
}

impl AtlasPage {
    pub(super) fn new(key: &'static str, channels: u32) -> Self {
        let width = 2048;
        let height = 2048;
        Self { key, channels, width, height, pixels: vec![0; (width * height * channels) as usize], cursor_x: 1, cursor_y: 1, row_height: 0, generation: 0, id: None }
    }

    /// 📍️ Bin-packs `w`×`h` into the current row, wrapping and growing as needed, blits `bitmap`, and
    /// returns the placement. Never panics on a pathological glyph wider than the whole page — worst
    /// case it silently clips via [`Self::blit`]'s own bounds checks, which real UI glyph sizes never
    /// approach.
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
    fn queue_upload(&mut self, resources: &mut ResourceRegistry) {
        let key = format!("{}:{}", self.key, self.generation);
        self.id = Some(resources.request_atlas_upload(&key, self.width, self.height, self.pixels.clone()));
    }
}

impl TextSystem {
    fn quantize_size(size_px: f32) -> u32 {
        size_px.round().max(1.0) as u32
    }

    /// 🔍️ Fetches (rasterizing and bin-packing on first use) `(font, glyph_id)`'s atlas placement at
    /// `size_px`. A cache hit touches `resources` not at all — no `ResourceOp` is queued for an
    /// already-packed glyph, which is what keeps "a second identical glyph produces none" true.
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

    pub fn atlas_id(&self) -> Option<AtlasId> {
        self.alpha_page.id
    }

    pub fn color_atlas_id(&self) -> Option<AtlasId> {
        self.color_page.id
    }

    pub fn atlas_dimensions(&self) -> (u32, u32) {
        (self.alpha_page.width, self.alpha_page.height)
    }

    pub fn color_atlas_dimensions(&self) -> (u32, u32) {
        (self.color_page.width, self.color_page.height)
    }
}
//#endregion 🔖️Atlas
