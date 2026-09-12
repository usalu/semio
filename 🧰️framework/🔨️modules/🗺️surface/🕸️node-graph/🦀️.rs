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
//! (`Widget`/`SynapseSpec` graph, consumed via `⚙️EngineCanvas` — see
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`).
//! That owner is real but **not yet properly event-sourced itself** — `🌊️flow/🌿️vcs/🦀️.rs`
//! still dispatches through the banned `CollectionMutation<K,V,P>`/`Patch` shape at the time of this
//! wave, per its own hot-file entry ("W3c flow agent" owns it; frozen, read-only, for this wave).
//! `move`/`connect`/`disconnect` are the right verbs once that lane lands (per this ticket's W3c
//! design docs, already delivered to SMO) — not invented here, since the target enum they'd bind to
//! does not exist yet in conforming form. No new `🧬️mutations` vocabulary is authored in THIS file.

pub use infinite_canvas as canvas;
pub use infinite_canvas::board::ports::directed_dag as dag;

use dag::{dag_screen_to_world, fit_node_size, DagHost};
use semio_framework_artifact_infinite_dag::{DagCamera, DagFixture, DagFixtureEdge, DagNodeKind, DagNodeSpec, IoPortSpec};
use semio_framework_os_kernel::{DomainHover, DomainSelection, SelectionMethod, Viewport2d};
// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use dsl::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️ScenePayload
#[derive(Clone, Debug, Default, Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GraphPortRecord {
    id: String,
    #[serde(default)]
    #[value(default)]
    label: Option<String>,
    #[serde(default)]
    #[value(default)]
    code: Option<String>,
    #[serde(default)]
    #[value(default)]
    abbreviation: Option<String>,
    #[serde(rename = "fullName", default)]
    #[value(rename = "fullName", default)]
    full_name: Option<String>,
    #[serde(rename = "resourceKind", default)]
    #[value(rename = "resourceKind", default)]
    artifact_kind: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GraphNodeRecord {
    id: String,
    #[serde(default)]
    #[value(default)]
    label: Option<String>,
    #[serde(default)]
    #[value(default)]
    instance_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    plugin_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    app_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    icon: Option<String>,
    #[serde(default)]
    #[value(default)]
    x: Option<f64>,
    #[serde(default)]
    #[value(default)]
    y: Option<f64>,
    #[serde(default)]
    #[value(default)]
    width: Option<f64>,
    #[serde(default)]
    #[value(default)]
    height: Option<f64>,
    #[serde(default)]
    #[value(default)]
    inputs: Option<Vec<GraphPortRecord>>,
    #[serde(default)]
    #[value(default)]
    outputs: Option<Vec<GraphPortRecord>>,
}

#[derive(Clone, Debug, Default, Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GraphEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}

//#region ⚠️ Errors
/// ⚠️ Node-graph host errors — JSON decode failures plus passthrough of the underlying DAG engine's own error.
#[derive(Debug)]
pub enum NodeGraphError {
    Json(serde_json::Error),
    Pack(store::PackError),
    Dag(dag::DagError),
}

impl std::fmt::Display for NodeGraphError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => error.fmt(formatter),
            Self::Pack(error) => error.fmt(formatter),
            Self::Dag(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for NodeGraphError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Pack(error) => Some(error),
            Self::Dag(error) => Some(error),
        }
    }
}

impl From<serde_json::Error> for NodeGraphError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<store::PackError> for NodeGraphError {
    fn from(error: store::PackError) -> Self {
        Self::Pack(error)
    }
}

impl From<dag::DagError> for NodeGraphError {
    fn from(error: dag::DagError) -> Self {
        Self::Dag(error)
    }
}
//#endregion ⚠️ Errors

fn port_label(port: &GraphPortRecord) -> String {
    port.label.clone().unwrap_or_else(|| {
        let segments: Vec<_> = port.id.split('@').collect();
        segments.last().map_or_else(|| port.id.clone(), |s| (*s).to_string())
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
            name,
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
    let mut node = DagNodeSpec::computation(record.id.clone(), &name, &abbreviation, icon, inputs, outputs, false, false, x, y, width, height);
    fit_node_size(&mut node);
    node
}

/// 🕸️ Projects the typed `NodeGraphScene` records into the retained DAG host.
pub fn fixture_from_node_graph_records(nodes: &[GraphNodeRecord], edges: &[GraphEdgeRecord], viewport: Option<&Viewport2d>) -> DagFixture {
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
    pub viewport: Option<Viewport2d>,
    pub preview_off_json: Option<String>,
    pub lod_json: Option<String>,
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
    pub fn from_json(value: &Value) -> Result<Self, NodeGraphError> {
        Ok(Self {
            nodes: value.get("nodes").cloned().and_then(|v| serde_json::from_value(v).ok()).unwrap_or_default(),
            edges: value.get("edges").cloned().and_then(|v| serde_json::from_value(v).ok()).unwrap_or_default(),
            viewport: value.get("viewport").cloned().map(serde_json::from_value).transpose()?,
            preview_off_json: value.get("previewOffJson").and_then(|v| v.as_str()).map(str::to_string),
            lod_json: value.get("lodJson").and_then(|v| v.as_str()).map(str::to_string),
            controls_json: value.get("controlsJson").and_then(|v| v.as_str()).map(str::to_string),
            clusters_json: value.get("clustersJson").and_then(|v| v.as_str()).map(str::to_string),
            computing_json: value.get("computingJson").and_then(|v| v.as_str()).map(str::to_string),
            status_json: value.get("statusJson").and_then(|v| v.as_str()).map(str::to_string),
            capabilities_json: value.get("capabilitiesJson").and_then(|v| v.as_str()).map(str::to_string),
            fixture_json: value.get("fixtureJson").and_then(|v| v.as_str()).map(str::to_string),
        })
    }
}
//#endregion 🔖️ScenePayload

/// 🎯️ Raw geometric hit-test result of one completed pick/marquee gesture — see
/// [`GraphHost::take_selection_gather`]. No merge/mode algebra lives on this type; the caller pairs it
/// with the active modifier→merge policy and dispatches ONE `interactionSelect`.
#[derive(Clone, Debug, PartialEq, Serialize, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug)]
pub struct GraphWheelPlan {
    revision: u64,
    expected: [f64; 3],
    next: [f64; 3],
}

impl GraphWheelPlan {
    pub fn camera(&self) -> [f64; 3] {
        self.next
    }
}

/// 🕸️ Retained generic node-graph host wrapping the DAG canvas engine.
pub struct GraphHost {
    /// 🧱️ (d) ephemeral working representation — hit-testing/layout structure rebuilt wholesale from
    /// `NodeGraphScenePayload` on every content-hash change (see [`GraphHost::sync_from_payload`]).
    /// Node positions/connections are real document content whose authoritative owner is OS `flow`'s
    /// `FlowFixture` (see module docstring); this field is a render-session mirror of it, not a second
    /// authoritative copy.
    pub dag: DagHost,
    /// 📇 (c) Preview/Effect — the app-static palette catalogue, pushed once per app instance through
    /// [`GraphHost::set_catalogue_json`] from the reserved `framework.section.catalogue` surface. It is
    /// NOT a scene field: with real operator sets installed it is ~100 KB against the 32 KiB fixed
    /// per-surface admission (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). Never persisted.
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
    interaction_revision: u64,
    interaction_projection: Option<dag::DagInteractionProjection>,
}

impl Default for GraphHost {
    fn default() -> Self {
        Self::from_fixture(DagFixture::default())
    }
}

impl GraphHost {
    pub fn from_fixture(fixture: DagFixture) -> Self {
        let dag = DagHost::from_fixture_without_layout(fixture);
        let interaction_projection = dag.bounded_interaction_projection(0).ok();
        Self { dag, catalogue_json: String::new(), controls_json: String::new(), capabilities_json: String::new(), last_payload_signature: 0, pending_gather: None, interaction_revision: 0, interaction_projection }
    }

    fn refresh_interaction_projection(&mut self) {
        self.interaction_projection = self.dag.bounded_interaction_projection(self.interaction_revision).ok();
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

    /// 🛍️ Installs the app-static palette catalogue. Its own channel, not a scene field: the catalogue
    /// is published once per app instance on the reserved `framework.section.catalogue` retained
    /// surface, while [`Self::sync_from_payload`] runs on every scene change.
    pub fn set_catalogue_json(&mut self, json: &str) {
        self.catalogue_json.clear();
        self.catalogue_json.push_str(json);
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
        self.controls_json = payload.controls_json.clone().unwrap_or_default();
        self.capabilities_json = payload.capabilities_json.clone().unwrap_or_default();
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
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
        let mut payload = NodeGraphScenePayload::from_json(value)?;
        expand_payload_pack_fields(&mut payload)?;
        self.sync_from_payload(&payload)
    }

    pub fn paint_scene(&self, scene: &mut canvas::Scene, width: u32, height: u32, dpr: f64) {
        self.dag.paint_scene(scene, width, height, dpr);
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.dag.set_viewport(width, height, dpr);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    pub fn viewport(&self) -> Viewport2d {
        let camera = &self.dag.fixture.camera;
        Viewport2d { x: camera.x, y: camera.y, zoom: camera.zoom }
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
        let plan = self.plan_wheel(sx, sy, delta_y, zoom_gesture);
        let _ = self.commit_wheel(plan);
    }

    pub fn plan_wheel(&self, sx: f64, sy: f64, delta_y: f64, zoom_gesture: bool) -> GraphWheelPlan {
        let cam = &self.dag.fixture.camera;
        let expected = [cam.x, cam.y, cam.zoom];
        let next = if !zoom_gesture {
            [cam.x, cam.y - delta_y / cam.zoom.max(1e-9), cam.zoom.max(1e-9)]
        } else {
            let (wx, wy) = dag_screen_to_world(&self.dag, sx, sy);
            let new_zoom = (cam.zoom * if delta_y < 0.0 { 1.1 } else { 0.9 }).clamp(0.05, 32.0);
            [wx - (wx - cam.x) * (new_zoom / cam.zoom), wy - (wy - cam.y) * (new_zoom / cam.zoom), new_zoom]
        };
        GraphWheelPlan { revision: self.interaction_revision, expected, next }
    }

    pub fn commit_wheel(&mut self, plan: GraphWheelPlan) -> bool {
        let cam = &self.dag.fixture.camera;
        if self.interaction_revision != plan.revision || [cam.x.to_bits(), cam.y.to_bits(), cam.zoom.to_bits()] != [plan.expected[0].to_bits(), plan.expected[1].to_bits(), plan.expected[2].to_bits()] {
            return false;
        }
        self.dag.set_camera(plan.next[0], plan.next[1], plan.next[2]);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
        true
    }

    pub fn plan_pointer(&self, intent: dag::DagPointerIntent) -> Result<dag::DagPointerPlan, dag::DagInteractionPlanFault> {
        let projection = self.interaction_projection.ok_or(dag::DagInteractionPlanFault::NodeCredits)?;
        if projection.revision() != self.interaction_revision {
            return Err(dag::DagInteractionPlanFault::Unsupported);
        }
        self.dag.derive_pointer_plan(projection, intent)
    }

    pub fn commit_pointer(&mut self, plan: dag::DagPointerPlan) -> bool {
        if self.interaction_revision != plan.expected_revision() {
            return false;
        }
        self.dag.apply_pointer_plan(&plan);
        self.interaction_projection = Some(*plan.projection());
        self.interaction_revision = plan.projection().revision();
        true
    }

    pub fn pointer_projection_snapshot(&self, plan: &dag::DagPointerPlan) -> Result<dag::DagPointerSnapshot, dag::DagInteractionPlanFault> {
        let projection = plan.projection();
        let mut bytes = 0usize;
        for id in self.dag.projection_selected_id_refs(projection).chain(self.dag.projection_hovered_id_ref(projection)) {
            bytes = bytes.checked_add(id.len()).ok_or(dag::DagInteractionPlanFault::StringCredits)?;
            if bytes > 16 * 1024 {
                return Err(dag::DagInteractionPlanFault::StringCredits);
            }
        }
        Ok(dag::DagPointerSnapshot { node_ids: self.dag.projection_selected_id_refs(projection).map(str::to_owned).collect(), hovered_id: self.dag.projection_hovered_id_ref(projection).map(str::to_owned), camera: projection.camera() })
    }

    pub fn pointer_down_screen(&mut self, position: [f64; 2], button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        let [sx, sy] = position;
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, pan);
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
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
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
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
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
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

/// 🧹 Incremental exact-owner retirement for one retained graph host.
pub struct GraphHostRetirement {
    dag: Option<dag::DagHostRetirement>,
    catalogue_json: String,
    controls_json: String,
    capabilities_json: String,
    pending_gather: Option<SelectionGather>,
    interaction_projection: Option<dag::DagInteractionProjection>,
    terminal: bool,
}

impl GraphHostRetirement {
    pub fn new(host: GraphHost) -> Self {
        let GraphHost { dag, catalogue_json, controls_json, capabilities_json, last_payload_signature: _, pending_gather, interaction_revision: _, interaction_projection } = host;
        Self { dag: Some(dag::DagHostRetirement::new(dag)), catalogue_json, controls_json, capabilities_json, pending_gather, interaction_projection, terminal: false }
    }

    pub fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        if let Some(dag) = self.dag.as_mut() {
            if dag.close_step(1, 1) == dag::DagRetirementStep::Complete {
                if !dag.terminal_is_empty() {
                    return false;
                }
                self.dag = None;
            }
            context.consume_fuel(1);
            return false;
        }
        if self.catalogue_json.pop().is_some() || self.controls_json.pop().is_some() || self.capabilities_json.pop().is_some() {
            context.consume_fuel(1);
            return false;
        }
        if let Some(gather) = self.pending_gather.as_mut() {
            if gather.target_ids.last_mut().is_some_and(|id| id.pop().is_some()) {
                context.consume_fuel(1);
                return false;
            }
            if gather.target_ids.pop().is_some() {
                context.consume_fuel(1);
                return false;
            }
            self.pending_gather = None;
            context.consume_fuel(1);
            return false;
        }
        if self.interaction_projection.take().is_some() {
            context.consume_fuel(1);
            return false;
        }
        self.terminal = true;
        context.consume_fuel(1);
        true
    }

    pub fn terminal_nonopaque_is_empty(&self) -> bool {
        self.terminal && self.dag.is_none() && self.catalogue_json.is_empty() && self.controls_json.is_empty() && self.capabilities_json.is_empty() && self.pending_gather.is_none() && self.interaction_projection.is_none()
    }
}

impl Drop for GraphHostRetirement {
    fn drop(&mut self) {
        debug_assert!(self.terminal_nonopaque_is_empty(), "GraphHostRetirement must reach terminal-empty before release");
    }
}
//#endregion 🔖️GraphHost

//#region 🔖️Wasm
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this session bridge is
// browser-only (attaches an `HtmlCanvasElement`), so it is narrowed to exclude the WASI
// component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod wasm_session {
    use super::*;
    use semio_framework_async::browser::future_to_promise;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
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

        #[wasm_bindgen(js_name = viewport)]
        pub fn viewport(&self) -> js_sys::Object {
            let viewport = self.state.borrow().host.viewport();
            let object = js_sys::Object::new();
            js_sys::Reflect::set(&object, &JsValue::from_str("x"), &JsValue::from_f64(viewport.x)).expect("viewport x property");
            js_sys::Reflect::set(&object, &JsValue::from_str("y"), &JsValue::from_f64(viewport.y)).expect("viewport y property");
            js_sys::Reflect::set(&object, &JsValue::from_str("zoom"), &JsValue::from_f64(viewport.zoom)).expect("viewport zoom property");
            object
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
            if let Ok(node) = dsl::os_pack::json::from_json_str::<DagNodeSpec>(json) {
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
            let opts = if options_json.trim().is_empty() { DagLayoutOptions::default() } else { dsl::os_pack::json::from_json_str(options_json).unwrap_or_default() };
            self.state.borrow_mut().host.dag.reorganize(&opts).map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use wasm_session::GraphSession;
//#endregion 🔖️Wasm

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use dag::{dag_take_pending_open_instance_id, DagLayoutOptions};
