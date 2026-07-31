//! ✍️ Text editor engine on the infinite canvas.

use cavas::camera::{Camera, Viewport};
use cavas::text as canvas_text;
pub use infinite_cavas::{self as cavas, *};
use serde::Deserialize;

// #region ⚠️ Errors
/// 🧯 Errors from `EditorHost`'s own JSON-boundary parsing (theme/scene sync). The
/// `#[cfg(target_arch = "wasm32")] #[wasm_bindgen]` methods on `EditorSession` stay
/// `Result<_, JsValue>` — that shape is dictated by the `wasm_bindgen` ABI, not this crate's own
/// error handling, so it is not migrated here.
#[derive(Debug, thiserror::Error)]
pub enum EditorError {
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}
// #endregion ⚠️ Errors

// #region 🔖Theme
#[derive(Clone, Copy, Debug)]
struct EditorCanvasTheme {
    raster_clear: Color,
    grid_minor_stroke: Color,
    label_fill: Color,
    label_fill_hovered: Color,
    label_halo: Color,
    hover_fill: Color,
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
        cavas::theme::merge_color_field(next, v, key);
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
// #endregion 🔖Theme

// #region 🔖EditorViewport
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
// #endregion 🔖EditorViewport

// #region 🔖EditorState

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditorSettingsJson {
    #[serde(default = "default_font_px")]
    font_px: f64,
    #[serde(default = "default_line_height")]
    line_height: f64,
    #[serde(default = "default_show_line_numbers")]
    show_line_numbers: bool,
    #[serde(default = "default_tab_size")]
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticTokenJson {
    start: usize,
    end: usize,
    class: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SelectableSpanJson {
    start: usize,
    end: usize,
    kind: String,
    head_end: Option<usize>,
    tail_start: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ByteRangeJson {
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlaceholderJson {
    offset: usize,
    label: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticJson {
    start: usize,
    end: usize,
    severity: Option<String>,
    #[allow(dead_code)]
    message: String,
}

#[derive(Clone, Debug, Deserialize)]
struct TextEditJson {
    range: TextRangeJson,
    #[serde(rename = "newText")]
    new_text: String,
}

#[derive(Clone, Debug, Deserialize)]
struct TextRangeJson {
    start: TextPosJson,
    end: TextPosJson,
}

#[derive(Clone, Debug, Deserialize)]
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
            if best.map(|current| size < current.end - current.start).unwrap_or(true) {
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
        if let Some(buffer) = value.get("buffer").and_then(|v| v.as_str()) {
            self.set_text(buffer.to_string());
        }
        if let Some(json) = value.get("selectionJson").and_then(|v| v.as_str()) {
            if let Ok(range) = serde_json::from_str::<serde_json::Value>(json) {
                let start = range.get("start").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                let end = range.get("end").and_then(|v| v.as_u64()).unwrap_or(start as u64) as usize;
                self.set_selection_range(start, end);
            }
        }
        if let Some(json) = value.get("tokensJson").and_then(|v| v.as_str()) {
            self.set_semantic_tokens_json(json);
        }
        if let Some(json) = value.get("diagnosticsJson").and_then(|v| v.as_str()) {
            self.set_diagnostics_json(json);
        }
        if let Some(json) = value.get("placeholdersJson").and_then(|v| v.as_str()) {
            self.set_placeholders_json(json);
        }
        if let Some(json) = value.get("occurrencesJson").and_then(|v| v.as_str()) {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(json) {
                if let Some(hover) = value.get("hover").and_then(|v| v.as_str()) {
                    self.set_hover_occurrences_json(hover);
                }
                if let Some(selection) = value.get("selection").and_then(|v| v.as_str()) {
                    self.set_selection_occurrences_json(selection);
                }
            }
        }
        if let Some(json) = value.get("extraCaretsJson").and_then(|v| v.as_str()) {
            self.set_extra_carets_json(json);
        }
        if let Some(json) = value.get("selectableSpansJson").and_then(|v| v.as_str()) {
            self.set_selectable_spans_json(json);
        }
        if let Some(json) = value.get("settingsJson").and_then(|v| v.as_str()) {
            self.set_editor_settings_json(json);
        }
        if let Some(json) = value.get("cameraJson").and_then(|v| v.as_str()) {
            if let Ok(camera) = serde_json::from_str::<serde_json::Value>(json) {
                let y = camera.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                self.set_camera(0.0, y, 1.0);
            }
        }
        if let Some(json) = value.get("overlaysJson").and_then(|v| v.as_str()) {
            if let Ok(overlays) = serde_json::from_str::<serde_json::Value>(json) {
                if let Some(y) = overlays.get("deadLineY").and_then(|v| v.as_f64()) {
                    self.set_dead_line_y(y);
                }
            }
        }
        if let Some(json) = value.get("hoverJson").and_then(|v| v.as_str()) {
            match serde_json::from_str::<serde_json::Value>(json) {
                Ok(serde_json::Value::Object(range)) => {
                    let start = range.get("start").and_then(|v| v.as_u64()).map(|v| v as usize);
                    let end = range.get("end").and_then(|v| v.as_u64()).map(|v| v as usize);
                    self.set_hover_range(start, end);
                }
                _ => self.set_hover_range(None, None),
            }
        }
        Ok(())
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
        let line_len = self.text.split('\n').nth(line).map(str::len).unwrap_or(0);
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

    /// @emoji 🎯 Returns pick-target rows at a screen point for DOM disambiguation menus.
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
        cavas::render::scale_scene_for_device_pixel_ratio(scene, self.viewport.dpr)
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
            let line_end = text[line_start..].find('\n').map(|idx| line_start + idx).unwrap_or(text.len());
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
    text[..index].char_indices().next_back().map(|(i, _)| i).unwrap_or(0)
}

fn next_char_boundary(text: &str, index: usize) -> usize {
    text[index..].char_indices().nth(1).map(|(i, _)| index + i).unwrap_or(text.len())
}
// #endregion 🔖EditorState

// #region 🔖Wasm
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
struct EditorSessionInner {
    host: EditorHost,
    gpu: cavas::gpu_session::CanvasGpuSession,
}

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct EditorSession {
    state: Rc<RefCell<EditorSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl EditorSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(EditorSessionInner { host: EditorHost::new(), gpu: cavas::gpu_session::CanvasGpuSession::default() })) }
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
            let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|e| JsValue::from_str(&e))?;
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
        let value: serde_json::Value = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let mut inner = self.state.borrow_mut();
        if let Some(buffer) = value.get("buffer").and_then(|v| v.as_str()) {
            inner.host.set_text(buffer.to_string());
        }
        if let Some(json) = value.get("selectionJson").and_then(|v| v.as_str()) {
            if let Ok(range) = serde_json::from_str::<serde_json::Value>(json) {
                let start = range.get("start").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                let end = range.get("end").and_then(|v| v.as_u64()).unwrap_or(start as u64) as usize;
                inner.host.set_selection_range(start, end);
            }
        }
        if let Some(json) = value.get("tokensJson").and_then(|v| v.as_str()) {
            inner.host.set_semantic_tokens_json(json);
        }
        if let Some(json) = value.get("diagnosticsJson").and_then(|v| v.as_str()) {
            inner.host.set_diagnostics_json(json);
        }
        if let Some(json) = value.get("placeholdersJson").and_then(|v| v.as_str()) {
            inner.host.set_placeholders_json(json);
        }
        if let Some(json) = value.get("occurrencesJson").and_then(|v| v.as_str()) {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(json) {
                if let Some(hover) = value.get("hover").and_then(|v| v.as_str()) {
                    inner.host.set_hover_occurrences_json(hover);
                }
                if let Some(selection) = value.get("selection").and_then(|v| v.as_str()) {
                    inner.host.set_selection_occurrences_json(selection);
                }
            }
        }
        if let Some(json) = value.get("extraCaretsJson").and_then(|v| v.as_str()) {
            inner.host.set_extra_carets_json(json);
        }
        if let Some(json) = value.get("selectableSpansJson").and_then(|v| v.as_str()) {
            inner.host.set_selectable_spans_json(json);
        }
        if let Some(json) = value.get("settingsJson").and_then(|v| v.as_str()) {
            inner.host.set_editor_settings_json(json);
        }
        if let Some(json) = value.get("cameraJson").and_then(|v| v.as_str()) {
            if let Ok(camera) = serde_json::from_str::<serde_json::Value>(json) {
                let x = camera.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                inner.host.set_camera(0.0, x, 1.0);
            }
        }
        if let Some(json) = value.get("hoverJson").and_then(|v| v.as_str()) {
            match serde_json::from_str::<serde_json::Value>(json) {
                Ok(serde_json::Value::Object(range)) => {
                    let start = range.get("start").and_then(|v| v.as_u64()).map(|v| v as usize);
                    let end = range.get("end").and_then(|v| v.as_u64()).map(|v| v as usize);
                    inner.host.set_hover_range(start, end);
                }
                _ => inner.host.set_hover_range(None, None),
            }
        }
        Ok(())
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
// #endregion 🔖Wasm

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_space_inside_token_inserts_without_replacing() {
        let mut host = EditorHost::new();
        host.set_text("MATCH".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(3);
        host.insert_text(" ");
        assert_eq!(host.text(), "MAT CH");
        assert_eq!(host.caret(), 4);
    }

    #[test]
    fn insert_space_at_token_end_appends() {
        let mut host = EditorHost::new();
        host.set_text("MATCH".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(5);
        host.insert_text(" ");
        assert_eq!(host.text(), "MATCH ");
        assert_eq!(host.caret(), 6);
    }

    #[test]
    fn auto_space_before_next_token_at_token_end() {
        let mut host = EditorHost::new();
        host.set_text("MATCH".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(5);
        host.insert_text("(");
        assert_eq!(host.text(), "MATCH (");
    }

    #[test]
    fn insert_and_caret() {
        let mut host = EditorHost::new();
        host.insert_text("MATCH");
        assert_eq!(host.text(), "MATCH");
        assert_eq!(host.caret(), 5);
    }

    #[test]
    fn sync_from_scene_json_sets_and_clears_hover_range() {
        let mut host = EditorHost::new();
        host.sync_from_scene_json(r#"{"buffer":"abc","hoverJson":"{\"start\":1,\"end\":2}"}"#).unwrap();
        assert_eq!(host.hover_token_range(), Some((1, 2)));
        host.sync_from_scene_json(r#"{"buffer":"abc","hoverJson":"null"}"#).unwrap();
        assert_eq!(host.hover_token_range(), None);
    }

    #[test]
    fn theme_merge_from_json_updates_clear() {
        let mut host = EditorHost::new();
        let json = r#"{"rasterClear":[240,236,221,255],"labelFill":[0,17,23,255]}"#;
        host.set_canvas_theme_from_json(json).expect("theme json");
        let _scene = host.build_scene();
        assert!(host.set_canvas_theme_from_json(json).is_ok());
    }

    #[test]
    fn select_all_sets_range() {
        let mut host = EditorHost::new();
        host.set_text("abc".into());
        host.select_all();
        assert_eq!(host.anchor(), 0);
        assert_eq!(host.caret(), 3);
    }

    #[test]
    fn selection_snaps_var_label_composite() {
        let mut host = EditorHost::new();
        host.set_text("MATCH (a1:Piece)".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"},{"start":7,"end":9,"class":"ident"},{"start":9,"end":10,"class":"operator"},{"start":10,"end":15,"class":"ident"}]"#);
        host.set_selectable_spans_json(r#"[{"start":7,"end":9,"kind":"atomic"},{"start":7,"end":15,"kind":"varLabel","headEnd":9},{"start":10,"end":15,"kind":"atomic"}]"#);
        host.set_selection(8, 12);
        assert_eq!(host.anchor(), 7);
        assert_eq!(host.caret(), 15);
    }

    #[test]
    fn select_span_at_picks_ident() {
        let mut host = EditorHost::new();
        host.set_text("RETURN a1.name".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":6,"class":"keyword"},{"start":7,"end":9,"class":"ident"},{"start":9,"end":10,"class":"operator"},{"start":10,"end":14,"class":"ident"}]"#);
        host.set_selectable_spans_json(r#"[{"start":7,"end":9,"kind":"atomic"},{"start":10,"end":14,"kind":"atomic"},{"start":7,"end":14,"kind":"propertyAccess","headEnd":9,"tailStart":10}]"#);
        host.select_span_at(11);
        assert_eq!(host.anchor(), 10);
        assert_eq!(host.caret(), 14);
    }

    #[test]
    fn selection_snaps_fixed_keywords() {
        let mut host = EditorHost::new();
        host.set_text("MATCH x".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_selection(2, 4);
        assert_eq!(host.anchor(), 0);
        assert_eq!(host.caret(), 5);
    }

    #[test]
    fn dead_line_ignores_wheel_when_content_fits() {
        let mut host = EditorHost::new();
        host.set_size(400, 300, 1.0);
        host.set_text("line one\nline two".into());
        host.set_dead_line_y(32.0);
        assert!(!host.scroll_overflows());
        host.wheel_scroll_screen(-40.0);
        assert!(!host.chrome_edgeless_scroll());
    }

    #[test]
    fn dead_line_toggles_edgeless_and_restores_on_scroll_back() {
        let mut host = EditorHost::new();
        host.set_size(400, 120, 1.0);
        let lines: String = (0..20).map(|i| format!("line {i}")).collect::<Vec<_>>().join("\n");
        host.set_text(lines);
        host.set_dead_line_y(32.0);
        assert!(host.scroll_overflows());
        let screen_at_rest: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(offset_to_world(&host, 0).0, offset_to_world(&host, 0).1)).unwrap();
        assert!(screen_at_rest["y"].as_f64().unwrap() >= 32.0);
        host.wheel_scroll_screen(-40.0);
        assert!(host.chrome_edgeless_scroll());
        let screen_edgeless: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(offset_to_world(&host, 0).0, offset_to_world(&host, 0).1)).unwrap();
        assert!(screen_edgeless["y"].as_f64().unwrap() < screen_at_rest["y"].as_f64().unwrap());
        host.wheel_scroll_screen(40.0);
        assert!(!host.chrome_edgeless_scroll());
        let screen_restored: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(offset_to_world(&host, 0).0, offset_to_world(&host, 0).1)).unwrap();
        assert!((screen_restored["y"].as_f64().unwrap() - screen_at_rest["y"].as_f64().unwrap()).abs() < 0.5);
    }

    #[test]
    fn editor_viewport_maps_text_to_top_left() {
        let mut host = EditorHost::new();
        host.set_size(400, 300, 1.0);
        host.set_text("hello".into());
        let (wx, wy) = offset_to_world(&host, 0);
        let screen: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(wx, wy)).unwrap();
        let sx = screen["x"].as_f64().unwrap();
        let sy = screen["y"].as_f64().unwrap();
        assert!(sx > 50.0 && sx < 90.0);
        assert!(sy > PAD_Y && sy < PAD_Y + DEFAULT_LINE_HEIGHT);
    }

    #[test]
    fn drag_select_extends_range() {
        let mut host = EditorHost::new();
        host.set_size(800, 600, 1.0);
        host.set_text("hello world".into());
        host.pointer_down_screen(68.0, 24.5, 0);
        host.pointer_move_screen(250.0, 24.5, 1);
        host.pointer_up_screen(250.0, 24.5, 0);
        assert_ne!(host.caret(), host.anchor());
        assert_eq!(host.anchor(), 0);
        assert!(host.caret() > host.anchor());
    }

    #[test]
    fn punctuated_token_line_builds_scene() {
        let mut host = EditorHost::new();
        host.set_text("MATCH (a:Piece)".into());
        host.set_semantic_tokens_json(
            r#"[{"start":0,"end":5,"class":"keyword"},{"start":5,"end":6,"class":"operator"},{"start":6,"end":7,"class":"operator"},{"start":7,"end":8,"class":"plain"},{"start":8,"end":9,"class":"operator"},{"start":9,"end":14,"class":"plain"}]"#,
        );
        let _scene = host.build_scene();
        assert!(!host.text().is_empty());
    }

    #[test]
    fn build_scene_has_content() {
        let mut host = EditorHost::new();
        host.set_text("MATCH (a:Piece)\nRETURN a.name".into());
        let _scene = host.build_scene();
        assert!(!host.text().is_empty());
    }

    #[test]
    fn backspace_deletes_fixed_keyword_tokenwise() {
        let mut host = EditorHost::new();
        host.set_text("MATCH (a:Piece)".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"},{"start":5,"end":6,"class":"operator"}]"#);
        host.set_caret_anchor(3);
        host.backspace();
        assert_eq!(host.text(), " (a:Piece)");
        assert_eq!(host.caret(), 0);
    }

    #[test]
    fn label_span_world_x_matches_scaled_render() {
        let line = "MATCH (a:Piece)";
        let origin = DEFAULT_GUTTER_WIDTH + PAD_X;
        let (x0, x5) = canvas_text::label_span_world_x(line, 0, 5, origin, DEFAULT_FONT_PX);
        let estimate = canvas_text::label_advance("MATCH", DEFAULT_FONT_PX);
        assert!(x5 - x0 < estimate);
        assert!(x5 > x0);
    }

    #[test]
    fn apply_text_edits() {
        let mut host = EditorHost::new();
        host.set_text("abc def".into());
        host.apply_text_edits_json(r#"[{"range":{"start":{"line":0,"character":4},"end":{"line":0,"character":7}},"newText":"xyz"}]"#);
        assert_eq!(host.text(), "abc xyz");
    }

    #[test]
    fn set_dead_line_y_clamps_negative_to_zero() {
        let mut host = EditorHost::new();
        host.set_text("a".into());
        host.set_dead_line_y(-50.0);
        let (_, y) = offset_to_world(&host, 0);
        assert_eq!(y, PAD_Y + DEFAULT_LINE_HEIGHT * 0.75);
    }

    #[test]
    fn set_chrome_edgeless_scroll_toggles_flag() {
        let mut host = EditorHost::new();
        assert!(!host.chrome_edgeless_scroll());
        host.set_chrome_edgeless_scroll(true);
        assert!(host.chrome_edgeless_scroll());
    }

    #[test]
    fn editor_settings_json_clamps_and_updates_flags() {
        let mut host = EditorHost::new();
        host.set_editor_settings_json(r#"{"fontPx":100,"lineHeight":100,"showLineNumbers":false,"tabSize":20}"#);
        assert_eq!(host.font_px, 28.0);
        assert_eq!(host.line_height, 48.0);
        assert!(!host.show_line_numbers);
        assert_eq!(host.tab_size, 8);
        assert_eq!(host.gutter_width(), 0.0);
    }

    #[test]
    fn editor_settings_json_invalid_falls_back_to_defaults() {
        let mut host = EditorHost::new();
        host.set_editor_settings_json("not json");
        assert_eq!(host.font_px, DEFAULT_FONT_PX);
        assert_eq!(host.line_height, DEFAULT_LINE_HEIGHT);
        assert!(host.show_line_numbers);
        assert_eq!(host.tab_size, DEFAULT_TAB_SIZE);
    }

    #[test]
    fn editor_settings_json_clamps_tab_size_minimum() {
        let mut host = EditorHost::new();
        host.set_editor_settings_json(r#"{"tabSize":0}"#);
        assert_eq!(host.tab_insert_text(), " ");
    }

    #[test]
    fn set_selection_range_clamps_to_text_length() {
        let mut host = EditorHost::new();
        host.set_text("abc".into());
        host.set_selection_range(0, 100);
        assert_eq!(host.caret(), 3);
        assert_eq!(host.anchor(), 0);
    }

    #[test]
    fn hover_token_range_reflects_set_hover_range() {
        let mut host = EditorHost::new();
        assert_eq!(host.hover_token_range(), None);
        host.set_hover_range(Some(2), Some(5));
        assert_eq!(host.hover_token_range(), Some((2, 5)));
        host.set_hover_range(None, Some(5));
        assert_eq!(host.hover_token_range(), None);
    }

    #[test]
    fn selection_text_returns_selected_substring() {
        let mut host = EditorHost::new();
        host.set_text("hello world".into());
        host.set_selection(0, 5);
        assert_eq!(host.selection_text(), "hello");
    }

    #[test]
    fn selection_text_empty_when_collapsed() {
        let mut host = EditorHost::new();
        host.set_text("hello".into());
        host.set_caret_anchor(2);
        assert_eq!(host.selection_text(), "");
    }

    #[test]
    fn replace_selection_inserts_when_collapsed() {
        let mut host = EditorHost::new();
        host.set_text("hello".into());
        host.set_caret_anchor(5);
        host.replace_selection(" world");
        assert_eq!(host.text(), "hello world");
    }

    #[test]
    fn replace_selection_replaces_range() {
        let mut host = EditorHost::new();
        host.set_text("hello world".into());
        host.set_selection(0, 5);
        host.replace_selection("bye");
        assert_eq!(host.text(), "bye world");
        assert_eq!(host.caret(), 3);
        assert_eq!(host.anchor(), 3);
    }

    #[test]
    fn set_json_collections_parse_into_fields() {
        let mut host = EditorHost::new();
        host.set_diagnostics_json(r#"[{"start":0,"end":2,"severity":"warning","message":"x"}]"#);
        assert_eq!(host.diagnostics.len(), 1);
        host.set_placeholders_json(r#"[{"offset":0,"label":"?"}]"#);
        assert_eq!(host.placeholders.len(), 1);
        host.set_hover_occurrences_json(r#"[{"start":0,"end":1}]"#);
        assert_eq!(host.hover_occurrences.len(), 1);
        host.set_selection_occurrences_json(r#"[{"start":0,"end":1}]"#);
        assert_eq!(host.selection_occurrences.len(), 1);
        host.set_extra_carets_json(r#"[2,4]"#);
        assert_eq!(host.extra_carets, vec![2, 4]);
    }

    #[test]
    fn set_json_collections_invalid_json_defaults_empty() {
        let mut host = EditorHost::new();
        host.set_diagnostics_json("nope");
        assert!(host.diagnostics.is_empty());
    }

    #[test]
    fn apply_text_edits_multiple_edits_apply_in_reverse_order() {
        let mut host = EditorHost::new();
        host.set_text("one two three".into());
        let json = r#"[
            {"range":{"start":{"line":0,"character":0},"end":{"line":0,"character":3}},"newText":"ONE"},
            {"range":{"start":{"line":0,"character":8},"end":{"line":0,"character":13}},"newText":"THREE"}
        ]"#;
        host.apply_text_edits_json(json);
        assert_eq!(host.text(), "ONE two THREE");
    }

    #[test]
    fn camera_json_reports_y_after_set_camera() {
        let mut host = EditorHost::new();
        host.set_size(400, 100, 1.0);
        host.set_text((0..50).map(|i| format!("line {i}")).collect::<Vec<_>>().join("\n"));
        host.set_camera(0.0, 500.0, 2.0);
        let camera: serde_json::Value = serde_json::from_str(&host.camera_json()).unwrap();
        assert_eq!(camera["x"], 0);
        assert_eq!(camera["zoom"], 1);
        assert!(camera["y"].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn set_size_clamps_to_minimum() {
        let mut host = EditorHost::new();
        host.set_size(0, 0, 0.0);
        assert_eq!(host.viewport.width, 1);
        assert_eq!(host.viewport.height, 1);
        assert_eq!(host.viewport.dpr, 1.0);
    }

    #[test]
    fn sync_from_scene_json_applies_all_optional_fields() {
        let mut host = EditorHost::new();
        let occurrences_inner = serde_json::json!({
            "hover": serde_json::json!([{"start":0,"end":1}]).to_string(),
            "selection": serde_json::json!([{"start":1,"end":2}]).to_string(),
        })
        .to_string();
        let outer = serde_json::json!({
            "buffer": "abc",
            "selectionJson": serde_json::json!({"start":0,"end":2}).to_string(),
            "tokensJson": serde_json::json!([{"start":0,"end":1,"class":"keyword"}]).to_string(),
            "diagnosticsJson": serde_json::json!([{"start":0,"end":1,"severity":"error","message":"x"}]).to_string(),
            "placeholdersJson": serde_json::json!([{"offset":0,"label":"?"}]).to_string(),
            "occurrencesJson": occurrences_inner,
            "extraCaretsJson": serde_json::json!([1,2]).to_string(),
            "selectableSpansJson": serde_json::json!([{"start":0,"end":1,"kind":"atomic"}]).to_string(),
            "settingsJson": serde_json::json!({"fontPx":18}).to_string(),
            "cameraJson": serde_json::json!({"y":5}).to_string(),
            "overlaysJson": serde_json::json!({"deadLineY":10}).to_string(),
        })
        .to_string();
        host.sync_from_scene_json(&outer).unwrap();
        assert_eq!(host.text(), "abc");
        assert_eq!(host.anchor(), 0);
        assert_eq!(host.caret(), 2);
        assert_eq!(host.diagnostics.len(), 1);
        assert_eq!(host.placeholders.len(), 1);
        assert_eq!(host.hover_occurrences.len(), 1);
        assert_eq!(host.selection_occurrences.len(), 1);
        assert_eq!(host.extra_carets, vec![1, 2]);
        assert_eq!(host.selectable_spans.len(), 1);
        assert_eq!(host.font_px, 18.0);
        assert_eq!(host.dead_line_y, 10.0);
    }

    #[test]
    fn wheel_scroll_moves_camera_when_overflowing_without_dead_line() {
        let mut host = EditorHost::new();
        host.set_size(400, 60, 1.0);
        let lines: String = (0..20).map(|i| format!("line {i}")).collect::<Vec<_>>().join("\n");
        host.set_text(lines);
        assert!(host.scroll_overflows());
        host.wheel_scroll_screen(100.0);
        assert!(host.camera.y > 0.0);
    }

    #[test]
    fn wheel_scroll_clamps_camera_to_zero() {
        let mut host = EditorHost::new();
        host.set_size(400, 60, 1.0);
        let lines: String = (0..20).map(|i| format!("line {i}")).collect::<Vec<_>>().join("\n");
        host.set_text(lines);
        host.wheel_scroll_screen(-1000.0);
        assert_eq!(host.camera.y, 0.0);
    }

    /// 🐛 W4 fix (`.🦑repo/🎫tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w4-scene-input.md`): a right-click
    /// used to be a total no-op here (this test used to assert the caret/anchor stayed put) — now it
    /// still repositions the caret to the click point (matching left-click, for the right-click
    /// context-menu's "open where you clicked" UX) but must never start a drag-selection.
    #[test]
    fn pointer_down_screen_repositions_caret_for_non_primary_button_but_does_not_start_a_drag_selection() {
        let mut host = EditorHost::new();
        host.set_text("hello world".into());
        host.set_caret_anchor(3);
        let expected_offset = host.hit_test_offset_screen(0.0, 0.0);
        host.pointer_down_screen(0.0, 0.0, 2);
        assert_eq!(host.caret(), expected_offset, "a right-click should still reposition the caret to the click point");
        assert_eq!(host.anchor(), expected_offset);
        assert!(!host.drag_selecting, "a right-click must not start a drag-selection");
    }

    #[test]
    fn pointer_down_screen_primary_button_still_starts_a_drag_selection() {
        let mut host = EditorHost::new();
        host.set_text("hello world".into());
        host.pointer_down_screen(0.0, 0.0, 0);
        assert!(host.drag_selecting, "a primary-button press must still start a drag-selection");
    }

    #[test]
    fn pointer_move_screen_sets_hover_without_drag() {
        let mut host = EditorHost::new();
        host.set_text("MATCH".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.pointer_move_screen(70.0, 24.5, 0);
        assert_eq!(host.hover_token_range(), Some((0, 5)));
    }

    #[test]
    fn pointer_up_screen_ignores_non_primary_button() {
        let mut host = EditorHost::new();
        host.set_text("hello".into());
        host.drag_selecting = true;
        host.pointer_up_screen(0.0, 0.0, 2);
        assert!(host.drag_selecting);
    }

    #[test]
    fn insert_text_no_auto_space_for_leading_punctuation() {
        let mut host = EditorHost::new();
        host.set_text("MATCH".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(5);
        host.insert_text(":");
        assert_eq!(host.text(), "MATCH:");
    }

    #[test]
    fn insert_text_no_auto_space_without_preceding_token() {
        let mut host = EditorHost::new();
        host.set_text("hello".into());
        host.set_caret_anchor(5);
        host.insert_text("world");
        assert_eq!(host.text(), "helloworld");
    }

    #[test]
    fn insert_text_no_auto_space_when_next_char_is_whitespace() {
        let mut host = EditorHost::new();
        host.set_text("MATCH x".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(5);
        host.insert_text("y");
        assert_eq!(host.text(), "MATCHy x");
    }

    #[test]
    fn backspace_deletes_selection_when_not_collapsed() {
        let mut host = EditorHost::new();
        host.set_text("hello world".into());
        host.set_selection(0, 5);
        host.backspace();
        assert_eq!(host.text(), " world");
        assert_eq!(host.caret(), 0);
    }

    #[test]
    fn backspace_at_start_is_noop() {
        let mut host = EditorHost::new();
        host.set_text("hello".into());
        host.set_caret_anchor(0);
        host.backspace();
        assert_eq!(host.text(), "hello");
    }

    #[test]
    fn backspace_removes_single_char_without_token() {
        let mut host = EditorHost::new();
        host.set_text("hello".into());
        host.set_caret_anchor(5);
        host.backspace();
        assert_eq!(host.text(), "hell");
        assert_eq!(host.caret(), 4);
    }

    #[test]
    fn delete_forward_deletes_selection_when_not_collapsed() {
        let mut host = EditorHost::new();
        host.set_text("hello world".into());
        host.set_selection(0, 5);
        host.delete_forward();
        assert_eq!(host.text(), " world");
    }

    #[test]
    fn delete_forward_at_end_is_noop() {
        let mut host = EditorHost::new();
        host.set_text("hello".into());
        host.set_caret_anchor(5);
        host.delete_forward();
        assert_eq!(host.text(), "hello");
    }

    #[test]
    fn delete_forward_removes_token_wholly() {
        let mut host = EditorHost::new();
        host.set_text("MATCH x".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(0);
        host.delete_forward();
        assert_eq!(host.text(), " x");
        assert_eq!(host.caret(), 0);
        assert_eq!(host.anchor(), 0);
    }

    #[test]
    fn delete_forward_removes_single_char_without_token() {
        let mut host = EditorHost::new();
        host.set_text("hello".into());
        host.set_caret_anchor(0);
        host.delete_forward();
        assert_eq!(host.text(), "ello");
    }

    #[test]
    fn move_line_start_and_end_navigate_and_extend() {
        let mut host = EditorHost::new();
        host.set_text("hello\nworld".into());
        host.set_caret_anchor(8);
        host.move_line_start(false);
        assert_eq!(host.caret(), 6);
        assert_eq!(host.anchor(), 6);
        host.set_caret_anchor(8);
        host.move_line_end(true);
        assert_eq!(host.caret(), 11);
        assert_eq!(host.anchor(), 8);
    }

    #[test]
    fn move_left_jumps_token_boundary() {
        let mut host = EditorHost::new();
        host.set_text("MATCH x".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(3);
        host.move_left(false);
        assert_eq!(host.caret(), 0);
        assert_eq!(host.anchor(), 0);
    }

    #[test]
    fn move_right_jumps_token_boundary_and_extends() {
        let mut host = EditorHost::new();
        host.set_text("MATCH x".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(2);
        host.move_right(true);
        assert_eq!(host.caret(), 5);
        assert_eq!(host.anchor(), 2);
    }

    #[test]
    fn move_left_at_start_stays_at_zero() {
        let mut host = EditorHost::new();
        host.set_text("hi".into());
        host.set_caret_anchor(0);
        host.move_left(false);
        assert_eq!(host.caret(), 0);
    }

    #[test]
    fn move_right_at_end_stays_at_end() {
        let mut host = EditorHost::new();
        host.set_text("hi".into());
        host.set_caret_anchor(2);
        host.move_right(false);
        assert_eq!(host.caret(), 2);
    }

    #[test]
    fn move_up_at_first_line_goes_to_zero() {
        let mut host = EditorHost::new();
        host.set_text("hello\nworld".into());
        host.set_caret_anchor(3);
        host.move_up(false);
        assert_eq!(host.caret(), 0);
    }

    #[test]
    fn move_down_clamps_to_last_line() {
        let mut host = EditorHost::new();
        host.set_text("a\nb".into());
        host.set_caret_anchor(0);
        host.move_down(false);
        assert_eq!(host.caret(), 2);
        host.move_down(false);
        assert_eq!(host.caret(), 2);
    }

    #[test]
    fn caret_world_json_reports_position() {
        let mut host = EditorHost::new();
        host.set_text("hi".into());
        host.set_caret_anchor(2);
        let v: serde_json::Value = serde_json::from_str(&host.caret_world_json()).unwrap();
        assert!(v["x"].as_f64().unwrap() > 0.0);
        assert!(v["y"].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn pick_targets_reports_line_and_token() {
        let mut host = EditorHost::new();
        host.set_text("MATCH x".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        let (wx, wy) = offset_to_world(&host, 2);
        let screen: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(wx, wy)).unwrap();
        let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());
        let rows: serde_json::Value = serde_json::from_str(&json).unwrap();
        let arr = rows.as_array().unwrap();
        assert_eq!(arr[0]["domain"], "line");
        assert!(arr.iter().any(|r| r["domain"] == "token"));
    }

    #[test]
    fn pick_targets_without_token_only_reports_line() {
        let mut host = EditorHost::new();
        host.set_text("hello".into());
        let (wx, wy) = offset_to_world(&host, 2);
        let screen: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(wx, wy)).unwrap();
        let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());
        let rows: serde_json::Value = serde_json::from_str(&json).unwrap();
        let arr = rows.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["domain"], "line");
    }

    #[test]
    fn select_span_at_screen_selects_atomic_span() {
        let mut host = EditorHost::new();
        host.set_text("RETURN a1.name".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":6,"class":"keyword"},{"start":7,"end":9,"class":"ident"},{"start":9,"end":10,"class":"operator"},{"start":10,"end":14,"class":"ident"}]"#);
        host.set_selectable_spans_json(r#"[{"start":7,"end":9,"kind":"atomic"},{"start":10,"end":14,"kind":"atomic"}]"#);
        let (wx, wy) = offset_to_world(&host, 8);
        let screen: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(wx, wy)).unwrap();
        host.select_span_at_screen(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());
        assert_eq!(host.anchor(), 7);
        assert_eq!(host.caret(), 9);
    }

    #[test]
    fn selection_snaps_property_access_tail_allowed() {
        let mut host = EditorHost::new();
        host.set_text("RETURN a1.name".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":6,"class":"keyword"},{"start":7,"end":9,"class":"ident"},{"start":9,"end":10,"class":"operator"},{"start":10,"end":14,"class":"ident"}]"#);
        host.set_selectable_spans_json(r#"[{"start":7,"end":9,"kind":"atomic"},{"start":10,"end":14,"kind":"atomic"},{"start":7,"end":14,"kind":"propertyAccess","headEnd":9,"tailStart":10}]"#);
        host.set_selection(10, 14);
        assert_eq!(host.anchor(), 10);
        assert_eq!(host.caret(), 14);
    }

    #[test]
    fn var_label_without_head_end_falls_back_to_span_end() {
        let mut host = EditorHost::new();
        host.set_text("RETURN a1".into());
        host.set_selectable_spans_json(r#"[{"start":7,"end":9,"kind":"varLabel"}]"#);
        host.set_selection(7, 9);
        assert_eq!(host.anchor(), 7);
        assert_eq!(host.caret(), 9);
    }

    #[test]
    fn build_scene_multiline_selection_and_hover_render_without_panic() {
        let mut host = EditorHost::new();
        host.set_text("line one\nline two\nline three".into());
        host.set_selection(2, 20);
        host.set_hover_range(Some(0), Some(4));
        let _scene = host.build_scene();
        assert_eq!(host.selection_text().len(), 18);
    }

    #[test]
    fn build_scene_uses_occurrences_when_present() {
        let mut host = EditorHost::new();
        host.set_text("abc abc abc".into());
        host.set_selection_occurrences_json(r#"[{"start":0,"end":3},{"start":8,"end":11}]"#);
        host.set_hover_occurrences_json(r#"[{"start":4,"end":7}]"#);
        let _scene = host.build_scene();
        assert_eq!(host.selection_occurrences.len(), 2);
    }

    #[test]
    fn build_scene_renders_diagnostics_with_and_without_warning_severity() {
        let mut host = EditorHost::new();
        host.set_text("abc def".into());
        host.set_diagnostics_json(r#"[{"start":0,"end":3,"severity":"warning","message":"w"},{"start":4,"end":7,"message":"e"}]"#);
        let _scene = host.build_scene();
        assert_eq!(host.diagnostics.len(), 2);
    }

    #[test]
    fn build_scene_renders_placeholders() {
        let mut host = EditorHost::new();
        host.set_text("abc".into());
        host.set_placeholders_json(r#"[{"offset":1,"label":"?"}]"#);
        let _scene = host.build_scene();
        assert_eq!(host.placeholders.len(), 1);
    }

    #[test]
    fn build_scene_renders_extra_carets() {
        let mut host = EditorHost::new();
        host.set_text("abc def".into());
        host.set_extra_carets_json(r#"[1,3]"#);
        host.set_caret_anchor(5);
        let _scene = host.build_scene();
        assert_eq!(host.extra_carets, vec![1, 3]);
    }

    #[test]
    fn build_scene_with_caret_hidden_does_not_panic() {
        let mut host = EditorHost::new();
        host.set_text("abc".into());
        host.set_caret_visible(false);
        let _scene = host.build_scene();
        assert!(!host.caret_visible);
    }

    #[test]
    fn build_scene_without_line_numbers_skips_gutter() {
        let mut host = EditorHost::new();
        host.set_editor_settings_json(r#"{"showLineNumbers":false}"#);
        host.set_text("abc".into());
        let _scene = host.build_scene();
        assert_eq!(host.gutter_width(), 0.0);
    }

    #[test]
    fn is_insert_whitespace_detects_whitespace_only() {
        assert!(is_insert_whitespace("  \t\n"));
        assert!(!is_insert_whitespace("a "));
        assert!(!is_insert_whitespace(""));
    }

    #[test]
    fn ranges_overlap_detects_overlap_and_disjoint() {
        assert!(ranges_overlap(0, 5, 3, 8));
        assert!(!ranges_overlap(0, 5, 5, 8));
        assert!(!ranges_overlap(0, 5, 6, 8));
    }

    #[test]
    fn offset_line_col_roundtrip() {
        let text = "abc\ndef\nghi";
        assert_eq!(offset_line_col(text, 5), (1, 1));
        assert_eq!(offset_at_line_col(text, 1, 1), 5);
        assert_eq!(offset_line_col(text, 100), (2, 3));
    }

    #[test]
    fn offset_at_line_col_beyond_last_line_clamps_to_end() {
        let text = "abc\ndef";
        assert_eq!(offset_at_line_col(text, 5, 0), text.len());
    }

    #[test]
    fn char_boundary_helpers_handle_multibyte() {
        let text = "a😀b";
        let emoji_start = 1;
        let emoji_end = 1 + '😀'.len_utf8();
        assert_eq!(next_char_boundary(text, emoji_start), emoji_end);
        assert_eq!(prev_char_boundary(text, emoji_end), emoji_start);
        assert_eq!(prev_char_boundary(text, 0), 0);
        assert_eq!(next_char_boundary(text, text.len()), text.len());
    }

    #[test]
    fn position_to_offset_converts_line_and_character() {
        let text = "abc\ndef";
        let pos = TextPosJson { line: 1, character: 2 };
        assert_eq!(position_to_offset(text, &pos), 6);
    }

    #[test]
    fn hit_byte_in_line_empty_line_returns_zero() {
        assert_eq!(hit_byte_in_line("", 100.0, 0.0, DEFAULT_FONT_PX), 0);
    }

    #[test]
    fn snap_offset_for_atomic_snaps_to_nearest_boundary() {
        let mut host = EditorHost::new();
        host.set_text("MATCH".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":6,"class":"keyword"}]"#);
        assert_eq!(host.snap_offset_for_atomic(2), 0);
        assert_eq!(host.snap_offset_for_atomic(4), 6);
        assert_eq!(host.snap_offset_for_atomic(10), 10);
    }

    #[test]
    fn token_span_at_offset_returns_none_outside_tokens() {
        let mut host = EditorHost::new();
        host.set_text("abc".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":1,"class":"x"}]"#);
        assert_eq!(host.token_span_at_offset(2), None);
        assert_eq!(host.token_span_at_offset(0), Some((0, 1)));
    }

    #[test]
    fn token_boundaries_detect_adjacent_tokens() {
        let mut host = EditorHost::new();
        host.set_text("ab cd".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":2,"class":"x"},{"start":3,"end":5,"class":"y"}]"#);
        assert_eq!(host.token_left_boundary(1), Some(0));
        assert_eq!(host.token_left_boundary(2), Some(0));
        assert_eq!(host.token_left_boundary(6), None);
        assert_eq!(host.token_right_boundary(4), Some(5));
        assert_eq!(host.token_right_boundary(2), None);
    }

    #[test]
    fn allowed_composite_selection_matches_full_span_or_default_false() {
        let mut host = EditorHost::new();
        host.set_text("abc".into());
        let span = SelectableSpanJson { start: 0, end: 3, kind: "custom".into(), head_end: None, tail_start: None };
        assert!(host.allowed_composite_selection(0, 3, &span));
        assert!(!host.allowed_composite_selection(0, 2, &span));
    }
}
// #endregion 🔖Tests
