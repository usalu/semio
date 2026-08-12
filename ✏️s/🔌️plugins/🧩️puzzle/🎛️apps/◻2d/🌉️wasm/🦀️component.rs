//! 🎲️ Puzzle board (2d) rendering surface — WebGPU canvas session wrapping `puzzle_2d_engine`'s
//! headless `BoardHost`/`BoardEngine` compute (constitutional: puzzle-owned rendering surface, moved
//! out of the generic framework `🗺️surface` module — see audit finding "A4" — because it names and
//! `use`s puzzle-specific artifact types directly; keeps the puzzle 2d app's `⚙️engine` slot free of
//! wasm-bindgen/web-sys/wgpu so a workflow runner can drive it headlessly). Mirrors
//! `framework/surface/node-graph`'s `GraphHost`/`GraphSession` split, except the pure host
//! (`BoardHost`) already lives in `infinite_board_port_directed_normal` and is re-exported by this
//! crate's own puzzle-2d engine — this module only owns the wasm session wrapper around it.

use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use crate::apps::puzzle2d::engine::{
    apply_edge_handle_snap_to_fixture_v1_json, canvas, compute_edge_bezier_points, distance_point_to_cubic_bezier, handle_position_on_circle, handle_position_on_rectangle, normalize_board_descriptor_hidden_to_visible, puzzle_2d_lod_scale_json,
    BoardHost, CubicBez, Point, SceneDescriptorJson,
};
use crate::apps::puzzle2d::engine::board_host::{puzzle_board_host, puzzle_board_host_normal};
use crate::apps::puzzle2d::engine::layout::redraw_layout_fixture_json;

// #region 🔖️WasmHost
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
use js_sys::Promise;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;
#[cfg(target_arch = "wasm32")]
use math::geometry::geometry::ray_from_origin_to_axis_aligned_rectangle_edge;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardComputeEdgeBezier)]
pub fn board_compute_edge_bezier(source_px: f64, source_py: f64, source_cx: f64, source_cy: f64, target_px: f64, target_py: f64, target_cx: f64, target_cy: f64) -> Vec<f64> {
    let c = compute_edge_bezier_points(Point::new(source_px, source_py), Point::new(target_px, target_py), Point::new(source_cx, source_cy), Point::new(target_cx, target_cy));
    vec![c.p0.x, c.p0.y, c.p1.x, c.p1.y, c.p2.x, c.p2.y, c.p3.x, c.p3.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardDistancePointCubic)]
pub fn board_distance_point_cubic(px: f64, py: f64, p0x: f64, p0y: f64, p1x: f64, p1y: f64, p2x: f64, p2y: f64, p3x: f64, p3y: f64, steps: u32) -> f64 {
    let curve = CubicBez::new(Point::new(p0x, p0y), Point::new(p1x, p1y), Point::new(p2x, p2y), Point::new(p3x, p3y));
    distance_point_to_cubic_bezier(Point::new(px, py), curve, steps.max(1) as usize)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRayRectEdge)]
pub fn board_ray_rect_edge(hw: f64, hh: f64, ux: f64, uy: f64) -> Vec<f64> {
    let p = ray_from_origin_to_axis_aligned_rectangle_edge(hw, hh, ux, uy);
    vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardHandlePositionCircle)]
pub fn board_handle_position_circle(cx: f64, cy: f64, radius: f64, angle: f64) -> Vec<f64> {
    let p = handle_position_on_circle(Point::new(cx, cy), radius, angle);
    vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardHandlePositionRectangle)]
pub fn board_handle_position_rectangle(cx: f64, cy: f64, width: f64, height: f64, angle: f64) -> Vec<f64> {
    let p = handle_position_on_rectangle(Point::new(cx, cy), width, height, angle);
    vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRedrawLayoutFixtureJson)]
pub fn board_redraw_layout_fixture_json(fixture_json: &str, options_json: &str) -> Result<String, JsValue> {
    redraw_layout_fixture_json(fixture_json, options_json).map_err(|e| JsValue::from_str(&e))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRedrawHandlesFixtureJson)]
pub fn board_redraw_handles_fixture_json(fixture_json: &str) -> Result<String, JsValue> {
    apply_edge_handle_snap_to_fixture_v1_json(fixture_json).map_err(|e| JsValue::from_str(&e))
}

/// 🔤️ Parses `.puzzle2d` DSL text (`Puzzle2dSnapshot`'s `dsl::DslArtifact` grammar) into the same camelCase JSON shape callers previously got from a hand-authored `*.2d.json` fixture — lets non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating the DSL grammar.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = puzzle2dParseDslJson)]
pub fn puzzle2d_parse_dsl_json(dsl_text: &str) -> Result<String, JsValue> {
    use store::ArtifactDsl;
    let snapshot = Puzzle2dSnapshot::parse_dsl(dsl_text).map_err(|error| JsValue::from_str(&error.to_string()))?;
    serde_json::to_string(&snapshot).map_err(|error| JsValue::from_str(&error.to_string()))
}

// #region 🔖️WasmSession
/// 🖥️ Single WASM entry: one {@link BoardHost}, optional WebGPU surface bound via {@link BoardSession::attach_canvas}.
#[cfg(target_arch = "wasm32")]
struct BoardSessionInner {
    host: BoardHost,
    gpu: canvas::gpu_session::CanvasGpuSession,
}

#[cfg(target_arch = "wasm32")]
impl BoardSessionInner {
    fn set_logical_size_and_maybe_resize_surface(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.host.set_size(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        let scene = self.host.build_vector_scene();
        let clear = self.host.canvas_theme.raster_clear;
        self.gpu.render_frame(&scene, clear)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct BoardSession {
    state: Rc<RefCell<BoardSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl BoardSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(BoardSessionInner { host: puzzle_board_host(), gpu: canvas::gpu_session::CanvasGpuSession::default() })) }
    }

    /// 🧠️ Construct a normal-graph session (no handles; edges connect node ids).
    #[wasm_bindgen(js_name = newNormal)]
    pub fn new_normal() -> Self {
        Self { state: Rc::new(RefCell::new(BoardSessionInner { host: puzzle_board_host_normal(), gpu: canvas::gpu_session::CanvasGpuSession::default() })) }
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = isDraggingAreaSelect)]
    pub fn is_dragging_area_select(&self) -> bool {
        self.state.borrow().host.is_dragging_area_select()
    }

    #[wasm_bindgen(js_name = defersDescriptorSyncFromJs)]
    pub fn defers_descriptor_sync_from_js(&self) -> bool {
        self.state.borrow().host.defers_descriptor_sync_from_js()
    }

    /// @emoji 🌊️ Binds WebGPU presentation to `canvas` once; `logical_w`/`logical_h` are CSS pixels, `dpr` scales the swapchain backing store; uses `future_to_promise` so wasm-bindgen does not hold `&mut BoardSession` across `await` (avoids `borrow_fail` vs `setSize` during GPU setup).
    #[wasm_bindgen(js_name = attach_canvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> Promise {
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
            let (render_ctx, renderer, surface) = canvas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
            let mut g = inner.borrow_mut();
            if g.gpu.gpu_ready() {
                return Err(JsValue::from_str("canvas surface already attached"));
            }
            g.host.set_size(lw, lh, dpr);
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
        let mut inner = self.state.borrow_mut();
        inner.set_logical_size_and_maybe_resize_surface(lw, lh, dpr, pw, ph);
    }

    #[wasm_bindgen(js_name = setSelectionScreenPreview)]
    pub fn set_selection_screen_preview(&mut self, flat_xy: &[f64]) {
        let mut inner = self.state.borrow_mut();
        if flat_xy.len() < 4 || flat_xy.len() % 2 != 0 {
            inner.host.set_selection_screen_preview(None);
            return;
        }
        let mut pts = Vec::with_capacity(flat_xy.len() / 2);
        for chunk in flat_xy.chunks_exact(2) {
            pts.push(Point::new(chunk[0], chunk[1]));
        }
        inner.host.set_selection_screen_preview(Some(pts));
    }

    #[wasm_bindgen(js_name = clearSelectionScreenPreview)]
    pub fn clear_selection_screen_preview(&mut self) {
        self.state.borrow_mut().host.set_selection_screen_preview(None);
    }

    #[wasm_bindgen(js_name = syncDescriptorJson)]
    pub fn sync_descriptor_json(&mut self, json: &str) -> Result<(), JsValue> {
        let mut raw: serde_json::Value = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        normalize_board_descriptor_hidden_to_visible(&mut raw);
        let desc: SceneDescriptorJson = serde_json::from_value(raw).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.state.borrow_mut().host.sync_descriptor(&desc).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(())
    }

    #[wasm_bindgen(js_name = setNodePositionsJson)]
    pub fn set_node_positions_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_node_positions_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setKindCatalogsJson)]
    pub fn set_board_kind_catalogs_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_board_kind_catalogs_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setCanvasThemeJson)]
    pub fn set_canvas_theme_json(&mut self, json: &str) {
        let _ = self.state.borrow_mut().host.set_canvas_theme_from_json(json);
    }

    #[wasm_bindgen(js_name = clearIconVectorCache)]
    pub fn clear_icon_vector_cache_wasm(&mut self) {
        self.state.borrow_mut().host.clear_icon_vector_cache();
    }

    #[wasm_bindgen(js_name = parseFixtureJson)]
    pub fn parse_fixture_json(&mut self, json: &str) -> bool {
        let raw: serde_json::Value = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(_) => return false,
        };
        self.state.borrow_mut().host.parse_fixture_v1(&raw)
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera_wasm(&mut self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    /// 🎥️ Sets the camera without enqueuing a `camera` event — for re-applying the framing camera after a fixture re-parse without echoing it back to the program.
    #[wasm_bindgen(js_name = setCameraSilent)]
    pub fn set_camera_silent_wasm(&mut self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera_silent(x, y, zoom);
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    pub fn pointer_down_screen_wasm(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen_wasm(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    #[wasm_bindgen(js_name = pickTargetsAtScreenJson)]
    pub fn pick_targets_at_screen_json_wasm(&self, sx: f64, sy: f64) -> String {
        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen_wasm(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    #[wasm_bindgen(js_name = pointerLeaveScreen)]
    pub fn pointer_leave_screen_wasm(&mut self, alt: bool) {
        self.state.borrow_mut().host.pointer_leave_screen(alt);
    }

    #[wasm_bindgen(js_name = cancelAreaSelect)]
    pub fn cancel_area_select_wasm(&mut self) -> bool {
        self.state.borrow_mut().host.cancel_area_select()
    }

    #[wasm_bindgen(js_name = wheelScreen)]
    pub fn wheel_screen_wasm(&mut self, sx: f64, sy: f64, delta_y: f64) {
        self.state.borrow_mut().host.wheel_screen(sx, sy, delta_y);
    }

    #[wasm_bindgen(js_name = setWheelZoomActive)]
    pub fn set_wheel_zoom_active_wasm(&mut self, active: bool) {
        self.state.borrow_mut().host.set_wheel_zoom_active(active);
    }

    #[wasm_bindgen(js_name = deleteSelection)]
    pub fn delete_selection_wasm(&mut self) {
        self.state.borrow_mut().host.delete_selection();
    }

    #[wasm_bindgen(js_name = drainEventsJson)]
    pub fn drain_events_json_wasm(&mut self) -> String {
        self.state.borrow_mut().host.drain_events_json()
    }

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json(&self) -> String {
        let inner = self.state.borrow();
        serde_json::json!({
            "x": inner.host.camera.x,
            "y": inner.host.camera.y,
            "zoom": inner.host.camera.zoom,
        })
        .to_string()
    }

    #[wasm_bindgen(js_name = overlayPaintStateJson)]
    pub fn overlay_paint_state_json_wasm(&self) -> String {
        self.state.borrow().host.overlay_paint_state_json()
    }

    #[wasm_bindgen(js_name = setSelectionOptions)]
    pub fn set_selection_options_wasm(&mut self, method: &str, mode: &str, select_nodes: bool, select_edges: bool, select_handles: bool) {
        self.state.borrow_mut().host.set_selection_options(method, mode, select_nodes, select_edges, select_handles);
    }

    #[wasm_bindgen(js_name = setHandleLinkCompatJson)]
    pub fn set_handle_link_compat_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_handle_link_compat_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setWorldRasterTiling)]
    pub fn set_world_raster_tiling_wasm(&mut self, mode: &str) {
        self.state.borrow_mut().host.set_world_raster_tiling(mode);
    }

    #[wasm_bindgen(js_name = lodScaleJson)]
    pub fn lod_scale_json_wasm(&self) -> String {
        puzzle_2d_lod_scale_json()
    }

    #[wasm_bindgen(js_name = setGridSnapEnabled)]
    pub fn set_grid_snap_enabled_wasm(&mut self, enabled: bool) {
        self.state.borrow_mut().host.set_grid_snap_enabled(enabled);
    }

    #[wasm_bindgen(js_name = setGridFactor)]
    pub fn set_grid_factor_wasm(&mut self, v: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_grid_factor(v).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setActiveUtility)]
    pub fn set_active_utility_wasm(&mut self, label: &str) {
        self.state.borrow_mut().host.set_active_utility(label);
    }

    #[wasm_bindgen(js_name = setSuggestionOffset)]
    pub fn set_suggestion_offset_wasm(&mut self, distance: f64) {
        self.state.borrow_mut().host.set_suggestion_offset(distance);
    }

    #[wasm_bindgen(js_name = setBrushKindWeights)]
    pub fn set_brush_kind_weights_wasm(&mut self, json: &str) {
        self.state.borrow_mut().host.set_brush_kind_weights(json);
    }

    #[wasm_bindgen(js_name = setBrushNodeSize)]
    pub fn set_brush_node_size_wasm(&mut self, size: f64) {
        self.state.borrow_mut().host.set_brush_node_size(size);
    }

    #[wasm_bindgen(js_name = brushCycleCandidate)]
    pub fn brush_cycle_candidate_wasm(&mut self, forward: bool) {
        self.state.borrow_mut().host.brush_cycle_candidate(forward);
    }

    #[wasm_bindgen(js_name = brushSetCandidateIndex)]
    pub fn brush_set_candidate_index_wasm(&mut self, index: u32) {
        self.state.borrow_mut().host.brush_set_candidate_index(index as usize);
    }

    #[wasm_bindgen(js_name = brushOpenSlot)]
    pub fn brush_open_slot_wasm(&mut self, handle_id: &str) {
        self.state.borrow_mut().host.brush_open_slot(handle_id);
    }

    #[wasm_bindgen(js_name = brushCommitSlot)]
    pub fn brush_commit_slot_wasm(&mut self) {
        self.state.borrow_mut().host.brush_commit_slot();
    }

    #[wasm_bindgen(js_name = brushCancelSlot)]
    pub fn brush_cancel_slot_wasm(&mut self) {
        self.state.borrow_mut().host.brush_cancel_slot();
    }

    #[wasm_bindgen(js_name = brushFillJson)]
    pub fn brush_fill_json_wasm(&self, max_count: u32, seed: u32) -> String {
        self.state.borrow().host.brush_fill_json(max_count, u64::from(seed))
    }

    #[wasm_bindgen(js_name = brushFillSessionBegin)]
    pub fn brush_fill_session_begin_wasm(&mut self, max_count: u32, seed: u32) {
        self.state.borrow_mut().host.brush_fill_session_begin(max_count, u64::from(seed));
    }

    #[wasm_bindgen(js_name = brushFillSessionStep)]
    pub fn brush_fill_session_step_wasm(&mut self, chunk_budget: u32) -> String {
        self.state.borrow_mut().host.brush_fill_session_step(chunk_budget)
    }

    #[wasm_bindgen(js_name = brushFillSessionClear)]
    pub fn brush_fill_session_clear_wasm(&mut self) {
        self.state.borrow_mut().host.brush_fill_session_clear();
    }

    #[wasm_bindgen(js_name = setBrushSessionJson)]
    pub fn set_brush_session_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_brush_session_mirror_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = clearBrushSessionJson)]
    pub fn clear_brush_session_json_wasm(&mut self) {
        let _ = self.state.borrow_mut().host.set_brush_session_mirror_json("");
    }

    #[wasm_bindgen(js_name = setFixtureDropPreviewJson)]
    pub fn set_fixture_drop_preview_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_fixture_drop_preview_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = clearFixtureDropPreview)]
    pub fn clear_fixture_drop_preview_wasm(&mut self) {
        let _ = self.state.borrow_mut().host.set_fixture_drop_preview_json("");
    }

    #[wasm_bindgen(js_name = setLinkSessionJson)]
    pub fn set_link_session_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_external_link_preview_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = clearLinkSessionJson)]
    pub fn clear_link_session_json_wasm(&mut self) {
        self.state.borrow_mut().host.clear_external_link_preview();
    }

    #[wasm_bindgen(js_name = setOriginalElementStyle)]
    pub fn set_original_element_style_wasm(&mut self, enabled: bool) {
        self.state.borrow_mut().host.set_original_element_style(enabled);
    }

    #[wasm_bindgen(js_name = setAutomaticLod)]
    pub fn set_automatic_lod_wasm(&mut self, enabled: bool) {
        self.state.borrow_mut().host.set_automatic_lod(enabled);
    }

    #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
    pub fn set_forced_draw_lod_label_wasm(&mut self, label: &str) {
        self.state.borrow_mut().host.set_forced_draw_lod_label(label);
    }

    #[wasm_bindgen(js_name = setSelectionIdsJson)]
    pub fn set_selection_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
        let ids: Vec<String> = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.set_selection_ids(&ids);
        Ok(())
    }

    #[wasm_bindgen(js_name = setSelectionIdsJsonSilent)]
    pub fn set_selection_ids_json_silent(&mut self, json: &str) -> Result<(), JsValue> {
        let ids: Vec<String> = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.set_selection_ids_silent(&ids);
        Ok(())
    }

    #[wasm_bindgen(js_name = setPreselectStateJsonSilent)]
    pub fn set_preselect_state_json_silent(&mut self, json: &str) -> Result<(), JsValue> {
        #[derive(serde::Deserialize)]
        struct PreselectSync {
            ids: Vec<String>,
            #[serde(default, rename = "removedIds")]
            removed_ids: Vec<String>,
        }
        let body: PreselectSync = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.set_preselect_state_silent(&body.ids, &body.removed_ids);
        Ok(())
    }

    #[wasm_bindgen(js_name = setHoveredIdSilent)]
    pub fn set_hovered_id_silent_wasm(&mut self, id: Option<String>) {
        self.state.borrow_mut().host.set_hovered_id_silent(id);
    }

    #[wasm_bindgen(js_name = setHoveredKindSilent)]
    pub fn set_hovered_kind_silent_wasm(&mut self, domain: Option<String>, kind_id: Option<String>) {
        self.state.borrow_mut().host.set_hovered_kind_silent(domain, kind_id);
    }

    #[wasm_bindgen(js_name = encodedSceneHint)]
    pub fn encoded_scene_hint_wasm(&self) -> usize {
        self.state.borrow().host.encoded_scene_hint()
    }

    /// @emoji 🎨️ Presents one frame when a GPU surface is attached; otherwise no-operation `Ok`.
    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }
}
// #endregion 🔖️WasmSession
// #endregion 🔖️WasmHost
