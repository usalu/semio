//! 🏷️ Real glyph-shaped label rendering (`append_label`/`append_label_tspans`) and
//! `usvg`-shaped advance measurement (`label_byte_world_x`'s native arm, via
//! `label_line_layout`/`label_prefix_advance_svg`) are host/browser only: every real caller
//! repo-wide is either inside `DagHost`/`BoardHost`/`TrinityBridge`'s paint tree (host-only,
//! see `IconPaintCache::get_or_build`'s docstring), behind flow's `wasm_session` browser
//! bridge (already `not(target_env = "p2")`-gated, e.g. `begin_note_edit`), or in
//! `semio-framework-editor` (not a dependency of any plugin's `wasm32-wasip2` build at all).
//! The `wasm32-wasip2` arms below use the character-width heuristic
//! (`label_extent`/`label_advance`/`label_text_inset`) already established in this module as
//! the "no glyph shaping available" fallback. Ticket
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
//! `🔍️research/📓️infinite-text-shaping.md`.

// #region 🔖️Text
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use std::cell::RefCell;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use std::collections::HashMap;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use std::rc::Rc;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use std::sync::{Arc, OnceLock};

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use super::svg_icon::render_svg_tree_literal;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use super::usvg;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use super::{Affine, Vec2};
use super::{Color, Point, Scene};

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
static MAP_LABEL_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
static MAP_LABEL_USVG_WITHOUT_FACE: OnceLock<usvg::Options<'static>> = OnceLock::new();

/// 🔤️ `usvg` options with the installed map label sans (delivered through the runtime asset port) for place-name labels.
/// Until a host installs the runtime assets the options carry no face and labels shape no glyphs.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn usvg_options_map_labels() -> &'static usvg::Options<'static> {
    match semio_framework_canvas_assets::label_font() {
        Some(face) => MAP_LABEL_USVG_OPTIONS.get_or_init(|| {
            let mut db = usvg::fontdb::Database::new();
            db.load_font_data(face.to_vec());
            let family = db.faces().next().and_then(|loaded| loaded.families.first().map(|(name, _)| name.clone())).unwrap_or_else(|| ui_styling::canvas_fonts::MAP_LABEL_SANS_FALLBACK.into());
            usvg::Options { fontdb: Arc::new(db), font_family: family, ..Default::default() }
        }),
        None => MAP_LABEL_USVG_WITHOUT_FACE.get_or_init(|| usvg::Options { font_family: ui_styling::canvas_fonts::MAP_LABEL_SANS_FALLBACK.into(), ..Default::default() }),
    }
}

/// 🗂️ The schema whose `const`s are the shaped-label cache's bounds — their only source.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub const LABEL_SHAPES_SCHEMA: &str = include_str!("../../🧬️schema/🏷️label-shapes/🔣️.json");

/// 📏️ Entry, byte and per-entry byte bounds of one [`BoundedLru`].
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelShapeBounds {
    pub maximum_entries: usize,
    pub maximum_bytes: usize,
    pub maximum_entry_bytes: usize,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl LabelShapeBounds {
    /// 📜️ Reads the bounds a label-shapes schema declares as `properties.<bound>.const`; refuses a
    /// schema without them, with no entry, or whose single entry could exceed the total.
    pub fn from_schema(schema: &str) -> Result<Self, String> {
        let document: serde_json::Value = serde_json::from_str(schema).map_err(|error| format!("label-shapes schema: {error}"))?;
        let bound = |name: &str| document["properties"][name]["const"].as_u64().and_then(|value| usize::try_from(value).ok()).ok_or_else(|| format!("label-shapes schema declares no integer `properties.{name}.const`"));
        let bounds = Self { maximum_entries: bound("maximumEntries")?, maximum_bytes: bound("maximumBytes")?, maximum_entry_bytes: bound("maximumEntryBytes")? };
        if bounds.maximum_entries == 0 || bounds.maximum_entry_bytes > bounds.maximum_bytes {
            return Err(format!("label-shapes schema bounds are inconsistent: {bounds:?}"));
        }
        Ok(bounds)
    }

    /// 🏷️ The bounds [`LABEL_SHAPES_SCHEMA`] declares.
    pub fn declared() -> Self {
        static DECLARED: OnceLock<LabelShapeBounds> = OnceLock::new();
        *DECLARED.get_or_init(|| Self::from_schema(LABEL_SHAPES_SCHEMA).expect("the label-shapes schema declares consistent bounds"))
    }
}

/// 🧾️ What one [`BoundedLru::admit`] did: stored after evicting these keys (least recent first), or bypassed
/// because the entry alone exceeds `maximum_entry_bytes`.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LruAdmission {
    Stored { evicted: Vec<String> },
    Bypassed,
}

/// 🔁️ A least-recently-used map bounded by entry count, total accounted bytes and per-entry bytes — the
/// policy of the shaped-label cache, generic so its laws replay without a font.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub struct BoundedLru<V> {
    bounds: LabelShapeBounds,
    entries: HashMap<String, (V, usize, u64)>,
    bytes: usize,
    clock: u64,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl<V> BoundedLru<V> {
    pub fn new(bounds: LabelShapeBounds) -> Self {
        Self { bounds, entries: HashMap::new(), bytes: 0, clock: 0 }
    }

    /// 🔎️ The value under `key`, made the most recent.
    pub fn get(&mut self, key: &str) -> Option<&V> {
        self.clock += 1;
        let clock = self.clock;
        let entry = self.entries.get_mut(key)?;
        entry.2 = clock;
        Some(&entry.0)
    }

    /// ➕️ Stores `value` under `key`, accounted at `bytes`, after evicting least-recently-used keys until
    /// both the entry count and the byte total fit; replaces a present key.
    pub fn admit(&mut self, key: String, value: V, bytes: usize) -> LruAdmission {
        if let Some((_, previous, _)) = self.entries.remove(&key) {
            self.bytes -= previous;
        }
        if bytes > self.bounds.maximum_entry_bytes {
            return LruAdmission::Bypassed;
        }
        let mut evicted = Vec::new();
        while self.entries.len() >= self.bounds.maximum_entries || self.bytes + bytes > self.bounds.maximum_bytes {
            let Some(oldest) = self.entries.iter().min_by_key(|(_, entry)| entry.2).map(|(oldest, _)| oldest.clone()) else {
                break;
            };
            if let Some((_, released, _)) = self.entries.remove(&oldest) {
                self.bytes -= released;
            }
            evicted.push(oldest);
        }
        self.clock += 1;
        self.bytes += bytes;
        self.entries.insert(key, (value, bytes, self.clock));
        LruAdmission::Stored { evicted }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// ⚖️ Accounted bytes of every resident entry.
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    /// 🗝️ Resident keys, least recent first.
    pub fn keys(&self) -> Vec<String> {
        let mut rows: Vec<(u64, &String)> = self.entries.iter().map(|(key, entry)| (entry.2, key)).collect();
        rows.sort_unstable();
        rows.into_iter().map(|(_, key)| key.clone()).collect()
    }
}

/// 🖋️ One shaped label: the usvg content bounds of its markup and, once painted, its outline scene.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
struct LabelShape {
    bounds: (f64, f64, f64, f64),
    scene: Option<Scene>,
}

/// 📊️ Counters of the calling thread's shaped-label cache: `shapes` counts usvg shapings (font face parse +
/// glyph shaping + outlining), `hits` the paints and measures that reused one.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LabelShapeStats {
    pub hits: u64,
    pub shapes: u64,
    pub evictions: u64,
    pub bypasses: u64,
    pub entries: usize,
    pub bytes: usize,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
struct LabelShapeCache {
    lru: BoundedLru<Rc<LabelShape>>,
    stats: LabelShapeStats,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
thread_local! {
    static LABEL_SHAPES: RefCell<LabelShapeCache> = RefCell::new(LabelShapeCache { lru: BoundedLru::new(LabelShapeBounds::declared()), stats: LabelShapeStats::default() });
}

/// 📊️ The calling thread's shaped-label cache counters (a browser canvas session runs on one thread; every
/// native test thread owns its own cache).
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn label_shape_stats() -> LabelShapeStats {
    LABEL_SHAPES.with(|cache| {
        let cache = cache.borrow();
        LabelShapeStats { entries: cache.lru.len(), bytes: cache.lru.bytes(), ..cache.stats }
    })
}

/// 🗃️ Shapes `markup` with usvg once per content: every later paint (`paint`) or measure of the same markup
/// reuses the shaped outline scene and bounds, so an unchanged frame re-parses no font face and reshapes no glyph
/// run (measured ticket 26/09/23 F1: an idle-then-typed trinity query editor spent ~85 ms of every paint in
/// `usvg::Tree::from_str`). The markup carries every input of the shaping — text, size, family, fill, halo.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn shaped_label(markup: &str, paint: bool) -> Option<Rc<LabelShape>> {
    let cached = LABEL_SHAPES.with(|cache| {
        let mut cache = cache.borrow_mut();
        let shape = cache.lru.get(markup).filter(|shape| !paint || shape.scene.is_some()).cloned();
        if shape.is_some() {
            cache.stats.hits += 1;
        }
        shape
    });
    if cached.is_some() {
        return cached;
    }
    let tree = usvg::Tree::from_str(markup, usvg_options_map_labels()).ok()?;
    let bounds = super::svg_icon::svg_icon_content_bounds(&tree);
    let scene = paint.then(|| {
        let mut scene = Scene::new();
        render_svg_tree_literal(&mut scene, &tree);
        scene
    });
    let bytes = markup.len() + size_of::<LabelShape>() + scene.as_ref().map_or(0, Scene::retained_bytes);
    let shape = Rc::new(LabelShape { bounds, scene });
    LABEL_SHAPES.with(|cache| {
        let mut cache = cache.borrow_mut();
        cache.stats.shapes += 1;
        match cache.lru.admit(markup.to_owned(), Rc::clone(&shape), bytes) {
            LruAdmission::Stored { evicted } => cache.stats.evictions += evicted.len() as u64,
            LruAdmission::Bypassed => cache.stats.bypasses += 1,
        }
    });
    Some(shape)
}

/// 📐️ Measure markup of one label line: its box, no paint — shared by the line layout and every prefix advance.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn label_measure_markup(text: &str, px: f64) -> String {
    let pad = px * ui_styling::metrics::label::PAD_RATIO;
    let (w, h) = label_extent(text, px);
    let text_y = pad + px;
    let family = usvg_options_map_labels().font_family.clone();
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}">{text}</text></svg>"##,
        family = escape_xml_attr(&family),
        text = escape_xml_attr(text),
    )
}

/// 🖌️ Paint markup of one single-color label with its halo.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn label_paint_markup(text: &str, px: f64, fill: Color, halo: Color) -> String {
    let pad = px * ui_styling::metrics::label::PAD_RATIO;
    let (w, h) = label_extent(text, px);
    let text_y = pad + px;
    let family = usvg_options_map_labels().font_family.clone();
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}" fill="{fill}" stroke="{halo}" stroke-width="{stroke}" paint-order="stroke">{text}</text></svg>"##,
        family = escape_xml_attr(&family),
        fill = color_to_svg(fill),
        halo = color_to_svg(halo),
        stroke = (px * ui_styling::metrics::label::HALO_STROKE_RATIO).max(ui_styling::metrics::label::HALO_STROKE_MIN),
        text = escape_xml_attr(text),
    )
}

/// 🌈️ Paint markup of one line with colored inline tspans; `None` when no span paints anything.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn label_tspans_markup(line: &str, spans: &[(usize, usize, Color)], px: f64) -> Option<String> {
    let pad = px * ui_styling::metrics::label::PAD_RATIO;
    let (w, h) = label_extent(line, px);
    let text_y = pad + px;
    let family = usvg_options_map_labels().font_family.clone();
    let mut inner = String::new();
    for &(start, end, fill) in spans {
        if start >= end || end > line.len() {
            continue;
        }
        let slice = &line[start..end];
        if slice.is_empty() {
            continue;
        }
        inner.push_str(&format!(r#"<tspan fill="{fill}">{text}</tspan>"#, fill = color_to_svg(fill), text = escape_xml_attr(slice)));
    }
    if inner.is_empty() {
        return None;
    }
    Some(format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}">{inner}</text></svg>"##,
        family = escape_xml_attr(&family),
    ))
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn escape_xml_attr(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn color_to_svg(c: Color) -> String {
    let rgba = c.to_rgba8();
    if rgba.a == 255 {
        format!("#{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b)
    } else {
        let a = f64::from(rgba.a) / 255.0;
        format!("rgba({},{},{},{a})", rgba.r, rgba.g, rgba.b)
    }
}

/// 📐️ Estimated label box size in screen px for layout (matches `append_label` padding).
pub fn label_extent(label: &str, px: f64) -> (f64, f64) {
    let trimmed = label.trim();
    if trimmed.is_empty() || px < ui_styling::metrics::label::MIN_PX {
        return (0.0, 0.0);
    }
    let pad = px * ui_styling::metrics::label::PAD_RATIO;
    let w = (trimmed.len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO + pad * 2.0).clamp(ui_styling::metrics::label::WIDTH_MIN, ui_styling::metrics::label::WIDTH_MAX);
    let h = (px * ui_styling::metrics::label::HEIGHT_RATIO + pad * 2.0).clamp(ui_styling::metrics::label::HEIGHT_MIN, ui_styling::metrics::label::HEIGHT_MAX);
    (w, h)
}

/// ✂️ The one glyph a clipped label ends on. Every presentation appends the same character, so a
/// truncated title reads identically on the GPU, on the 2D replay and on the wgpu shell.
pub const LABEL_ELLIPSIS: &str = "…";

/// ✂️ Longest prefix of `text` that still fits `max_width` once {@link LABEL_ELLIPSIS} is
/// appended, measured by the CALLER's own text measure — a glyph atlas on the wgpu shell, the
/// canvas `measureText` in the browser, a synthetic advance in a law.
///
/// This is the whole reason a title is never shrunk to an unreadable font instead: a label that
/// does not fit is CLIPPED at its measured width, not scaled until it does. Returns `text`
/// unchanged when it already fits, and the bare ellipsis when not even one glyph plus the
/// ellipsis does.
pub fn ellipsize_by_measure(text: &str, max_width: f64, mut measure: impl FnMut(&str) -> f64) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() || max_width <= 0.0 {
        return String::new();
    }
    if measure(trimmed) <= max_width {
        return trimmed.to_string();
    }
    let mut best = String::new();
    let mut candidate = String::new();
    for (byte_index, ch) in trimmed.char_indices() {
        candidate.clear();
        candidate.push_str(&trimmed[..byte_index + ch.len_utf8()]);
        candidate.push_str(LABEL_ELLIPSIS);
        if measure(&candidate) <= max_width {
            best.clear();
            best.push_str(&candidate);
        } else {
            break;
        }
    }
    if best.is_empty() {
        LABEL_ELLIPSIS.to_string()
    } else {
        best
    }
}

/// ↔ Horizontal text advance inside a label box (excludes outer padding).
pub fn label_advance(label: &str, px: f64) -> f64 {
    if label.is_empty() || px < ui_styling::metrics::label::MIN_PX {
        return 0.0;
    }
    label.len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO
}

/// 📏️ Left inset from label origin to first glyph baseline start.
pub fn label_text_inset(px: f64) -> f64 {
    if px < ui_styling::metrics::label::MIN_PX {
        return 0.0;
    }
    px * ui_styling::metrics::label::PAD_RATIO
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Clone, Copy, Debug)]
struct LabelLineLayout {
    bx: f64,
    scale: f64,
    pad: f64,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn label_line_layout(line: &str, px: f64) -> Option<LabelLineLayout> {
    if px < ui_styling::metrics::label::MIN_PX {
        return None;
    }
    let pad = px * ui_styling::metrics::label::PAD_RATIO;
    let (bx, _, bw, bh) = shaped_label(&label_measure_markup(if line.is_empty() { " " } else { line }, px), false)?.bounds;
    if bw <= 0.0 || bh <= 0.0 {
        return None;
    }
    let scale = (px * ui_styling::metrics::label::SCALE_RATIO / bh).min(ui_styling::metrics::label::SCALE_MAX);
    Some(LabelLineLayout { bx, scale, pad })
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn label_prefix_advance_svg(line: &str, byte_end: usize, px: f64) -> f64 {
    let end = byte_end.min(line.len());
    if end == 0 {
        return 0.0;
    }
    if !line.is_char_boundary(end) {
        let prev = line[..end].char_indices().next_back().map_or(0, |(i, _)| i);
        return label_prefix_advance_svg(line, prev, px);
    }
    let prefix = &line[..end];
    let pad = px * ui_styling::metrics::label::PAD_RATIO;
    let Some(shape) = shaped_label(&label_measure_markup(prefix, px), false) else {
        return label_advance(prefix, px);
    };
    let (bx, _, bw, bh) = shape.bounds;
    if bw <= 0.0 || bh <= 0.0 {
        return label_advance(prefix, px);
    }
    (bx + bw) - pad
}

/// ↔ World x for a byte offset in a code line (matches `append_label_tspans` layout).
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn label_byte_world_x(line: &str, byte_offset: usize, origin_x: f64, px: f64) -> f64 {
    let Some(layout) = label_line_layout(line, px) else {
        return origin_x;
    };
    let advance = label_prefix_advance_svg(line, byte_offset, px);
    origin_x + (layout.pad + advance - layout.bx) * layout.scale
}

/// ↔ `wasm32-wasip2` arm: no `usvg` text shaper is linked on this target (see this module's
/// top docstring — every real caller is host/browser-only or unreachable). Uses the same
/// character-width heuristic (`label_advance`/`label_text_inset`) this module already falls
/// back to when real glyph shaping is unavailable (see `label_prefix_advance_svg`'s own
/// `usvg::Tree::from_str` failure arm above) — a disclosed, intentional precision difference,
/// not a stub: byte offsets still map to a monotonically increasing world x. Ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn label_byte_world_x(line: &str, byte_offset: usize, origin_x: f64, px: f64) -> f64 {
    let end = byte_offset.min(line.len());
    let end = if line.is_char_boundary(end) { end } else { line[..end].char_indices().next_back().map_or(0, |(i, _)| i) };
    origin_x + label_text_inset(px) + label_advance(&line[..end], px)
}

/// ↔ World x range for a byte span in a code line.
pub fn label_span_world_x(line: &str, byte_start: usize, byte_end: usize, origin_x: f64, px: f64) -> (f64, f64) {
    (label_byte_world_x(line, byte_start, origin_x, px), label_byte_world_x(line, byte_end, origin_x, px))
}

/// 🏷️ Renders a single map label via SVG text at `origin` (screen px, baseline), shaped once per content.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn append_label(scene: &mut Scene, label: &str, origin: Point, px: f64, fill: Color, halo: Color) {
    let trimmed = label.trim();
    if trimmed.is_empty() || px < ui_styling::metrics::label::MIN_PX {
        return;
    }
    let Some(shape) = shaped_label(&label_paint_markup(trimmed, px, fill, halo), true) else {
        return;
    };
    append_shaped_label(scene, &shape, origin, px);
}

/// 📌️ Places one shaped label's outline scene at `origin` (screen px, baseline) at its fitted scale.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn append_shaped_label(scene: &mut Scene, shape: &LabelShape, origin: Point, px: f64) {
    let (bx, by, bw, bh) = shape.bounds;
    let Some(label_scene) = shape.scene.as_ref().filter(|_| bw > 0.0 && bh > 0.0) else {
        return;
    };
    let scale = (px * ui_styling::metrics::label::SCALE_RATIO / bh).min(ui_styling::metrics::label::SCALE_MAX);
    let aff = Affine::IDENTITY.translate(Vec2::new(origin.x() - bx * scale, origin.y() - by * scale - px * ui_styling::metrics::label::VERTICAL_OFFSET_RATIO)).scale(scale);
    scene.append(label_scene, Some(aff));
}

/// 🚫️ `wasm32-wasip2` arm: label *painting* (rasterizing shaped glyphs into `Scene`) is
/// host-only, same reasoning as `IconPaintCache::get_or_build`'s `wasm32-wasip2` arm — a WASI
/// guest has no display to paint onto, and this module's top docstring traces every real
/// caller as host/browser-only or unreachable. A no-op, not a stub: nothing on this target
/// could ever observe a painted label today. Ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn append_label(_scene: &mut Scene, _label: &str, _origin: Point, _px: f64, _fill: Color, _halo: Color) {}

/// 🏷️ Renders one label with colored inline tspans (single padding box, no per-span gaps), shaped once per content.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn append_label_tspans(scene: &mut Scene, line: &str, spans: &[(usize, usize, Color)], origin: Point, px: f64, _halo: Color) {
    if line.is_empty() || spans.is_empty() || px < ui_styling::metrics::label::MIN_PX {
        return;
    }
    let Some(shape) = label_tspans_markup(line, spans, px).and_then(|markup| shaped_label(&markup, true)) else {
        return;
    };
    append_shaped_label(scene, &shape, origin, px);
}

/// 🚫️ `wasm32-wasip2` arm: see `append_label`'s `wasm32-wasip2` arm docstring — identical
/// reasoning, this is the multi-span sibling.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn append_label_tspans(_scene: &mut Scene, _line: &str, _spans: &[(usize, usize, Color)], _origin: Point, _px: f64, _halo: Color) {}

#[cfg(all(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
include!("../../🧪️tests/🏷️label-shapes/🦀️.rs");
// #endregion 🔖️Text
