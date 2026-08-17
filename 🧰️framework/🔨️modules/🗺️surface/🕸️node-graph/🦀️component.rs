//! 🕸️ Generic node-graph engine for framework renderers.
//!
//! `GraphHost` diffs `NodeGraphScenePayload` into a retained `DagHost` for hit-testing and layout;
//! the OS infinite-board projection remains authoritative — this host is a render-session cache.
//!
//! 🧭️ Doctrine classification (ticket `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`,
//! W3b): [`GraphHost`] owns **no tier-(a) authoritative state**, traced rather than assumed (full
//! per-field table in `📓️wave3b-reports/surface-report.md`). Node positions/connections — genuine
//! document content per the ticket's own framing — are NOT owned here: `self.dag` is rebuilt
//! wholesale from `NodeGraphScenePayload` every time [`GraphHost::sync_from_payload`]'s content-hash
//! signature changes, and the payload itself is produced from `💻️os/🔨️modules/🌊️flow`'s `FlowFixture`
//! (`Widget`/`SynapseSpec` graph, consumed via `EngineCanvas` — see
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs`).
//! That owner is real but **not yet properly event-sourced itself** — `🌊️flow/🌿️vcs/🦀️component.rs`
//! still dispatches through the banned `CollectionMutation<K,V,P>`/`Patch` shape at the time of this
//! wave, per its own hot-file entry ("W3c flow agent" owns it; frozen, read-only, for this wave).
//! `move`/`connect`/`disconnect` are the right verbs once that lane lands (per this ticket's W3c
//! design docs, already delivered to SMO) — not invented here, since the target enum they'd bind to
//! does not exist yet in conforming form. No new `🧬️mutations` vocabulary is authored in THIS file.

pub use infinite_canvas::board::ports::directed_dag as dag;
pub use infinite_canvas as canvas;

use dag::{dag_screen_to_world, fit_node_size, DagCamera, DagFixture, DagFixtureEdge, DagHost, DagNodeKind, DagNodeSpec, IoPortSpec};
use semio_framework_os_kernel::{DomainHover, DomainSelection, SelectionMethod};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️ScenePayload
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphPortRecord {
    id: String,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    abbreviation: Option<String>,
    #[serde(rename = "fullName", default)]
    full_name: Option<String>,
    #[serde(rename = "resourceKind", default)]
    artifact_kind: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNodeRecord {
    id: String,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    instance_id: Option<String>,
    #[serde(default)]
    plugin_id: Option<String>,
    #[serde(default)]
    app_id: Option<String>,
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    x: Option<f64>,
    #[serde(default)]
    y: Option<f64>,
    #[serde(default)]
    width: Option<f64>,
    #[serde(default)]
    height: Option<f64>,
    #[serde(default)]
    inputs: Option<Vec<GraphPortRecord>>,
    #[serde(default)]
    outputs: Option<Vec<GraphPortRecord>>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphViewport {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "default_zoom")]
    zoom: f64,
}

fn default_zoom() -> f64 {
    1.0
}

//#region ⚠️ Errors
/// ⚠️ Node-graph host errors — JSON decode failures plus passthrough of the underlying DAG engine's own error.
#[derive(Debug, thiserror::Error)]
pub enum NodeGraphError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Pack(#[from] store::PackError),
    #[error(transparent)]
    Dag(#[from] dag::DagError),
}
//#endregion ⚠️ Errors

fn port_label(port: &GraphPortRecord) -> String {
    port.label.clone().unwrap_or_else(|| {
        let segments: Vec<_> = port.id.split('@').collect();
        segments.last().map(|s| (*s).to_string()).unwrap_or_else(|| port.id.clone())
    })
}

fn port_to_io(port: &GraphPortRecord) -> IoPortSpec {
    let label = port_label(port);
    let mut spec = IoPortSpec::simple(port.id.clone(), label);
    if let Some(code) = &port.code {
        spec.code = code.clone();
    }
    if let Some(abbrev) = &port.abbreviation {
        spec.abbreviation = abbrev.clone();
    }
    if let Some(full) = &port.full_name {
        spec.full_name = full.clone();
    }
    if let Some(kind) = &port.artifact_kind {
        spec.artifact_kind = Some(kind.clone());
    }
    spec
}

fn node_record_to_spec(record: &GraphNodeRecord) -> DagNodeSpec {
    let name = record.label.clone().unwrap_or_else(|| record.id.clone());
    let abbreviation = name.chars().take(3).collect::<String>();
    let icon = record.icon.clone().unwrap_or_else(|| "emoji:🔷️".into());
    let x = record.x.unwrap_or(0.0);
    let y = record.y.unwrap_or(0.0);
    let width = record.width.unwrap_or(180.0);
    let height = record.height.unwrap_or(72.0);
    let inputs: Vec<IoPortSpec> = record.inputs.as_deref().unwrap_or(&[]).iter().map(port_to_io).collect();
    let outputs: Vec<IoPortSpec> = record.outputs.as_deref().unwrap_or(&[]).iter().map(port_to_io).collect();
    if let Some(instance_id) = &record.instance_id {
        let mut node = DagNodeSpec {
            id: record.id.clone(),
            name: name.clone(),
            abbreviation,
            icon: icon.clone(),
            x,
            y,
            width,
            height,
            kind: DagNodeKind::AppInstance { instance_id: instance_id.clone(), plugin_id: record.plugin_id.clone().unwrap_or_else(|| "app".into()), app_id: record.app_id.clone().unwrap_or_else(|| record.id.clone()), icon, inputs, outputs },
            ..Default::default()
        };
        fit_node_size(&mut node);
        return node;
    }
    let mut node = DagNodeSpec::computation(record.id.clone(), name, abbreviation, icon, inputs, outputs, false, false, x, y, width, height);
    fit_node_size(&mut node);
    node
}

pub fn fixture_from_node_graph_json(nodes_json: &str, edges_json: &str, viewport_json: &str) -> Result<DagFixture, NodeGraphError> {
    let nodes: Vec<GraphNodeRecord> = if nodes_json.trim().is_empty() { vec![] } else { serde_json::from_str(nodes_json)? };
    let edges: Vec<GraphEdgeRecord> = if edges_json.trim().is_empty() { vec![] } else { serde_json::from_str(edges_json)? };
    let viewport: GraphViewport = if viewport_json.trim().is_empty() { GraphViewport::default() } else { serde_json::from_str(viewport_json)? };
    Ok(fixture_from_node_graph_records(&nodes, &edges, Some(&viewport)))
}

/// 🕸️ Same as [`fixture_from_node_graph_json`] but over already-typed records (the `NodeGraphScene`
/// wire shape decodes straight into these, no per-field JSON-string hop).
pub fn fixture_from_node_graph_records(nodes: &[GraphNodeRecord], edges: &[GraphEdgeRecord], viewport: Option<&GraphViewport>) -> DagFixture {
    let viewport = viewport.cloned().unwrap_or_default();
    DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: viewport.x, y: viewport.y, zoom: viewport.zoom },
        nodes: nodes.iter().map(node_record_to_spec).collect(),
        edges: edges.iter().map(|edge| DagFixtureEdge { id: edge.id.clone(), source: format!("{}@{}", edge.source_node_id, edge.source_port_id), target: format!("{}@{}", edge.target_node_id, edge.target_port_id), ..Default::default() }).collect(),
    }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3c: `selection`/`hover` deleted —
/// they used to mirror whatever an app pushed in, duplicating the framework's own `InteractionState`.
/// [`GraphHost::sync_interaction`] reads the framework's `DomainSelection`/`DomainHover` for this
/// domain instead, kept as a call separate from geometry sync since interaction state changes far
/// more often than the node/edge content this payload still carries.
#[derive(Clone, Debug, Default)]
pub struct NodeGraphScenePayload {
    pub nodes: Vec<GraphNodeRecord>,
    pub edges: Vec<GraphEdgeRecord>,
    pub viewport: Option<GraphViewport>,
    pub preview_off_json: Option<String>,
    pub lod_json: Option<String>,
    pub catalogue_json: Option<String>,
    pub controls_json: Option<String>,
    pub clusters_json: Option<String>,
    pub computing_json: Option<String>,
    pub status_json: Option<String>,
    pub capabilities_json: Option<String>,
    pub fixture_json: Option<String>,
}

fn expand_payload_pack_fields(payload: &mut NodeGraphScenePayload) -> Result<(), NodeGraphError> {
    if let Some(json) = payload.preview_off_json.as_mut() {
        *json = store::pack_rt::scene_field_json_text(json)?;
    }
    if let Some(json) = payload.lod_json.as_mut() {
        *json = store::pack_rt::scene_field_json_text(json)?;
    }
    if let Some(json) = payload.catalogue_json.as_mut() {
        *json = store::pack_rt::scene_field_json_text(json)?;
    }
    if let Some(json) = payload.controls_json.as_mut() {
        *json = store::pack_rt::scene_field_json_text(json)?;
    }
    if let Some(json) = payload.clusters_json.as_mut() {
        *json = store::pack_rt::scene_field_json_text(json)?;
    }
    if let Some(json) = payload.status_json.as_mut() {
        *json = store::pack_rt::scene_field_json_text(json)?;
    }
    if let Some(json) = payload.computing_json.as_mut() {
        *json = store::pack_rt::scene_field_json_text(json)?;
    }
    if let Some(json) = payload.capabilities_json.as_mut() {
        *json = store::pack_rt::scene_field_json_text(json)?;
    }
    if let Some(json) = payload.fixture_json.as_mut() {
        *json = store::pack_rt::scene_field_json_text(json)?;
    }
    Ok(())
}

impl NodeGraphScenePayload {
    pub fn from_json(value: &Value) -> Self {
        Self {
            nodes: value.get("nodes").cloned().and_then(|v| serde_json::from_value(v).ok()).unwrap_or_default(),
            edges: value.get("edges").cloned().and_then(|v| serde_json::from_value(v).ok()).unwrap_or_default(),
            viewport: value.get("viewport").cloned().and_then(|v| serde_json::from_value(v).ok()),
            preview_off_json: value.get("previewOffJson").and_then(|v| v.as_str()).map(str::to_string),
            lod_json: value.get("lodJson").and_then(|v| v.as_str()).map(str::to_string),
            catalogue_json: value.get("catalogueJson").and_then(|v| v.as_str()).map(str::to_string),
            controls_json: value.get("controlsJson").and_then(|v| v.as_str()).map(str::to_string),
            clusters_json: value.get("clustersJson").and_then(|v| v.as_str()).map(str::to_string),
            computing_json: value.get("computingJson").and_then(|v| v.as_str()).map(str::to_string),
            status_json: value.get("statusJson").and_then(|v| v.as_str()).map(str::to_string),
            capabilities_json: value.get("capabilitiesJson").and_then(|v| v.as_str()).map(str::to_string),
            fixture_json: value.get("fixtureJson").and_then(|v| v.as_str()).map(str::to_string),
        }
    }
}
//#endregion 🔖️ScenePayload

/// 🎯️ Raw geometric hit-test result of one completed pick/marquee gesture — see
/// [`GraphHost::take_selection_gather`]. No merge/mode algebra lives on this type; the caller pairs it
/// with the active modifier→merge policy and dispatches ONE `interactionSelect`.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionGather {
    pub target_ids: Vec<String>,
    pub method: SelectionMethod,
}

/// 🔤️ `DagHost::selection_preview_method` returns its own lowercase label vocabulary (not the
/// framework `SelectionMethod` wire enum) — this is the one narrow translation point.
fn selection_method_from_dag_label(label: &str) -> SelectionMethod {
    match label {
        "lasso" => SelectionMethod::Lasso,
        _ => SelectionMethod::Rectangle,
    }
}

//#region 🔖️GraphHost
/// 🕸️ Retained generic node-graph host wrapping the DAG canvas engine.
pub struct GraphHost {
    /// 🧱️ (d) ephemeral working representation — hit-testing/layout structure rebuilt wholesale from
    /// `NodeGraphScenePayload` on every content-hash change (see [`GraphHost::sync_from_payload`]).
    /// Node positions/connections are real document content whose authoritative owner is OS `flow`'s
    /// `FlowFixture` (see module docstring); this field is a render-session mirror of it, not a second
    /// authoritative copy.
    pub dag: DagHost,
    /// 📇 (c) Preview/Effect — transient catalogue-panel UI state, never persisted.
    pub catalogue_json: String,
    /// 🎛️ (c) Preview/Effect — transient control-overlay UI state, never persisted.
    pub controls_json: String,
    /// 📡️ (c) Preview/Effect — transient capability-advertisement UI state, never persisted.
    pub capabilities_json: String,
    /// 🔗️ (d) runtime wiring — content-hash of the last-applied payload, so [`GraphHost::sync_from_payload`]
    /// only rebuilds `dag` when the upstream content actually changed. A change-detection cache, not state.
    last_payload_signature: u64,
    /// 🎯️ (c) Preview/Effect — the raw geometric hit-test result of the last completed pick/marquee
    /// gesture, read once by [`GraphHost::take_selection_gather`] so the caller can dispatch it as ONE
    /// batched `interactionSelect` — no merge algebra lives here, `next_selection` owns that.
    pending_gather: Option<SelectionGather>,
}

impl Default for GraphHost {
    fn default() -> Self {
        Self::from_fixture(DagFixture::default())
    }
}

impl GraphHost {
    pub fn from_fixture(fixture: DagFixture) -> Self {
        Self { dag: DagHost::from_fixture_without_layout(fixture), catalogue_json: String::new(), controls_json: String::new(), capabilities_json: String::new(), last_payload_signature: 0, pending_gather: None }
    }

    fn payload_signature(payload: &NodeGraphScenePayload) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        format!("{:?}", payload.nodes).hash(&mut hasher);
        format!("{:?}", payload.edges).hash(&mut hasher);
        format!("{:?}", payload.viewport).hash(&mut hasher);
        payload.preview_off_json.hash(&mut hasher);
        payload.lod_json.hash(&mut hasher);
        payload.computing_json.hash(&mut hasher);
        payload.status_json.hash(&mut hasher);
        hasher.finish()
    }

    pub fn sync_from_payload(&mut self, payload: &NodeGraphScenePayload) -> Result<(), NodeGraphError> {
        let signature = Self::payload_signature(payload);
        if signature != self.last_payload_signature {
            let fixture = fixture_from_node_graph_records(&payload.nodes, &payload.edges, payload.viewport.as_ref());
            self.dag = DagHost::from_fixture_without_layout(fixture);
            self.last_payload_signature = signature;
        }
        if let Some(preview_off_json) = &payload.preview_off_json {
            if let Ok(ids) = serde_json::from_str::<Vec<String>>(preview_off_json) {
                self.dag.set_dimmed(&ids);
            }
        }
        if let Some(lod_json) = &payload.lod_json {
            if let Ok(value) = serde_json::from_str::<Value>(lod_json) {
                if let Some(automatic) = value.get("automatic").and_then(|v| v.as_bool()) {
                    self.dag.set_automatic_lod(automatic);
                }
                if let Some(label) = value.get("lod").and_then(|v| v.as_str()) {
                    self.dag.set_forced_draw_lod_label(label);
                }
                if let Some(distance) = value.get("proximityDistance").and_then(|v| v.as_f64()) {
                    self.dag.set_proximity_distance(distance);
                }
                if let Some(visible) = value.get("gridVisible").and_then(|v| v.as_bool()) {
                    self.dag.set_grid_visible(visible);
                }
                if let Some(enabled) = value.get("gridSnapEnabled").and_then(|v| v.as_bool()) {
                    self.dag.set_grid_snap_enabled(enabled);
                }
                if let Some(factor) = value.get("gridFactor").and_then(|v| v.as_f64()) {
                    let _ = self.dag.set_grid_factor(factor);
                }
                if let Some(label) = value.get("forcedLabel").and_then(|v| v.as_str()) {
                    self.dag.set_forced_draw_lod_label(label);
                }
            }
        }
        if let Some(status_json) = &payload.status_json {
            self.dag.set_node_statuses_from_json(status_json);
        } else if let Some(computing_json) = &payload.computing_json {
            if let Ok(value) = serde_json::from_str::<Value>(computing_json) {
                let active = value.get("active").and_then(|v| v.as_str()).map(str::to_string);
                let stale: Vec<String> = value.get("stale").and_then(|v| v.as_array()).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect()).unwrap_or_default();
                self.dag.set_computing_progress(active.as_deref(), &stale);
            }
        }
        self.catalogue_json = payload.catalogue_json.clone().unwrap_or_default();
        self.controls_json = payload.controls_json.clone().unwrap_or_default();
        self.capabilities_json = payload.capabilities_json.clone().unwrap_or_default();
        Ok(())
    }

    pub fn sync_from_scene_json(&mut self, scene_json: &str) -> Result<(), NodeGraphError> {
        let value: Value = serde_json::from_str(scene_json)?;
        self.sync_from_scene_value(&value)
    }

    pub fn sync_from_scene_pack(&mut self, bytes: &[u8]) -> Result<(), NodeGraphError> {
        // 📦️ Host TS `encodePackValue` is the wire-body twin of `encode_wire_value` (no SPK shell);
        // accept that first, then fall back to `decode_pack_value` for native pack-shell callers/tests.
        let dsl = store::pack_rt::decode_wire_value(bytes).or_else(|_| store::pack_rt::decode_pack_value(bytes))?;
        let value = store::pack_rt::dsl_value_to_json(dsl);
        self.sync_from_scene_value(&value)
    }

    fn sync_from_scene_value(&mut self, value: &Value) -> Result<(), NodeGraphError> {
        let mut payload = NodeGraphScenePayload::from_json(value);
        expand_payload_pack_fields(&mut payload)?;
        self.sync_from_payload(&payload)
    }

    pub fn paint_scene(&self, scene: &mut canvas::Scene, width: u32, height: u32, dpr: f64) {
        self.dag.paint_scene(scene, width, height, dpr);
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.dag.set_viewport(width, height, dpr);
    }

    pub fn camera_json(&self) -> String {
        serde_json::to_string(&self.dag.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into())
    }

    pub fn selected_node_ids_json(&self) -> String {
        serde_json::to_string(&self.dag.selected_node_ids()).unwrap_or_else(|_| "[]".into())
    }

    pub fn hovered_node_id(&self) -> Option<String> {
        self.dag.hovered_node_id()
    }

    pub fn hovered_channel_json(&self) -> String {
        self.dag.hovered_channel_json()
    }

    pub fn label_overlay_paint_state_json(&self) -> Result<String, NodeGraphError> {
        Ok(self.dag.label_overlay_paint_state_json()?)
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64, zoom_gesture: bool) {
        if !zoom_gesture {
            let cam = &self.dag.fixture.camera;
            let zoom = cam.zoom.max(1e-9);
            self.dag.set_camera(cam.x, cam.y - delta_y / zoom, zoom);
            return;
        }
        let (wx, wy) = dag_screen_to_world(&self.dag, sx, sy);
        let factor = if delta_y < 0.0 { 1.1 } else { 0.9 };
        let cam = &self.dag.fixture.camera;
        let new_zoom = (cam.zoom * factor).clamp(0.05, 32.0);
        let nx = wx - (wx - cam.x) * (new_zoom / cam.zoom);
        let ny = wy - (wy - cam.y) * (new_zoom / cam.zoom);
        self.dag.set_camera(nx, ny, new_zoom);
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        self.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, pan);
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.dag.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3c: the DAG engine still owns
    /// geometric hit-testing (plain pick, rectangle/lasso marquee) — that stays here, it is not
    /// selection algebra. What changed: the resulting hit ids no longer become this host's committed
    /// selection by themselves. They are captured into `pending_gather` for the caller to read via
    /// [`GraphHost::take_selection_gather`] and dispatch as ONE batched `interactionSelect`; the
    /// os-kernel `next_selection` machine (not this file) applies merge/mode algebra, and the result
    /// flows back down through [`GraphHost::sync_interaction`] to become what actually paints.
    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        let was_marquee = !self.dag.preselect_widget_ids().is_empty();
        let method = if was_marquee { selection_method_from_dag_label(self.dag.selection_preview_method()) } else { SelectionMethod::Pick };
        self.dag.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
        let target_ids = self.dag.selected_node_ids();
        self.pending_gather = if target_ids.is_empty() { None } else { Some(SelectionGather { target_ids, method }) };
    }

    /// 🎯️ Reads (and clears) the batch of node ids the last completed pick/marquee gesture hit — the
    /// caller turns this into ONE `interactionSelect{targets,method,merge}` dispatch, applying the
    /// modifier→merge policy itself (this host receives no merge concept, only raw geometry).
    pub fn take_selection_gather(&mut self) -> Option<SelectionGather> {
        self.pending_gather.take()
    }

    /// 🕹️ Reads the framework's current selection/hover for this domain and applies it to the paint
    /// backend — replaces the deleted `set_hover`/`set_hover_channel`/scene-payload push path. Called
    /// at render time, independent of geometry sync (interaction state changes far more often).
    pub fn sync_interaction(&mut self, selection: Option<&DomainSelection>, hover: Option<&DomainHover>) {
        let ids: Vec<String> = selection.map(|selection| selection.ids.clone()).unwrap_or_default();
        self.dag.set_selection(&ids);
        let hover_id = hover.and_then(|hover| hover.ids.first()).map(String::as_str);
        self.dag.set_hover(hover_id);
    }

    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        self.dag.pick_targets_at_screen_json(sx, sy)
    }

    /// @emoji 🎯️ Screen-space geometry for a live entity (`domain`/`id` in the pick-target grammar) —
    /// see `DagHost::entity_screen_json`. Powers introduction-demonstration semantic targeting.
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        self.dag.entity_screen_json(domain, id)
    }

    pub fn align_selection(&mut self, mode: &str) -> Result<(), NodeGraphError> {
        Ok(self.dag.align_selection(mode)?)
    }

    pub fn fixture_json(&self) -> Result<String, NodeGraphError> {
        Ok(self.dag.fixture_json()?)
    }

    pub fn set_canvas_theme_dark(&mut self, dark: bool) {
        self.dag.canvas_theme = dag::CanvasPalette::from_board_palette(if dark { &ui_styling::BOARD_DARK } else { &ui_styling::BOARD_LIGHT });
    }
}
//#endregion 🔖️GraphHost

//#region 🔖️Wasm
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::future_to_promise;
    use web_sys::HtmlCanvasElement;

    struct GraphSessionInner {
        host: GraphHost,
        gpu: canvas::gpu_session::CanvasGpuSession,
        width: u32,
        height: u32,
        dpr: f64,
    }

    #[wasm_bindgen]
    pub struct GraphSession {
        state: Rc<RefCell<GraphSessionInner>>,
    }

    #[wasm_bindgen]
    impl GraphSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self { state: Rc::new(RefCell::new(GraphSessionInner { host: GraphHost::default(), gpu: canvas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
        }

        #[wasm_bindgen(js_name = syncFromSceneJson)]
        pub fn sync_from_scene_json(&self, json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.sync_from_scene_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = syncFromScenePack)]
        pub fn sync_from_scene_pack(&self, bytes: &[u8]) -> Result<(), JsValue> {
            self.state.borrow_mut().host.sync_from_scene_pack(bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = attachCanvas)]
        pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
            let inner = self.state.clone();
            let lw = logical_w.max(1);
            let lh = logical_h.max(1);
            let dpr = dpr.max(1.0);
            let pw = ((lw as f64 * dpr).round() as u32).max(1);
            let ph = ((lh as f64 * dpr).round() as u32).max(1);
            future_to_promise(async move {
                let (render_ctx, renderer, surface) = canvas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
                let mut g = inner.borrow_mut();
                g.width = lw;
                g.height = lh;
                g.dpr = dpr;
                g.host.set_viewport(lw, lh, dpr);
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
            let mut inner = self.state.borrow_mut();
            inner.width = width.max(1);
            inner.height = height.max(1);
            inner.dpr = dpr.max(1.0);
            let (w, h, d) = (inner.width, inner.height, inner.dpr);
            inner.host.set_viewport(w, h, d);
            let pw = ((w as f64 * d).round() as u32).max(1);
            let ph = ((h as f64 * d).round() as u32).max(1);
            inner.gpu.resize_surface(pw, ph);
        }

        #[wasm_bindgen(js_name = setCanvasThemeJson)]
        pub fn set_canvas_theme_json(&mut self, json: &str) {
            let _ = self.state.borrow_mut().host.dag.set_canvas_theme_from_json(json);
        }

        #[wasm_bindgen(js_name = renderFrame)]
        pub fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            let mut scene = canvas::Scene::new();
            let clear = inner.host.dag.canvas_theme.raster_clear;
            inner.host.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
            let scene = canvas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
            inner.gpu.render_frame(&scene, clear)
        }

        #[wasm_bindgen(js_name = pointerDownScreen)]
        pub fn pointer_down_screen(&self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool) {
            self.state.borrow_mut().host.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, false);
        }

        #[wasm_bindgen(js_name = pointerMoveScreen)]
        pub fn pointer_move_screen(&self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
            self.state.borrow_mut().host.dag.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
        }

        #[wasm_bindgen(js_name = pointerUpScreen)]
        pub fn pointer_up_screen(&self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
            // 🕹️ Routed through the `GraphHost` wrapper (not straight to `dag`) so a completed
            // pick/marquee gesture is captured into `pending_gather` — see `take_selection_gather_json`.
            self.state.borrow_mut().host.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
        }

        #[wasm_bindgen(js_name = wheelScreen)]
        pub fn wheel_screen(&self, sx: f64, sy: f64, _delta_x: f64, delta_y: f64, zoom_gesture: bool) {
            self.state.borrow_mut().host.wheel_screen(sx, sy, delta_y, zoom_gesture);
        }

        #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
        pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.dag.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = sliderOverlayStateJson)]
        pub fn slider_overlay_state_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.dag.slider_overlay_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = selectionUnionBoundsScreenJson)]
        pub fn selection_union_bounds_screen_json(&self) -> String {
            self.state.borrow().host.dag.selection_union_bounds_screen_json()
        }

        #[wasm_bindgen(js_name = selectionPreviewPointsJson)]
        pub fn selection_preview_points_json(&self) -> String {
            self.state.borrow().host.dag.selection_preview_points_json()
        }

        #[wasm_bindgen(js_name = selectionPreviewCrossing)]
        pub fn selection_preview_crossing(&self) -> bool {
            self.state.borrow().host.dag.selection_preview_crossing()
        }

        #[wasm_bindgen(js_name = selectionPreviewMethod)]
        pub fn selection_preview_method(&self) -> String {
            self.state.borrow().host.dag.selection_preview_method().to_string()
        }

        #[wasm_bindgen(js_name = selectedNodeIdsJson)]
        pub fn selected_node_ids_json(&self) -> String {
            self.state.borrow().host.selected_node_ids_json()
        }

        #[wasm_bindgen(js_name = hoveredNodeId)]
        pub fn hovered_node_id(&self) -> Option<String> {
            self.state.borrow().host.hovered_node_id()
        }

        #[wasm_bindgen(js_name = hoveredChannelJson)]
        pub fn hovered_channel_json(&self) -> String {
            self.state.borrow().host.hovered_channel_json()
        }

        #[wasm_bindgen(js_name = cameraJson)]
        pub fn camera_json(&self) -> String {
            self.state.borrow().host.camera_json()
        }

        #[wasm_bindgen(js_name = lodScaleJson)]
        pub fn lod_scale_json(&self) -> String {
            dag::dag_lod_scale_json()
        }

        #[wasm_bindgen(js_name = drawLodLabel)]
        pub fn draw_lod_label(&self) -> String {
            self.state.borrow().host.dag.draw_lod_label().to_string()
        }

        #[wasm_bindgen(js_name = setAutomaticLod)]
        pub fn set_automatic_lod(&self, enabled: bool) {
            self.state.borrow_mut().host.dag.set_automatic_lod(enabled);
        }

        #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
        pub fn set_forced_draw_lod_label(&self, label: &str) {
            self.state.borrow_mut().host.dag.set_forced_draw_lod_label(label);
        }

        #[wasm_bindgen(js_name = setGhostNodeJson)]
        pub fn set_ghost_node_json(&self, json: &str) {
            if json.trim().is_empty() {
                self.state.borrow_mut().host.dag.set_ghost_node(None);
                return;
            }
            if let Ok(node) = serde_json::from_str::<DagNodeSpec>(json) {
                self.state.borrow_mut().host.dag.set_ghost_node(Some(node));
            }
        }

        #[wasm_bindgen(js_name = clearGhostNode)]
        pub fn clear_ghost_node(&self) {
            self.state.borrow_mut().host.dag.set_ghost_node(None);
        }

        #[wasm_bindgen(js_name = pickTargetsAtScreenJson)]
        pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
            self.state.borrow().host.dag.pick_targets_at_screen_json(sx, sy)
        }

        #[wasm_bindgen(js_name = entityScreenJson)]
        pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
            self.state.borrow().host.dag.entity_screen_json(domain, id)
        }

        /// 🕹️ Replaces the deleted `setHover`/`setHoverChannel` push-setters — `selectedIdsJson`/
        /// `hoveredId` are the caller's resolved `DomainSelection.ids`/`DomainHover.ids.first()` for
        /// this domain, read from the framework's `InteractionState` at render time, not pushed
        /// arbitrarily from app code.
        #[wasm_bindgen(js_name = syncInteraction)]
        pub fn sync_interaction(&self, selected_ids_json: &str, hovered_id: Option<String>) -> Result<(), JsValue> {
            let ids: Vec<String> = if selected_ids_json.trim().is_empty() { Vec::new() } else { serde_json::from_str(selected_ids_json).map_err(|e| JsValue::from_str(&e.to_string()))? };
            let selection = DomainSelection { granularity: String::new(), ids, anchor_id: None };
            let hover = hovered_id.map(|id| DomainHover { channel: "pointer".into(), ids: vec![id] });
            self.state.borrow_mut().host.sync_interaction(Some(&selection), hover.as_ref());
            Ok(())
        }

        /// 🎯️ Drains the last completed pick/marquee gesture's raw hit targets — the JS host pairs
        /// this with its own modifier→merge policy and dispatches ONE `interactionSelect`.
        #[wasm_bindgen(js_name = takeSelectionGatherJson)]
        pub fn take_selection_gather_json(&self) -> Option<String> {
            self.state.borrow_mut().host.take_selection_gather().map(|gather| serde_json::to_string(&gather).unwrap_or_else(|_| "null".into()))
        }

        #[wasm_bindgen(js_name = alignSelection)]
        pub fn align_selection(&self, mode: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.align_selection(mode).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = fixtureJson)]
        pub fn fixture_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = takePendingOpenInstanceId)]
        pub fn take_pending_open_instance_id(&self) -> Option<String> {
            dag_take_pending_open_instance_id(&mut self.state.borrow_mut().host.dag)
        }

        #[wasm_bindgen(js_name = screenToWorld)]
        pub fn screen_to_world(&self, x: f64, y: f64) -> js_sys::Array {
            let (wx, wy) = dag_screen_to_world(&self.state.borrow().host.dag, x, y);
            let out = js_sys::Array::new();
            out.push(&JsValue::from_f64(wx));
            out.push(&JsValue::from_f64(wy));
            out
        }

        #[wasm_bindgen(js_name = worldFromScreen)]
        pub fn world_from_screen(&self, x: f64, y: f64) -> js_sys::Array {
            self.screen_to_world(x, y)
        }

        #[wasm_bindgen(js_name = selectAll)]
        pub fn select_all(&self) {
            self.state.borrow_mut().host.dag.select_all();
        }

        #[wasm_bindgen(js_name = deleteSelection)]
        pub fn delete_selection(&self) {
            self.state.borrow_mut().host.dag.delete_selected();
        }

        #[wasm_bindgen(js_name = cancelAreaSelect)]
        pub fn cancel_area_select(&self) {
            self.state.borrow_mut().host.dag.cancel_area_select();
        }

        #[wasm_bindgen(js_name = reorganize)]
        pub fn reorganize(&self, options_json: &str) -> Result<(), JsValue> {
            let opts = if options_json.trim().is_empty() { DagLayoutOptions::default() } else { serde_json::from_str(options_json).unwrap_or_default() };
            self.state.borrow_mut().host.dag.reorganize(&opts).map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_session::GraphSession;
//#endregion 🔖️Wasm

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_from_workflow_json() {
        let nodes = r#"[{"id":"a","label":"Alpha","x":10,"y":20,"inputs":[],"outputs":[{"id":"out","label":"Out"}]}]"#;
        let edges = r#"[]"#;
        let fixture = fixture_from_node_graph_json(nodes, edges, r#"{"x":0,"y":0,"zoom":1}"#).expect("fixture");
        assert_eq!(fixture.nodes.len(), 1);
        assert_eq!(fixture.nodes[0].id, "a");
    }

    #[test]
    fn graph_host_syncs_selection_from_framework_interaction_state() {
        let mut host = GraphHost::default();
        let payload = NodeGraphScenePayload {
            nodes: vec![GraphNodeRecord { id: "a".into(), label: Some("A".into()), outputs: Some(vec![GraphPortRecord { id: "out".into(), ..Default::default() }]), ..Default::default() }],
            edges: Vec::new(),
            viewport: Some(GraphViewport { x: 0.0, y: 0.0, zoom: 1.0 }),
            ..Default::default()
        };
        host.sync_from_payload(&payload).expect("sync");
        let selection = DomainSelection { granularity: "node".into(), ids: vec!["a".into()], anchor_id: None };
        host.sync_interaction(Some(&selection), None);
        assert_eq!(host.dag.selected_node_ids(), vec!["a"]);
    }

    #[test]
    fn graph_host_pointer_up_after_plain_click_gathers_one_pick_target() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        host.set_viewport(400, 400, 1.0);
        host.pointer_down_screen(200.0, 200.0, 0, false, false, false, false);
        host.pointer_up_screen(200.0, 200.0, false, false, false);
        let gather = host.take_selection_gather().expect("gather");
        assert_eq!(gather.target_ids, vec!["a".to_string()]);
        assert_eq!(gather.method, SelectionMethod::Pick);
        assert!(host.take_selection_gather().is_none(), "gather is take-once");
    }

    #[test]
    fn set_canvas_theme_dark_applies_board_palette() {
        let mut host = GraphHost::default();
        host.set_canvas_theme_dark(true);
        let dark_stroke = host.dag.canvas_theme.node_stroke.to_rgba8();
        host.set_canvas_theme_dark(false);
        let light_stroke = host.dag.canvas_theme.node_stroke.to_rgba8();
        assert_ne!(dark_stroke.r, light_stroke.r);
    }

    //#region 🔖️PortHelpers
    #[test]
    fn port_label_uses_last_at_segment_when_label_missing() {
        let port = GraphPortRecord { id: "node@channel@foo".into(), label: None, ..Default::default() };
        assert_eq!(port_label(&port), "foo");
    }

    #[test]
    fn port_label_falls_back_to_full_id_without_at() {
        let port = GraphPortRecord { id: "solo".into(), label: None, ..Default::default() };
        assert_eq!(port_label(&port), "solo");
    }

    #[test]
    fn port_label_prefers_explicit_label() {
        let port = GraphPortRecord { id: "node@out".into(), label: Some("Output".into()), ..Default::default() };
        assert_eq!(port_label(&port), "Output");
    }

    #[test]
    fn port_to_io_copies_optional_metadata() {
        let port = GraphPortRecord { id: "p1".into(), label: Some("Speed".into()), code: Some("SPD".into()), abbreviation: Some("Sp".into()), full_name: Some("Speed Value".into()), artifact_kind: Some("number".into()) };
        let spec = port_to_io(&port);
        assert_eq!(spec.id, "p1");
        assert_eq!(spec.label, "Speed");
        assert_eq!(spec.code, "SPD");
        assert_eq!(spec.abbreviation, "Sp");
        assert_eq!(spec.full_name, "Speed Value");
        assert_eq!(spec.artifact_kind.as_deref(), Some("number"));
    }

    #[test]
    fn port_to_io_uses_simple_defaults_when_optional_fields_absent() {
        let port = GraphPortRecord { id: "p2".into(), ..Default::default() };
        let spec = port_to_io(&port);
        assert_eq!(spec.id, "p2");
        assert!(spec.artifact_kind.is_none());
    }
    //#endregion 🔖️PortHelpers

    //#region 🔖️NodeRecordConversion
    #[test]
    fn node_record_to_spec_builds_app_instance_kind() {
        let record = GraphNodeRecord { id: "n1".into(), label: Some("Widget".into()), instance_id: Some("inst-1".into()), plugin_id: None, app_id: None, ..Default::default() };
        let spec = node_record_to_spec(&record);
        match spec.kind {
            DagNodeKind::AppInstance { instance_id, plugin_id, app_id, .. } => {
                assert_eq!(instance_id, "inst-1");
                assert_eq!(plugin_id, "app");
                assert_eq!(app_id, "n1");
            }
            other => panic!("expected AppInstance kind, got {other:?}"),
        }
        assert_eq!(spec.abbreviation, "Wid");
    }

    #[test]
    fn node_record_to_spec_defaults_computation_kind_without_instance_id() {
        let record = GraphNodeRecord { id: "n2".into(), label: Some("Compute".into()), ..Default::default() };
        let spec = node_record_to_spec(&record);
        assert!(matches!(spec.kind, DagNodeKind::Computation { .. }));
    }

    #[test]
    fn node_record_to_spec_defaults_position_when_missing() {
        let record = GraphNodeRecord { id: "n3".into(), label: Some("Anchor".into()), ..Default::default() };
        let spec = node_record_to_spec(&record);
        assert_eq!(spec.x, 0.0);
        assert_eq!(spec.y, 0.0);
        assert_eq!(spec.icon, "emoji:🔷️");
    }

    #[test]
    fn node_record_to_spec_falls_back_to_id_when_label_missing() {
        let record = GraphNodeRecord { id: "n4".into(), ..Default::default() };
        let spec = node_record_to_spec(&record);
        // 🔤️ Computation kind routes through `DagNodeSpec::computation`, which pascal-cases the display name.
        assert_eq!(spec.name, "N4");
    }
    //#endregion 🔖️NodeRecordConversion

    //#region 🔖️FixtureFromJson
    #[test]
    fn fixture_from_node_graph_json_defaults_when_inputs_blank() {
        let fixture = fixture_from_node_graph_json("", "", "").expect("fixture");
        assert_eq!(fixture.schema, "dag.fixture");
        assert!(fixture.nodes.is_empty());
        assert!(fixture.edges.is_empty());
        // 🐛️ blank viewport_json takes the `GraphViewport::default()` (derived) path, which zeroes zoom
        // instead of using `default_zoom()` (1.0) — that helper only fires for missing-key JSON parsing.
        assert_eq!(fixture.camera.zoom, 0.0);
    }

    #[test]
    fn fixture_from_node_graph_json_builds_composite_edge_endpoints() {
        let nodes = r#"[{"id":"a","outputs":[{"id":"out"}]},{"id":"b","inputs":[{"id":"in"}]}]"#;
        let edges = r#"[{"id":"e1","sourceNodeId":"a","sourcePortId":"out","targetNodeId":"b","targetPortId":"in"}]"#;
        let fixture = fixture_from_node_graph_json(nodes, edges, "").expect("fixture");
        assert_eq!(fixture.edges.len(), 1);
        assert_eq!(fixture.edges[0].source, "a@out");
        assert_eq!(fixture.edges[0].target, "b@in");
    }

    #[test]
    fn fixture_from_node_graph_json_propagates_malformed_nodes_json() {
        let err = fixture_from_node_graph_json("not json", "[]", "").unwrap_err();
        assert!(matches!(err, NodeGraphError::Json(_)));
    }

    #[test]
    fn fixture_from_node_graph_json_reads_custom_viewport() {
        let fixture = fixture_from_node_graph_json("[]", "[]", r#"{"x":5,"y":-3,"zoom":2.5}"#).expect("fixture");
        assert_eq!(fixture.camera.x, 5.0);
        assert_eq!(fixture.camera.y, -3.0);
        assert_eq!(fixture.camera.zoom, 2.5);
    }
    //#endregion 🔖️FixtureFromJson

    //#region 🔖️ScenePayloadFromJson
    #[test]
    fn node_graph_scene_payload_from_json_defaults_missing_fields() {
        let value = serde_json::json!({});
        let payload = NodeGraphScenePayload::from_json(&value);
        assert!(payload.nodes.is_empty());
        assert!(payload.edges.is_empty());
        assert!(payload.viewport.is_none());
        assert!(payload.catalogue_json.is_none());
    }

    #[test]
    fn node_graph_scene_payload_from_json_reads_optional_fields() {
        let value = serde_json::json!({
            "nodes": [{"id": "1", "x": 0.0, "y": 0.0, "width": 1.0, "height": 1.0}],
            "edges": [{"id": "2", "sourceNodeId": "a", "sourcePortId": "out", "targetNodeId": "b", "targetPortId": "in"}],
            "viewport": {"x": 0.0, "y": 0.0, "zoom": 1.0},
            "previewOffJson": "[]",
            "lodJson": "{}",
            "catalogueJson": "cat",
            "controlsJson": "ctl",
            "clustersJson": "clu",
            "computingJson": "{}",
            "capabilitiesJson": "cap",
            "fixtureJson": "fix",
        });
        let payload = NodeGraphScenePayload::from_json(&value);
        assert_eq!(payload.nodes.len(), 1);
        assert_eq!(payload.edges.len(), 1);
        assert_eq!(payload.catalogue_json.as_deref(), Some("cat"));
        assert_eq!(payload.controls_json.as_deref(), Some("ctl"));
        assert_eq!(payload.clusters_json.as_deref(), Some("clu"));
        assert_eq!(payload.capabilities_json.as_deref(), Some("cap"));
        assert_eq!(payload.fixture_json.as_deref(), Some("fix"));
    }
    //#endregion 🔖️ScenePayloadFromJson

    //#region 🔖️GraphHostSync
    fn payload_with_node(id: &str) -> NodeGraphScenePayload {
        NodeGraphScenePayload {
            nodes: vec![GraphNodeRecord {
                id: id.into(),
                label: Some("A".into()),
                x: Some(0.0),
                y: Some(0.0),
                outputs: Some(vec![GraphPortRecord { id: "out".into(), ..Default::default() }]),
                inputs: Some(vec![GraphPortRecord { id: "in".into(), ..Default::default() }]),
                ..Default::default()
            }],
            edges: Vec::new(),
            viewport: Some(GraphViewport { x: 0.0, y: 0.0, zoom: 1.0 }),
            ..Default::default()
        }
    }

    #[test]
    fn graph_host_sync_from_payload_updates_catalogue_without_signature_change() {
        let mut host = GraphHost::default();
        let mut payload = payload_with_node("a");
        payload.catalogue_json = Some("first".into());
        host.sync_from_payload(&payload).expect("sync");
        assert_eq!(host.catalogue_json, "first");
        payload.catalogue_json = Some("second".into());
        host.sync_from_payload(&payload).expect("sync");
        assert_eq!(host.catalogue_json, "second");
    }

    #[test]
    fn graph_host_sync_interaction_sets_hover_node_only() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        let hover = DomainHover { channel: "pointer".into(), ids: vec!["a".into()] };
        host.sync_interaction(None, Some(&hover));
        assert_eq!(host.hovered_node_id().as_deref(), Some("a"));
        assert_eq!(host.hovered_channel_json(), "null");
    }

    #[test]
    fn graph_host_sync_interaction_clears_hover_when_absent() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        let hover = DomainHover { channel: "pointer".into(), ids: vec!["a".into()] };
        host.sync_interaction(None, Some(&hover));
        assert_eq!(host.hovered_node_id().as_deref(), Some("a"));
        host.sync_interaction(None, None);
        assert_eq!(host.hovered_node_id(), None);
    }

    #[test]
    fn graph_host_sync_from_payload_dims_preview_off_nodes() {
        let mut host = GraphHost::default();
        let mut payload = payload_with_node("a");
        payload.preview_off_json = Some(r#"["a"]"#.into());
        host.sync_from_payload(&payload).expect("sync");
        assert_eq!(host.dag.dimmed_node_ids(), vec!["a".to_string()]);
    }

    #[test]
    fn graph_host_sync_from_payload_applies_lod_settings() {
        let mut host = GraphHost::default();
        let mut payload = payload_with_node("a");
        payload.lod_json = Some(r#"{"automatic":false,"lod":"micro","proximityDistance":12.5,"gridVisible":false,"gridSnapEnabled":true,"gridFactor":2.0}"#.into());
        host.sync_from_payload(&payload).expect("sync");
        assert_eq!(host.dag.draw_lod_label(), "micro");
    }

    #[test]
    fn graph_host_sync_from_payload_applies_computing_progress() {
        let mut host = GraphHost::default();
        let mut payload = payload_with_node("a");
        payload.computing_json = Some(r#"{"active":"a","stale":[]}"#.into());
        host.sync_from_payload(&payload).expect("sync");
        assert_eq!(host.dag.hovered_node_id(), None);
    }

    #[test]
    fn graph_host_sync_from_scene_json_parses_raw_json() {
        let mut host = GraphHost::default();
        let scene = r#"{"nodes":[{"id":"a","x":0.0,"y":0.0,"width":1.0,"height":1.0,"outputs":[{"id":"out"}]}],"edges":[],"viewport":{"x":0,"y":0,"zoom":1},"selection":["a"]}"#;
        host.sync_from_scene_json(scene).expect("sync");
        assert_eq!(host.dag.selected_node_ids(), vec!["a"]);
    }

    #[test]
    fn graph_host_sync_from_scene_pack_decodes_pack_shell() {
        let mut host = GraphHost::default();
        let scene = serde_json::json!({
            "nodes": [{"id": "a", "x": 0.0, "y": 0.0, "width": 1.0, "height": 1.0, "outputs": [{"id": "out"}]}],
            "edges": [],
            "viewport": {"x": 0.0, "y": 0.0, "zoom": 1.0},
            "selection": ["a"]
        });
        let dsl = dsl::to_dsl_value(&scene).expect("dsl");
        let bytes = store::pack_rt::encode_pack_value(&dsl);
        host.sync_from_scene_pack(&bytes).expect("sync");
        assert_eq!(host.dag.selected_node_ids(), vec!["a"]);
    }

    #[test]
    fn graph_host_sync_from_scene_json_rejects_invalid_json() {
        let mut host = GraphHost::default();
        let err = host.sync_from_scene_json("not json").unwrap_err();
        assert!(matches!(err, NodeGraphError::Json(_)));
    }
    //#endregion 🔖️GraphHostSync

    //#region 🔖️GraphHostQueries
    #[test]
    fn graph_host_camera_json_reflects_viewport() {
        let mut host = GraphHost::default();
        let mut payload = payload_with_node("a");
        payload.viewport = Some(GraphViewport { x: 11.0, y: 22.0, zoom: 3.0 });
        host.sync_from_payload(&payload).expect("sync");
        assert_eq!(host.camera_json(), r#"{"x":11.0,"y":22.0,"zoom":3.0}"#);
    }

    #[test]
    fn graph_host_selected_node_ids_json_matches_selection() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        let selection = DomainSelection { granularity: "node".into(), ids: vec!["a".into()], anchor_id: None };
        host.sync_interaction(Some(&selection), None);
        assert_eq!(host.selected_node_ids_json(), r#"["a"]"#);
    }

    #[test]
    fn graph_host_wheel_screen_pan_without_zoom_gesture() {
        let mut host = GraphHost::default();
        host.set_viewport(400, 400, 1.0);
        let before = host.dag.fixture.camera.y;
        host.wheel_screen(200.0, 200.0, 10.0, false);
        assert!(host.dag.fixture.camera.y < before);
        assert_eq!(host.dag.fixture.camera.zoom, 1.0);
    }

    #[test]
    fn graph_host_wheel_screen_zoom_gesture_changes_zoom() {
        let mut host = GraphHost::default();
        host.set_viewport(400, 400, 1.0);
        host.wheel_screen(200.0, 200.0, -10.0, true);
        assert!(host.dag.fixture.camera.zoom > 1.0);
    }

    #[test]
    fn graph_host_pointer_click_selects_node() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        host.set_viewport(400, 400, 1.0);
        host.pointer_down_screen(200.0, 200.0, 0, false, false, false, false);
        host.pointer_up_screen(200.0, 200.0, false, false, false);
        assert_eq!(host.dag.selected_node_ids(), vec!["a".to_string()]);
    }

    #[test]
    fn graph_host_pick_targets_at_screen_json_finds_node() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        host.set_viewport(400, 400, 1.0);
        let json = host.pick_targets_at_screen_json(200.0, 200.0);
        assert!(json.contains("\"a\""));
    }

    #[test]
    fn graph_host_entity_screen_json_visible_for_known_node() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        host.set_viewport(400, 400, 1.0);
        let json = host.entity_screen_json("node", "a");
        assert!(json.contains("\"visible\":true"));
    }

    #[test]
    fn graph_host_entity_screen_json_invisible_for_unknown_node() {
        let host = GraphHost::default();
        let json = host.entity_screen_json("node", "missing");
        assert_eq!(json, r#"{"visible":false}"#);
    }

    #[test]
    fn graph_host_align_selection_errors_on_unknown_mode() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        let selection = DomainSelection { granularity: "node".into(), ids: vec!["a".into()], anchor_id: None };
        host.sync_interaction(Some(&selection), None);
        let err = host.align_selection("bogusMode").unwrap_err();
        assert!(matches!(err, NodeGraphError::Dag(_)));
    }

    #[test]
    fn graph_host_align_selection_ok_for_single_node() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        let selection = DomainSelection { granularity: "node".into(), ids: vec!["a".into()], anchor_id: None };
        host.sync_interaction(Some(&selection), None);
        host.align_selection("alignLeft").expect("align");
    }

    #[test]
    fn graph_host_fixture_json_round_trips_nodes() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        let json = host.fixture_json().expect("fixture json");
        assert!(json.contains("\"a\""));
    }

    #[test]
    fn graph_host_label_overlay_paint_state_json_includes_camera() {
        let mut host = GraphHost::default();
        let payload = payload_with_node("a");
        host.sync_from_payload(&payload).expect("sync");
        let json = host.label_overlay_paint_state_json().expect("labels");
        assert!(json.contains("\"camera\""));
    }
    //#endregion 🔖️GraphHostQueries
}
//#endregion 🔖️Tests
