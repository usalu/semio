//! 🌐️ Browser session of the editor: the canvas-bound editor session and its inner state, compiled only for the browser target. See [`super`].

use super::*;

// #region 🔖️Wasm
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
struct EditorSessionInner {
    pub(super) host: EditorHost,
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
            let admission = gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|e| JsValue::from_str(&e))?;
            let mut g = inner.borrow_mut();
            g.set_logical_size(lw, lh, dpr, pw, ph);
            g.gpu.finish_attach(admission);
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

    #[wasm_bindgen(js_name = synchronizeScene)]
    pub fn synchronize_scene(&mut self, bytes: &[u8], format: u8, maximum_input_bytes: u32, maximum_owned_bytes: u32, progress: Function) -> Result<(), JsValue> {
        let format=match format { 0=>scene::IntrinsicFormat::Body,1=>scene::IntrinsicFormat::Document,_=>return Err(JsValue::from_str("unknown Editor scene frame")) };
        if bytes.len()>maximum_input_bytes as usize { return Err(JsValue::from_str("Editor scene exceeds caller input limit")); }
        let mut options=scene::DecodeOptions::default();options.preserve_unknown=false;
        options.limits.max_file_len=u64::from(maximum_input_bytes);options.limits.max_total_alloc=u64::from(maximum_owned_bytes);
        let mut report=|step:semio_framework_value::native_decoding::NativeDecodeProgress|progress.call3(&JsValue::NULL,&JsValue::from_f64(step.completed as f64),&JsValue::from_f64(step.total as f64),&JsValue::from_f64(step.owned_bytes as f64)).ok().and_then(|value|value.as_bool())==Some(true);
        let mut control=semio_framework_value::NativeDecodeControl::new(maximum_owned_bytes as usize,&mut report);
        let decoded=scene::decode(bytes,format,&options,&mut control).map_err(|error|JsValue::from_str(&error.to_string()))?;
        control.checkpoint().map_err(|error|JsValue::from_str(&error.to_string()))?;
        self.state.borrow_mut().host.synchronize_scene(decoded);Ok(())
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

    #[wasm_bindgen(js_name = pointerCancelScreen)]
    pub fn pointer_cancel_screen(&mut self) {
        self.state.borrow_mut().host.pointer_cancel_screen();
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
