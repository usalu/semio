//! 🌐️ Browser session of surface paint: the canvas-bound raster session and its inner state, compiled only for the browser target. See [`super`].

use super::*;

// #region 🔖️WasmSession
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
struct RasterSessionInner {
    pub(super) host: RasterHost,
    gpu: gpu_session::CanvasGpuSession,
    isolated_view: Option<String>,
    view_mode: String,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl RasterSessionInner {
    fn set_logical_size(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.host.set_size(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        let scene = match self.view_mode.as_str() {
            "layer" => {
                let id = self.isolated_view.clone().unwrap_or_default();
                render::scale_scene_for_device_pixel_ratio(self.host.build_layer_scene(&id), self.host.viewport.dpr)
            }
            "mask" => {
                let id = self.isolated_view.clone().unwrap_or_default();
                render::scale_scene_for_device_pixel_ratio(self.host.build_mask_scene(&id), self.host.viewport.dpr)
            }
            _ => self.host.build_render_scene(),
        };
        self.gpu.render_frame(&scene, canvas_content::CanvasContent::clear_color(&self.host))
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
pub struct RasterSession {
    state: Rc<RefCell<RasterSessionInner>>,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
impl RasterSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(RasterSessionInner { host: RasterHost::new(), gpu: gpu_session::CanvasGpuSession::default(), isolated_view: None, view_mode: "composite".into() })) }
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
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
            if g.gpu.gpu_ready() {
                g.set_logical_size(lw, lh, dpr, pw, ph);
                return Ok(JsValue::UNDEFINED);
            }
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
    pub fn render_frame(&mut self)->Result<bool,JsValue> {
        let mut state=self.state.borrow_mut();state.render_frame_gpu()?;Ok(state.host.composite_pending())
    }

    #[wasm_bindgen(js_name = renderProgressJson)]
    pub fn render_progress_json(&self)->String{self.state.borrow().host.composite_progress_json()}

    #[wasm_bindgen(js_name = cancelRender)]
    pub fn cancel_render(&mut self){self.state.borrow_mut().host.cancel_composite();}

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = wheelScreen)]
    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        self.state.borrow_mut().host.wheel_screen(sx, sy, delta_y);
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy);
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy);
    }

    #[wasm_bindgen(js_name = pointerCancelScreen)]
    pub fn pointer_cancel_screen(&mut self) {
        self.state.borrow_mut().host.pointer_cancel_screen();
    }

    #[wasm_bindgen(js_name = syncDocumentJson)]
    pub fn sync_document_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.sync_document_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = uploadLayerImage)]
    pub fn upload_layer_image(&mut self, layer_id: &str, bytes: &[u8]) -> Result<(), JsValue> {
        self.state.borrow_mut().host.upload_layer_image(layer_id, bytes).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = uploadRasterImageKey)]
    pub fn upload_raster_image_key(&mut self, key: &str, bytes: &[u8]) -> Result<(), JsValue> {
        self.state.borrow_mut().host.upload_raster_image_key(key, bytes).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setActiveUtility)]
    pub fn set_active_utility(&mut self, utility: &str) {
        self.state.borrow_mut().host.set_active_utility(utility);
    }

    #[wasm_bindgen(js_name = setBrushSize)]
    pub fn set_brush_size(&mut self, size: f32) {
        self.state.borrow_mut().host.set_brush_size(size);
    }

    #[wasm_bindgen(js_name = setBrushOpacity)]
    pub fn set_brush_opacity(&mut self, opacity: f32) {
        self.state.borrow_mut().host.set_brush_opacity(opacity);
    }

    /// 🕹️ Replaces the deleted `setHoveredIdSilent`/`setSelectionIdsJson` — `selectedIdsJson`/
    /// `hoveredId` are the caller's resolved `DomainSelection.ids`/`DomainHover.ids.first()`, read from
    /// the framework's `InteractionState` at render time.
    #[wasm_bindgen(js_name = syncInteraction)]
    pub fn sync_interaction(&mut self, selected_ids_json: &str, hovered_id: Option<String>) -> Result<(), JsValue> {
        let ids: Vec<String> = if selected_ids_json.trim().is_empty() { Vec::new() } else { serde_json::from_str(selected_ids_json).map_err(|e| JsValue::from_str(&e.to_string()))? };
        self.state.borrow_mut().host.sync_interaction(&ids, hovered_id.as_deref());
        Ok(())
    }

    #[wasm_bindgen(js_name = setCanvasThemeJson)]
    pub fn set_canvas_theme_json(&mut self, json: &str) {
        let _ = self.state.borrow_mut().host.set_canvas_theme_from_json(json);
    }

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json(&self) -> String {
        self.state.borrow().host.camera_json()
    }

    #[wasm_bindgen(js_name = setViewMode)]
    pub fn set_view_mode(&mut self, mode: &str, layer_id: Option<String>) {
        let mut g = self.state.borrow_mut();
        g.view_mode = mode.to_string();
        g.isolated_view = layer_id;
        g.host.set_show_selection_chrome(mode != "navigator");
    }

    #[wasm_bindgen(js_name = pickTargetsAtScreenJson)]
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)
    }

    #[wasm_bindgen(js_name = marqueeHitsJson)]
    pub fn marquee_hits_json(&self, query_json: &str) -> Result<String, JsValue> {
        self.state.borrow().host.marquee_hits_json(query_json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = navigatorFitCameraJson)]
    pub fn navigator_fit_camera_json(&self, viewport_w: f64, viewport_h: f64) -> String {
        self.state.borrow().host.navigator_fit_camera_json(viewport_w, viewport_h)
    }

    #[wasm_bindgen(js_name = navigatorViewportOverlayJson)]
    pub fn navigator_viewport_overlay_json(&self, content_camera_json: &str, content_viewport_json: &str) -> Result<String, JsValue> {
        self.state.borrow().host.navigator_viewport_overlay_json(content_camera_json, content_viewport_json).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
// #endregion 🔖️WasmSession
