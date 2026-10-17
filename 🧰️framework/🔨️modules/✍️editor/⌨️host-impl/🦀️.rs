//! ⌨️ Host implementation of the editor: the single implementation block of the editor host with its text, selection, composition, camera and projection operations. See [`super`].

use super::*;

// #region 🔖️EditorState
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

    pub(super) fn gutter_width(&self) -> f64 {
        if self.show_line_numbers {
            DEFAULT_GUTTER_WIDTH
        } else {
            0.0
        }
    }

    pub(super) fn line_origin_x(&self) -> f64 {
        self.gutter_width() + PAD_X
    }

    fn content_origin_y(&self) -> f64 {
        if self.chrome_edgeless_scroll || self.dead_line_y <= 0.0 {
            0.0
        } else {
            self.dead_line_y
        }
    }

    pub(super) fn line_y(&self, line: usize) -> f64 {
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

    pub(super) fn scroll_overflows(&self) -> bool {
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
    pub(super) fn set_caret_anchor(&mut self, offset: usize) {
        self.caret = offset;
        self.anchor = offset;
    }

    #[cfg(test)]
    pub(super) fn set_selection(&mut self, anchor: usize, caret: usize) {
        self.set_selection_range(anchor, caret);
    }

    /// ⌨️ Places the selection exactly: token and span boundaries never widen a selection the input made.
    pub fn set_selection_range(&mut self, anchor: usize, caret: usize) {
        self.anchor = char_boundary_at_or_before(&self.text, anchor);
        self.caret = char_boundary_at_or_before(&self.text, caret);
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
            let offset = char_boundary_at_or_before(&self.text, offset);
            self.anchor = offset;
            self.caret = offset;
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

    /// 🎬️ Commits one completely decoded typed update without transport inference.
    pub fn synchronize_scene(&mut self, scene: EditorScene) {
        if let Some(buffer)=scene.buffer { self.set_text(buffer); }
        if let Some(range)=scene.selection { self.set_selection_range(range.start,range.end); }
        if let Some(tokens)=scene.tokens { self.semantic_tokens=tokens; }
        if let Some(diagnostics)=scene.diagnostics { self.diagnostics=diagnostics; }
        if let Some(placeholders)=scene.placeholders { self.placeholders=placeholders; }
        if let Some(occurrences)=scene.occurrences {
            if let Some(hover)=occurrences.hover { self.hover_occurrences=hover; }
            if let Some(selection)=occurrences.selection { self.selection_occurrences=selection; }
        }
        if let Some(carets)=scene.extra_carets { self.extra_carets=carets; }
        if let Some(spans)=scene.selectable_spans { self.selectable_spans=spans; }
        if let Some(settings)=scene.settings {
            self.font_px=settings.font_px.clamp(10.0,28.0);
            self.line_height=settings.line_height.clamp(16.0,48.0);
            self.show_line_numbers=settings.show_line_numbers;
            self.tab_size=settings.tab_size.clamp(1,8);
            self.clamp_camera();
        }
        if let Some(camera)=scene.camera { self.set_camera(0.0,camera.y,1.0); }
        if let Some(overlays)=scene.overlays { if let Some(y)=overlays.dead_line_y { self.set_dead_line_y(y); } }
        if let Some(hover)=scene.hover {
            match hover { scene::Hover::Clear=>self.set_hover_range(None,None),scene::Hover::Range{start,end}=>self.set_hover_range(Some(start),Some(end)) }
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
        let offset = self.hit_test_offset(world);
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
            self.caret = self.hit_test_offset(world);
            return;
        }
        let world = editor_screen_to_world(&self.camera, Point::new(sx, sy));
        self.set_hover_at_offset(self.hit_test_offset(world));
    }

    pub fn pointer_up_screen(&mut self, _sx: f64, _sy: f64, button: i32) {
        if button == 0 {
            self.drag_selecting = false;
            self.finish_caret_update();
        }
    }

    pub fn pointer_cancel_screen(&mut self) {
        self.drag_selecting = false;
    }

    /// ⌨️ The text-input model: `chunk` replaces the exact selection, or is inserted at the caret, and the caret lands after
    /// it — never a space the input did not type, never a token replaced because the caret sat inside it. Semantic tokens
    /// colour and describe the text; they never move an edit (ticket 26/09/23 F1: typing a word into the trinity query
    /// editor inserted a space before every letter once the guest's tokens for the half-typed word arrived, and a letter
    /// typed inside a stale token replaced the whole token).
    pub fn insert_text(&mut self, chunk: &str) {
        let (start, end) = self.selection_range();
        self.text.replace_range(start..end, chunk);
        self.caret = start + chunk.len();
        self.anchor = self.caret;
        self.finish_caret_update();
    }

    /// ⌫️ Deletes the exact selection, or the one character before the caret.
    pub fn backspace(&mut self) {
        let (start, end) = self.selection_range();
        if start != end {
            self.insert_text("");
            return;
        }
        if self.caret == 0 {
            return;
        }
        let prev = prev_char_boundary(&self.text, self.caret);
        self.text.replace_range(prev..self.caret, "");
        self.caret = prev;
        self.anchor = prev;
        self.finish_caret_update();
    }

    /// ⌦️ Deletes the exact selection, or the one character after the caret.
    pub fn delete_forward(&mut self) {
        let (start, end) = self.selection_range();
        if start != end {
            self.insert_text("");
            return;
        }
        if self.caret >= self.text.len() {
            return;
        }
        let next = next_char_boundary(&self.text, self.caret);
        self.text.replace_range(self.caret..next, "");
        self.anchor = self.caret;
        self.finish_caret_update();
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

    /// ⬅️ One character left; without `extend` a selection collapses to its start.
    pub fn move_left(&mut self, extend: bool) {
        let (start, end) = self.selection_range();
        self.caret = if !extend && start != end {
            start
        } else if self.caret == 0 {
            0
        } else {
            prev_char_boundary(&self.text, self.caret)
        };
        if !extend {
            self.anchor = self.caret;
            self.finish_caret_update();
        }
    }

    /// ➡️ One character right; without `extend` a selection collapses to its end.
    pub fn move_right(&mut self, extend: bool) {
        let (start, end) = self.selection_range();
        self.caret = if !extend && start != end {
            end
        } else if self.caret >= self.text.len() {
            self.text.len()
        } else {
            next_char_boundary(&self.text, self.caret)
        };
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

    pub(super) fn token_span_at_offset(&self, offset: usize) -> Option<(usize, usize)> {
        for token in &self.semantic_tokens {
            if offset >= token.start && offset < token.end {
                return Some((token.start, token.end));
            }
        }
        None
    }

    fn selection_range(&self) -> (usize, usize) {
        (self.caret.min(self.anchor), self.caret.max(self.anchor))
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

    /// 🎯️ Returns pick-target rows at a screen point for DOM disambiguation menus.
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
// #endregion 🔖️EditorState
