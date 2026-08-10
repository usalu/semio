//! 🌉️ Flow WASM session bindings.

use crate::infinite::board::ports::directed_dag as dag;
use crate::infinite::canvas as canvas;
use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

use dag::{
    computation_node_height, computation_node_width, dag_fixture_execution_rows, dag_fixture_to_wire_literal, fit_node_size, image_widget_size, io_widget_height, io_widget_width, normalize_node_display, note_widget_size, preview_widget_size,
    slider_widget_height, slider_widget_width, would_create_cycle, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodeSpec, DagPreviewContent, EdgeRouteStyle, IoPortSpec,
};
use math::graph::manifest::{PropertyBag, PropertyValue};
use neural::{
    channel_output, cluster_operator_info, compute_dirty_set, Atom, BudgetedEval, ChannelSpec, Dictionary, EvalChannels, EvalError, Evaluator, NeuralCache, Neuron, OperatorImpl, OperatorInfo, Synapse, Tree, TreeSnapshot, Value as NeuralValue, CLUSTER_KIND,
    INPUT_KIND, OUTPUT_KIND,
};
use flow_extension_sdk::FlowExtensionManifest;
use serde::{Deserialize, Serialize};

use crate::artifact::*;
use crate::catalogue::*;
use crate::registry::*;
use crate::bridge::*;
use crate::host::*;
use crate::drawing::*;
use crate::vcs::*;
use crate::brep_geometry::{dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry};


// #region 🔖️WasmSession
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
struct FlowSessionInner {
    host: FlowHost,
    gpu: canvas::gpu_session::CanvasGpuSession,
    width: u32,
    height: u32,
    dpr: f64,
}

#[cfg(target_arch = "wasm32")]
impl FlowSessionInner {
    fn set_logical_size_and_maybe_resize_surface(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.width = lw;
        self.height = lh;
        self.dpr = dpr;
        self.host.set_viewport(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        self.host.sync_dag_ghost();
        let mut scene = canvas::Scene::new();
        let clear = self.host.dag.canvas_theme.raster_clear;
        self.host.paint_scene(&mut scene, self.width, self.height, self.dpr);
        let scene = canvas::render::scale_scene_for_device_pixel_ratio(scene, self.dpr);
        self.gpu.render_frame(&scene, clear)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct FlowSession {
    state: Rc<RefCell<FlowSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl FlowSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(FlowSessionInner { host: FlowHost::default(), gpu: canvas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
    }

    #[wasm_bindgen(js_name = loadFixtureJson)]
    pub fn load_fixture_json(&self, json: &str) -> Result<(), JsValue> {
        let fixture = FlowHost::parse_fixture_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let mut inner = self.state.borrow_mut();
        inner.host.replace_fixture(fixture);
        Ok(())
    }

    #[wasm_bindgen(js_name = resyncFixtureJson)]
    pub fn resync_fixture_json(&self, json: &str) -> Result<(), JsValue> {
        let fixture = FlowHost::parse_fixture_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let mut inner = self.state.borrow_mut();
        inner.host.resync_fixture_from_scene(fixture);
        Ok(())
    }

    #[wasm_bindgen(js_name = fixtureJson)]
    pub fn fixture_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = catalogueJson)]
    pub fn catalogue_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.catalogue_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setCatalogueJson)]
    pub fn set_catalogue_json(&self, json: &str) {
        self.state.borrow_mut().host.set_host_catalogue_json(json);
    }

    #[wasm_bindgen(js_name = setNeuronKindInfosJson)]
    pub fn set_neuron_kind_infos_json(&self, json: &str) {
        self.state.borrow_mut().host.set_neuron_kind_infos_json(json);
    }

    #[wasm_bindgen(js_name = addInputPort)]
    pub fn add_input_port(&self, widget_id: &str, index: u32) -> Result<(), JsValue> {
        self.state.borrow_mut().host.add_input_port(widget_id, index as usize).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = removeInputPort)]
    pub fn remove_input_port(&self, widget_id: &str, port_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.remove_input_port(widget_id, port_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = addOutputPort)]
    pub fn add_output_port(&self, widget_id: &str, index: u32) -> Result<(), JsValue> {
        self.state.borrow_mut().host.add_output_port(widget_id, index as usize).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = removeOutputPort)]
    pub fn remove_output_port(&self, widget_id: &str, port_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.remove_output_port(widget_id, port_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = connectPorts)]
    pub fn connect_ports(&self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, JsValue> {
        self.state.borrow_mut().host.connect_ports(from_id, from_port, to_id, to_port).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = compiledWireLiteral)]
    pub fn compiled_wire_literal(&self) -> String {
        self.state.borrow().host.compiled_wire_literal()
    }

    #[wasm_bindgen(js_name = applyEvalOutputsJson)]
    pub fn apply_eval_outputs_json(&self, json: &str) {
        self.state.borrow_mut().host.apply_eval_outputs_json(json);
    }

    #[wasm_bindgen(js_name = setComputingProgress)]
    pub fn set_computing_progress(&self, json: &str) {
        let payload: serde_json::Value = serde_json::from_str(json).unwrap_or(serde_json::Value::Null);
        let active = payload.get("active").and_then(|value| value.as_str()).map(str::to_string);
        let stale: Vec<String> = payload.get("stale").and_then(|value| value.as_array()).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect()).unwrap_or_default();
        self.state.borrow_mut().host.set_computing_progress(active.as_deref(), &stale);
    }

    #[wasm_bindgen(js_name = setNodeStatuses)]
    pub fn set_node_statuses(&self, json: &str) {
        self.state.borrow_mut().host.set_node_statuses_from_json(json);
    }

    #[wasm_bindgen(js_name = clearComputingWidgetIds)]
    pub fn clear_computing_widget_ids(&self) {
        self.state.borrow_mut().host.clear_computing_widget_ids();
    }

    #[wasm_bindgen(js_name = previewText)]
    pub fn preview_text(&self) -> String {
        self.state.borrow().host.preview_text()
    }

    #[wasm_bindgen(js_name = selectedWidgetIds)]
    pub fn selected_widget_ids(&self) -> String {
        self.state.borrow().host.selected_widget_ids_json()
    }

    #[wasm_bindgen(js_name = selectedEdgeIds)]
    pub fn selected_edge_ids(&self) -> String {
        let domains: serde_json::Value = serde_json::from_str(&self.state.borrow().host.selection_domains_json()).unwrap_or_default();
        domains.get("edges").and_then(|value| serde_json::to_string(value).ok()).unwrap_or_else(|| "[]".into())
    }

    #[wasm_bindgen(js_name = selectionDomainsJson)]
    pub fn selection_domains_json(&self) -> String {
        self.state.borrow().host.selection_domains_json()
    }

    #[wasm_bindgen(js_name = hoveredWidgetId)]
    pub fn hovered_widget_id(&self) -> Option<String> {
        self.state.borrow().host.hovered_widget_id()
    }

    #[wasm_bindgen(js_name = hoveredChannelJson)]
    pub fn hovered_channel_json(&self) -> String {
        self.state.borrow().host.hovered_channel_json()
    }

    #[wasm_bindgen(js_name = selectedChannelsJson)]
    pub fn selected_channels_json(&self) -> String {
        self.state.borrow().host.selected_channels_json()
    }

    #[wasm_bindgen(js_name = previewOffWidgetIds)]
    pub fn preview_off_widget_ids(&self) -> String {
        serde_json::to_string(&self.state.borrow().host.preview_off_widget_ids()).unwrap_or_else(|_| "[]".into())
    }

    #[wasm_bindgen(js_name = setSelection)]
    pub fn set_selection(&self, json: &str) {
        self.state.borrow_mut().host.set_selection_json(json);
    }

    #[wasm_bindgen(js_name = setHover)]
    pub fn set_hover(&self, widget_id: Option<String>) {
        self.state.borrow_mut().host.set_hover(widget_id.as_deref());
    }

    #[wasm_bindgen(js_name = setHoverChannel)]
    pub fn set_hover_channel(&self, widget_id: Option<String>, port: Option<String>) {
        self.state.borrow_mut().host.set_hover_channel(widget_id.as_deref(), port.as_deref());
    }

    #[wasm_bindgen(js_name = setSelectedChannels)]
    pub fn set_selected_channels(&self, json: &str) {
        self.state.borrow_mut().host.set_selected_channels_json(json);
    }

    #[wasm_bindgen(js_name = setPreviewOff)]
    pub fn set_preview_off(&self, json: &str) {
        self.state.borrow_mut().host.set_preview_off_json(json);
    }

    #[wasm_bindgen(js_name = togglePreview)]
    pub fn toggle_preview(&self, widget_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.toggle_preview(widget_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = collapseSelection)]
    pub fn collapse_selection(&self, ids_json: &str) -> Result<String, JsValue> {
        let ids: Vec<String> = serde_json::from_str(ids_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.collapse_selection(&ids).map_err(|err| JsValue::from_str(&err.to_string()))
    }

    #[wasm_bindgen(js_name = explodeCluster)]
    pub fn explode_cluster(&self, cluster_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.explode_cluster(cluster_id).map_err(|err| JsValue::from_str(&err.to_string()))
    }

    #[wasm_bindgen(js_name = takePendingExportClick)]
    pub fn take_pending_export_click(&self) -> Option<String> {
        self.state.borrow_mut().host.take_pending_export_click()
    }

    #[wasm_bindgen(js_name = exportPayloadJson)]
    pub fn export_payload_json(&self, widget_id: &str) -> Result<String, JsValue> {
        self.state.borrow().host.export_payload_json(widget_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = takePendingClusterExplode)]
    pub fn take_pending_cluster_explode(&self) -> Option<String> {
        self.state.borrow_mut().host.take_pending_cluster_explode()
    }

    #[wasm_bindgen(js_name = setSliderValue)]
    pub fn set_slider_value(&self, widget_id: &str, value: f64) {
        self.state.borrow_mut().host.set_slider_value(widget_id, value);
    }

    #[wasm_bindgen(js_name = sliderOverlayStateJson)]
    pub fn slider_overlay_state_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.slider_overlay_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setNoteText)]
    pub fn set_note_text(&self, widget_id: &str, text: &str) {
        self.state.borrow_mut().host.set_note_text(widget_id, text);
    }

    #[wasm_bindgen(js_name = beginNoteEdit)]
    pub fn begin_note_edit(&self, widget_id: &str, world_x: f64, world_y: f64) {
        self.state.borrow_mut().host.begin_note_edit(widget_id, world_x, world_y);
    }

    #[wasm_bindgen(js_name = noteInsertText)]
    pub fn note_insert_text(&self, chunk: &str) {
        self.state.borrow_mut().host.note_insert_text(chunk);
    }

    #[wasm_bindgen(js_name = noteBackspace)]
    pub fn note_backspace(&self) {
        self.state.borrow_mut().host.note_backspace();
    }

    #[wasm_bindgen(js_name = noteDeleteForward)]
    pub fn note_delete_forward(&self) {
        self.state.borrow_mut().host.note_delete_forward();
    }

    #[wasm_bindgen(js_name = noteMoveCaret)]
    pub fn note_move_caret(&self, direction: &str, extend: bool) {
        self.state.borrow_mut().host.note_move_caret(direction, extend);
    }

    #[wasm_bindgen(js_name = noteCommitEdit)]
    pub fn note_commit_edit(&self) {
        self.state.borrow_mut().host.note_commit_edit();
    }

    #[wasm_bindgen(js_name = setNoteCaretVisible)]
    pub fn set_note_caret_visible(&self, visible: bool) {
        self.state.borrow_mut().host.set_note_caret_visible(visible);
    }

    #[wasm_bindgen(js_name = setImageSrc)]
    pub fn set_image_src(&self, widget_id: &str, src: &str) {
        self.state.borrow_mut().host.set_image_src(widget_id, src);
    }

    #[wasm_bindgen(js_name = schemasJson)]
    pub fn schemas_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.schemas_json().map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = setVariableName)]
    pub fn set_variable_name(&self, widget_id: &str, name: &str) {
        self.state.borrow_mut().host.set_variable_name(widget_id, name);
    }

    #[wasm_bindgen(js_name = setVariableSchema)]
    pub fn set_variable_schema(&self, widget_id: &str, schema: &str) {
        self.state.borrow_mut().host.set_variable_schema(widget_id, schema);
    }

    #[wasm_bindgen(js_name = addWidget)]
    pub fn add_widget(&self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<String, JsValue> {
        self.state.borrow_mut().host.add_widget(descriptor_json, world_x, world_y).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setGhostWidget)]
    pub fn set_ghost_widget(&self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_ghost_widget(descriptor_json, world_x, world_y).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = clearGhostWidget)]
    pub fn clear_ghost_widget(&self) {
        self.state.borrow_mut().host.clear_ghost_widget();
    }

    #[wasm_bindgen(js_name = removeWidget)]
    pub fn remove_widget(&self, widget_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.remove_widget(widget_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = moveWidget)]
    pub fn move_widget(&self, widget_id: &str, x: f64, y: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.move_widget(widget_id, x, y).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = insertBetween)]
    pub fn insert_between(&self, anchor_id: &str, anchor_out_port: &str, mid_id: &str, mid_in_port: &str, mid_out_port: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.insert_between(anchor_id, anchor_out_port, mid_id, mid_in_port, mid_out_port).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = makeSpace)]
    pub fn make_space(&self, anchor_id: &str, dx: f64, dy: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.make_space(anchor_id, dx, dy).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = setNeuronParams)]
    pub fn set_neuron_params(&self, widget_id: &str, params_json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_neuron_params(widget_id, params_json).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = connect)]
    pub fn connect(&self, from_id: &str, to_id: &str) -> Result<String, JsValue> {
        self.state.borrow_mut().host.connect(from_id, to_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = disconnect)]
    pub fn disconnect(&self, synapse_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.disconnect(synapse_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = undo)]
    pub fn undo(&self) -> bool {
        self.state.borrow_mut().host.undo()
    }

    #[wasm_bindgen(js_name = redo)]
    pub fn redo(&self) -> bool {
        self.state.borrow_mut().host.redo()
    }

    #[wasm_bindgen(js_name = canUndo)]
    pub fn can_undo(&self) -> bool {
        self.state.borrow().host.can_undo()
    }

    #[wasm_bindgen(js_name = canRedo)]
    pub fn can_redo(&self) -> bool {
        self.state.borrow().host.can_redo()
    }

    #[wasm_bindgen(js_name = worldFromScreen)]
    pub fn world_from_screen(&self, sx: f64, sy: f64) -> String {
        let (x, y) = self.state.borrow().host.world_from_screen(sx, sy);
        serde_json::json!({ "x": x, "y": y }).to_string()
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json(&self) -> String {
        serde_json::to_string(&self.state.borrow().host.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into())
    }

    #[wasm_bindgen(js_name = wheelScreen)]
    pub fn wheel_screen(&self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) {
        self.state.borrow_mut().host.wheel_screen(sx, sy, delta_x, delta_y, zoom_gesture);
    }

    #[wasm_bindgen(js_name = setWheelZoomActive)]
    pub fn set_wheel_zoom_active(&self, active: bool) {
        self.state.borrow_mut().host.dag.set_wheel_zoom_active(active);
    }

    #[wasm_bindgen(js_name = lodScaleJson)]
    pub fn lod_scale_json(&self) -> String {
        dag::dag_lod_scale_json()
    }

    #[wasm_bindgen(js_name = setAutomaticLod)]
    pub fn set_automatic_lod(&self, enabled: bool) {
        self.state.borrow_mut().host.set_automatic_lod(enabled);
    }

    #[wasm_bindgen(js_name = setProximityDistance)]
    pub fn set_proximity_distance(&self, world: f64) {
        self.state.borrow_mut().host.set_proximity_distance(world);
    }

    #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
    pub fn set_forced_draw_lod_label(&self, label: &str) {
        self.state.borrow_mut().host.set_forced_draw_lod_label(label);
    }

    #[wasm_bindgen(js_name = drawLodLabel)]
    pub fn draw_lod_label(&self) -> String {
        self.state.borrow().host.draw_lod_label().to_string()
    }

    #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
    pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
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
            let (render_ctx, renderer, surface) = canvas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
            let mut g = inner.borrow_mut();
            if g.gpu.gpu_ready() {
                return Err(JsValue::from_str("canvas surface already attached"));
            }
            g.set_logical_size_and_maybe_resize_surface(lw, lh, dpr, pw, ph);
            g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
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

    #[wasm_bindgen(js_name = setCanvasThemeJson)]
    pub fn set_canvas_theme_json(&mut self, json: &str) {
        let _ = self.state.borrow_mut().host.set_canvas_theme_from_json(json);
    }

    #[wasm_bindgen(js_name = reorganize)]
    pub fn reorganize(&self, options_json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.reorganize(options_json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    #[allow(clippy::too_many_arguments, reason = "thin wasm_bindgen forwarder for FlowHost::pointer_down_screen's own justified-allow shape")]
    pub fn pointer_down_screen(&self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, pan);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen(&self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    #[wasm_bindgen(js_name = pickTargetsAtScreenJson)]
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)
    }

    #[wasm_bindgen(js_name = entityScreenJson)]
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        self.state.borrow().host.entity_screen_json(domain, id)
    }

    #[wasm_bindgen(js_name = widgetDragActive)]
    pub fn widget_drag_active(&self) -> bool {
        self.state.borrow().host.widget_drag_active()
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen(&self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    #[wasm_bindgen(js_name = setSelectionOptions)]
    pub fn set_selection_options(&self, method: &str, mode: &str) {
        self.state.borrow_mut().host.set_selection_options(method, mode);
    }

    #[wasm_bindgen(js_name = selectionPreviewPointsJson)]
    pub fn selection_preview_points_json(&self) -> String {
        self.state.borrow().host.selection_preview_points_json()
    }

    #[wasm_bindgen(js_name = selectionPreviewCrossing)]
    pub fn selection_preview_crossing(&self) -> bool {
        self.state.borrow().host.selection_preview_crossing()
    }

    #[wasm_bindgen(js_name = selectionPreviewMethod)]
    pub fn selection_preview_method(&self) -> String {
        self.state.borrow().host.selection_preview_method().to_string()
    }

    #[wasm_bindgen(js_name = selectionUnionBoundsScreenJson)]
    pub fn selection_union_bounds_screen_json(&self) -> String {
        self.state.borrow().host.selection_union_bounds_screen_json()
    }

    #[wasm_bindgen(js_name = alignSelection)]
    pub fn align_selection(&self, mode: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.align_selection(mode).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = preselectWidgetIdsJson)]
    pub fn preselect_widget_ids_json(&self) -> String {
        self.state.borrow().host.preselect_widget_ids_json()
    }

    #[wasm_bindgen(js_name = cancelAreaSelect)]
    pub fn cancel_area_select(&self) -> bool {
        self.state.borrow_mut().host.cancel_area_select()
    }

    #[wasm_bindgen(js_name = deleteSelection)]
    pub fn delete_selection(&self) -> Result<(), JsValue> {
        self.state.borrow_mut().host.delete_selection().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = hasSelection)]
    pub fn has_selection(&self) -> bool {
        self.state.borrow().host.has_selection()
    }

    #[wasm_bindgen(js_name = selectAll)]
    pub fn select_all(&self) {
        self.state.borrow_mut().host.select_all();
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn tessellate(handle: &str, tolerance: f64) -> String {
    match crate::tessellate_geometry(handle, tolerance) {
        Ok(mesh) => serde_json::to_string(&mesh).unwrap_or_else(|_| serde_json::json!({ "error": "mesh encode failed" }).to_string()),
        Err(error) => serde_json::json!({ "error": error }).to_string(),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn render_drawing_scene(handle: &str) -> String {
    render_scene_json(handle)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn export_drawing_svg(handle: &str) -> String {
    export_svg_json(handle)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn export_drawing_pdf(handle: &str) -> String {
    export_pdf_json(handle)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn export_drawing_dwg(handle: &str) -> String {
    export_dwg_json(handle)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn import_drawing_dwg(data_base64: &str) -> String {
    import_dwg_json(data_base64)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn trace_drawing_bitmap(width: u32, height: u32, mask: &[u8], threshold: f64, simplify_epsilon: f64) -> String {
    trace_bitmap_json(width, height, mask, threshold, simplify_epsilon)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn boolean_drawing_segments(a_json: &str, b_json: &str, operation: &str) -> String {
    boolean_segments_json(a_json, b_json, operation)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn dispose(handle: &str) {
    crate::dispose_geometry(handle);
}

/// 📐️ Encodes a `MeshData` JSON payload as base64 DWG bytes, for JS consumers holding a mesh but no drawing/geometry handle.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn dwg_encode_mesh_json(mesh_json: &str) -> String {
    let Ok(mesh) = serde_json::from_str::<semio_framework::MeshData>(mesh_json) else {
        return serde_json::json!({ "error": "invalid mesh json" }).to_string();
    };
    let drawing = semio_framework::mesh_to_dwg_drawing(&mesh);
    match semio_framework::dwg_to_bytes(&drawing) {
        Ok(bytes) => {
            use base64::Engine;
            serde_json::json!({ "dwg": base64::engine::general_purpose::STANDARD.encode(bytes) }).to_string()
        }
        Err(error) => serde_json::json!({ "error": error }).to_string(),
    }
}

/// 📐️ Decodes base64 DWG bytes into a `MeshData` JSON payload.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn dwg_decode_mesh_json(data_base64: &str) -> String {
    use base64::Engine;
    let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(data_base64) else {
        return serde_json::json!({ "error": "invalid base64 dwg payload" }).to_string();
    };
    match semio_framework::dwg_from_bytes(&bytes) {
        Ok(drawing) => {
            let mesh = semio_framework::dwg_drawing_to_mesh(&drawing);
            serde_json::to_string(&mesh).unwrap_or_else(|_| serde_json::json!({ "error": "failed to serialize mesh" }).to_string())
        }
        Err(error) => serde_json::json!({ "error": error }).to_string(),
    }
}
// #endregion 🔖️WasmSession
