//! ✍️ Text editor engine on the infinite canvas.
//!
//! `EditorHost` mirrors projection text and LSP adornments for WASM play surfaces only; authoritative
//! packs and edits belong in the OS `ArtifactStore` (see `sync_from_scene_json` / pack apply paths).

use canvas::camera::Viewport;
use canvas::text as canvas_text;
pub use infinite_canvas::{self as canvas, *};
use serde::Deserialize;
// 🧬️ `#[derive(FromValue)]` additive alongside `Deserialize` (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01): every `...Json` struct below is Deserialize-ONLY (one-way WASM-boundary JSON parsing,
// never serialized back out), so its additive twin is `FromValue` alone — mirroring what serde
// actually derives here, not more. `semio-framework-os-kernel` above is an unconditional
// dependency, so the derive's default `::semio_framework_os_kernel` crate path resolves without a
// `#[value(crate = "...")]` override.
use semio_framework_value_derive::FromValue;

// #region ⚠️ Errors
/// 🧯️ Errors from `EditorHost`'s own JSON-boundary parsing (theme/scene sync). The
/// `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))] #[wasm_bindgen]` methods on `EditorSession` stay
/// `Result<_, JsValue>` — that shape is dictated by the `wasm_bindgen` ABI, not this crate's own
/// error handling, so it is not migrated here.
// 🧬️ ToValue/FromValue coverage deliberately SKIPPED (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01): both variants wrap a genuinely foreign, non-data error type — `serde_json::Error`
// (opaque parser diagnostic state, not `ToValue`/`FromValue` anywhere) and `store::PackError`
// (another owner's module) — and `EditorError` itself never crosses a wire (an internal
// `Result<_, EditorError>`; the actual WASM boundary returns `JsValue` per the docstring above).
#[derive(Debug)]
pub enum EditorError {
    Json(serde_json::Error),
    Pack(store::PackError),
}

impl std::fmt::Display for EditorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "json: {error}"),
            Self::Pack(error) => write!(formatter, "pack: {error}"),
        }
    }
}

impl std::error::Error for EditorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Pack(error) => Some(error),
        }
    }
}

impl From<serde_json::Error> for EditorError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<store::PackError> for EditorError {
    fn from(error: store::PackError) -> Self {
        Self::Pack(error)
    }
}
// #endregion ⚠️ Errors

// #region 🔖️Theme
/// 🌉️ `Color` bridge — `Color` (`♾️infinite`, a different owner's module) has no `ToValue`/
/// `FromValue`; hand-written via its own public `components()`/`new()` rather than editing that
/// module. `EditorCanvasTheme` below names this via `#[value(with = "color_bridge")]`.
mod color_bridge {
    use semio_framework_os_kernel as dsl_core;

    pub fn to_value(c: &super::Color) -> dsl_core::DslValue {
        dsl_core::ToValue::to_value(&c.components())
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<super::Color, dsl_core::ValueError> {
        <[f32; 4] as dsl_core::FromValue>::from_value(value).map(super::Color::new)
    }
}

// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`, so no `#[value(...)]` rename is needed beyond the `Color` bridge.
#[derive(Clone, Copy, Debug, semio_framework_value_derive::ToValue, FromValue)]
struct EditorCanvasTheme {
    #[value(with = "color_bridge")]
    raster_clear: Color,
    #[value(with = "color_bridge")]
    grid_minor_stroke: Color,
    #[value(with = "color_bridge")]
    label_fill: Color,
    #[value(with = "color_bridge")]
    label_fill_hovered: Color,
    #[value(with = "color_bridge")]
    label_halo: Color,
    #[value(with = "color_bridge")]
    hover_fill: Color,
    #[value(with = "color_bridge")]
    selection_fill: Color,
}

impl Default for EditorCanvasTheme {
    fn default() -> Self {
        Self::from_board(&ui_styling::BOARD_LIGHT)
    }
}

impl EditorCanvasTheme {
    fn from_board(t: &ui_styling::BoardPalette) -> Self {
        Self {
            raster_clear: Color::new(t.raster_clear),
            grid_minor_stroke: Color::new(t.grid_minor_stroke),
            label_fill: Color::new(t.label_fill),
            label_fill_hovered: Color::new(t.label_fill_hovered),
            label_halo: Color::new(t.label_halo),
            hover_fill: Color::new(t.node_fill_hovered),
            selection_fill: Color::new(t.node_fill_selected),
        }
    }

    fn merge_color_field(next: &mut Color, v: &serde_json::Value, key: &str) {
        theme::merge_color_field(next, v, key);
    }

    fn merge_from_json(&mut self, json: &str) -> Result<(), EditorError> {
        let v: serde_json::Value = serde_json::from_str(json)?;
        let mut next = *self;
        Self::merge_color_field(&mut next.raster_clear, &v, "rasterClear");
        Self::merge_color_field(&mut next.grid_minor_stroke, &v, "gridMinorStroke");
        Self::merge_color_field(&mut next.label_fill, &v, "labelFill");
        Self::merge_color_field(&mut next.label_fill_hovered, &v, "labelFillHovered");
        Self::merge_color_field(&mut next.label_halo, &v, "labelHalo");
        Self::merge_color_field(&mut next.hover_fill, &v, "nodeFillHovered");
        Self::merge_color_field(&mut next.selection_fill, &v, "nodeFillSelected");
        *self = next;
        Ok(())
    }
}
// #endregion 🔖️Theme

// #region 🔖️EditorViewport
const PAD_X: f64 = 12.0;
const PAD_Y: f64 = 8.0;
const DEFAULT_GUTTER_WIDTH: f64 = 56.0;
const DEFAULT_FONT_PX: f64 = 14.0;
const DEFAULT_LINE_HEIGHT: f64 = 22.0;
const DEFAULT_TAB_SIZE: usize = 2;

fn editor_screen_to_world(camera: &Camera, p: Point) -> Point {
    Point::new(p.x + camera.x, p.y + camera.y)
}

fn editor_world_to_screen(camera: &Camera, p: Point) -> Point {
    Point::new(p.x - camera.x, p.y - camera.y)
}

fn editor_content_affine(camera: &Camera) -> Affine {
    Affine::new([1.0, 0.0, 0.0, 1.0, -camera.x, -camera.y])
}
// #endregion 🔖️EditorViewport

// #region 🔖️EditorState

#[derive(Clone, Debug, Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct EditorSettingsJson {
    #[serde(default = "default_font_px")]
    #[value(default = "default_font_px")]
    font_px: f64,
    #[serde(default = "default_line_height")]
    #[value(default = "default_line_height")]
    line_height: f64,
    #[serde(default = "default_show_line_numbers")]
    #[value(default = "default_show_line_numbers")]
    show_line_numbers: bool,
    #[serde(default = "default_tab_size")]
    #[value(default = "default_tab_size")]
    tab_size: usize,
}

fn default_font_px() -> f64 {
    DEFAULT_FONT_PX
}

fn default_line_height() -> f64 {
    DEFAULT_LINE_HEIGHT
}

fn default_show_line_numbers() -> bool {
    true
}

fn default_tab_size() -> usize {
    DEFAULT_TAB_SIZE
}

#[derive(Clone, Debug, Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct SemanticTokenJson {
    start: usize,
    end: usize,
    class: String,
}

#[derive(Clone, Debug, Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct SelectableSpanJson {
    start: usize,
    end: usize,
    kind: String,
    head_end: Option<usize>,
    tail_start: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct ByteRangeJson {
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct PlaceholderJson {
    offset: usize,
    label: String,
}

#[derive(Clone, Debug, Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct DiagnosticJson {
    start: usize,
    end: usize,
    severity: Option<String>,
    #[allow(dead_code)]
    message: String,
}

#[derive(Clone, Debug, Deserialize, FromValue)]
struct TextEditJson {
    range: TextRangeJson,
    #[serde(rename = "newText")]
    #[value(rename = "newText")]
    new_text: String,
}

#[derive(Clone, Debug, Deserialize, FromValue)]
struct TextRangeJson {
    start: TextPosJson,
    end: TextPosJson,
}

#[derive(Clone, Debug, Deserialize, FromValue)]
struct TextPosJson {
    line: u32,
    character: u32,
}

pub struct EditorHost {
    text: String,
    caret: usize,
    anchor: usize,
    camera: Camera,
    viewport: Viewport,
    semantic_tokens: Vec<SemanticTokenJson>,
    selectable_spans: Vec<SelectableSpanJson>,
    diagnostics: Vec<DiagnosticJson>,
    placeholders: Vec<PlaceholderJson>,
    hover_occurrences: Vec<ByteRangeJson>,
    selection_occurrences: Vec<ByteRangeJson>,
    extra_carets: Vec<usize>,
    font_px: f64,
    line_height: f64,
    show_line_numbers: bool,
    tab_size: usize,
    drag_selecting: bool,
    hover_token_start: Option<usize>,
    hover_token_end: Option<usize>,
    theme: EditorCanvasTheme,
    caret_visible: bool,
    dead_line_y: f64,
    chrome_edgeless_scroll: bool,
}

/// 🧹️ Retained text-editor owner that releases one scalar or collection item per close grant.
pub struct EditorHostRetirement {
    text: String,
    semantic_tokens: Vec<SemanticTokenJson>,
    selectable_spans: Vec<SelectableSpanJson>,
    diagnostics: Vec<DiagnosticJson>,
    placeholders: Vec<PlaceholderJson>,
    hover_occurrences: Vec<ByteRangeJson>,
    selection_occurrences: Vec<ByteRangeJson>,
    extra_carets: Vec<usize>,
    released: bool,
}

impl EditorHostRetirement {
    pub fn new(host: EditorHost) -> Self {
        let EditorHost {
            text,
            caret: _,
            anchor: _,
            camera: _,
            viewport: _,
            semantic_tokens,
            selectable_spans,
            diagnostics,
            placeholders,
            hover_occurrences,
            selection_occurrences,
            extra_carets,
            font_px: _,
            line_height: _,
            show_line_numbers: _,
            tab_size: _,
            drag_selecting: _,
            hover_token_start: _,
            hover_token_end: _,
            theme: _,
            caret_visible: _,
            dead_line_y: _,
            chrome_edgeless_scroll: _,
        } = host;
        Self { text, semantic_tokens, selectable_spans, diagnostics, placeholders, hover_occurrences, selection_occurrences, extra_carets, released: false }
    }

    pub fn close_step(&mut self) -> bool {
        if self.released {
            return true;
        }
        if self.text.pop().is_some()
            || self.semantic_tokens.pop().is_some()
            || self.selectable_spans.pop().is_some()
            || self.diagnostics.pop().is_some()
            || self.placeholders.pop().is_some()
            || self.hover_occurrences.pop().is_some()
            || self.selection_occurrences.pop().is_some()
            || self.extra_carets.pop().is_some()
        {
            return false;
        }
        self.released = true;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.released
            && self.text.is_empty()
            && self.semantic_tokens.is_empty()
            && self.selectable_spans.is_empty()
            && self.diagnostics.is_empty()
            && self.placeholders.is_empty()
            && self.hover_occurrences.is_empty()
            && self.selection_occurrences.is_empty()
            && self.extra_carets.is_empty()
    }
}

impl Drop for EditorHostRetirement {
    fn drop(&mut self) {
        debug_assert!(self.terminal_is_empty(), "EditorHostRetirement must reach terminal-empty before release");
    }
}

impl Default for EditorHost {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorHost {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            caret: 0,
            anchor: 0,
            camera: Camera { x: 0.0, y: 0.0, zoom: 1.0 },
            viewport: Viewport { width: 800, height: 600, dpr: 1.0 },
            semantic_tokens: Vec::new(),
            selectable_spans: Vec::new(),
            diagnostics: Vec::new(),
            placeholders: Vec::new(),
            hover_occurrences: Vec::new(),
            selection_occurrences: Vec::new(),
            extra_carets: Vec::new(),
            font_px: DEFAULT_FONT_PX,
            line_height: DEFAULT_LINE_HEIGHT,
            show_line_numbers: true,
            tab_size: DEFAULT_TAB_SIZE,
            drag_selecting: false,
            hover_token_start: None,
            hover_token_end: None,
            theme: EditorCanvasTheme::default(),
            caret_visible: true,
            dead_line_y: 0.0,
            chrome_edgeless_scroll: false,
        }
    }

    pub fn set_dead_line_y(&mut self, y: f64) {
        self.dead_line_y = y.max(0.0);
        self.clamp_camera();
    }

    pub fn set_chrome_edgeless_scroll(&mut self, enabled: bool) {
        self.chrome_edgeless_scroll = enabled;
        self.clamp_camera();
    }

    pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), EditorError> {
        self.theme.merge_from_json(json)
    }

    pub fn set_caret_visible(&mut self, visible: bool) {
        self.caret_visible = visible;
    }

    pub fn set_editor_settings_json(&mut self, json: &str) {
        let settings: EditorSettingsJson = serde_json::from_str(json).unwrap_or(EditorSettingsJson { font_px: DEFAULT_FONT_PX, line_height: DEFAULT_LINE_HEIGHT, show_line_numbers: true, tab_size: DEFAULT_TAB_SIZE });
        self.font_px = settings.font_px.clamp(10.0, 28.0);
        self.line_height = settings.line_height.clamp(16.0, 48.0);
        self.show_line_numbers = settings.show_line_numbers;
        self.tab_size = settings.tab_size.clamp(1, 8);
        self.clamp_camera();
    }

    pub fn tab_insert_text(&self) -> String {
        " ".repeat(self.tab_size)
    }

    fn gutter_width(&self) -> f64 {
        if self.show_line_numbers {
            DEFAULT_GUTTER_WIDTH
        } else {
            0.0
        }
    }

    fn line_origin_x(&self) -> f64 {
        self.gutter_width() + PAD_X
    }

    fn content_origin_y(&self) -> f64 {
        if self.chrome_edgeless_scroll || self.dead_line_y <= 0.0 {
            0.0
        } else {
            self.dead_line_y
        }
    }

    fn line_y(&self, line: usize) -> f64 {
        self.content_origin_y() + PAD_Y + line as f64 * self.line_height + self.line_height * 0.75
    }

    fn line_top_y(&self, line: usize) -> f64 {
        self.content_origin_y() + PAD_Y + line as f64 * self.line_height
    }

    fn content_height(&self, line_count: usize) -> f64 {
        self.content_origin_y() + PAD_Y * 2.0 + line_count as f64 * self.line_height
    }

    fn rest_content_height(&self, line_count: usize) -> f64 {
        let rest_origin = if self.dead_line_y > 0.0 { self.dead_line_y } else { 0.0 };
        rest_origin + PAD_Y * 2.0 + line_count as f64 * self.line_height
    }

    fn scroll_overflows(&self) -> bool {
        let line_count = self.text.matches('\n').count() + 1;
        self.rest_content_height(line_count) > self.viewport.height as f64
    }

    pub fn chrome_edgeless_scroll(&self) -> bool {
        self.chrome_edgeless_scroll
    }

    fn gutter_number_x(&self, label: &str) -> f64 {
        let advance = canvas_text::label_advance(label, self.font_px);
        (self.gutter_width() - PAD_X * 0.75 - advance).max(4.0)
    }

    fn clamp_camera(&mut self) {
        self.camera.x = 0.0;
        self.camera.zoom = 1.0;
        let line_count = self.text.matches('\n').count() + 1;
        let content_h = self.content_height(line_count);
        let view_h = self.viewport.height as f64;
        let scroll_max = (content_h - view_h).max(0.0);
        self.camera.y = self.camera.y.clamp(0.0, scroll_max);
    }

    fn scroll_caret_into_view(&mut self) {
        let (line, _) = offset_line_col(&self.text, self.caret);
        let caret_top = self.line_top_y(line);
        let view_h = self.viewport.height as f64;
        let top = self.camera.y;
        let bottom = top + view_h - self.line_height;
        if caret_top < top {
            self.camera.y = caret_top;
        } else if caret_top + self.line_height > bottom {
            self.camera.y = (caret_top + self.line_height - view_h).max(0.0);
        }
        self.clamp_camera();
    }

    fn finish_caret_update(&mut self) {
        self.scroll_caret_into_view();
    }

    pub fn anchor(&self) -> usize {
        self.anchor
    }

    #[cfg(test)]
    fn set_caret_anchor(&mut self, offset: usize) {
        self.caret = offset;
        self.anchor = offset;
    }

    #[cfg(test)]
    fn set_selection(&mut self, anchor: usize, caret: usize) {
        self.set_selection_range(anchor, caret);
    }

    pub fn set_selection_range(&mut self, anchor: usize, caret: usize) {
        self.anchor = anchor.min(self.text.len());
        self.caret = caret.min(self.text.len());
        if self.caret != self.anchor {
            let (start, end) = self.normalize_edit_range(self.caret.min(self.anchor), self.caret.max(self.anchor));
            self.anchor = start;
            self.caret = end;
        }
        self.finish_caret_update();
    }

    pub fn hover_token_range(&self) -> Option<(usize, usize)> {
        match (self.hover_token_start, self.hover_token_end) {
            (Some(start), Some(end)) => Some((start, end)),
            _ => None,
        }
    }

    pub fn set_hover_range(&mut self, start: Option<usize>, end: Option<usize>) {
        self.hover_token_start = start;
        self.hover_token_end = end;
    }

    pub fn select_all(&mut self) {
        self.anchor = 0;
        self.caret = self.text.len();
    }

    pub fn set_text(&mut self, text: String) {
        if self.text == text {
            return;
        }
        self.text = text;
        self.caret = self.caret.min(self.text.len());
        self.anchor = self.anchor.min(self.text.len());
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn caret(&self) -> usize {
        self.caret
    }

    pub fn set_semantic_tokens_json(&mut self, json: &str) {
        self.semantic_tokens = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn set_selectable_spans_json(&mut self, json: &str) {
        self.selectable_spans = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn select_span_at(&mut self, offset: usize) {
        let offset = offset.min(self.text.len());
        if let Some(ch) = self.text[offset..].chars().next() {
            if ch == ':' || ch == '.' {
                for span in &self.selectable_spans {
                    if span.kind != "atomic" && offset >= span.start && offset < span.end {
                        self.anchor = span.start;
                        self.caret = span.end;
                        self.finish_caret_update();
                        return;
                    }
                }
            }
        }
        let probe = if offset > 0 && (!self.text.is_char_boundary(offset) || offset == self.text.len()) { prev_char_boundary(&self.text, offset) } else { offset };
        let mut best: Option<&SelectableSpanJson> = None;
        for span in &self.selectable_spans {
            if span.kind != "atomic" || probe < span.start || probe >= span.end {
                continue;
            }
            let size = span.end - span.start;
            if best.is_none_or(|current| size < current.end - current.start) {
                best = Some(span);
            }
        }
        if let Some(span) = best {
            self.anchor = span.start;
            self.caret = span.end;
        } else {
            let snapped = self.snap_offset_for_atomic(offset);
            self.anchor = snapped;
            self.caret = snapped;
        }
        self.finish_caret_update();
    }

    pub fn selection_text(&self) -> String {
        let (start, end) = self.selection_range();
        if start >= end {
            return String::new();
        }
        self.text[start..end].to_string()
    }

    pub fn replace_selection(&mut self, next: &str) {
        let (start, end) = self.selection_range();
        if start >= end {
            self.insert_text(next);
            return;
        }
        self.text.replace_range(start..end, next);
        self.caret = start + next.len();
        self.anchor = self.caret;
    }

    pub fn set_diagnostics_json(&mut self, json: &str) {
        self.diagnostics = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn set_placeholders_json(&mut self, json: &str) {
        self.placeholders = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn set_hover_occurrences_json(&mut self, json: &str) {
        self.hover_occurrences = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn set_selection_occurrences_json(&mut self, json: &str) {
        self.selection_occurrences = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn set_extra_carets_json(&mut self, json: &str) {
        self.extra_carets = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn apply_text_edits_json(&mut self, json: &str) {
        let edits: Vec<TextEditJson> = serde_json::from_str(json).unwrap_or_default();
        let mut text = self.text.clone();
        let mut sorted = edits;
        sorted.sort_by_key(|edit| std::cmp::Reverse(position_to_offset(&text, &edit.range.start)));
        for edit in sorted {
            let start = position_to_offset(&text, &edit.range.start);
            let end = position_to_offset(&text, &edit.range.end).max(start);
            text.replace_range(start..end, &edit.new_text);
        }
        self.set_text(text);
    }

    pub fn set_camera(&mut self, _x: f64, y: f64, _zoom: f64) {
        self.camera.x = 0.0;
        self.camera.y = y;
        self.camera.zoom = 1.0;
        self.clamp_camera();
    }

    pub fn camera_json(&self) -> String {
        serde_json::json!({ "x": 0, "y": self.camera.y, "zoom": 1 }).to_string()
    }

    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport.width = width.max(1);
        self.viewport.height = height.max(1);
        self.viewport.dpr = dpr.max(1.0);
        self.clamp_camera();
    }

    pub fn sync_from_scene_json(&mut self, json: &str) -> Result<(), EditorError> {
        let value: serde_json::Value = serde_json::from_str(json)?;
        self.sync_from_scene_value(&value);
        Ok(())
    }

    pub fn sync_from_scene_pack(&mut self, bytes: &[u8]) -> Result<(), EditorError> {
        // 📦️ Host TS `encodePackValue` is the wire-body twin of `encode_wire_value` (no SPK shell);
        // accept that first, then fall back to `decode_pack_value` for native pack-shell callers/tests.
        let dsl = store::pack_rt::decode_wire_value(bytes).or_else(|_| store::pack_rt::decode_pack_value(bytes))?;
        let value = store::pack_rt::dsl_value_to_json(dsl);
        self.sync_from_scene_value(&value);
        Ok(())
    }

    fn expand_scene_json_field(raw: &str) -> String {
        store::pack_rt::scene_field_json_text(raw).unwrap_or_else(|_| raw.to_string())
    }

    fn sync_from_scene_value(&mut self, value: &serde_json::Value) {
        if let Some(buffer) = value.get("buffer").and_then(|v| v.as_str()) {
            self.set_text(buffer.to_string());
        }
        if let Some(json) = value.get("selectionJson").and_then(|v| v.as_str()) {
            let json = Self::expand_scene_json_field(json);
            if let Ok(range) = serde_json::from_str::<serde_json::Value>(&json) {
                let start = range.get("start").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                let end = range.get("end").and_then(|v| v.as_u64()).unwrap_or(start as u64) as usize;
                self.set_selection_range(start, end);
            }
        }
        if let Some(json) = value.get("tokensJson").and_then(|v| v.as_str()) {
            self.set_semantic_tokens_json(&Self::expand_scene_json_field(json));
        }
        if let Some(json) = value.get("diagnosticsJson").and_then(|v| v.as_str()) {
            self.set_diagnostics_json(&Self::expand_scene_json_field(json));
        }
        if let Some(json) = value.get("placeholdersJson").and_then(|v| v.as_str()) {
            self.set_placeholders_json(&Self::expand_scene_json_field(json));
        }
        if let Some(json) = value.get("occurrencesJson").and_then(|v| v.as_str()) {
            let json = Self::expand_scene_json_field(json);
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json) {
                if let Some(hover) = value.get("hover").and_then(|v| v.as_str()) {
                    self.set_hover_occurrences_json(hover);
                }
                if let Some(selection) = value.get("selection").and_then(|v| v.as_str()) {
                    self.set_selection_occurrences_json(selection);
                }
            }
        }
        if let Some(json) = value.get("extraCaretsJson").and_then(|v| v.as_str()) {
            self.set_extra_carets_json(&Self::expand_scene_json_field(json));
        }
        if let Some(json) = value.get("selectableSpansJson").and_then(|v| v.as_str()) {
            self.set_selectable_spans_json(&Self::expand_scene_json_field(json));
        }
        if let Some(json) = value.get("settingsJson").and_then(|v| v.as_str()) {
            self.set_editor_settings_json(&Self::expand_scene_json_field(json));
        }
        if let Some(json) = value.get("cameraJson").and_then(|v| v.as_str()) {
            let json = Self::expand_scene_json_field(json);
            if let Ok(camera) = serde_json::from_str::<serde_json::Value>(&json) {
                let y = camera.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                self.set_camera(0.0, y, 1.0);
            }
        }
        if let Some(json) = value.get("overlaysJson").and_then(|v| v.as_str()) {
            let json = Self::expand_scene_json_field(json);
            if let Ok(overlays) = serde_json::from_str::<serde_json::Value>(&json) {
                if let Some(y) = overlays.get("deadLineY").and_then(|v| v.as_f64()) {
                    self.set_dead_line_y(y);
                }
            }
        }
        if let Some(json) = value.get("hoverJson").and_then(|v| v.as_str()) {
            let json = Self::expand_scene_json_field(json);
            match serde_json::from_str::<serde_json::Value>(&json) {
                Ok(serde_json::Value::Object(range)) => {
                    let start = range.get("start").and_then(|v| v.as_u64()).map(|v| v as usize);
                    let end = range.get("end").and_then(|v| v.as_u64()).map(|v| v as usize);
                    self.set_hover_range(start, end);
                }
                _ => self.set_hover_range(None, None),
            }
        }
    }

    pub fn wheel_scroll_screen(&mut self, delta_y: f64) {
        if !self.scroll_overflows() {
            return;
        }
        let at_top = self.camera.y <= f64::EPSILON;
        if at_top && self.dead_line_y > 0.0 {
            if delta_y < 0.0 && !self.chrome_edgeless_scroll {
                self.chrome_edgeless_scroll = true;
                return;
            }
            if delta_y > 0.0 && self.chrome_edgeless_scroll {
                self.chrome_edgeless_scroll = false;
                return;
            }
        }
        self.camera.y = (self.camera.y + delta_y * 0.5).max(0.0);
        self.clamp_camera();
    }

    /// 🖱️➡️ W4 fix: every button repositions the caret to the click point — a right-click's context-menu
    /// UX wants the same "click lands the caret here" behavior a left click gets (see
    /// `framework/renderer/wgpu`'s text-editor context-menu caller, which used to work around this by
    /// forcing `button` to `0` before calling in, since this fn used to no-op entirely for `button != 0`).
    /// Only a primary-button (`button == 0`) press also starts a drag-selection — a right/middle click
    /// must never extend the current selection.
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: i32) {
        let world = editor_screen_to_world(&self.camera, Point::new(sx, sy));
        let offset = self.snap_offset_for_atomic(self.hit_test_offset(world));
        self.caret = offset;
        self.anchor = offset;
        self.set_hover_at_offset(offset);
        if button == 0 {
            self.drag_selecting = true;
        }
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, _buttons: i32) {
        if self.drag_selecting {
            let world = editor_screen_to_world(&self.camera, Point::new(sx, sy));
            self.caret = self.snap_offset_for_atomic(self.hit_test_offset(world));
            return;
        }
        let world = editor_screen_to_world(&self.camera, Point::new(sx, sy));
        self.set_hover_at_offset(self.hit_test_offset(world));
    }

    pub fn pointer_up_screen(&mut self, _sx: f64, _sy: f64, button: i32) {
        if button == 0 {
            self.drag_selecting = false;
            if self.caret != self.anchor {
                let (start, end) = self.normalize_edit_range(self.caret.min(self.anchor), self.caret.max(self.anchor));
                self.anchor = start;
                self.caret = end;
            }
            self.finish_caret_update();
        }
    }

    pub fn insert_text(&mut self, chunk: &str) {
        let mut start = self.caret.min(self.anchor);
        let mut end = self.caret.max(self.anchor);
        let collapsed = start == end;
        let mut insert = chunk.to_string();
        if collapsed && self.should_prefix_auto_space(start, &insert) {
            insert = format!(" {insert}");
        }
        if collapsed && is_insert_whitespace(&insert) {
            self.text.insert_str(start, &insert);
            self.caret = start + insert.len();
            self.anchor = self.caret;
            return;
        }
        if collapsed {
            for token in &self.semantic_tokens {
                if start > token.start && start < token.end {
                    start = token.start;
                    end = token.end;
                    break;
                }
            }
        }
        let (start, end) = self.normalize_edit_range(start, end);
        self.text.replace_range(start..end, &insert);
        self.caret = start + insert.len();
        self.anchor = self.caret;
        self.finish_caret_update();
    }

    fn should_prefix_auto_space(&self, offset: usize, chunk: &str) -> bool {
        let first = match chunk.chars().next() {
            Some(ch) => ch,
            None => return false,
        };
        if is_insert_whitespace(chunk) {
            return false;
        }
        if matches!(first, ':' | '.' | ',' | ')' | ']' | '-' | '!' | '=') {
            return false;
        }
        if self.token_ending_at(offset).is_none() {
            return false;
        }
        if offset < self.text.len() {
            let next = self.text[offset..].chars().next().unwrap_or(' ');
            if next.is_whitespace() {
                return false;
            }
        }
        true
    }

    fn token_ending_at(&self, offset: usize) -> Option<&SemanticTokenJson> {
        self.semantic_tokens.iter().find(|token| token.end == offset && token.start < offset)
    }

    pub fn backspace(&mut self) {
        if self.caret != self.anchor {
            let (start, end) = self.normalize_edit_range(self.caret.min(self.anchor), self.caret.max(self.anchor));
            self.caret = start;
            self.anchor = end;
            self.insert_text("");
            return;
        }
        if self.caret == 0 {
            return;
        }
        for token in &self.semantic_tokens {
            if self.caret > token.start && self.caret <= token.end {
                self.text.replace_range(token.start..token.end, "");
                self.caret = token.start;
                self.anchor = self.caret;
                return;
            }
        }
        let prev = prev_char_boundary(&self.text, self.caret);
        self.text.replace_range(prev..self.caret, "");
        self.caret = prev;
        self.anchor = self.caret;
    }

    pub fn delete_forward(&mut self) {
        if self.caret != self.anchor {
            let (start, end) = self.normalize_edit_range(self.caret.min(self.anchor), self.caret.max(self.anchor));
            self.caret = start;
            self.anchor = end;
            self.insert_text("");
            return;
        }
        if self.caret >= self.text.len() {
            return;
        }
        for token in &self.semantic_tokens {
            if self.caret >= token.start && self.caret < token.end {
                self.text.replace_range(token.start..token.end, "");
                self.anchor = self.caret;
                return;
            }
        }
        let next = next_char_boundary(&self.text, self.caret);
        self.text.replace_range(self.caret..next, "");
        self.anchor = self.caret;
    }

    pub fn move_line_start(&mut self, extend: bool) {
        let (line, _) = offset_line_col(&self.text, self.caret);
        self.caret = offset_at_line_col(&self.text, line, 0);
        if !extend {
            self.anchor = self.caret;
            self.finish_caret_update();
        }
    }

    pub fn move_line_end(&mut self, extend: bool) {
        let (line, _) = offset_line_col(&self.text, self.caret);
        let line_len = self.text.split('\n').nth(line).map_or(0, str::len);
        self.caret = offset_at_line_col(&self.text, line, line_len);
        if !extend {
            self.anchor = self.caret;
            self.finish_caret_update();
        }
    }

    pub fn move_left(&mut self, extend: bool) {
        let next = self.token_left_boundary(self.caret).unwrap_or_else(|| if self.caret == 0 { 0 } else { prev_char_boundary(&self.text, self.caret) });
        self.caret = next;
        if !extend {
            self.anchor = self.caret;
            self.finish_caret_update();
        }
    }

    pub fn move_right(&mut self, extend: bool) {
        let next = self.token_right_boundary(self.caret).unwrap_or_else(|| if self.caret >= self.text.len() { self.text.len() } else { next_char_boundary(&self.text, self.caret) });
        self.caret = next;
        if !extend {
            self.anchor = self.caret;
            self.finish_caret_update();
        }
    }

    pub fn move_up(&mut self, extend: bool) {
        let (line, col) = offset_line_col(&self.text, self.caret);
        if line == 0 {
            self.caret = 0;
        } else {
            self.caret = offset_at_line_col(&self.text, line - 1, col);
        }
        if !extend {
            self.anchor = self.caret;
            self.finish_caret_update();
        }
    }

    pub fn move_down(&mut self, extend: bool) {
        let (line, col) = offset_line_col(&self.text, self.caret);
        let max_line = self.text.matches('\n').count();
        self.caret = offset_at_line_col(&self.text, (line + 1).min(max_line), col);
        if !extend {
            self.anchor = self.caret;
            self.finish_caret_update();
        }
    }

    pub fn world_to_screen_json(&self, wx: f64, wy: f64) -> String {
        let p = editor_world_to_screen(&self.camera, Point::new(wx, wy));
        serde_json::json!({ "x": p.x, "y": p.y }).to_string()
    }

    pub fn caret_world_json(&self) -> String {
        let (x, y) = offset_to_world(self, self.caret);
        serde_json::json!({ "x": x, "y": y }).to_string()
    }

    fn set_hover_at_offset(&mut self, offset: usize) {
        self.hover_token_start = None;
        self.hover_token_end = None;
        if let Some(span) = self.token_span_at_offset(offset) {
            self.hover_token_start = Some(span.0);
            self.hover_token_end = Some(span.1);
        }
    }

    fn token_span_at_offset(&self, offset: usize) -> Option<(usize, usize)> {
        for token in &self.semantic_tokens {
            if offset >= token.start && offset < token.end {
                return Some((token.start, token.end));
            }
        }
        None
    }

    fn snap_offset_for_atomic(&self, offset: usize) -> usize {
        for token in &self.semantic_tokens {
            if offset > token.start && offset < token.end {
                let mid = token.start + (token.end - token.start) / 2;
                return if offset < mid { token.start } else { token.end };
            }
        }
        offset
    }

    fn normalize_edit_range(&self, start: usize, end: usize) -> (usize, usize) {
        let (mut s, mut e) = if start <= end { (start, end) } else { (end, start) };
        loop {
            let mut changed = false;
            for token in &self.semantic_tokens {
                if s < token.end && e > token.start && (s > token.start || e < token.end) {
                    if s > token.start {
                        s = token.start;
                        changed = true;
                    }
                    if e < token.end {
                        e = token.end;
                        changed = true;
                    }
                }
            }
            for span in &self.selectable_spans {
                if span.kind == "atomic" || !ranges_overlap(s, e, span.start, span.end) {
                    continue;
                }
                if self.allowed_composite_selection(s, e, span) {
                    continue;
                }
                if s > span.start {
                    s = span.start;
                    changed = true;
                }
                if e < span.end {
                    e = span.end;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        if start <= end {
            (s, e)
        } else {
            (e, s)
        }
    }

    fn allowed_composite_selection(&self, start: usize, end: usize, span: &SelectableSpanJson) -> bool {
        if start == span.start && end == span.end {
            return true;
        }
        match span.kind.as_str() {
            "varLabel" => {
                let head_end = span.head_end.unwrap_or(span.end);
                start == span.start && end == head_end
            }
            "propertyAccess" => {
                let head_end = span.head_end.unwrap_or(span.start);
                let tail_start = span.tail_start.unwrap_or(span.end);
                (start == span.start && end == head_end) || (start == tail_start && end == span.end)
            }
            _ => false,
        }
    }

    fn token_left_boundary(&self, offset: usize) -> Option<usize> {
        for token in &self.semantic_tokens {
            if offset > token.start && offset <= token.end {
                return Some(token.start);
            }
        }
        None
    }

    fn token_right_boundary(&self, offset: usize) -> Option<usize> {
        for token in &self.semantic_tokens {
            if offset >= token.start && offset < token.end {
                return Some(token.end);
            }
        }
        None
    }

    fn selection_range(&self) -> (usize, usize) {
        self.normalize_edit_range(self.caret.min(self.anchor), self.caret.max(self.anchor))
    }

    fn text_fill_for_abs_range(&self, start: usize, end: usize) -> Color {
        let (sel_s, sel_e) = self.selection_range();
        if sel_s != sel_e && ranges_overlap(start, end, sel_s, sel_e) {
            return self.theme.label_fill_hovered;
        }
        if let (Some(hs), Some(he)) = (self.hover_token_start, self.hover_token_end) {
            if ranges_overlap(start, end, hs, he) {
                return self.theme.label_fill_hovered;
            }
        }
        self.theme.label_fill
    }

    fn render_abs_range_highlight(&self, scene: &mut Scene, start: usize, end: usize, color: Color) {
        if start >= end {
            return;
        }
        let origin_x = self.line_origin_x();
        let font_px = self.font_px;
        let (s_line, s_byte) = offset_line_col(&self.text, start);
        let (e_line, e_byte) = offset_line_col(&self.text, end);
        if s_line == e_line {
            let line_text = self.text.split('\n').nth(s_line).unwrap_or("");
            let y = self.line_y(s_line);
            let (x0, x1) = canvas_text::label_span_world_x(line_text, s_byte, e_byte, origin_x, font_px);
            self.fill_highlight_rect(scene, x0, x1, y, color);
            return;
        }
        for line in s_line..=e_line {
            let line_text = self.text.split('\n').nth(line).unwrap_or("");
            let y = self.line_y(line);
            let byte_start = if line == s_line { s_byte } else { 0 };
            let byte_end = if line == e_line { e_byte } else { line_text.len() };
            let (x0, x1) = canvas_text::label_span_world_x(line_text, byte_start, byte_end, origin_x, font_px);
            self.fill_highlight_rect(scene, x0, x1, y, color);
        }
    }

    fn fill_highlight_rect(&self, scene: &mut Scene, x0: f64, x1: f64, y: f64, fill: Color) {
        let left = x0.min(x1);
        let right = x0.max(x1);
        if right <= left {
            return;
        }
        let lh = self.line_height;
        let rect = Rect::new(left, y - lh * 0.8, right, y + lh * 0.2);
        scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &rect);
    }

    fn hit_test_offset(&self, world: Point) -> usize {
        let rel_x = world.x;
        let rel_y = world.y;
        let origin = self.content_origin_y();
        if rel_y < origin + PAD_Y {
            return 0;
        }
        let line = ((rel_y - origin - PAD_Y) / self.line_height).floor().max(0.0) as usize;
        let max_line = self.text.matches('\n').count();
        let line = line.min(max_line);
        let line_text = self.text.split('\n').nth(line).unwrap_or("");
        let col = hit_byte_in_line(line_text, rel_x, self.line_origin_x(), self.font_px);
        offset_at_line_col(&self.text, line, col)
    }

    pub fn hit_test_offset_screen(&self, sx: f64, sy: f64) -> usize {
        let world = editor_screen_to_world(&self.camera, Point::new(sx, sy));
        self.hit_test_offset(world)
    }

    /// @emoji 🎯️ Returns pick-target rows at a screen point for DOM disambiguation menus.
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        let offset = self.hit_test_offset_screen(sx, sy);
        let (line, _col) = offset_line_col(&self.text, offset);
        let mut rows = Vec::new();
        rows.push(serde_json::json!({
            "domain": "line",
            "id": line.to_string(),
            "generality": 0,
            "label": format!("Line {}", line + 1),
        }));
        if let Some((start, end)) = self.token_span_at_offset(offset) {
            rows.push(serde_json::json!({
                "domain": "token",
                "id": format!("{start}:{end}"),
                "generality": 2,
                "label": self.text.get(start..end).unwrap_or("").to_string(),
            }));
        }
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
    }

    pub fn select_span_at_screen(&mut self, sx: f64, sy: f64) {
        let offset = self.hit_test_offset_screen(sx, sy);
        self.select_span_at(offset);
    }

    pub fn build_scene(&self) -> Scene {
        let mut world_scene = Scene::new();
        let bg = self.theme.raster_clear;
        world_scene.fill(FillRule::NonZero, Affine::IDENTITY, bg, None, &Rect::new(-10_000.0, -10_000.0, 10_000.0, 10_000.0));
        let lines: Vec<&str> = if self.text.is_empty() { vec![""] } else { self.text.split('\n').collect() };
        let content_h = self.content_height(lines.len());
        if self.show_line_numbers {
            let gutter_bg = self.theme.grid_minor_stroke.multiply_alpha(0.12);
            world_scene.fill(FillRule::NonZero, Affine::IDENTITY, gutter_bg, None, &Rect::new(0.0, 0.0, self.gutter_width(), content_h));
            world_scene.fill(FillRule::NonZero, Affine::IDENTITY, self.theme.grid_minor_stroke.multiply_alpha(0.35), None, &Rect::new(self.gutter_width() - 1.0, 0.0, self.gutter_width(), content_h));
        }
        let (sel_start, sel_end) = self.selection_range();
        if !self.selection_occurrences.is_empty() {
            for range in &self.selection_occurrences {
                self.render_abs_range_highlight(&mut world_scene, range.start, range.end, self.theme.selection_fill);
            }
        } else if sel_start != sel_end {
            self.render_abs_range_highlight(&mut world_scene, sel_start, sel_end, self.theme.selection_fill);
        }
        if !self.hover_occurrences.is_empty() {
            for range in &self.hover_occurrences {
                self.render_abs_range_highlight(&mut world_scene, range.start, range.end, self.theme.hover_fill);
            }
        }
        for (i, line) in lines.iter().enumerate() {
            let y = self.line_y(i);
            if self.show_line_numbers {
                let gutter = format!("{}", i + 1);
                canvas_text::append_label(&mut world_scene, &gutter, Point::new(self.gutter_number_x(&gutter), y), self.font_px, self.theme.label_fill.multiply_alpha(0.62), self.theme.label_halo);
            }
            if self.hover_occurrences.is_empty() {
                if let (Some(hs), Some(he)) = (self.hover_token_start, self.hover_token_end) {
                    if sel_start == sel_end || he <= sel_start || hs >= sel_end {
                        let line_start = offset_at_line_col(&self.text, i, 0);
                        let line_end = line_start + line.len();
                        if hs < line_end && he > line_start {
                            let start = hs.max(line_start) - line_start;
                            let end = he.min(line_end) - line_start;
                            let abs_s = line_start + start;
                            let abs_e = line_start + end;
                            self.render_abs_range_highlight(&mut world_scene, abs_s, abs_e, self.theme.hover_fill);
                        }
                    }
                }
            }
            self.render_colored_line(&mut world_scene, line, i, y);
        }
        self.render_placeholders(&mut world_scene);
        for offset in &self.extra_carets {
            if *offset != self.caret {
                self.render_caret_bar(&mut world_scene, *offset);
            }
        }
        self.render_caret(&mut world_scene, self.caret);
        for diag in &self.diagnostics {
            self.render_diagnostic(&mut world_scene, diag);
        }
        let aff = editor_content_affine(&self.camera);
        let mut scene = Scene::new();
        scene.append(&world_scene, Some(aff));
        render::scale_scene_for_device_pixel_ratio(scene, self.viewport.dpr)
    }

    fn render_colored_line(&self, scene: &mut Scene, line: &str, line_index: usize, y: f64) {
        if line.is_empty() {
            return;
        }
        let line_start = offset_at_line_col(&self.text, line_index, 0);
        let line_end = line_start + line.len();
        let mut cursor = 0usize;
        let mut spans = Vec::new();
        for token in &self.semantic_tokens {
            if token.end <= line_start || token.start >= line_end {
                continue;
            }
            let start = token.start.saturating_sub(line_start);
            let end = token.end.min(line_end).saturating_sub(line_start);
            if start > cursor {
                spans.push((cursor, start, "plain"));
            }
            spans.push((start, end, token.class.as_str()));
            cursor = end;
        }
        if cursor < line.len() {
            spans.push((cursor, line.len(), "plain"));
        }
        if spans.is_empty() {
            canvas_text::append_label(scene, line, Point::new(self.line_origin_x(), y), self.font_px, self.theme.label_fill, self.theme.label_halo);
            return;
        }
        let color_spans: Vec<(usize, usize, Color)> = spans
            .iter()
            .map(|(start, end, _class)| {
                let abs_s = line_start + start;
                let abs_e = line_start + end;
                (*start, *end, self.text_fill_for_abs_range(abs_s, abs_e))
            })
            .collect();
        canvas_text::append_label_tspans(scene, line, &color_spans, Point::new(self.line_origin_x(), y), self.font_px, self.theme.label_halo);
    }

    fn render_placeholders(&self, scene: &mut Scene) {
        for placeholder in &self.placeholders {
            let (x, y) = offset_to_world(self, placeholder.offset);
            canvas_text::append_label(scene, &placeholder.label, Point::new(x, y), self.font_px, self.theme.grid_minor_stroke, self.theme.label_halo);
        }
    }

    fn render_caret_bar(&self, scene: &mut Scene, offset: usize) {
        let (x, y) = offset_to_world(self, offset);
        let lh = self.line_height;
        let rect = Rect::new(x, y - lh * 0.8, x + 1.5, y + lh * 0.2);
        scene.fill(FillRule::NonZero, Affine::IDENTITY, self.theme.label_fill, None, &rect);
    }

    fn render_caret(&self, scene: &mut Scene, offset: usize) {
        if self.caret == self.anchor && !self.caret_visible {
            return;
        }
        self.render_caret_bar(scene, offset);
    }

    fn render_diagnostic(&self, scene: &mut Scene, diag: &DiagnosticJson) {
        let (x, y) = offset_to_world(self, diag.start);
        let (x2, _) = offset_to_world(self, diag.end.max(diag.start + 1));
        let color = match diag.severity.as_deref() {
            Some("warning") => self.theme.grid_minor_stroke,
            _ => self.theme.label_fill_hovered,
        };
        let rect = Rect::new(x, y + 2.0, x2.max(x + 8.0), y + 4.0);
        scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &rect);
    }
}

fn is_insert_whitespace(chunk: &str) -> bool {
    !chunk.is_empty() && chunk.chars().all(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r'))
}

fn ranges_overlap(a_start: usize, a_end: usize, b_start: usize, b_end: usize) -> bool {
    a_start < b_end && b_start < a_end
}

fn hit_byte_in_line(line: &str, world_x: f64, line_origin_x: f64, font_px: f64) -> usize {
    if line.is_empty() {
        return 0;
    }
    let mut boundaries = vec![0usize];
    for (index, _) in line.char_indices() {
        if index > 0 {
            boundaries.push(index);
        }
    }
    if boundaries.last().copied() != Some(line.len()) {
        boundaries.push(line.len());
    }
    for pair in boundaries.windows(2) {
        let start = pair[0];
        let end = pair[1];
        let x0 = canvas_text::label_byte_world_x(line, start, line_origin_x, font_px);
        let x1 = canvas_text::label_byte_world_x(line, end, line_origin_x, font_px);
        if world_x < (x0 + x1) * 0.5 {
            return start;
        }
    }
    line.len()
}

fn offset_line_col(text: &str, offset: usize) -> (usize, usize) {
    let clamped = offset.min(text.len());
    let mut line = 0usize;
    let mut last = 0usize;
    for (i, ch) in text.char_indices() {
        if i >= clamped {
            break;
        }
        if ch == '\n' {
            line += 1;
            last = i + 1;
        }
    }
    (line, clamped - last)
}

fn offset_at_line_col(text: &str, line: usize, col: usize) -> usize {
    let mut current_line = 0usize;
    let mut line_start = 0usize;
    for (i, ch) in text.char_indices() {
        if current_line == line {
            let line_end = text[line_start..].find('\n').map_or(text.len(), |idx| line_start + idx);
            return line_start + col.min(line_end.saturating_sub(line_start));
        }
        if ch == '\n' {
            current_line += 1;
            line_start = i + 1;
        }
    }
    if current_line == line {
        return line_start + col.min(text.len().saturating_sub(line_start));
    }
    text.len()
}

fn offset_to_world(host: &EditorHost, offset: usize) -> (f64, f64) {
    let (line, byte) = offset_line_col(&host.text, offset);
    let line_text = host.text.split('\n').nth(line).unwrap_or("");
    let x = canvas_text::label_byte_world_x(line_text, byte, host.line_origin_x(), host.font_px);
    let y = host.line_y(line);
    (x, y)
}

fn position_to_offset(text: &str, pos: &TextPosJson) -> usize {
    offset_at_line_col(text, pos.line as usize, pos.character as usize)
}

fn prev_char_boundary(text: &str, index: usize) -> usize {
    text[..index].char_indices().next_back().map_or(0, |(i, _)| i)
}

fn next_char_boundary(text: &str, index: usize) -> usize {
    text[index..].char_indices().nth(1).map_or(text.len(), |(i, _)| index + i)
}
// #endregion 🔖️EditorState

// #region 🔖️Wasm
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this session bridge is browser-only
// (attaches an `HtmlCanvasElement`), so it is narrowed to exclude the WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_async::browser::future_to_promise;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use std::cell::RefCell;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use std::rc::Rc;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use web_sys::HtmlCanvasElement;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
struct EditorSessionInner {
    host: EditorHost,
    gpu: gpu_session::CanvasGpuSession,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl EditorSessionInner {
    fn set_logical_size(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.host.set_size(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        let scene = self.host.build_scene();
        self.gpu.render_frame(&scene, self.host.theme.raster_clear)
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
pub struct EditorSession {
    state: Rc<RefCell<EditorSessionInner>>,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
impl EditorSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(EditorSessionInner { host: EditorHost::new(), gpu: gpu_session::CanvasGpuSession::default() })) }
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = detachGpu)]
    pub fn detach_gpu(&mut self) {
        self.state.borrow_mut().gpu.detach();
    }

    #[wasm_bindgen(js_name = attachCanvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
        let inner = self.state.clone();
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        if inner.borrow().gpu.gpu_ready() {
            inner.borrow_mut().set_logical_size(lw, lh, dpr, pw, ph);
            return future_to_promise(async move { Ok(JsValue::UNDEFINED) });
        }
        let canvas = canvas.clone();
        future_to_promise(async move {
            let (render_ctx, renderer, surface) = gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|e| JsValue::from_str(&e))?;
            let mut g = inner.borrow_mut();
            g.set_logical_size(lw, lh, dpr, pw, ph);
            g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen(js_name = setSize)]
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        let lw = width.max(1);
        let lh = height.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        self.state.borrow_mut().set_logical_size(lw, lh, dpr, pw, ph);
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) {
        let _ = self.state.borrow_mut().render_frame_gpu();
    }

    #[wasm_bindgen(js_name = setText)]
    pub fn set_text(&mut self, text: String) {
        self.state.borrow_mut().host.set_text(text);
    }

    #[wasm_bindgen(js_name = syncFromSceneJson)]
    pub fn sync_from_scene_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.sync_from_scene_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = syncFromScenePack)]
    pub fn sync_from_scene_pack(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        self.state.borrow_mut().host.sync_from_scene_pack(bytes).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = text)]
    pub fn text(&self) -> String {
        self.state.borrow().host.text().to_string()
    }

    #[wasm_bindgen(js_name = caret)]
    pub fn caret(&self) -> usize {
        self.state.borrow().host.caret()
    }

    #[wasm_bindgen(js_name = setSelectableSpansJson)]
    pub fn set_selectable_spans_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_selectable_spans_json(json);
    }

    #[wasm_bindgen(js_name = setSemanticTokensJson)]
    pub fn set_semantic_tokens_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_semantic_tokens_json(json);
    }

    #[wasm_bindgen(js_name = setDiagnosticsJson)]
    pub fn set_diagnostics_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_diagnostics_json(json);
    }

    #[wasm_bindgen(js_name = setPlaceholdersJson)]
    pub fn set_placeholders_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_placeholders_json(json);
    }

    #[wasm_bindgen(js_name = setHoverOccurrencesJson)]
    pub fn set_hover_occurrences_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_hover_occurrences_json(json);
    }

    #[wasm_bindgen(js_name = setSelectionOccurrencesJson)]
    pub fn set_selection_occurrences_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_selection_occurrences_json(json);
    }

    #[wasm_bindgen(js_name = setExtraCaretsJson)]
    pub fn set_extra_carets_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_extra_carets_json(json);
    }

    #[wasm_bindgen(js_name = applyTextEditsJson)]
    pub fn apply_text_edits_json(&mut self, json: &str) {
        self.state.borrow_mut().host.apply_text_edits_json(json);
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json(&self) -> String {
        self.state.borrow().host.camera_json()
    }

    #[wasm_bindgen(js_name = setCanvasThemeJson)]
    pub fn set_canvas_theme_json(&mut self, json: &str) {
        let _ = self.state.borrow_mut().host.set_canvas_theme_from_json(json);
    }

    #[wasm_bindgen(js_name = setCaretVisible)]
    pub fn set_caret_visible(&mut self, visible: bool) {
        self.state.borrow_mut().host.set_caret_visible(visible);
    }

    #[wasm_bindgen(js_name = anchor)]
    pub fn anchor(&self) -> usize {
        self.state.borrow().host.anchor()
    }

    #[wasm_bindgen(js_name = selectSpanAtScreen)]
    pub fn select_span_at_screen(&mut self, sx: f64, sy: f64) {
        self.state.borrow_mut().host.select_span_at_screen(sx, sy);
    }

    #[wasm_bindgen(js_name = pickTargetsAtScreenJson)]
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)
    }

    #[wasm_bindgen(js_name = selectSpanAt)]
    pub fn select_span_at(&mut self, offset: usize) {
        self.state.borrow_mut().host.select_span_at(offset);
    }

    #[wasm_bindgen(js_name = setSelectionRange)]
    pub fn set_selection_range(&mut self, anchor: usize, caret: usize) {
        self.state.borrow_mut().host.set_selection_range(anchor, caret);
    }

    #[wasm_bindgen(js_name = hoverTokenRangeJson)]
    pub fn hover_token_range_json(&self) -> String {
        match self.state.borrow().host.hover_token_range() {
            Some((start, end)) => serde_json::json!({ "start": start, "end": end }).to_string(),
            None => "null".into(),
        }
    }

    #[wasm_bindgen(js_name = setHoverRange)]
    pub fn set_hover_range(&mut self, start: usize, end: usize) {
        if start >= end {
            self.state.borrow_mut().host.set_hover_range(None, None);
        } else {
            self.state.borrow_mut().host.set_hover_range(Some(start), Some(end));
        }
    }

    #[wasm_bindgen(js_name = selectionText)]
    pub fn selection_text(&self) -> String {
        self.state.borrow().host.selection_text()
    }

    #[wasm_bindgen(js_name = replaceSelection)]
    pub fn replace_selection(&mut self, next: &str) {
        self.state.borrow_mut().host.replace_selection(next);
    }

    #[wasm_bindgen(js_name = selectAll)]
    pub fn select_all(&mut self) {
        self.state.borrow_mut().host.select_all();
    }

    #[wasm_bindgen(js_name = tabInsertText)]
    pub fn tab_insert_text(&self) -> String {
        self.state.borrow().host.tab_insert_text()
    }

    #[wasm_bindgen(js_name = setEditorSettingsJson)]
    pub fn set_editor_settings_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_editor_settings_json(json);
    }

    #[wasm_bindgen(js_name = setDeadLineY)]
    pub fn set_dead_line_y(&mut self, y: f64) {
        self.state.borrow_mut().host.set_dead_line_y(y);
    }

    #[wasm_bindgen(js_name = setChromeEdgelessScroll)]
    pub fn set_chrome_edgeless_scroll(&mut self, enabled: bool) {
        self.state.borrow_mut().host.set_chrome_edgeless_scroll(enabled);
    }

    #[wasm_bindgen(js_name = chromeEdgelessScroll)]
    pub fn chrome_edgeless_scroll(&self) -> bool {
        self.state.borrow().host.chrome_edgeless_scroll()
    }

    #[wasm_bindgen(js_name = wheelScrollScreen)]
    pub fn wheel_scroll_screen(&mut self, delta_y: f64) {
        self.state.borrow_mut().host.wheel_scroll_screen(delta_y);
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: i32) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, buttons: i32) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy, buttons);
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, button: i32) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy, button);
    }

    #[wasm_bindgen(js_name = insertText)]
    pub fn insert_text(&mut self, chunk: &str) {
        self.state.borrow_mut().host.insert_text(chunk);
    }

    #[wasm_bindgen(js_name = backspace)]
    pub fn backspace(&mut self) {
        self.state.borrow_mut().host.backspace();
    }

    #[wasm_bindgen(js_name = deleteForward)]
    pub fn delete_forward(&mut self) {
        self.state.borrow_mut().host.delete_forward();
    }

    #[wasm_bindgen(js_name = moveLineStart)]
    pub fn move_line_start(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_line_start(extend);
    }

    #[wasm_bindgen(js_name = moveLineEnd)]
    pub fn move_line_end(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_line_end(extend);
    }

    #[wasm_bindgen(js_name = moveLeft)]
    pub fn move_left(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_left(extend);
    }

    #[wasm_bindgen(js_name = moveRight)]
    pub fn move_right(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_right(extend);
    }

    #[wasm_bindgen(js_name = moveUp)]
    pub fn move_up(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_up(extend);
    }

    #[wasm_bindgen(js_name = moveDown)]
    pub fn move_down(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_down(extend);
    }

    #[wasm_bindgen(js_name = caretWorldJson)]
    pub fn caret_world_json(&self) -> String {
        self.state.borrow().host.caret_world_json()
    }

    #[wasm_bindgen(js_name = worldToScreenJson)]
    pub fn world_to_screen_json(&self, wx: f64, wy: f64) -> String {
        self.state.borrow().host.world_to_screen_json(wx, wy)
    }
}
// #endregion 🔖️Wasm

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
