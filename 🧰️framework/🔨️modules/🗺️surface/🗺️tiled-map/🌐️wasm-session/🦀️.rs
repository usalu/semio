//! 🌐️ Browser `MapSession` bridge: a wasm-bindgen wrapper around [`super::MapHost`] plus its GPU session.
//!
//! Compiled only for `wasm32` browser targets (not `wasm32-wasip2`); every item keeps its own `cfg` gate.

use super::*;

// #region 🔖️WasmSession
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this session bridge is
// browser-only (attaches an `HtmlCanvasElement`), so it is narrowed to exclude the WASI
// component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_async::browser::future_to_promise;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use std::rc::Rc;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use web_sys::HtmlCanvasElement;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
struct MapSessionInner {
    host: MapHost,
    gpu: gpu_session::CanvasGpuSession,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl MapSessionInner {
    fn set_logical_size(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.host.set_size(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        self.host.prepare_visible_tiles();
        let scene = self.host.build_render_scene();
        self.gpu.render_frame(&scene, canvas_content::CanvasContent::clear_color(&self.host))
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
pub struct MapSession {
    state: Rc<RefCell<MapSessionInner>>,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
impl MapSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(MapSessionInner { host: MapHost::new(), gpu: gpu_session::CanvasGpuSession::default() })) }
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = attachCanvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
        let inner = self.state.clone();
        if inner.borrow().gpu.gpu_ready() {
            return future_to_promise(async move { Err(JsValue::from_str("canvas surface already attached")) });
        }
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let canvas = canvas.clone();
        future_to_promise(async move {
            let admission = gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|e| JsValue::from_str(&e))?;
            let mut g = inner.borrow_mut();
            if g.gpu.gpu_ready() {
                return Err(JsValue::from_str("canvas surface already attached"));
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

    #[wasm_bindgen(js_name = fitWorldCamera)]
    pub fn fit_world_camera(&mut self) {
        self.state.borrow_mut().host.fit_world_camera();
    }

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

    #[wasm_bindgen(js_name = syncMapJson)]
    pub fn sync_map_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.sync_map_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = uploadTile)]
    pub fn upload_tile(&mut self, z: u32, x: u32, y: u32, bytes: &[u8]) -> Result<(), JsValue> {
        self.state.borrow_mut().host.upload_tile(z, x, y, bytes).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = uploadVectorTile)]
    pub fn upload_vector_tile(&mut self, z: u32, x: u32, y: u32, bytes: &[u8]) -> Result<(), JsValue> {
        self.state.borrow_mut().host.upload_vector_tile(z, x, y, bytes).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setRenderMode)]
    pub fn set_render_mode(&mut self, mode: &str) {
        self.state.borrow_mut().host.set_render_mode(mode);
    }

    #[wasm_bindgen(js_name = setVectorStyle)]
    pub fn set_vector_style(&mut self, style: &str) {
        self.state.borrow_mut().host.set_vector_style(style);
    }

    #[wasm_bindgen(js_name = vectorStyleStr)]
    pub fn vector_style_str(&self) -> String {
        self.state.borrow().host.vector_style_str().to_string()
    }

    #[wasm_bindgen(js_name = setLodMode)]
    pub fn set_lod_mode(&mut self, mode: &str) {
        self.state.borrow_mut().host.set_lod_mode(mode);
    }

    #[wasm_bindgen(js_name = setLayerVisibilityJson)]
    pub fn set_layer_visibility_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_layer_visibility_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = layerVisibilityJson)]
    pub fn layer_visibility_json_wasm(&self) -> String {
        self.state.borrow().host.layer_visibility_json()
    }

    #[wasm_bindgen(js_name = setLayerStrokeScaleJson)]
    pub fn set_layer_stroke_scale_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_layer_stroke_scale_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = layerStrokeScaleJson)]
    pub fn layer_stroke_scale_json_wasm(&self) -> String {
        self.state.borrow().host.layer_stroke_scale_json()
    }

    #[wasm_bindgen(js_name = layerWeightSliderIdsJson)]
    pub fn layer_weight_slider_ids_json(&self, lod_id: &str, render_mode: &str) -> String {
        gis_map_layer_weight_slider_ids_json(lod_id, render_mode)
    }

    #[wasm_bindgen(js_name = setMapThemeJson)]
    pub fn set_map_theme_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_map_theme_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = visibleVectorTilesJson)]
    pub fn visible_vector_tiles_json_wasm(&self) -> String {
        self.state.borrow().host.visible_vector_tiles_json()
    }

    #[wasm_bindgen(js_name = drainEventsJson)]
    pub fn drain_events_json(&mut self) -> String {
        self.state.borrow_mut().host.drain_events_json()
    }

    #[wasm_bindgen(js_name = positionScreenJson)]
    pub fn position_screen_json_wasm(&self, id: &str) -> String {
        self.state.borrow().host.position_screen_json(id)
    }

    #[wasm_bindgen(js_name = hitTestFeatureJson)]
    pub fn hit_test_feature_json_wasm(&self, sx: f64, sy: f64) -> String {
        self.state.borrow().host.hit_test_feature_json(sx, sy)
    }

    #[wasm_bindgen(js_name = featuresInRectJson)]
    pub fn features_in_rect_json_wasm(&self, x0: f64, y0: f64, x1: f64, y1: f64, crossing: bool) -> String {
        self.state.borrow().host.features_in_rect_json(x0, y0, x1, y1, crossing)
    }

    #[wasm_bindgen(js_name = featuresInPolygonJson)]
    pub fn features_in_polygon_json_wasm(&self, points_json: &str, crossing: bool) -> String {
        self.state.borrow().host.features_in_polygon_json(points_json, crossing)
    }

    #[wasm_bindgen(js_name = featureScreenJson)]
    pub fn feature_screen_json_wasm(&self, kind: &str, id: &str) -> String {
        self.state.borrow().host.feature_screen_json(kind, id)
    }

    /// 🕹️ Replaces the deleted `setSelectionJson`/`setHoverJson` — `granularity` is the domain's active
    /// granularity (`"position"`|`"route"`), `selectedIdsJson`/`hoveredId` the caller's resolved
    /// `DomainSelection.ids`/`DomainHover.ids.first()`, read from `InteractionState` at render time.
    #[wasm_bindgen(js_name = syncInteraction)]
    pub fn sync_interaction_wasm(&mut self, granularity: &str, selected_ids_json: &str, hovered_id: Option<String>) -> Result<(), JsValue> {
        let ids: Vec<String> = if selected_ids_json.trim().is_empty() { Vec::new() } else { serde_json::from_str(selected_ids_json).map_err(|e| JsValue::from_str(&e.to_string()))? };
        self.state.borrow_mut().host.sync_interaction(granularity, &ids, hovered_id.as_deref());
        Ok(())
    }

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json_wasm(&self) -> String {
        self.state.borrow().host.camera_json()
    }

    #[wasm_bindgen(js_name = visibleTilesJson)]
    pub fn visible_tiles_json_wasm(&self) -> String {
        self.state.borrow().host.visible_tiles_json()
    }

    #[wasm_bindgen(js_name = hasTile)]
    pub fn has_tile_wasm(&self, z: u32, x: u32, y: u32) -> bool {
        self.state.borrow().host.has_tile(&tiles::tile_key(z, x, y))
    }

    #[wasm_bindgen(js_name = hasVectorTile)]
    pub fn has_vector_tile_wasm(&self, z: u32, x: u32, y: u32) -> bool {
        self.state.borrow().host.has_vector_tile(&tiles::tile_key(z, x, y))
    }

    /// 🔢️ Masked to 53 bits so the `u64` revision round-trips exactly through a JS `number`.
    #[wasm_bindgen(js_name = visibleTilesRevision)]
    pub fn visible_tiles_revision_wasm(&self) -> f64 {
        (self.state.borrow().host.visible_tiles_revision() & ((1u64 << 53) - 1)) as f64
    }

    #[wasm_bindgen(js_name = visibleVectorTilesRevision)]
    pub fn visible_vector_tiles_revision_wasm(&self) -> f64 {
        (self.state.borrow().host.visible_vector_tiles_revision() & ((1u64 << 53) - 1)) as f64
    }

    #[wasm_bindgen(js_name = prefetchTilesJson)]
    pub fn prefetch_tiles_json_wasm(&self) -> String {
        self.state.borrow().host.prefetch_tiles_json()
    }

    #[wasm_bindgen(js_name = prefetchVectorTilesJson)]
    pub fn prefetch_vector_tiles_json_wasm(&self) -> String {
        self.state.borrow().host.prefetch_vector_tiles_json()
    }

    #[wasm_bindgen(js_name = currentLodJson)]
    pub fn current_lod_json_wasm(&self) -> String {
        self.state.borrow().host.current_lod_json()
    }

    #[wasm_bindgen(js_name = lodScaleJson)]
    pub fn lod_scale_json_wasm(&self) -> String {
        gis_map_lod_scale_json()
    }

    #[wasm_bindgen(js_name = cameraLimitsJson)]
    pub fn camera_limits_json_wasm(&self) -> String {
        gis_map_camera_limits_json_for_viewport(&self.state.borrow().host.viewport)
    }

    #[wasm_bindgen(js_name = reclampCamera)]
    pub fn reclamp_camera(&mut self) {
        self.state.borrow_mut().host.clamp_camera_to_world();
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }
}
// #endregion 🔖️WasmSession
