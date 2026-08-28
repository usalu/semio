//! 🖥️ Flow host: canvas editing, evaluation session, and host errors.

use crate::infinite::board::ports::directed_dag as dag;
use crate::infinite::canvas;
use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

use dag::{dag_fixture_execution_rows, dag_fixture_to_wire_literal, fit_node_size, would_create_cycle, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodeSpec, EdgeRouteStyle};
use graph::dsl::{WireEdge, WireNode};
use graph::manifest::{PropertyBag, PropertyValue};
use neural::{channel_output, compute_dirty_set, Atom, BudgetedEval, Dictionary, EvalChannels, EvalError, Evaluator, NeuralCache, Neuron, OperatorInfo, Synapse, Tree, TreeSnapshot, Value as NeuralValue, CLUSTER_KIND, INPUT_KIND, OUTPUT_KIND};
use serde::{Deserialize, Serialize};

use crate::artifact::*;
use crate::bridge::*;
use crate::catalogue::*;
use crate::drawing::*;
use crate::os_store::{create_document_envelope, ArtifactCommand, MemberStoreOwner, SnapshotRetirementStep, SpaceMember};
use crate::registry::*;
use crate::vcs::*;
use semio_framework::io::resolve_ready;

// #region ⚠️ Errors
/// 🧯️ `FlowHost`'s error type — wraps JSON codec failures, the `dag` crate's own `DagError`, and
/// this crate's own graph-editing validation failures. Every variant's Display text is byte-for-byte
/// identical to the `String` it replaces, so downstream `.to_string()` call sites and JSON error
/// envelopes are unaffected.
#[derive(Debug)]
pub enum FlowCoreError {
    Json(serde_json::Error),
    Dag(dag::DagError),
    WidgetIdExists(String),
    UnknownWidget(String),
    UnknownNeuronWidget(String),
    NotVariadicInput(String),
    NotVariadicOutput(String),
    NotNeuron(String),
    WidgetNotNeuron(String),
    MaxInputPortsReached(String),
    MaxOutputPortsReached(String),
    UnknownInputPort(String),
    UnknownOutputPort(String),
    MinInputPorts { widget: String, min: usize },
    MinOutputPorts { widget: String, min: usize },
    NoOutputPort(String),
    NoInputPort(String),
    SelfConnection,
    SelfInsertion,
    CycleWouldBeCreated,
    ConnectionAlreadyExists,
    UnknownSynapse(String),
    UnknownWidgetLayout(String),
    CollapseNeedsTwoWidgets,
    CollapseUnknownWidgets,
    CollapseContainsClusters,
    UnknownCluster(String),
    WidgetNotCluster(String),
}

impl std::fmt::Display for FlowCoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => std::fmt::Display::fmt(error, formatter),
            Self::Dag(error) => std::fmt::Display::fmt(error, formatter),
            Self::WidgetIdExists(id) => write!(formatter, "widget id already exists: {id}"),
            Self::UnknownWidget(id) => write!(formatter, "unknown widget: {id}"),
            Self::UnknownNeuronWidget(id) => write!(formatter, "unknown neuron widget: {id}"),
            Self::NotVariadicInput(id) => write!(formatter, "{id} is not variadic"),
            Self::NotVariadicOutput(id) => write!(formatter, "{id} is not variadic output"),
            Self::NotNeuron(id) => write!(formatter, "{id} is not a neuron"),
            Self::WidgetNotNeuron(id) => write!(formatter, "widget is not a neuron: {id}"),
            Self::MaxInputPortsReached(id) => write!(formatter, "{id} reached max input ports"),
            Self::MaxOutputPortsReached(id) => write!(formatter, "{id} reached max output ports"),
            Self::UnknownInputPort(id) => write!(formatter, "unknown input port: {id}"),
            Self::UnknownOutputPort(id) => write!(formatter, "unknown output port: {id}"),
            Self::MinInputPorts { widget, min } => write!(formatter, "{widget} requires at least {min} inputs"),
            Self::MinOutputPorts { widget, min } => write!(formatter, "{widget} requires at least {min} outputs"),
            Self::NoOutputPort(id) => write!(formatter, "{id} has no output port"),
            Self::NoInputPort(id) => write!(formatter, "{id} has no input port"),
            Self::SelfConnection => formatter.write_str("cannot connect widget to itself"),
            Self::SelfInsertion => formatter.write_str("cannot insert widget between itself"),
            Self::CycleWouldBeCreated => formatter.write_str("connection would create cycle"),
            Self::ConnectionAlreadyExists => formatter.write_str("connection already exists"),
            Self::UnknownSynapse(id) => write!(formatter, "unknown synapse: {id}"),
            Self::UnknownWidgetLayout(id) => write!(formatter, "unknown widget layout: {id}"),
            Self::CollapseNeedsTwoWidgets => formatter.write_str("select at least two widgets to collapse"),
            Self::CollapseUnknownWidgets => formatter.write_str("selection contains unknown widgets"),
            Self::CollapseContainsClusters => formatter.write_str("cannot collapse clusters"),
            Self::UnknownCluster(id) => write!(formatter, "unknown cluster: {id}"),
            Self::WidgetNotCluster(id) => write!(formatter, "widget is not a cluster: {id}"),
        }
    }
}

impl std::error::Error for FlowCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Dag(error) => Some(error),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for FlowCoreError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<dag::DagError> for FlowCoreError {
    fn from(error: dag::DagError) -> Self {
        Self::Dag(error)
    }
}
// #endregion ⚠️ Errors

// #region 🔖️FlowHost
#[derive(Clone, Copy, Debug)]
pub struct FlowWheelPlan {
    revision: u64,
    expected: [f64; 3],
    next: [f64; 3],
}

impl FlowWheelPlan {
    pub fn camera(&self) -> [f64; 3] {
        self.next
    }
}

/// 🏠️ Retained flow host: fixture, dag scene, evaluation cache.
pub struct FlowHost {
    pub fixture: FlowFixture,
    pub dag: DagHost,
    pub outputs: BTreeMap<String, Dictionary>,
    export_payloads: BTreeMap<String, Dictionary>,
    pub last_eval_json: String,
    eval_bridge: Option<EvalBridge>,
    host_catalogue_json: String,
    kind_infos: HashMap<String, OperatorInfo>,
    neural_cache: Arc<NeuralCache>,
    previous_snapshot: Option<TreeSnapshot>,
    previous_channels: Option<EvalChannels>,
    next_widget_serial: u64,
    next_synapse_serial: u64,
    viewport_w: u32,
    viewport_h: u32,
    viewport_dpr: f64,
    pan_anchor: Option<(f64, f64, f64, f64)>,
    ghost_node: Option<DagNodeSpec>,
    /// ↩️ Undo/redo, backed by the standard `crate::os_store::ArtifactStore<FlowFixture, FlowMutation>`
    /// mechanism (see the `impl FlowHost`'s `🔖️History` region) instead of a hand-rolled snapshot stack.
    history_store: Option<FlowStore>,
    pending_history_baseline: Option<FlowFixture>,
    /// 🚩️ Armed by `begin_change`/`begin_gesture` for a discrete mutation not yet flushed into
    /// `history_store` — lets `can_undo` reflect it immediately, mirroring how the old snapshot stack's
    /// `begin_change` pushed synchronously instead of lazily.
    pending_change: bool,
    /// 🖐️ `true` while a coalescing gesture (drag, inline note edit) is in progress — guards
    /// `begin_change` from checkpointing mid-gesture; see `begin_gesture`/`commit_gesture_history`.
    gesture_active: bool,
    pending_extension_eval: Option<neural::PendingExtensionEval>,
    interaction_revision: u64,
    interaction_projection: Option<dag::DagInteractionProjection>,
}

impl Default for FlowHost {
    fn default() -> Self {
        Self::from_fixture(FlowFixture::default())
    }
}

impl FlowHost {
    pub fn from_fixture(fixture: FlowFixture) -> Self {
        Self::from_fixture_with_cache(fixture, Arc::new(NeuralCache::new()))
    }

    /// 🧠️ Builds a host sharing an existing [`NeuralCache`] — lets a long-lived caller (e.g. a
    /// stateless request/response program boundary that reconstructs `FlowHost` on every call)
    /// keep per-node memoization alive across those reconstructions instead of discarding it.
    pub fn from_fixture_with_cache(mut fixture: FlowFixture, neural_cache: Arc<NeuralCache>) -> Self {
        dedupe_fixture_widgets(&mut fixture);
        let mut host = Self {
            fixture,
            dag: DagHost::from_fixture(DagFixture { schema: "dag.fixture".into(), camera: dag::DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            outputs: BTreeMap::new(),
            export_payloads: BTreeMap::new(),
            last_eval_json: String::new(),
            eval_bridge: None,
            host_catalogue_json: String::new(),
            kind_infos: HashMap::new(),
            neural_cache,
            previous_snapshot: None,
            previous_channels: None,
            next_widget_serial: 1,
            next_synapse_serial: 100,
            viewport_w: 1,
            viewport_h: 1,
            viewport_dpr: 1.0,
            pan_anchor: None,
            ghost_node: None,
            history_store: None,
            pending_history_baseline: None,
            pending_change: false,
            gesture_active: false,
            pending_extension_eval: None,
            interaction_revision: 0,
            interaction_projection: None,
        };
        host.rebuild_dag();
        host.refresh_interaction_projection();
        host
    }

    /// 📥️ Replaces fixture content while keeping catalogue, operator metadata, eval bridge, and the live camera.
    pub fn replace_fixture(&mut self, fixture: FlowFixture) {
        self.apply_fixture(fixture, true, false);
    }

    /// 📥️ Scene resync: reloads fixture layout/content without discarding eval baseline or cached outputs.
    pub fn resync_fixture_from_scene(&mut self, fixture: FlowFixture) {
        self.apply_fixture(fixture, false, true);
    }

    /// 📥️ Replaces fixture content without clearing undo/redo history.
    pub fn set_fixture_preserving_history(&mut self, fixture: FlowFixture) {
        self.apply_fixture(fixture, false, false);
    }

    fn apply_fixture(&mut self, mut fixture: FlowFixture, reset_history: bool, preserve_eval: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        dedupe_fixture_widgets(&mut fixture);
        // 🎥️ Camera is ephemeral view state (same as undo/redo) — never snap the live pan/zoom when a
        // scene resync reloads fixture content (hover, eval tick, remote operations, …).
        let camera = self.fixture.camera.clone();
        fixture.camera = camera;
        std::mem::replace(&mut self.fixture, fixture).retire_cold();
        if !preserve_eval {
            self.outputs.clear();
            self.export_payloads.clear();
            self.last_eval_json.clear();
            self.previous_snapshot = None;
            self.previous_channels = None;
        }
        self.pan_anchor = None;
        self.ghost_node = None;
        self.rebuild_dag();
        self.refresh_interaction_projection();
        if reset_history {
            if let Some(store) = self.history_store.as_mut() {
                let envelope = create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow-host", self.fixture.clone(), None);
                resolve_ready(store.reset(envelope, Vec::new(), Vec::new())).expect("failed to reset flow history store");
                store.install_member_store_owners_exact(FlowFixture::member_store_owners());
            }
            self.pending_history_baseline = None;
            self.pending_change = false;
            self.gesture_active = false;
        }
    }

    pub fn parse_fixture_json(json: &str) -> Result<FlowFixture, FlowCoreError> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn fixture_json(&self) -> Result<String, FlowCoreError> {
        Ok(serde_json::to_string(&self.fixture)?)
    }

    pub fn document(&self) -> FlowArtifact {
        self.fixture.to_artifact()
    }

    pub fn catalogue_json(&self) -> Result<String, FlowCoreError> {
        let sections = merge_catalogue_sections(&self.host_catalogue_json)?;
        Ok(serde_json::to_string(&sections)?)
    }

    pub fn set_host_catalogue_json(&mut self, json: &str) {
        self.host_catalogue_json = json.to_string();
    }

    pub fn set_neuron_kind_infos_json(&mut self, json: &str) {
        self.kind_infos = if json.trim().is_empty() { HashMap::new() } else { serde_json::from_str::<Vec<OperatorInfo>>(json).map(|items| items.into_iter().map(|info| (info.id.clone(), info)).collect()).unwrap_or_default() };
        self.rebuild_dag();
    }

    /// 🧠️ Same as `set_neuron_kind_infos_json` but over the typed `NodeGraphScene.operators` records.
    pub fn set_neuron_kind_infos(&mut self, infos: &[ui_wgpu::wgpu::NodeGraphOperatorRecord]) {
        self.kind_infos = infos.iter().map(|record| (record.id.clone(), node_graph_operator_record_to_operator_info(record))).collect();
        self.rebuild_dag();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_eval_bridge_fn(&mut self, cb: Box<EvalBridgeFn>) {
        self.eval_bridge = Some(EvalBridge { cb });
    }

    pub fn evaluate(&mut self) -> Result<String, FlowCoreError> {
        self.evaluate_internal();
        Ok(self.last_eval_json.clone())
    }

    /// 📥️ Applies channel-structured eval JSON from an off-thread worker without re-running operators.
    pub fn apply_eval_outputs_json(&mut self, json: &str) {
        if is_global_eval_error_json(json) {
            self.dag.clear_computing();
            return;
        }
        let outputs = outputs_from_channel_eval_json(json);
        let inputs = inputs_from_channel_eval_json(json);
        let channels = EvalChannels { outputs, inputs };
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.previous_snapshot.as_ref(), &snapshot);
        let converged = self.probe_eval_outputs_converged(&tree, &seeds, &dirty, &channels);
        self.last_eval_json = json.to_string();
        if converged {
            self.outputs = channels.outputs.clone();
            self.apply_preview_outputs(&channels.outputs);
            self.apply_export_outputs(&channels.outputs);
            self.previous_snapshot = Some(snapshot);
            self.previous_channels = Some(channels);
            self.dag.clear_computing();
        } else {
            self.refresh_computing_chrome_from_pending();
        }
    }

    fn probe_eval_outputs_converged(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>, dirty: &HashSet<String>, channels: &EvalChannels) -> bool {
        let registry = flow_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        let mut probe_never_dispatches = |kind: &str, _: &Dictionary| -> Result<Dictionary, EvalError> { Err(EvalError::InvalidInput(format!("apply_eval_outputs_json probed a dispatch for {kind}"))) };
        match evaluator.evaluate_channels_budgeted(tree, seeds, &self.kind_infos, &mut probe_never_dispatches, &self.neural_cache, dirty, Some(channels), 0) {
            Ok(BudgetedEval { remaining, .. }) => remaining.is_empty(),
            Err(_) => false,
        }
    }

    /// 🧵️ Installs a durable eval baseline from an off-thread driver onto this ephemeral host.
    pub fn install_eval_baseline(&mut self, snapshot: Option<TreeSnapshot>, channels: Option<EvalChannels>) {
        self.previous_snapshot = snapshot;
        self.previous_channels = channels;
        // Receiverless ArtifactApp rebuilds a fresh FlowHost per call; restore outputs so
        // blocked-port status and preview wiring see the last completed eval channels.
        if let Some(channels) = self.previous_channels.as_ref() {
            self.outputs = channels.outputs.clone();
        }
    }

    /// 🧵️ Captures this host's eval baseline for persistence on a durable driver.
    pub fn eval_baseline(&self) -> (Option<TreeSnapshot>, Option<EvalChannels>) {
        (self.previous_snapshot.clone(), self.previous_channels.clone())
    }

    /// ⚙️ Probes pending nodes and paints active/stale computing chrome on the DAG canvas.
    pub fn refresh_computing_chrome_from_pending(&mut self) {
        let remaining = self.pending_eval_widget_ids();
        if remaining.is_empty() {
            self.dag.clear_computing();
            return;
        }
        let active = remaining.first().map(|id| id.as_str());
        let stale = remaining.get(1..).unwrap_or(&[]).to_vec();
        self.dag.set_computing_progress(active, &stale);
    }

    /// ⚙️ Marks one actively computing widget and downstream widgets as stale.
    pub fn set_node_statuses_from_json(&mut self, json: &str) {
        self.dag.set_node_statuses_from_json(json);
    }

    pub fn set_computing_progress(&mut self, active_widget_id: Option<&str>, stale_widget_ids: &[String]) {
        self.dag.set_computing_progress(active_widget_id, stale_widget_ids);
    }

    /// ✅️ Clears computing chrome from all widgets.
    pub fn clear_computing_widget_ids(&mut self) {
        self.dag.clear_computing();
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport_w = width.max(1);
        self.viewport_h = height.max(1);
        self.viewport_dpr = dpr.max(1.0);
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    pub fn world_from_screen(&self, sx: f64, sy: f64) -> (f64, f64) {
        let p = self.screen_to_world_point(sx, sy);
        (p.x, p.y)
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.fixture.camera = CameraJson { x, y, zoom: zoom.clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX) };
        self.dag.set_camera(x, y, self.fixture.camera.zoom);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    fn refresh_interaction_projection(&mut self) {
        self.interaction_projection = self.dag.bounded_interaction_projection(self.interaction_revision).ok();
    }

    pub fn plan_wheel(&self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) -> FlowWheelPlan {
        let camera = &self.fixture.camera;
        let expected = [camera.x, camera.y, camera.zoom];
        let next = if zoom_gesture {
            use canvas::camera::{screen_to_world, Camera, Viewport};
            let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
            let before_camera = Camera { x: camera.x, y: camera.y, zoom: camera.zoom };
            let before = screen_to_world(&before_camera, &viewport, canvas::Point::new(sx, sy));
            let zoom = (camera.zoom * if delta_y < 0.0 { ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR } else { ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR })
                .clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
            let after_camera = Camera { x: camera.x, y: camera.y, zoom };
            let after = screen_to_world(&after_camera, &viewport, canvas::Point::new(sx, sy));
            [camera.x + before.x - after.x, camera.y + before.y - after.y, zoom]
        } else {
            [camera.x - delta_x / camera.zoom, camera.y - delta_y / camera.zoom, camera.zoom]
        };
        FlowWheelPlan { revision: self.interaction_revision, expected, next }
    }

    pub fn commit_wheel(&mut self, plan: FlowWheelPlan) -> bool {
        let camera = &self.fixture.camera;
        if self.interaction_revision != plan.revision || [camera.x.to_bits(), camera.y.to_bits(), camera.zoom.to_bits()] != [plan.expected[0].to_bits(), plan.expected[1].to_bits(), plan.expected[2].to_bits()] {
            return false;
        }
        self.fixture.camera = CameraJson { x: plan.next[0], y: plan.next[1], zoom: plan.next[2] };
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
        if !plan.previous_gesture_active() && plan.gesture_active() {
            self.begin_gesture();
        }
        for index in 0..plan.move_len() {
            let Some((id, x, y)) = self.dag.pointer_plan_move(&plan, index) else {
                continue;
            };
            if self.fixture.layout.contains_key(id) {
                self.fixture.layout.insert(id.to_owned(), WidgetLayout { x, y });
            }
        }
        self.dag.apply_pointer_plan(&plan);
        if plan.previous_gesture_active() && !plan.gesture_active() {
            self.commit_gesture_history();
        }
        self.interaction_projection = Some(*plan.projection());
        self.interaction_revision = plan.projection().revision();
        true
    }

    pub fn pointer_projection_snapshot(&self, plan: &dag::DagPointerPlan) -> Result<(Vec<String>, Option<String>, [f64; 3]), dag::DagInteractionPlanFault> {
        let projection = plan.projection();
        let mut bytes = 0usize;
        for id in self.dag.projection_selected_id_refs(projection).chain(self.dag.projection_hovered_id_ref(projection)) {
            bytes = bytes.checked_add(id.len()).ok_or(dag::DagInteractionPlanFault::StringCredits)?;
            if bytes > 16 * 1024 {
                return Err(dag::DagInteractionPlanFault::StringCredits);
            }
        }
        Ok((self.dag.projection_selected_id_refs(projection).map(str::to_owned).collect(), self.dag.projection_hovered_id_ref(projection).map(str::to_owned), projection.camera()))
    }

    pub fn wheel_zoom_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        let before = self.screen_to_world_point(sx, sy);
        let factor = if delta_y < 0.0 { ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR } else { ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR };
        let zoom = (self.fixture.camera.zoom * factor).clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
        self.fixture.camera.zoom = zoom;
        self.dag.set_camera(self.fixture.camera.x, self.fixture.camera.y, zoom);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        let after = self.screen_to_world_point(sx, sy);
        self.fixture.camera.x += before.x - after.x;
        self.fixture.camera.y += before.y - after.y;
        self.dag.set_camera(self.fixture.camera.x, self.fixture.camera.y, zoom);
    }

    pub fn wheel_pan_screen(&mut self, delta_x: f64, delta_y: f64) {
        let zoom = self.fixture.camera.zoom;
        let x = self.fixture.camera.x - delta_x / zoom;
        let y = self.fixture.camera.y - delta_y / zoom;
        self.set_camera(x, y, zoom);
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) {
        if zoom_gesture {
            self.wheel_zoom_screen(sx, sy, delta_y);
        } else {
            self.wheel_pan_screen(delta_x, delta_y);
        }
    }

    pub fn set_ghost_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<(), FlowCoreError> {
        let descriptor: WidgetDescriptor = serde_json::from_str(descriptor_json)?;
        let id: String = "__ghost__".into();
        let widget = widget_from_descriptor(&descriptor, id.clone(), &self.kind_infos);
        let mut layout = crate::OrderedMap::new();
        layout.insert(id, WidgetLayout { x: world_x, y: world_y });
        let mut node = widget_to_dag_node(&widget, 0, &layout, &[], &self.kind_infos);
        widget.retire_cold();
        let mut retirement = crate::retained::FlowRetirement::default();
        retirement.push(crate::retained::FlowOwner::Layouts(layout));
        retirement.retire_cold();
        fit_node_size(&mut node);
        self.ghost_node = Some(node.clone());
        self.dag.set_ghost_node(Some(node));
        Ok(())
    }

    pub fn clear_ghost_widget(&mut self) {
        self.ghost_node = None;
        self.dag.set_ghost_node(None);
    }

    pub fn add_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<String, FlowCoreError> {
        self.begin_change();
        self.clear_ghost_widget();
        let descriptor: WidgetDescriptor = serde_json::from_str(descriptor_json)?;
        let id = descriptor_explicit_id(&descriptor).unwrap_or_else(|| self.next_widget_id(&descriptor));
        if self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == id) {
            return Err(FlowCoreError::WidgetIdExists(id));
        }
        let widget = widget_from_descriptor(&descriptor, id.clone(), &self.kind_infos);
        self.fixture.widgets.push(widget);
        self.fixture.layout.insert(id.clone(), WidgetLayout { x: world_x, y: world_y });
        self.rebuild_dag();
        Ok(id)
    }

    pub fn remove_widget(&mut self, widget_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let before = self.fixture.widgets.len();
        self.fixture.widgets.retain(|w| widget_id_for(w) != widget_id);
        if self.fixture.widgets.len() == before {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        }
        self.fixture.layout.remove(widget_id);
        self.fixture.synapses.retain(|s| s.from != widget_id && s.to != widget_id);
        self.rebuild_dag();
        Ok(())
    }

    pub fn move_widget(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), FlowCoreError> {
        if !self.fixture.widgets.iter().any(|w| widget_id_for(w) == widget_id) {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        }
        self.fixture.layout.insert(widget_id.to_string(), WidgetLayout { x, y });
        self.dag.set_widget_position(widget_id, x, y)?;
        Ok(())
    }

    pub fn connect(&mut self, from_id: &str, to_id: &str) -> Result<String, FlowCoreError> {
        let from_port = first_output_port(from_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos);
        let to_port = first_input_port(to_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos);
        self.connect_ports(from_id, &from_port, to_id, &to_port)
    }

    pub fn connect_ports(&mut self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, FlowCoreError> {
        self.begin_change();
        if from_id == to_id {
            return Err(FlowCoreError::SelfConnection);
        }
        if !widget_has_output(from_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(from_id.to_string()));
        }
        if !widget_has_input(to_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoInputPort(to_id.to_string()));
        }
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err(FlowCoreError::CycleWouldBeCreated);
        }
        if self.fixture.synapses.iter().any(|s| s.from == from_id && s.from_port == from_port && s.to == to_id && s.to_port == to_port) {
            return Err(FlowCoreError::ConnectionAlreadyExists);
        }
        self.fixture.synapses.retain(|s| !(s.to == to_id && s.to_port == to_port));
        self.next_synapse_serial += 1;
        let synapse_id = format!("s{}", self.next_synapse_serial);
        self.fixture.synapses.push(SynapseSpec { id: synapse_id.clone(), from: from_id.to_string(), to: to_id.to_string(), from_port: from_port.to_string(), to_port: to_port.to_string() });
        self.rebuild_dag();
        Ok(synapse_id)
    }

    pub fn add_input_port(&mut self, widget_id: &str, index: usize) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_input.clone()).ok_or_else(|| FlowCoreError::NotVariadicInput(widget_id.to_string()))?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { input_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let mut ports = default_neuron_input_ports(&neuron_kind, input_ports, &self.kind_infos);
        if let Some(max) = spec.max {
            if ports.len() >= max {
                return Err(FlowCoreError::MaxInputPortsReached(widget_id.to_string()));
            }
        }
        let insert_at = index.min(ports.len());
        ports.insert(insert_at, insert_at.to_string());
        for synapse in &mut self.fixture.synapses {
            if synapse.to != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.to_port.parse::<usize>() {
                if old_index >= insert_at {
                    synapse.to_port = (old_index + 1).to_string();
                }
            }
        }
        *input_ports = (0..ports.len()).map(|slot| slot.to_string()).collect();
        self.rebuild_dag();
        Ok(())
    }

    pub fn remove_input_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_input.clone()).ok_or_else(|| FlowCoreError::NotVariadicInput(widget_id.to_string()))?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { input_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let ports = default_neuron_input_ports(&neuron_kind, input_ports, &self.kind_infos);
        if ports.len() <= spec.min {
            return Err(FlowCoreError::MinInputPorts { widget: widget_id.to_string(), min: spec.min });
        }
        let Some(remove_index) = ports.iter().position(|port| port == port_id) else {
            return Err(FlowCoreError::UnknownInputPort(port_id.to_string()));
        };
        self.fixture.synapses.retain(|synapse| !(synapse.to == widget_id && synapse.to_port == port_id));
        for synapse in &mut self.fixture.synapses {
            if synapse.to != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.to_port.parse::<usize>() {
                if old_index > remove_index {
                    synapse.to_port = (old_index - 1).to_string();
                }
            }
        }
        let mut next_ports = ports;
        next_ports.remove(remove_index);
        *input_ports = (0..next_ports.len()).map(|slot| slot.to_string()).collect();
        self.rebuild_dag();
        Ok(())
    }

    pub fn add_output_port(&mut self, widget_id: &str, index: usize) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_output.clone()).ok_or_else(|| FlowCoreError::NotVariadicOutput(widget_id.to_string()))?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { output_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let mut ports = default_neuron_output_ports(&neuron_kind, output_ports, &self.kind_infos);
        if let Some(max) = spec.max {
            if ports.len() >= max {
                return Err(FlowCoreError::MaxOutputPortsReached(widget_id.to_string()));
            }
        }
        let insert_at = index.min(ports.len());
        ports.insert(insert_at, insert_at.to_string());
        for synapse in &mut self.fixture.synapses {
            if synapse.from != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.from_port.parse::<usize>() {
                if old_index >= insert_at {
                    synapse.from_port = (old_index + 1).to_string();
                }
            }
        }
        *output_ports = (0..ports.len()).map(|slot| slot.to_string()).collect();
        self.rebuild_dag();
        Ok(())
    }

    pub fn remove_output_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_output.clone()).ok_or_else(|| FlowCoreError::NotVariadicOutput(widget_id.to_string()))?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { output_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let ports = default_neuron_output_ports(&neuron_kind, output_ports, &self.kind_infos);
        if ports.len() <= spec.min {
            return Err(FlowCoreError::MinOutputPorts { widget: widget_id.to_string(), min: spec.min });
        }
        let Some(remove_index) = ports.iter().position(|port| port == port_id) else {
            return Err(FlowCoreError::UnknownOutputPort(port_id.to_string()));
        };
        self.fixture.synapses.retain(|synapse| !(synapse.from == widget_id && synapse.from_port == port_id));
        for synapse in &mut self.fixture.synapses {
            if synapse.from != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.from_port.parse::<usize>() {
                if old_index > remove_index {
                    synapse.from_port = (old_index - 1).to_string();
                }
            }
        }
        let mut next_ports = ports;
        next_ports.remove(remove_index);
        *output_ports = (0..next_ports.len()).map(|slot| slot.to_string()).collect();
        self.rebuild_dag();
        Ok(())
    }

    pub fn disconnect(&mut self, synapse_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let before = self.fixture.synapses.len();
        self.fixture.synapses.retain(|s| s.id != synapse_id);
        if self.fixture.synapses.len() == before {
            return Err(FlowCoreError::UnknownSynapse(synapse_id.to_string()));
        }
        self.rebuild_dag();
        Ok(())
    }

    // #region GumballEditing
    /// 🔀️ Splices `mid_id` between `anchor_id` and its downstream consumers on `anchor_out_port`.
    pub fn insert_between(&mut self, anchor_id: &str, anchor_out_port: &str, mid_id: &str, mid_in_port: &str, mid_out_port: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        if !self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == anchor_id) {
            return Err(FlowCoreError::UnknownWidget(anchor_id.to_string()));
        }
        if !self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == mid_id) {
            return Err(FlowCoreError::UnknownWidget(mid_id.to_string()));
        }
        if anchor_id == mid_id {
            return Err(FlowCoreError::SelfInsertion);
        }
        if !widget_has_output(anchor_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(anchor_id.to_string()));
        }
        if !widget_has_input(mid_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoInputPort(mid_id.to_string()));
        }
        if !widget_has_output(mid_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(mid_id.to_string()));
        }
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|synapse| (synapse.from.clone(), synapse.to.clone())).collect();
        if would_create_cycle(&existing, anchor_id, mid_id) {
            return Err(FlowCoreError::CycleWouldBeCreated);
        }
        let mid_has_input = self.fixture.synapses.iter().any(|synapse| synapse.to == mid_id);
        if !mid_has_input {
            for synapse in &mut self.fixture.synapses {
                if synapse.from == anchor_id && synapse.from_port == anchor_out_port {
                    synapse.from = mid_id.to_string();
                    synapse.from_port = mid_out_port.to_string();
                }
            }
        }
        if self.fixture.synapses.iter().any(|synapse| synapse.from == anchor_id && synapse.from_port == anchor_out_port && synapse.to == mid_id && synapse.to_port == mid_in_port) {
            self.rebuild_dag();
            return Ok(());
        }
        self.next_synapse_serial += 1;
        let synapse_id = format!("s{}", self.next_synapse_serial);
        self.fixture.synapses.push(SynapseSpec { id: synapse_id, from: anchor_id.to_string(), to: mid_id.to_string(), from_port: anchor_out_port.to_string(), to_port: mid_in_port.to_string() });
        self.rebuild_dag();
        Ok(())
    }

    /// ↔ Shifts widgets to the right of `anchor_id` to open layout space for inserted nodes.
    pub fn make_space(&mut self, anchor_id: &str, dx: f64, dy: f64) -> Result<(), FlowCoreError> {
        self.begin_change();
        let anchor_x = self.fixture.layout.get(anchor_id).map(|layout| layout.x).ok_or_else(|| FlowCoreError::UnknownWidgetLayout(anchor_id.to_string()))?;
        let previous = std::mem::take(&mut self.fixture.layout);
        for (widget_id, layout) in &previous {
            let mut layout = layout.clone();
            if layout.x > anchor_x {
                layout.x += dx;
                layout.y += dy;
            }
            let _ = self.dag.set_widget_position(widget_id, layout.x, layout.y);
            self.fixture.layout.insert(widget_id.clone(), layout);
        }
        let mut retirement = crate::retained::FlowRetirement::default();
        retirement.push(crate::retained::FlowOwner::Layouts(previous));
        retirement.retire_cold();
        Ok(())
    }

    /// 🧬️ Merges JSON params into a neuron widget for compact transform values.
    pub fn set_neuron_params(&mut self, widget_id: &str, params_json: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let patch: Dictionary = serde_json::from_str(params_json)?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { params, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        *params = params.merge(&patch);
        self.sync_dag_display_from_widgets();
        Ok(())
    }
    // #endregion GumballEditing

    /// 🌳️ Recomputes widget positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts_json: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let opts: DagLayoutOptions = if opts_json.trim().is_empty() { DagLayoutOptions::default() } else { serde_json::from_str(opts_json)? };
        let theme = self.dag.canvas_theme;
        self.dag = DagHost::from_fixture_without_layout(self.build_dag_fixture_v1());
        self.dag.canvas_theme = theme;
        self.dag.reorganize(&opts)?;
        self.sync_from_dag();
        Ok(())
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "pointer-event handler mirroring this file's other screen-space input methods (pointer_move_screen/pointer_up_screen/wheel_screen) — position + button + modifier-key flags is the natural shape for this UI event, not a bundling candidate on its own without also restructuring its siblings"
    )]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        if pan {
            self.pan_anchor = Some((sx, sy, self.fixture.camera.x, self.fixture.camera.y));
            return;
        }
        self.clear_ghost_widget();
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.begin_gesture();
        self.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, false);
        if let Some((side, widget_id, index)) = self.dag.take_pending_port_insert() {
            match side {
                dag::DagPortSide::Input => {
                    let _ = self.add_input_port(&widget_id, index);
                }
                dag::DagPortSide::Output => {
                    let _ = self.add_output_port(&widget_id, index);
                }
            }
            return;
        }
        self.sync_from_dag();
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.pan_anchor {
            let zoom = self.fixture.camera.zoom;
            let dx = (sx - start_sx) / zoom;
            let dy = (sy - start_sy) / zoom;
            self.set_camera(cam_x - dx, cam_y - dy, zoom);
            return;
        }
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
        self.sync_from_dag();
    }

    pub fn widget_drag_active(&self) -> bool {
        self.dag.widget_drag_active()
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.pan_anchor = None;
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
        self.sync_from_dag();
        self.commit_gesture_history();
    }

    pub fn set_selection_options(&mut self, method: &str, mode: &str) {
        self.dag.set_selection_options(method, mode, true, true, true);
    }

    pub fn selection_preview_points_json(&self) -> String {
        self.dag.selection_preview_points_json()
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.dag.selection_preview_crossing()
    }

    pub fn selection_preview_method(&self) -> &str {
        self.dag.selection_preview_method()
    }

    pub fn preselect_widget_ids_json(&self) -> String {
        serde_json::json!({
            "ids": self.dag.preselect_widget_ids(),
            "removedIds": self.dag.preselect_removed_widget_ids(),
        })
        .to_string()
    }

    pub fn cancel_area_select(&mut self) -> bool {
        let cancelled = self.dag.cancel_area_select();
        if cancelled {
            self.sync_from_dag();
        }
        cancelled
    }

    pub fn delete_selection(&mut self) -> Result<(), FlowCoreError> {
        if !self.dag.has_selection() {
            return Ok(());
        }
        self.begin_change();
        self.dag.delete_selected();
        self.sync_from_dag();
        Ok(())
    }

    /// ✅️ Whether the canvas has any committed node, edge, or handle selection.
    pub fn has_selection(&self) -> bool {
        self.dag.has_selection()
    }

    pub fn select_all(&mut self) {
        self.dag.select_all();
        self.sync_from_dag();
    }

    fn evaluate_internal(&mut self) {
        self.evaluate_step(usize::MAX);
    }

    /// ⏳️🧵️ Evaluates at most `budget` cache-missed (dirty) nodes and returns the not-yet-computed
    /// widget ids in topo order — `remaining[0]` is the node currently blocking, `remaining[1..]`
    /// are downstream widgets waiting behind it. An off-main-thread caller (a plugin worker) resumes
    /// with another `evaluate_step` call until `remaining` is empty; a single `evaluate_step(usize::MAX)`
    /// call (via [`FlowHost::evaluate`]/`evaluate_internal`) still evaluates everything synchronously
    /// in one shot for callers that don't need to spread the work across ticks (tests, explicit
    /// worker-side `evaluate` actions that already run off the caller's main thread).
    ///
    /// `begin_epoch`/`sweep` bracket the *whole run* (every tick up to and including the completing
    /// one), not each tick: `begin_epoch` is cheap to call repeatedly (just bumps a counter), while
    /// `sweep` evicts anything not touched since — calling it before the run completes would discard
    /// earlier ticks' results. A run interleaved with another unrelated evaluation sharing the same
    /// [`NeuralCache`] (e.g. a generation-preview eval firing mid-chain) may have its in-progress
    /// entries swept early by that other call's completion; the next tick simply recomputes them —
    /// extra work, never a wrong result.
    pub fn evaluate_step(&mut self, budget: usize) -> Vec<String> {
        self.pending_extension_eval = None;
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.previous_snapshot.as_ref(), &snapshot);
        if dirty.is_empty() && self.previous_channels.is_some() && !self.outputs.is_empty() {
            return Vec::new();
        }
        let registry = flow_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        self.neural_cache.begin_epoch();
        let previous = self.previous_channels.as_ref();
        let budgeted = if let Some(bridge) = self.eval_bridge.as_ref() {
            let mut dispatch = |kind: &str, input: &Dictionary| bridge.evaluate(kind, input);
            evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut dispatch, &self.neural_cache, &dirty, previous, budget)
        } else {
            let mut dispatch = |kind: &str, input: &Dictionary| registry.as_ref().dispatch(kind, input);
            evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut dispatch, &self.neural_cache, &dirty, previous, budget)
        };
        match budgeted {
            Ok(BudgetedEval { channels, remaining, pending_extension }) => {
                self.pending_extension_eval = pending_extension;
                self.outputs = channels.outputs.clone();
                self.apply_preview_outputs(&channels.outputs);
                self.apply_export_outputs(&channels.outputs);
                self.last_eval_json = build_channel_eval_json(&self.fixture, &channels, &self.kind_infos);
                if !remaining.is_empty() {
                    return remaining;
                }
                self.neural_cache.sweep();
                let live_handles = collect_live_geometry_handles_from_channels(&channels);
                crate::retain_geometry_handles(&live_handles);
                let live_drawing_handles = collect_live_drawing_handles_from_channels(&channels);
                retain_drawing_handles(&live_drawing_handles);
                // 🔒️ Only advance the snapshot/channels pair together, and only on success — a
                // failed evaluation keeps diffing against the last known-good state next time,
                // which is always a safe (never under-dirty) baseline.
                self.previous_snapshot = Some(snapshot);
                self.previous_channels = Some(channels);
                Vec::new()
            }
            Err(err) => {
                self.neural_cache.sweep();
                if self.last_eval_json.is_empty() || is_global_eval_error_json(&self.last_eval_json) {
                    self.last_eval_json = serde_json::json!({ "error": err.to_string() }).to_string();
                }
                Vec::new()
            }
        }
    }

    /// 🔌️ Consumes the last budgeted step's contributed-extension eval request, if any.
    pub fn take_pending_extension_eval(&mut self) -> Option<neural::PendingExtensionEval> {
        self.pending_extension_eval.take()
    }

    /// 👀️ Probes which widget ids still need evaluation without computing anything (`budget = 0`) —
    /// used to decide whether a tick chain must be (re)armed and what to mark as computing/stale.
    pub fn eval_baseline_snapshot(&self) -> Option<&TreeSnapshot> {
        self.previous_snapshot.as_ref()
    }

    pub fn widget_blocked_ports(&self, widget_id: &str) -> Vec<String> {
        let Some(operator_info) = self.fixture.widgets.iter().find(|widget| widget_id_for(widget) == widget_id).and_then(|widget| widget_operator_info(widget, &self.kind_infos)) else {
            return Vec::new();
        };
        if operator_info.variadic_input.is_some() {
            return Vec::new();
        }
        let tree = self.build_tree();
        let mut outputs = self.build_seeds();
        outputs.extend(self.outputs.clone());
        let mut missing = Vec::new();
        for channel in &operator_info.inputs {
            if channel.name == "*" {
                continue;
            }
            if channel.cardinality != neural::Cardinality::ExactlyOne {
                continue;
            }
            if channel.default.is_some() {
                continue;
            }
            let wired = tree.synapses.iter().any(|syn| syn.to == widget_id && syn.to_port == channel.name);
            if !wired {
                continue;
            }
            let source_ready = tree.synapses.iter().filter(|syn| syn.to == widget_id && syn.to_port == channel.name).all(|syn| outputs.contains_key(&syn.from));
            if !source_ready {
                missing.push(channel.name.clone());
            }
        }
        missing
    }

    pub(crate) fn build_tree_for_status(&self) -> Tree {
        self.build_tree()
    }

    pub(crate) fn build_seeds_for_status(&self) -> HashMap<String, Dictionary> {
        self.build_seeds()
    }

    pub fn pending_eval_widget_ids(&self) -> Vec<String> {
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.previous_snapshot.as_ref(), &snapshot);
        if dirty.is_empty() && self.previous_channels.is_some() && !self.outputs.is_empty() {
            return Vec::new();
        }
        let registry = flow_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        let previous = self.previous_channels.as_ref();
        let mut probe_never_dispatches = |kind: &str, _: &Dictionary| -> Result<Dictionary, EvalError> { Err(EvalError::InvalidInput(format!("pending_eval_widget_ids probed a dispatch for {kind}"))) };
        match evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut probe_never_dispatches, &self.neural_cache, &dirty, previous, 0) {
            Ok(BudgetedEval { remaining, .. }) => remaining,
            Err(_) => Vec::new(),
        }
    }

    // #region 🌳️TreeBuilding
    fn build_tree(&self) -> Tree {
        let fixture = self.build_dag_fixture_v1();
        let (nodes, edges) = dag_fixture_execution_rows(&fixture);
        Self::tree_from_dag(&nodes, &edges)
    }

    // #region 🔗️DagTreeConversion
    fn tree_from_dag(nodes: &[WireNode], edges: &[WireEdge]) -> Tree {
        let neurons = nodes
            .iter()
            .map(|node| {
                let mut params = Self::dictionary_from_property_bag(&node.properties);
                if node.kind == CLUSTER_KIND {
                    if let Some(PropertyValue::String(name)) = node.properties.get("name") {
                        params = params.insert("name", NeuralValue::Atom(Atom::String(name.clone())));
                    }
                }
                let nested = if node.kind == CLUSTER_KIND { Self::cluster_tree_from_node(node).map(Box::new) } else { None };
                Neuron { id: node.id.clone(), kind: node.kind.clone(), params, tree: nested }
            })
            .collect();
        let synapses = edges.iter().enumerate().map(|(index, edge)| Synapse { id: format!("synapse-{index}"), from: edge.from.clone(), to: edge.to.clone(), from_port: edge.from_port.clone(), to_port: edge.to_port.clone() }).collect();
        Tree { neurons, synapses }
    }

    fn cluster_tree_from_node(node: &WireNode) -> Option<Tree> {
        let PropertyValue::String(json) = node.properties.get("clusterTree")? else {
            return None;
        };
        serde_json::from_str(json).ok()
    }

    fn dictionary_from_property_bag(bag: &PropertyBag) -> Dictionary {
        let mut dict = Dictionary::new();
        for (key, value) in bag {
            dict = dict.insert(key, Self::property_value_to_neural(value));
        }
        dict
    }

    fn property_value_to_neural(value: &PropertyValue) -> NeuralValue {
        match value {
            PropertyValue::String(s) => NeuralValue::Atom(Atom::String(s.clone())),
            PropertyValue::Number(n) => NeuralValue::Atom(Atom::Decimal(*n)),
            PropertyValue::Bool(b) => NeuralValue::Atom(Atom::Boolean(*b)),
            PropertyValue::Null => NeuralValue::Atom(Atom::Null),
            PropertyValue::Array(items) => {
                let mut dict = Dictionary::new();
                for (index, row) in items.iter().enumerate() {
                    dict = dict.insert(index.to_string(), Self::property_value_to_neural(row));
                }
                NeuralValue::Dictionary(dict)
            }
            PropertyValue::Object(map) => {
                let mut dict = Dictionary::new();
                for (key, row) in map {
                    dict = dict.insert(key, Self::property_value_to_neural(row));
                }
                NeuralValue::Dictionary(dict)
            }
        }
    }
    // #endregion 🔗️DagTreeConversion
    // #endregion 🌳️TreeBuilding

    /// 📝️ Renders the compiled DAG fixture as wire-literal text.
    pub fn compiled_wire_literal(&self) -> String {
        dag_fixture_to_wire_literal(&self.build_dag_fixture_v1())
    }

    fn build_seeds(&self) -> HashMap<String, Dictionary> {
        let mut seeds = HashMap::new();
        for widget in &self.fixture.widgets {
            match widget {
                Widget::InputSlider { id, value, .. } => {
                    seeds.insert(id.clone(), channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(*value)))));
                }
                Widget::InputNote { id, text } => {
                    seeds.insert(id.clone(), channel_output("text", Dictionary::with_schema("text").insert("value", NeuralValue::Atom(Atom::String(text.clone())))));
                }
                Widget::InputImage { id, src } => {
                    seeds.insert(id.clone(), channel_output("image", Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone())))));
                }
                _ => {}
            }
        }
        seeds
    }

    fn apply_preview_outputs(&mut self, outputs: &BTreeMap<String, Dictionary>) {
        for widget in &mut self.fixture.widgets {
            if let Widget::OutputPreview { id, preview, .. } = widget {
                if let Some(out) = outputs.get(id) {
                    *preview = out.clone();
                } else if let Some(syn) = self.fixture.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        *preview = preview_dict_from_connection(src, &syn.from_port, &syn.to_port);
                    }
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.dag.fit_preview_sizes();
    }

    fn apply_export_outputs(&mut self, outputs: &BTreeMap<String, Dictionary>) {
        for widget in &self.fixture.widgets {
            if let Widget::OutputExport { id, .. } = widget {
                if let Some(out) = outputs.get(id) {
                    self.export_payloads.insert(id.clone(), out.clone());
                } else if let Some(syn) = self.fixture.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        let payload = preview_dict_from_connection(src, &syn.from_port, &syn.to_port);
                        self.export_payloads.insert(id.clone(), payload);
                    }
                }
            }
        }
    }

    pub fn export_payload_json(&self, widget_id: &str) -> Result<String, FlowCoreError> {
        let payload = self.export_payloads.get(widget_id).cloned().unwrap_or_default();
        Ok(serde_json::to_string(&payload)?)
    }

    /// 📤️ Returns and clears a pending export control click from the last pointer hit.
    pub fn take_pending_export_click(&mut self) -> Option<String> {
        self.dag.take_pending_export_click()
    }

    fn sync_dag_display_from_widgets(&mut self) {
        for widget in &self.fixture.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.fixture.nodes.iter_mut().find(|n| n.id == *id) else {
                continue;
            };
            match (widget, &mut node.kind) {
                (Widget::InputSlider { value, .. }, DagNodeKind::Slider { value: dag_value, .. }) => {
                    *dag_value = *value;
                }
                (Widget::InputNote { text, .. }, DagNodeKind::Note { text: dag_text, .. }) => {
                    *dag_text = text.clone();
                }
                (Widget::InputImage { src, .. }, DagNodeKind::Image { src: dag_src, .. }) => {
                    *dag_src = src.clone();
                }
                (Widget::OutputPreview { preview, expanded, .. }, DagNodeKind::Preview { content, expanded: dag_expanded, .. }) => {
                    *content = dag_preview_content_from_dict(preview);
                    *dag_expanded = expanded.iter().cloned().collect();
                }
                (Widget::OutputAction { action, .. }, DagNodeKind::Action { label, .. }) => {
                    *label = action.clone();
                }
                (Widget::OutputExport { format, .. }, DagNodeKind::Export { label, format: dag_format, .. }) => {
                    *label = format.to_uppercase();
                    *dag_format = format.clone();
                }
                _ => {}
            }
        }
    }

    pub(crate) fn sync_dag_ghost(&mut self) {
        self.dag.set_ghost_node(self.ghost_node.clone());
    }

    fn rebuild_dag(&mut self) {
        let fixture = self.build_dag_fixture_v1();
        let theme = self.dag.canvas_theme;
        let automatic_lod = self.dag.automatic_lod();
        let forced_draw_lod = self.dag.forced_draw_lod_label().map(str::to_string);
        let ghost = self.ghost_node.clone();
        self.dag.replace_fixture_without_layout(fixture);
        self.dag.canvas_theme = theme;
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.set_automatic_lod(automatic_lod);
        if let Some(label) = forced_draw_lod {
            self.dag.set_forced_draw_lod_label(&label);
        }
        self.dag.set_ghost_node(ghost);
        self.dag.set_minimap_widget_visible(true);
        self.sync_preview_dimmed();
        self.sync_from_dag();
    }

    fn sync_preview_dimmed(&mut self) {
        let off = self.preview_off_widget_ids();
        self.dag.set_dimmed(&off);
    }

    /// 🎯️ Selected widget ids as JSON array (legacy — prefer {@link selection_domains_json}).
    pub fn selected_widget_ids_json(&self) -> String {
        serde_json::to_string(&self.dag.selected_node_ids()).unwrap_or_else(|_| "[]".into())
    }

    /// 🎯️ Full selection snapshot as JSON (`nodes`, `edges`, `handles`).
    pub fn selection_domains_json(&self) -> String {
        self.dag.selection_domains_json()
    }

    /// 🖱️ Hovered widget id when the pointer is over a node or port handle.
    pub fn hovered_widget_id(&self) -> Option<String> {
        self.dag.hovered_node_id()
    }

    /// @emoji 🎯️ All pick targets under a screen point as JSON for DOM disambiguation menus.
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        self.dag.pick_targets_at_screen_json(sx, sy)
    }

    /// @emoji 🎯️ Screen-space geometry for a live entity (`domain`/`id` in the pick-target grammar) —
    /// see `DagHost::entity_screen_json`. Powers introduction-demonstration semantic targeting.
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        self.dag.entity_screen_json(domain, id)
    }

    /// 🔌️ Hovered widget channel when the pointer is over a port row or handle.
    pub fn hovered_channel_json(&self) -> String {
        self.dag.hovered_channel_json()
    }

    /// 🔌️ Selected widget channels from handle picks.
    pub fn selected_channels_json(&self) -> String {
        self.dag.selected_channels_json()
    }

    /// ✅️ Replaces selection from domain JSON or a legacy widget-id array.
    pub fn set_selection_json(&mut self, json: &str) {
        self.dag.set_selection_domains_json(json);
    }

    /// ✅️ Same as `set_selection_json` but over a flat node-id list (the `NodeGraphScene.selection` wire shape).
    pub fn set_selection(&mut self, ids: &[String]) {
        let json = serde_json::json!({ "nodes": ids, "edges": [], "handles": [] }).to_string();
        self.dag.set_selection_domains_json(&json);
    }

    /// 📦️ Screen-space union bounds of the current selection for DOM overlays.
    pub fn selection_union_bounds_screen_json(&self) -> String {
        self.dag.selection_union_bounds_screen_json()
    }

    /// 📐️ Aligns or distributes the current multi-node selection.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        self.dag.align_selection(mode)?;
        self.sync_from_dag();
        Ok(())
    }

    /// 🖱️ Sets hover to a widget id, or clears hover.
    pub fn set_hover(&mut self, widget_id: Option<&str>) {
        self.dag.set_hover(widget_id);
    }

    /// 🔌️ Sets hover to a widget channel, or clears hover.
    pub fn set_hover_channel(&mut self, widget_id: Option<&str>, port_id: Option<&str>) {
        self.dag.set_hover_channel(widget_id, port_id);
    }

    /// 🔌️ Replaces channel selection from JSON.
    pub fn set_selected_channels_json(&mut self, json: &str) {
        self.dag.set_selected_channels_json(json);
    }

    /// 🌫️ Widget ids with preview disabled.
    pub fn preview_off_widget_ids(&self) -> Vec<String> {
        self.fixture
            .widgets
            .iter()
            .filter_map(|widget| match widget {
                Widget::Neuron { id, preview: false, .. } => Some(id.clone()),
                _ => None,
            })
            .collect()
    }

    /// 🌫️ Sets preview-off neurons from a JSON array of widget ids.
    pub fn set_preview_off_json(&mut self, json: &str) {
        let ids: Vec<String> = serde_json::from_str(json).unwrap_or_default();
        for widget in &mut self.fixture.widgets {
            if let Widget::Neuron { id, preview, .. } = widget {
                *preview = !ids.contains(id);
            }
        }
        self.sync_preview_dimmed();
    }

    /// 👁️ Toggles preview on a neuron widget.
    pub fn toggle_preview(&mut self, widget_id: &str) -> Result<(), FlowCoreError> {
        let Some(widget) = self.fixture.widgets.iter_mut().find(|w| widget_id_for(w) == widget_id) else {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        };
        let Widget::Neuron { preview, .. } = widget else {
            return Err(FlowCoreError::WidgetNotNeuron(widget_id.to_string()));
        };
        *preview = !*preview;
        self.sync_preview_dimmed();
        Ok(())
    }

    fn sync_from_dag(&mut self) {
        let dag_ids: BTreeSet<String> = self.dag.fixture.nodes.iter().map(|node| node.id.clone()).collect();
        self.fixture.widgets.retain(|widget| dag_ids.contains(widget_id_for(widget)));
        for node in &self.dag.fixture.nodes {
            self.fixture.layout.insert(node.id.clone(), WidgetLayout { x: node.x, y: node.y });
        }
        for widget in &mut self.fixture.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.fixture.nodes.iter().find(|n| n.id == *id) else {
                continue;
            };
            match (widget, &node.kind) {
                (Widget::InputSlider { value, .. }, DagNodeKind::Slider { value: dag_value, .. }) => {
                    *value = *dag_value;
                }
                (Widget::InputNote { text, .. }, DagNodeKind::Note { text: dag_text, .. }) => {
                    *text = dag_text.clone();
                }
                (Widget::InputImage { src, .. }, DagNodeKind::Image { src: dag_src, .. }) => {
                    *src = dag_src.clone();
                }
                (Widget::OutputPreview { expanded, .. }, DagNodeKind::Preview { expanded: dag_expanded, .. }) => {
                    std::mem::replace(expanded, dag_expanded.iter().cloned().collect()).retire_cold();
                }
                (Widget::OutputAction { action, .. }, DagNodeKind::Action { label, .. }) => {
                    *action = label.clone();
                }
                (Widget::OutputExport { format, .. }, DagNodeKind::Export { format: dag_format, .. }) => {
                    *format = dag_format.clone();
                }
                _ => {}
            }
        }
        self.fixture.synapses = self
            .dag
            .fixture
            .edges
            .iter()
            .map(|edge| {
                let (from, from_port) = parse_port_endpoint(&edge.source, "");
                let (to, to_port) = parse_port_endpoint(&edge.target, "");
                SynapseSpec { id: edge.id.clone(), from, to, from_port, to_port }
            })
            .collect();
        self.fixture.camera = CameraJson { x: self.dag.fixture.camera.x, y: self.dag.fixture.camera.y, zoom: self.dag.fixture.camera.zoom };
    }

    fn build_dag_fixture_v1(&self) -> DagFixture {
        let mut seen = BTreeSet::new();
        let nodes: Vec<DagNodeSpec> =
            self.fixture.widgets.iter().enumerate().filter(|(_, widget)| seen.insert(widget_id_for(widget).to_string())).map(|(i, w)| widget_to_dag_node(w, i, &self.fixture.layout, &self.fixture.synapses, &self.kind_infos)).collect();
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        let edges: Vec<DagFixtureEdge> = self
            .fixture
            .synapses
            .iter()
            .filter(|syn| !would_create_cycle(&existing.iter().filter(|(a, b)| !(a == &syn.from && b == &syn.to)).cloned().collect::<Vec<_>>(), &syn.from, &syn.to))
            .map(|syn| DagFixtureEdge { id: syn.id.clone(), source: format!("{}@{}", syn.from, syn.from_port), target: format!("{}@{}", syn.to, syn.to_port), route_style: EdgeRouteStyle::default(), properties: PropertyBag::new() })
            .collect();
        DagFixture { schema: "dag.fixture".into(), camera: dag::DagCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom }, nodes, edges }
    }

    fn screen_to_world_point(&self, sx: f64, sy: f64) -> canvas::Point {
        use canvas::camera::{screen_to_world, Camera, Viewport};
        use canvas::Point;
        let cam = Camera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn next_widget_id(&mut self, descriptor: &WidgetDescriptor) -> String {
        self.next_widget_serial += 1;
        let prefix = match descriptor {
            WidgetDescriptor::Neuron { neuron_kind, .. } => neuron_kind.replace('.', "_"),
            WidgetDescriptor::InputSlider { .. } => "slider".into(),
            WidgetDescriptor::InputNote { .. } => "note".into(),
            WidgetDescriptor::InputImage { .. } => "image".into(),
            WidgetDescriptor::Variable { .. } => "variable".into(),
            WidgetDescriptor::OutputPreview { .. } => "preview".into(),
            WidgetDescriptor::OutputAction { .. } => "action".into(),
            WidgetDescriptor::OutputExport { .. } => "export".into(),
        };
        format!("{prefix}_{}", self.next_widget_serial)
    }

    pub fn set_slider_value(&mut self, widget_id: &str, value: f64) {
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::InputSlider { id, .. } = widget {
                if id == widget_id {
                    crate::set_widget_slider_value(widget, value);
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.refresh_computing_chrome_from_pending();
    }

    pub fn slider_overlay_state_json(&self) -> Result<String, FlowCoreError> {
        Ok(self.dag.slider_overlay_state_json()?)
    }

    pub fn set_note_text(&mut self, widget_id: &str, text: &str) {
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::InputNote { id, text: note } = widget {
                if id == widget_id {
                    *note = text.to_string();
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.dag.fit_note_sizes();
        self.refresh_computing_chrome_from_pending();
    }

    /// ✏️ Begins inline note editing for a widget at a world-space click.
    pub fn begin_note_edit(&mut self, widget_id: &str, world_x: f64, world_y: f64) {
        self.begin_gesture();
        self.dag.begin_note_edit(widget_id, world_x, world_y);
    }

    /// ✏️ Inserts text into the active note editor.
    pub fn note_insert_text(&mut self, chunk: &str) {
        if !self.dag.note_insert_text(chunk) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Backspaces in the active note editor.
    pub fn note_backspace(&mut self) {
        if !self.dag.note_backspace() {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Deletes forward in the active note editor.
    pub fn note_delete_forward(&mut self) {
        if !self.dag.note_delete_forward() {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Moves the active note caret.
    pub fn note_move_caret(&mut self, direction: &str, extend: bool) {
        if !self.dag.note_move_caret(direction, extend) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Commits inline note editing into fixture history.
    pub fn note_commit_edit(&mut self) {
        self.dag.note_commit_edit();
        self.sync_from_dag();
        self.commit_gesture_history();
    }

    /// ✏️ Toggles native caret visibility while editing a note.
    pub fn set_note_caret_visible(&mut self, visible: bool) {
        self.dag.set_note_caret_visible(visible);
    }

    pub fn schemas_json(&self) -> Result<String, FlowCoreError> {
        let refs = flow_registry().schema_refs();
        Ok(serde_json::to_string(&refs)?)
    }

    pub fn set_variable_name(&mut self, widget_id: &str, name: &str) {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::Variable { id, name: variable_name, .. } = widget {
                if id == widget_id {
                    *variable_name = trimmed.to_string();
                }
            }
        }
        self.rebuild_dag();
    }

    pub fn set_variable_schema(&mut self, widget_id: &str, schema: &str) {
        let trimmed = schema.trim();
        if trimmed.is_empty() {
            return;
        }
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::Variable { id, schema: variable_schema, .. } = widget {
                if id == widget_id {
                    *variable_schema = trimmed.to_string();
                }
            }
        }
        self.rebuild_dag();
    }

    pub fn set_image_src(&mut self, widget_id: &str, src: &str) {
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::InputImage { id, src: image } = widget {
                if id == widget_id {
                    *image = src.to_string();
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.dag.fit_preview_sizes();
        self.refresh_computing_chrome_from_pending();
    }

    pub fn preview_text(&self) -> String {
        self.fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::OutputPreview { preview, .. } => Some(preview_content_summary(&dag_preview_content_from_dict(preview))),
                _ => None,
            })
            .unwrap_or_else(|| "—".into())
    }

    pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), FlowCoreError> {
        Ok(self.dag.set_canvas_theme_from_json(json)?)
    }

    pub fn set_canvas_theme_dark(&mut self, dark: bool) {
        self.dag.canvas_theme = dag::CanvasPalette::from_board_palette(if dark { &ui_styling::BOARD_DARK } else { &ui_styling::BOARD_LIGHT });
    }

    pub fn paint_scene(&self, scene: &mut canvas::Scene, width: u32, height: u32, dpr: f64) {
        self.dag.paint_scene(scene, width, height, dpr);
    }

    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.dag.set_automatic_lod(enabled);
    }

    pub fn set_proximity_distance(&mut self, world: f64) {
        self.dag.set_proximity_distance(world);
    }

    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        self.dag.set_forced_draw_lod_label(label);
    }

    pub fn set_grid_visible(&mut self, visible: bool) {
        self.dag.set_grid_visible(visible);
    }

    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.dag.set_grid_snap_enabled(enabled);
    }

    pub fn set_grid_factor(&mut self, factor: f64) -> Result<(), FlowCoreError> {
        self.dag.set_grid_factor(factor)?;
        Ok(())
    }

    pub fn focus_selection_camera(&self, pad: f64) -> Option<CameraJson> {
        self.dag.focus_selection_camera(pad).map(|camera| CameraJson { x: camera.x, y: camera.y, zoom: camera.zoom })
    }

    pub fn draw_lod_label(&self) -> &'static str {
        self.dag.draw_lod_label()
    }

    pub fn label_overlay_paint_state_json(&self) -> Result<String, FlowCoreError> {
        Ok(self.dag.label_overlay_paint_state_json()?)
    }

    /// 💥️ Returns and clears a pending cluster explode target from the last pointer hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.dag.take_pending_cluster_explode()
    }

    /// 🧩️ Collapses the selected widgets into one cluster neuron.
    pub fn collapse_selection(&mut self, selected_ids: &[String]) -> Result<String, FlowCoreError> {
        if selected_ids.len() < 2 {
            return Err(FlowCoreError::CollapseNeedsTwoWidgets);
        }
        let selected: BTreeSet<String> = selected_ids.iter().cloned().collect();
        if !selected.iter().all(|id| self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == id)) {
            return Err(FlowCoreError::CollapseUnknownWidgets);
        }
        if selected.iter().any(|id| self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == id && matches!(widget, Widget::Cluster { .. }))) {
            return Err(FlowCoreError::CollapseContainsClusters);
        }
        self.begin_change();
        let mut crossing_external = Vec::new();
        for synapse in &self.fixture.synapses {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if (from_selected || to_selected) && !(from_selected && to_selected) {
                crossing_external.push(synapse.clone());
            }
        }
        let boundary_variables = boundary_variable_widget_ids(&selected, &crossing_external, &self.fixture.widgets);
        let mut inner_neurons = Vec::new();
        let mut inner_layout = BTreeMap::new();
        for widget in &self.fixture.widgets {
            let id = widget_id_for(widget).to_string();
            if !selected.contains(&id) {
                continue;
            }
            if boundary_variables.contains(&id) {
                continue;
            }
            if let Some(neuron) = widget_to_inner_neuron(widget) {
                inner_neurons.push(neuron);
            }
            if let Some(layout) = self.fixture.layout.get(&id) {
                inner_layout.insert(id, layout.clone());
            }
        }
        let mut inner_synapses = Vec::new();
        let mut retained_external = Vec::new();
        for synapse in &self.fixture.synapses {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if from_selected && to_selected {
                if boundary_variables.contains(&synapse.from) || boundary_variables.contains(&synapse.to) {
                    continue;
                }
                inner_synapses.push(Synapse { id: synapse.id.clone(), from: synapse.from.clone(), to: synapse.to.clone(), from_port: synapse.from_port.clone(), to_port: synapse.to_port.clone() });
            } else if from_selected || to_selected {
            } else {
                retained_external.push(synapse.clone());
            }
        }
        let mut used_channels = BTreeSet::new();
        let mut input_serial = 0usize;
        let mut output_serial = 0usize;
        let mut boundary_index = 0usize;
        let mut cluster_external = Vec::new();
        let outputs = self.outputs.clone();
        let kind_infos = self.kind_infos.clone();
        let widgets = self.fixture.widgets.clone();
        let synapses_snapshot = self.fixture.synapses.clone();
        for synapse in crossing_external {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if to_selected && !from_selected {
                let inner_target = if boundary_variables.contains(&synapse.to) {
                    self.fixture.synapses.iter().find(|entry| entry.from == synapse.to && selected.contains(&entry.to)).map(|entry| (entry.to.clone(), entry.to_port.clone())).unwrap_or_else(|| (synapse.to.clone(), synapse.to_port.clone()))
                } else {
                    (synapse.to.clone(), synapse.to_port.clone())
                };
                let (channel, schema) = if let Some((name, schema)) = variable_widget_meta(&widgets, &synapse.to) {
                    let schema = if schema.is_empty() { infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &synapse.from, &synapse.from_port) } else { schema };
                    (name, schema)
                } else {
                    let channel = unique_generated_boundary_name("input", &mut input_serial, &used_channels);
                    let schema = infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &synapse.from, &synapse.from_port);
                    (channel, schema)
                };
                used_channels.insert(channel.clone());
                boundary_index += 1;
                let boundary_id = format!("__in_{boundary_index}");
                inner_neurons.push(Neuron::with_kind(&boundary_id, INPUT_KIND, contract_boundary_params(&channel, &schema)));
                inner_synapses.push(Synapse { id: format!("{boundary_id}_link"), from: boundary_id, to: inner_target.0, from_port: String::new(), to_port: inner_target.1 });
                cluster_external.push(SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: String::new(), from_port: synapse.from_port.clone(), to_port: channel });
            } else if from_selected && !to_selected {
                let inner_source = if boundary_variables.contains(&synapse.from) {
                    self.fixture.synapses.iter().find(|entry| entry.to == synapse.from && selected.contains(&entry.from)).map(|entry| (entry.from.clone(), entry.from_port.clone())).unwrap_or_else(|| (synapse.from.clone(), synapse.from_port.clone()))
                } else {
                    (synapse.from.clone(), synapse.from_port.clone())
                };
                let (channel, schema) = if let Some((name, schema)) = variable_widget_meta(&widgets, &synapse.from) {
                    let schema = if schema.is_empty() { infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &inner_source.0, &inner_source.1) } else { schema };
                    (name, schema)
                } else {
                    let channel = unique_generated_boundary_name("output", &mut output_serial, &used_channels);
                    let schema = infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &inner_source.0, &inner_source.1);
                    (channel, schema)
                };
                used_channels.insert(channel.clone());
                boundary_index += 1;
                let boundary_id = format!("__out_{boundary_index}");
                inner_neurons.push(Neuron::with_kind(&boundary_id, OUTPUT_KIND, contract_boundary_params(&channel, &schema)));
                inner_synapses.push(Synapse { id: format!("{boundary_id}_link"), from: inner_source.0, to: boundary_id, from_port: inner_source.1, to_port: String::new() });
                cluster_external.push(SynapseSpec { id: synapse.id.clone(), from: String::new(), to: synapse.to.clone(), from_port: channel, to_port: synapse.to_port.clone() });
            }
        }
        let (sum_x, sum_y, layout_count) = selected.iter().filter_map(|id| self.fixture.layout.get(id)).fold((0.0, 0.0, 0usize), |(sx, sy, count), layout| (sx + layout.x, sy + layout.y, count + 1));
        let count = layout_count.max(1) as f64;
        let cluster_x = sum_x / count;
        let cluster_y = sum_y / count;
        self.next_widget_serial += 1;
        let cluster_id = format!("cluster_{}", self.next_widget_serial);
        let inner_tree = Tree { neurons: inner_neurons, synapses: inner_synapses };
        let cluster = Widget::Cluster {
            id: cluster_id.clone(),
            name: "Cluster".into(),
            tree: inner_tree,
            flow: FlowGui { camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: inner_layout.into_iter().map(|(id, layout)| (id, FlowNodeGui { layout, chrome: NodeChrome::Plain { preview: true } })).collect(), previews: vec![] },
        };
        self.fixture.widgets.retain(|widget| !selected.contains(widget_id_for(widget)));
        self.fixture.widgets.push(cluster);
        for id in &selected {
            self.fixture.layout.remove(id);
        }
        self.fixture.layout.insert(cluster_id.clone(), WidgetLayout { x: cluster_x, y: cluster_y });
        self.fixture.synapses = retained_external;
        for synapse in cluster_external {
            if synapse.to.is_empty() {
                self.fixture.synapses.push(SynapseSpec { id: synapse.id, from: synapse.from, to: cluster_id.clone(), from_port: synapse.from_port, to_port: synapse.to_port });
            } else {
                self.fixture.synapses.push(SynapseSpec { id: synapse.id, from: cluster_id.clone(), to: synapse.to, from_port: synapse.from_port, to_port: synapse.to_port });
            }
        }
        self.rebuild_dag();
        Ok(cluster_id)
    }

    /// 💥️ Explodes a cluster back into its inner widgets.
    pub fn explode_cluster(&mut self, cluster_id: &str) -> Result<(), FlowCoreError> {
        let cluster_index = self.fixture.widgets.iter().position(|widget| matches!(widget, Widget::Cluster { id, .. } if id == cluster_id)).ok_or_else(|| FlowCoreError::UnknownCluster(cluster_id.to_string()))?;
        let Widget::Cluster { tree, flow, .. } = self.fixture.widgets[cluster_index].clone() else {
            return Err(FlowCoreError::WidgetNotCluster(cluster_id.to_string()));
        };
        let cluster_layout = self.fixture.layout.get(cluster_id).cloned().unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
        self.begin_change();
        let mut boundary_channels: HashMap<String, (String, String)> = HashMap::new();
        for neuron in &tree.neurons {
            if neuron.kind == INPUT_KIND {
                let channel = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
                boundary_channels.insert(channel.clone(), (format!("{cluster_id}/{}", neuron.id), channel));
            } else if neuron.kind == OUTPUT_KIND {
                let channel = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
                boundary_channels.insert(channel.clone(), (format!("{cluster_id}/{}", neuron.id), channel));
            }
        }
        let mut restored_widgets = Vec::new();
        for neuron in &tree.neurons {
            let namespaced_id = format!("{cluster_id}/{}", neuron.id);
            if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                let widget = neuron_to_exploded_widget(neuron);
                let widget = match widget {
                    Widget::Variable { name, schema, .. } => Widget::Variable { id: namespaced_id.clone(), name, schema },
                    other => other,
                };
                let layout = flow.nodes.get(&neuron.id).map(|node| node.layout.clone()).unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
                self.fixture.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
                restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
                continue;
            }
            let mut widget = neuron_to_exploded_widget(neuron);
            match &mut widget {
                Widget::Neuron { id, .. } | Widget::InputSlider { id, .. } | Widget::InputNote { id, .. } | Widget::InputImage { id, .. } | Widget::Variable { id, .. } => *id = namespaced_id.clone(),
                _ => {}
            }
            let layout = flow.nodes.get(&neuron.id).map(|node| node.layout.clone()).unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
            self.fixture.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
            restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
        }
        let id_map: HashMap<String, String> = restored_widgets.iter().map(|(namespaced, original, _)| (original.clone(), namespaced.clone())).collect();
        self.fixture.widgets.remove(cluster_index);
        self.fixture.layout.remove(cluster_id);
        for (_, _, widget) in restored_widgets {
            self.fixture.widgets.push(widget);
        }
        let mut next_synapses = Vec::new();
        for synapse in &self.fixture.synapses {
            if synapse.to == cluster_id {
                if let Some((variable_id, variable_port)) = boundary_channels.get(&synapse.to_port) {
                    next_synapses.push(SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: variable_id.clone(), from_port: synapse.from_port.clone(), to_port: variable_port.clone() });
                    continue;
                }
            } else if synapse.from == cluster_id {
                if let Some((variable_id, variable_port)) = boundary_channels.get(&synapse.from_port) {
                    next_synapses.push(SynapseSpec { id: synapse.id.clone(), from: variable_id.clone(), to: synapse.to.clone(), from_port: variable_port.clone(), to_port: synapse.to_port.clone() });
                    continue;
                }
            } else {
                next_synapses.push(synapse.clone());
            }
        }
        for synapse in &tree.synapses {
            let Some(from) = id_map.get(&synapse.from) else { continue };
            let Some(to) = id_map.get(&synapse.to) else { continue };
            let from_port = tree
                .neurons
                .iter()
                .find(|neuron| neuron.id == synapse.from && neuron.kind == INPUT_KIND)
                .and_then(|neuron| neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()))
                .map(str::to_string)
                .unwrap_or_else(|| synapse.from_port.clone());
            let to_port = tree
                .neurons
                .iter()
                .find(|neuron| neuron.id == synapse.to && neuron.kind == OUTPUT_KIND)
                .and_then(|neuron| neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()))
                .map(str::to_string)
                .unwrap_or_else(|| synapse.to_port.clone());
            self.next_synapse_serial += 1;
            next_synapses.push(SynapseSpec { id: format!("s{}", self.next_synapse_serial), from: from.clone(), to: to.clone(), from_port, to_port });
        }
        self.fixture.synapses = next_synapses;
        self.rebuild_dag();
        Ok(())
    }

    // #region History
    fn content_changed(a: &FlowFixture, b: &FlowFixture) -> bool {
        a.widgets != b.widgets || a.synapses != b.synapses || a.layout != b.layout
    }

    fn history_store_from_baseline(&mut self, baseline: FlowFixture) -> Option<&mut FlowStore> {
        if self.history_store.is_none() {
            let mut store = resolve_ready(FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow-host", baseline, None))).ok()?;
            store.install_member_store_owners_exact(FlowFixture::member_store_owners());
            self.history_store = Some(store);
        }
        self.history_store.as_mut()
    }

    /// 🧾️ Flushes an armed-but-not-yet-recorded discrete mutation into `history_store` as one
    /// invertible `FlowMutation::ReplaceFlowFixture` edit — the standard `crate::os_store::ArtifactStore`/`Mutation`/
    /// `MutationDiff` mechanism (see `🔖️Mutations`) driving undo/redo here instead of the old
    /// hand-rolled `Vec<FlowFixture>` snapshot stack. Unconditional once armed (no `content_changed`
    /// gate), mirroring the old stack's unconditional `past.push` on a discrete `begin_change` — only
    /// the gesture-coalescing path (`commit_gesture_history`) skips a no-op edit.
    fn flush_pending_change(&mut self) {
        if self.pending_change {
            self.pending_change = false;
            let baseline = self.pending_history_baseline.take().unwrap_or_else(|| self.fixture.clone());
            let fixture = self.fixture.clone();
            if let Some(store) = self.history_store_from_baseline(baseline) {
                let _ = resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![FlowMutation::ReplaceFlowFixture(ReplaceFlowFixture { fixture })], description: None }));
            }
        }
    }

    /// ↩️ Arms a checkpoint for the mutation about to happen, unless a gesture (`begin_gesture`) is
    /// currently coalescing several mutations into one.
    pub fn begin_change(&mut self) {
        if !self.gesture_active {
            self.flush_pending_change();
            self.pending_history_baseline = Some(self.fixture.clone());
            self.pending_change = true;
        }
    }

    /// 🖐️ Starts a coalescing gesture (drag, inline note edit): flushes anything already armed first,
    /// then suppresses further `begin_change` checkpoints until `commit_gesture_history`.
    fn begin_gesture(&mut self) {
        self.flush_pending_change();
        self.pending_history_baseline = Some(self.fixture.clone());
        self.gesture_active = true;
    }

    fn commit_gesture_history(&mut self) {
        if self.gesture_active {
            self.gesture_active = false;
            let baseline = self.pending_history_baseline.take().unwrap_or_else(|| self.fixture.clone());
            if Self::content_changed(&baseline, &self.fixture) {
                let fixture = self.fixture.clone();
                if let Some(store) = self.history_store_from_baseline(baseline) {
                    let _ = resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![FlowMutation::ReplaceFlowFixture(ReplaceFlowFixture { fixture })], description: None }));
                }
            }
        }
    }

    /// ↩️ Restores the previous fixture content snapshot, keeping the current camera.
    pub fn undo(&mut self) -> bool {
        self.flush_pending_change();
        let camera = self.fixture.camera.clone();
        let Some(store) = self.history_store.as_mut() else {
            return false;
        };
        if resolve_ready(store.dispatch(ArtifactCommand::Undo)).is_err() {
            return false;
        }
        let Ok(mut restored) = store.snapshot() else {
            return false;
        };
        restored.camera = camera;
        std::mem::replace(&mut self.fixture, restored).retire_cold();
        self.rebuild_dag();
        true
    }

    /// ↪️ Re-applies a fixture content snapshot undone earlier, keeping the current camera.
    pub fn redo(&mut self) -> bool {
        let camera = self.fixture.camera.clone();
        let Some(store) = self.history_store.as_mut() else {
            return false;
        };
        if resolve_ready(store.dispatch(ArtifactCommand::Redo)).is_err() {
            return false;
        }
        let Ok(mut restored) = store.snapshot() else {
            return false;
        };
        restored.camera = camera;
        std::mem::replace(&mut self.fixture, restored).retire_cold();
        self.rebuild_dag();
        true
    }

    /// ↩️ Whether a content undo step is available.
    pub fn can_undo(&self) -> bool {
        self.pending_change || self.history_store.as_ref().is_some_and(|store| !store.applied_edit_ids().is_empty())
    }

    /// ↪️ Whether a content redo step is available.
    pub fn can_redo(&self) -> bool {
        self.history_store.as_ref().is_some_and(|store| !store.redo_edit_ids().is_empty())
    }
    // #endregion History
}

/// 🧹 Incremental exact-owner retirement for one retained flow host.
#[doc(hidden)]
pub struct FlowHostRetirementState {
    fixture: FlowFixture,
    dag: Option<dag::DagHostRetirement>,
    outputs: BTreeMap<String, Dictionary>,
    export_payloads: BTreeMap<String, Dictionary>,
    last_eval_json: String,
    eval_bridge: Option<EvalBridge>,
    host_catalogue_json: String,
    kind_infos: HashMap<String, OperatorInfo>,
    neural_cache: Option<neural::NeuralCacheRetirement>,
    previous_snapshot: Option<TreeSnapshot>,
    previous_channels: Option<EvalChannels>,
    ghost_node: Option<DagNodeSpec>,
    history_store: Option<FlowStore>,
    pending_history_baseline: Option<FlowFixture>,
    pending_extension_eval: Option<neural::PendingExtensionEval>,
    interaction_projection: Option<dag::DagInteractionProjection>,
    domain: crate::retained::FlowRetirement,
    neural: neural::ValueRetirement,
    terminal: bool,
    faulted: bool,
}

/// 🔒️ Host ownership is guarded until every retained field has crossed its close boundary.
pub struct FlowHostRetirement { state: std::mem::ManuallyDrop<FlowHostRetirementState> }
impl std::ops::Deref for FlowHostRetirement { type Target = FlowHostRetirementState; fn deref(&self) -> &Self::Target { &self.state } }
impl std::ops::DerefMut for FlowHostRetirement { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl FlowHostRetirement {
    pub fn new(host: FlowHost) -> Self {
        let FlowHost {
            fixture,
            dag,
            outputs,
            export_payloads,
            last_eval_json,
            eval_bridge,
            host_catalogue_json,
            kind_infos,
            neural_cache,
            previous_snapshot,
            previous_channels,
            next_widget_serial: _,
            next_synapse_serial: _,
            viewport_w: _,
            viewport_h: _,
            viewport_dpr: _,
            pan_anchor: _,
            ghost_node,
            history_store,
            pending_history_baseline,
            pending_change: _,
            gesture_active: _,
            pending_extension_eval,
            interaction_revision: _,
            interaction_projection,
        } = host;
        Self { state: std::mem::ManuallyDrop::new(FlowHostRetirementState {
            fixture,
            dag: Some(dag::DagHostRetirement::new(dag)),
            outputs,
            export_payloads,
            last_eval_json,
            eval_bridge,
            host_catalogue_json,
            kind_infos,
            neural_cache: Some(neural::NeuralCacheRetirement::new(neural_cache)),
            previous_snapshot,
            previous_channels,
            ghost_node,
            history_store,
            pending_history_baseline,
            pending_extension_eval,
            interaction_projection,
            domain: crate::retained::FlowRetirement::default(),
            neural: neural::ValueRetirement::default(),
            terminal: false,
            faulted: false,
        }) }
    }

    pub fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        use crate::os_store::ErasedSnapshotRetirement;
        use crate::retained::FlowOwner;
        let state = &mut *self.state;
        if context.should_yield() || state.faulted { return false; }
        if let Some(dag) = state.dag.as_mut() {
            if dag.close_step() {
                if !dag.terminal_is_empty() { state.faulted = true; return false; }
                state.dag = None;
            }
        } else if !state.domain.is_empty() {
            if state.domain.close_step(1, 4096).is_err() { state.faulted = true; }
        } else if !state.neural.terminal_is_empty() {
            state.neural.close_step(1, 4096);
        } else if let Some(widget) = state.fixture.widgets.pop() {
            state.domain.push(FlowOwner::Widget(widget));
        } else if let Some(synapse) = state.fixture.synapses.pop() {
            state.domain.push(FlowOwner::Specs(vec![synapse]));
        } else if !state.fixture.layout.is_empty() {
            state.domain.push(FlowOwner::Layouts(std::mem::take(&mut state.fixture.layout)));
        } else if !state.fixture.schema.is_empty() {
            state.domain.text(std::mem::take(&mut state.fixture.schema));
        } else if let Some((key, value)) = state.outputs.pop_first() {
            state.neural.text(key); state.neural.push_dictionary(value);
        } else if let Some((key, value)) = state.export_payloads.pop_first() {
            state.neural.text(key); state.neural.push_dictionary(value);
        } else if state.last_eval_json.capacity() != 0 {
            state.domain.text(std::mem::take(&mut state.last_eval_json));
        } else if state.host_catalogue_json.capacity() != 0 {
            state.domain.text(std::mem::take(&mut state.host_catalogue_json));
        } else if let Some((key, value)) = state.kind_infos.extract_if(|_, _| true).next() {
            state.neural.text(key); state.neural.push_operator(value);
        } else if let Some(snapshot) = state.previous_snapshot.take() {
            state.neural.push_snapshot(snapshot);
        } else if let Some(channels) = state.previous_channels.take() {
            state.neural.push_channels(channels);
        } else if let Some(fixture) = state.pending_history_baseline.take() {
            state.domain.push(FlowOwner::Fixture(fixture));
        } else if let Some(pending) = state.pending_extension_eval.take() {
            state.neural.text(pending.extension_id); state.neural.text(pending.operator_id); state.neural.text(pending.input_json);
        } else if state.eval_bridge.take().is_some() || state.ghost_node.take().is_some() || state.interaction_projection.take().is_some() {
        } else if let Some(cache) = state.neural_cache.as_mut() {
            if matches!(cache.close_step(1, 4096), neural::ValueRetirementStep::Complete) {
                if !cache.terminal_nonopaque_is_empty() { state.faulted = true; return false; }
                state.neural_cache = None;
            }
        } else if let Some(store) = state.history_store.as_mut() {
            match store.close_owned_step(1, 4096) {
                Ok(SnapshotRetirementStep::Complete) if store.close_owned_terminal_is_empty() => state.history_store = None,
                Ok(_) => {}, Err(_) => state.faulted = true,
            }
        } else { state.terminal = true; context.consume_fuel(1); return true; }
        context.consume_fuel(1); false
    }

    pub fn terminal_nonopaque_is_empty(&self) -> bool {
        self.terminal
            && !self.faulted
            && self.dag.is_none()
            && self.fixture.widgets.is_empty()
            && self.fixture.synapses.is_empty()
            && self.fixture.layout.is_empty()
            && self.fixture.schema.is_empty()
            && self.outputs.is_empty()
            && self.export_payloads.is_empty()
            && self.last_eval_json.is_empty()
            && self.eval_bridge.is_none()
            && self.host_catalogue_json.is_empty()
            && self.kind_infos.is_empty()
            && self.neural_cache.is_none()
            && self.previous_snapshot.is_none()
            && self.previous_channels.is_none()
            && self.ghost_node.is_none()
            && self.history_store.is_none()
            && self.pending_history_baseline.is_none()
            && self.pending_extension_eval.is_none()
            && self.interaction_projection.is_none()
            && self.domain.is_empty()
            && self.neural.terminal_is_empty()
    }
}

impl Drop for FlowHostRetirement {
    fn drop(&mut self) {
        if !self.terminal_nonopaque_is_empty() { assert!(std::thread::panicking(), "FlowHostRetirement must reach terminal-empty before release"); return; }
        unsafe { std::mem::ManuallyDrop::drop(&mut self.state); }
    }
}

// #region 🔖️EvalSession
#[cfg(test)]
#[path = "🧹️retirement/🧪️component.rs"]
mod session_retirement_tests;

use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

/// ⏱️ Max cache-missed neuron dispatches per `flowEvalTick` — keeps one dispatch from blocking the worker while still converging small graphs in a single tick.
pub const FLOW_EVAL_TICK_STEP_BUDGET: usize = 512;

static FLOW_SESSION_GEOMETRY: LazyLock<Mutex<HashMap<u64, BTreeSet<String>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static NEXT_FLOW_SESSION_ID: AtomicU64 = AtomicU64::new(1);

fn sync_flow_geometry_retention() {
    let merged: HashSet<String> = FLOW_SESSION_GEOMETRY.lock().map(|entries| entries.values().flat_map(|set| set.iter().cloned()).collect()).unwrap_or_default();
    let live: Vec<String> = merged.into_iter().collect();
    crate::retain_geometry_handles(&live);
}

/// 🚦 Per-widget evaluation state for flow graph chrome (not persisted in config).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum NodeEvalStatus {
    Ok,
    Stale,
    Queued,
    Computing,
    Error { message: String },
    Blocked { ports: Vec<String> },
}

/// 🧵️ In-process evaluation session: neural cache, incremental baseline, eval output, and status — one per app instance, never serialized.
#[doc(hidden)]
pub struct FlowEvalSessionState {
    session_id: u64,
    neural_cache: Option<Arc<NeuralCache>>,
    previous_snapshot: Option<TreeSnapshot>,
    previous_channels: Option<EvalChannels>,
    eval_json: String,
    status_json: String,
    tick_scheduled: bool,
    live_geometry_handles: BTreeSet<String>,
    /// 🧊 Tessellated preview meshes keyed by geometry handle — filled via extension `tessellate`
    /// because runtime-installable brep owns the kernel that minted the handles.
    preview_mesh_json_by_handle: BTreeMap<String, String>,
    /// ⏳ In-flight tessellate requests keyed by `nodeHash` forwarded through `InvokeExtension`.
    pending_tessellate_by_hash: BTreeMap<u64, String>,
    retiring_cache: Option<neural::NeuralCacheRetirement>,
    retirement: neural::ValueRetirement,
    retiring_collections: std::collections::LinkedList<SessionCollectionOwner>,
    closing: bool,
}

enum SessionCollectionOwner { Handles(BTreeSet<String>), Meshes(BTreeMap<String, String>), Pending(BTreeMap<u64, String>) }

/// 🔒️ Evaluation ownership stays guarded until every collection, domain, cache, and byte frontier is empty.
pub struct FlowEvalSession { state: std::mem::ManuallyDrop<FlowEvalSessionState> }
impl std::ops::Deref for FlowEvalSession { type Target = FlowEvalSessionState; fn deref(&self) -> &Self::Target { &self.state } }
impl std::ops::DerefMut for FlowEvalSession { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for FlowEvalSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FlowEvalSession {
    fn drop(&mut self) {
        if !self.terminal_is_empty() { assert!(std::thread::panicking(), "FlowEvalSession must finish explicit close before drop"); return; }
        unsafe { std::mem::ManuallyDrop::drop(&mut self.state); }
    }
}

impl FlowEvalSession {
    pub fn new() -> Self {
        Self { state: std::mem::ManuallyDrop::new(FlowEvalSessionState {
            session_id: NEXT_FLOW_SESSION_ID.fetch_add(1, AtomicOrdering::Relaxed),
            neural_cache: Some(Arc::new(NeuralCache::new())),
            previous_snapshot: None,
            previous_channels: None,
            eval_json: String::new(),
            status_json: "{}".into(),
            tick_scheduled: false,
            live_geometry_handles: BTreeSet::new(),
            preview_mesh_json_by_handle: BTreeMap::new(),
            pending_tessellate_by_hash: BTreeMap::new(),
            retiring_cache: None,
            retirement: neural::ValueRetirement::default(),
            retiring_collections: std::collections::LinkedList::new(),
            closing: false,
        }) }
    }

    pub fn neural_cache(&self) -> Arc<NeuralCache> {
        self.neural_cache.as_ref().expect("live Flow evaluation session owns its neural cache").clone()
    }

    pub fn install_baseline_into(&self, host: &mut FlowHost) {
        host.install_eval_baseline(self.previous_snapshot.clone(), self.previous_channels.clone());
    }

    pub fn capture_baseline_from(&mut self, host: &FlowHost) {
        let (snapshot, channels) = host.eval_baseline();
        let state = &mut *self.state;
        if let Some(previous) = std::mem::replace(&mut state.previous_snapshot, snapshot) { state.retirement.push_snapshot(previous); }
        if let Some(previous) = std::mem::replace(&mut state.previous_channels, channels) { state.retirement.push_channels(previous); }
        if let Some(channels) = state.previous_channels.as_ref() {
            let next = collect_live_geometry_handles_from_channels(channels).into_iter().collect();
            state.retiring_collections.push_back(SessionCollectionOwner::Handles(std::mem::replace(&mut state.live_geometry_handles, next)));
            if let Ok(mut map) = FLOW_SESSION_GEOMETRY.lock() {
                if let Some(previous) = map.insert(state.session_id, state.live_geometry_handles.clone()) { state.retiring_collections.push_back(SessionCollectionOwner::Handles(previous)); }
            }
            sync_flow_geometry_retention();
        }
    }

    pub fn sync(&mut self, host: &FlowHost) -> bool {
        let remaining = host.pending_eval_widget_ids();
        self.status_json = build_flow_status_json(host, &remaining);
        if remaining.is_empty() {
            return false;
        }
        if self.tick_scheduled {
            return false;
        }
        self.tick_scheduled = true;
        true
    }

    pub fn tick(&mut self, host: &mut FlowHost) -> bool {
        let remaining = host.evaluate_step(FLOW_EVAL_TICK_STEP_BUDGET);
        self.eval_json = host.last_eval_json.clone();
        self.status_json = build_flow_status_json(host, &remaining);
        if remaining.is_empty() {
            self.capture_baseline_from(host);
        }
        self.tick_scheduled = !remaining.is_empty();
        self.tick_scheduled
    }

    pub fn eval_json(&self) -> &str {
        &self.eval_json
    }

    pub fn status_json(&self) -> &str {
        &self.status_json
    }

    pub fn status_json_for_host(&self, host: &FlowHost) -> String {
        let remaining = host.pending_eval_widget_ids();
        build_flow_status_json(host, &remaining)
    }

    pub fn set_eval_json(&mut self, eval_json: String) {
        let state = &mut *self.state;
        state.retirement.text(std::mem::replace(&mut state.eval_json, eval_json));
        state.tick_scheduled = false;
        if let Some(previous) = state.previous_snapshot.take() { state.retirement.push_snapshot(previous); }
        if let Some(previous) = state.previous_channels.take() { state.retirement.push_channels(previous); }
        state.retirement.text(std::mem::replace(&mut state.status_json, "{}".into()));
        state.retiring_collections.push_back(SessionCollectionOwner::Handles(std::mem::take(&mut state.live_geometry_handles)));
        state.retiring_collections.push_back(SessionCollectionOwner::Meshes(std::mem::take(&mut state.preview_mesh_json_by_handle)));
        state.retiring_collections.push_back(SessionCollectionOwner::Pending(std::mem::take(&mut state.pending_tessellate_by_hash)));
        if let Ok(mut map) = FLOW_SESSION_GEOMETRY.lock() {
            if let Some(previous) = map.remove(&state.session_id) { state.retiring_collections.push_back(SessionCollectionOwner::Handles(previous)); }
        }
        sync_flow_geometry_retention();
    }

    pub fn pending(&self) -> bool {
        self.tick_scheduled
    }

    pub fn seed_node_cache(&self, node_hash: u64, output_json: &str) -> Result<(), FlowCoreError> {
        let cache = self.neural_cache.as_deref().expect("live Flow evaluation session owns its neural cache");
        seed_flow_eval_node_cache(cache, node_hash, output_json)
    }

    /// 🧊 Preview mesh JSON previously resolved through the owning geometry extension.
    pub fn preview_mesh_json(&self, handle: &str) -> Option<&str> {
        self.preview_mesh_json_by_handle.get(handle).map(String::as_str)
    }

    /// 🧹 Drops preview meshes/pending tessellates whose handles are no longer live.
    pub fn retain_preview_meshes(&mut self, live_handles: &HashSet<String>) {
        self.preview_mesh_json_by_handle.retain(|handle, _| live_handles.contains(handle));
        self.pending_tessellate_by_hash.retain(|_, handle| live_handles.contains(handle));
    }

    /// 📨 Notes an in-flight tessellate; returns true when the caller should emit `InvokeExtension`.
    pub fn note_pending_tessellate(&mut self, node_hash: u64, handle: String) -> bool {
        if let Some(json) = self.preview_mesh_json_by_handle.get(&handle) {
            if preview_mesh_json_has_geometry(json) {
                return false;
            }
            self.preview_mesh_json_by_handle.remove(&handle);
        }
        if self.pending_tessellate_by_hash.values().any(|pending| pending == &handle) {
            return false;
        }
        self.pending_tessellate_by_hash.insert(node_hash, handle);
        true
    }

    /// ✅ Stores a tessellate response under the handle recorded for `node_hash`.
    pub fn resolve_preview_tessellate(&mut self, node_hash: u64, output_json: &str) -> bool {
        let Some(handle) = self.pending_tessellate_by_hash.remove(&node_hash) else {
            return false;
        };
        if !preview_mesh_json_has_geometry(output_json) {
            return false;
        }
        self.preview_mesh_json_by_handle.insert(handle, output_json.to_string());
        true
    }

    /// 🧹 Begins exact incremental retirement of this instance-owned evaluation session.
    pub fn begin_close(&mut self) {
        self.closing = true;
        self.tick_scheduled = false;
        if let Ok(mut map) = FLOW_SESSION_GEOMETRY.lock() {
            if let Some(previous) = map.remove(&self.session_id) { self.retiring_collections.push_back(SessionCollectionOwner::Handles(previous)); }
        }
        sync_flow_geometry_retention();
    }

    /// 📄 Releases at most one retained owner under the caller's close-page grant.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep as Step;
        let state = &mut *self.state;
        if !state.closing || maximum_items == 0 || maximum_bytes == 0 { return Step::Blocked; }
        if !state.retirement.terminal_is_empty() {
            return match state.retirement.close_step(maximum_items, maximum_bytes) {
                neural::ValueRetirementStep::Pending { released_items, released_bytes } => Step::Pending { released_items, released_bytes },
                neural::ValueRetirementStep::Complete => Step::Pending { released_items: 1, released_bytes: 0 },
                neural::ValueRetirementStep::Blocked => Step::Blocked,
            };
        }
        if let Some(owner) = state.retiring_collections.pop_front() {
            match owner {
                SessionCollectionOwner::Handles(mut values) => {
                    if let Some(value) = values.pop_first() { state.retirement.text(value); }
                    if !values.is_empty() { state.retiring_collections.push_front(SessionCollectionOwner::Handles(values)); }
                }
                SessionCollectionOwner::Meshes(mut values) => {
                    if let Some((key, value)) = values.pop_first() { state.retirement.text(key); state.retirement.text(value); }
                    if !values.is_empty() { state.retiring_collections.push_front(SessionCollectionOwner::Meshes(values)); }
                }
                SessionCollectionOwner::Pending(mut values) => {
                    if let Some((_, value)) = values.pop_first() { state.retirement.text(value); }
                    if !values.is_empty() { state.retiring_collections.push_front(SessionCollectionOwner::Pending(values)); }
                }
            }
            return Step::Pending { released_items: 1, released_bytes: 0 };
        }
        if !state.preview_mesh_json_by_handle.is_empty() {
            state.retiring_collections.push_back(SessionCollectionOwner::Meshes(std::mem::take(&mut state.preview_mesh_json_by_handle)));
        } else if !state.pending_tessellate_by_hash.is_empty() {
            state.retiring_collections.push_back(SessionCollectionOwner::Pending(std::mem::take(&mut state.pending_tessellate_by_hash)));
        } else if !state.live_geometry_handles.is_empty() {
            state.retiring_collections.push_back(SessionCollectionOwner::Handles(std::mem::take(&mut state.live_geometry_handles)));
        } else if let Some(snapshot) = state.previous_snapshot.take() {
            state.retirement.push_snapshot(snapshot);
        } else if let Some(channels) = state.previous_channels.take() {
            state.retirement.push_channels(channels);
        } else if state.eval_json.capacity() != 0 {
            state.retirement.text(std::mem::take(&mut state.eval_json));
        } else if state.status_json.capacity() != 0 {
            state.retirement.text(std::mem::take(&mut state.status_json));
        } else if let Some(cache) = state.neural_cache.take() {
            state.retiring_cache = Some(neural::NeuralCacheRetirement::new(cache));
        } else if let Some(cache) = state.retiring_cache.as_mut() {
            match cache.close_step(maximum_items, maximum_bytes) {
                neural::ValueRetirementStep::Pending { released_items, released_bytes } => return Step::Pending { released_items, released_bytes },
                neural::ValueRetirementStep::Blocked => return Step::Blocked,
                neural::ValueRetirementStep::Complete => { assert!(cache.terminal_nonopaque_is_empty()); state.retiring_cache = None; }
            }
        } else { return Step::Complete; }
        Step::Pending { released_items: 1, released_bytes: 0 }
    }

    /// ✅️ Proves that every retained evaluation owner has crossed the close boundary.
    pub fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.neural_cache.is_none()
            && self.previous_snapshot.is_none()
            && self.previous_channels.is_none()
            && self.eval_json.capacity() == 0
            && self.status_json.capacity() == 0
            && self.live_geometry_handles.is_empty()
            && self.preview_mesh_json_by_handle.is_empty()
            && self.pending_tessellate_by_hash.is_empty()
            && self.retiring_cache.is_none()
            && self.retirement.terminal_is_empty()
            && self.retiring_collections.is_empty()
    }
}

fn preview_mesh_json_has_geometry(output_json: &str) -> bool {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(output_json) else {
        return false;
    };
    if value.get("error").is_some() {
        return false;
    }
    let positions = value.get("positions").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
    let indices = value.get("indices").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
    let edges = value.get("edgePositions").or_else(|| value.get("edge_positions")).and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
    (indices > 0 && positions >= 9) || edges >= 6 || (positions >= 3 && indices == 0)
}

/// 🧬 Stable `nodeHash` for an extension tessellate request (mirrored through ShellHost).
pub fn preview_tessellate_node_hash(handle: &str, tolerance_bits: u64) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    handle.hash(&mut hasher);
    tolerance_bits.hash(&mut hasher);
    hasher.finish()
}

/// 🏠 Builds a host wired to `session`'s shared cache and converged baseline.
pub fn flow_host_with_session(fixture: &FlowFixture, session: &FlowEvalSession) -> FlowHost {
    let mut host = FlowHost::from_fixture_with_cache(fixture.clone(), session.neural_cache());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    session.install_baseline_into(&mut host);
    if !session.eval_json().is_empty() {
        host.last_eval_json = session.eval_json().to_string();
    }
    host
}

fn build_flow_status_json(host: &FlowHost, remaining: &[String]) -> String {
    let eval: serde_json::Value = serde_json::from_str(&host.last_eval_json).unwrap_or(serde_json::json!({}));
    let tree = host.build_tree_for_status();
    let seeds = host.build_seeds_for_status();
    let snapshot = TreeSnapshot::capture(&tree, &seeds);
    let dirty = compute_dirty_set(host.eval_baseline_snapshot(), &snapshot);
    let active = remaining.first().map(String::as_str);
    let mut widgets = serde_json::Map::new();
    for widget in &host.fixture.widgets {
        let id = widget_id_for(widget);
        if matches!(widget, Widget::InputSlider { .. } | Widget::InputNote { .. } | Widget::InputImage { .. } | Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } | Widget::Cluster { .. }) {
            widgets.insert(id.to_string(), serde_json::to_value(NodeEvalStatus::Ok).unwrap_or(serde_json::json!({"status":"ok"})));
            continue;
        }
        if let Some(entry) = eval.get(id) {
            if let Some(message) = entry.get("error").and_then(|v| v.as_str()) {
                widgets.insert(id.to_string(), serde_json::to_value(NodeEvalStatus::Error { message: message.to_string() }).unwrap());
                continue;
            }
        }
        let blocked = host.widget_blocked_ports(id);
        if !blocked.is_empty() {
            widgets.insert(id.to_string(), serde_json::to_value(NodeEvalStatus::Blocked { ports: blocked }).unwrap());
            continue;
        }
        if active == Some(id) {
            widgets.insert(id.to_string(), serde_json::to_value(NodeEvalStatus::Computing).unwrap());
            continue;
        }
        if remaining.iter().any(|entry| entry == id) {
            widgets.insert(id.to_string(), serde_json::to_value(NodeEvalStatus::Queued).unwrap());
            continue;
        }
        if dirty.contains(id) && !remaining.is_empty() {
            widgets.insert(id.to_string(), serde_json::to_value(NodeEvalStatus::Stale).unwrap());
            continue;
        }
        widgets.insert(id.to_string(), serde_json::to_value(NodeEvalStatus::Ok).unwrap());
    }
    serde_json::to_string(&widgets).unwrap_or_else(|_| "{}".into())
}
// #endregion 🔖️EvalSession

fn dedupe_fixture_widgets(fixture: &mut FlowFixture) {
    let mut seen = BTreeSet::new();
    fixture.widgets.retain(|widget| seen.insert(widget_id_for(widget).to_string()));
}

pub(crate) fn widget_id_for(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

fn widget_has_output(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> bool {
    widgets.iter().any(|w| widget_id_for(w) == widget_id && !widget_io_ports(w, synapses, kind_infos).1.is_empty())
}

fn first_output_port(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> String {
    widgets.iter().find(|w| widget_id_for(w) == widget_id).and_then(|w| widget_io_ports(w, synapses, kind_infos).1.first().map(|port| port.id.clone())).unwrap_or_default()
}

fn first_input_port(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> String {
    widgets
        .iter()
        .find(|w| widget_id_for(w) == widget_id)
        .map(|w| match w {
            Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } => String::new(),
            _ => widget_io_ports(w, synapses, kind_infos).0.first().map(|port| port.id.clone()).unwrap_or_default(),
        })
        .unwrap_or_default()
}

fn widget_has_input(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> bool {
    widgets.iter().any(|w| {
        if widget_id_for(w) != widget_id {
            return false;
        }
        matches!(w, Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. }) || !widget_io_ports(w, synapses, kind_infos).0.is_empty()
    })
}
// #endregion 🔖️FlowHost

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use canvas::camera::{world_to_screen, Camera, Viewport};
    use canvas::Point;
    use dag::{computation_node_width, slider_widget_height, DagPreviewContent, HandleRole};
    use graph::dsl::{WireEdge, WireNode};
    use graph::manifest::PropertyBag;
    use neural::{ChannelSpec as InputSpec, OperatorInfo as NeuronKindInfo, Registry};
    use std::sync::{Mutex, OnceLock};

    const NUMBER_OPS: &[&str] = &["core.number"];
    static RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    #[test]
    fn tree_from_dag_builds_neurons_and_synapses() {
        let nodes = vec![WireNode { id: "a".into(), kind: "core.number".into(), port: None, properties: PropertyBag::new() }, WireNode { id: "b".into(), kind: "math.add".into(), port: None, properties: PropertyBag::new() }];
        let edges = vec![WireEdge { from: "a".into(), from_port: "number".into(), to: "b".into(), to_port: "a".into(), directed: true, properties: PropertyBag::new() }];
        let tree = FlowHost::tree_from_dag(&nodes, &edges);
        assert_eq!(tree.neurons.len(), 2);
        assert_eq!(tree.synapses.len(), 1);
    }

    fn test_math_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if kind == "core.number" {
            let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(value)))));
        }
        if kind == "core.text" {
            let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
            return Ok(channel_output("text", Dictionary::with_schema("text").insert("value", NeuralValue::Atom(Atom::String(value.into())))));
        }
        if kind == "core.image" {
            let value = input.get("dataUrl").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
            return Ok(channel_output("image", Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(value.into())))));
        }
        if kind == "math.add" {
            let a = input.get("a").or_else(|| input.get("number")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("a".into()))?;
            let b = input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(channel_output("sum", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(a + b)))));
        }
        if kind == "math.passThrough" {
            let n = input.get("number").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("number".into()))?;
            return Ok(channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(n)))));
        }
        if kind == "core.variable" {
            let name = input.get("name").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).ok_or_else(|| EvalError::MissingInput("name".into()))?;
            let payload = input.get(name).and_then(|v| v.as_dictionary()).cloned().ok_or_else(|| EvalError::MissingInput(name.into()))?;
            return Ok(channel_output(name, payload));
        }
        Err(EvalError::UnknownKind(kind.into()))
    }

    fn complete_fixture_registration<T>(future: impl std::future::Future<Output = T>) -> T {
        match std::pin::pin!(future).as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
            std::task::Poll::Ready(value) => value,
            std::task::Poll::Pending => panic!("fixture registration must not depend on external work"),
        }
    }

    /// 🧪️ Installs first-party light (+brep) flow extension manifests and real in-process ops for fixture tests.
    fn install_first_party_light_flow_extensions_for_tests() {
        use std::sync::Once;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            for (plugin_id, manifest) in [
                ("flow-extension-primitive", semio_s_plugin_flow_extension_primitive::extension_manifest_json()),
                ("flow-extension-math", semio_s_plugin_flow_extension_math::extension_manifest_json()),
                ("flow-extension-text", semio_s_plugin_flow_extension_text::extension_manifest_json()),
                ("flow-extension-logic", semio_s_plugin_flow_extension_logic::extension_manifest_json()),
                ("flow-extension-dictionary", semio_s_plugin_flow_extension_dictionary::extension_manifest_json()),
                ("flow-extension-list", semio_s_plugin_flow_extension_list::extension_manifest_json()),
                ("flow-extension-brep", complete_fixture_registration(semio_s_plugin_flow_extension_brep::extension_manifest_json())),
            ] {
                install_flow_extension_manifest(plugin_id, &manifest).expect("fixture extension admission");
            }
            let mut state = flow_extension_state().lock().expect("flow extension registry");
            let admission = begin_flow_registry_replacement(&mut state).expect("fixture registry admission");
            let mut registry = neural::ColdOwner::new(neural::Registry::new());
            semio_s_plugin_flow_extension_primitive::register(&mut registry);
            semio_s_plugin_flow_extension_math::register(&mut registry);
            semio_s_plugin_flow_extension_text::register(&mut registry);
            semio_s_plugin_flow_extension_logic::register(&mut registry);
            semio_s_plugin_flow_extension_dictionary::register(&mut registry);
            semio_s_plugin_flow_extension_list::register(&mut registry);
            complete_fixture_registration(semio_s_plugin_flow_extension_brep::register(&mut registry));
            registry.finalize();
            admission.publish(registry.into_inner());
            drop(state);
            while retire_flow_extension_registries_step(1, 4096).expect("fixture registry retirement") != neural::ValueRetirementStep::Complete {}
        });
    }

    fn fixture_kind_infos_json() -> String {
        install_first_party_light_flow_extensions_for_tests();
        crate::catalogue::flow_neuron_kind_infos_json()
    }

    fn test_kind_infos_json() -> String {
        serde_json::to_string(&[
            NeuronKindInfo {
                id: "math.add".into(),
                extension: "math".into(),
                name: "Add".into(),
                abbreviation: "Add".into(),
                icon: "emoji:➕️".into(),
                summary: "Sums two numbers".into(),
                inputs: vec![InputSpec::number("a", NUMBER_OPS), InputSpec::number_default("b", 0.0, NUMBER_OPS)],
                outputs: vec![InputSpec::named("S", "Sum", "sum", "Sum")],
                ..Default::default()
            },
            NeuronKindInfo {
                id: "math.passThrough".into(),
                extension: "math".into(),
                name: "PassThrough".into(),
                abbreviation: "Pass".into(),
                icon: "emoji:➡️".into(),
                summary: "Forwards a number".into(),
                inputs: vec![InputSpec::number_default("number", 0.0, NUMBER_OPS)],
                outputs: vec![InputSpec::named("N", "Num", "number", "Number")],
                ..Default::default()
            },
        ])
        .unwrap()
    }

    fn host_with_test_bridge() -> FlowHost {
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(test_math_bridge));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        host.set_host_catalogue_json(
            &serde_json::to_string(&[CatalogueSection {
                id: "math".into(),
                title: "Math".into(),
                groups: vec![],
                items: vec![
                    CatalogueItem { kind: "neuron".into(), neuron_kind: Some("math.add".into()), action: None, format: None, name: "Add".into(), abbreviation: "Add".into(), icon: "emoji:➕️".into(), summary: "Sums two numbers".into() },
                    CatalogueItem {
                        kind: "neuron".into(),
                        neuron_kind: Some("math.passThrough".into()),
                        action: None,
                        format: None,
                        name: "PassThrough".into(),
                        abbreviation: "Pass".into(),
                        icon: "emoji:➡️".into(),
                        summary: "Forwards a number".into(),
                    },
                ],
            }])
            .unwrap(),
        );
        host.evaluate_internal();
        host
    }

    fn widget_slider_track_screen_point(host: &FlowHost, widget_id: &str) -> (f64, f64) {
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == widget_id).expect("node");
        let (wx, wy) = dag::slider_track_center(node).expect("slider track");
        let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
        let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
        let screen = world_to_screen(&cam, &viewport, Point::new(wx, wy));
        (screen.x, screen.y)
    }

    #[test]
    fn default_fixture_maps_widgets_to_native_dag_kinds() {
        let host = host_with_test_bridge();
        let slider = host.dag.fixture.nodes.iter().find(|n| n.id == "slider").expect("slider");
        assert!(matches!(slider.kind, DagNodeKind::Slider { .. }));
        assert_eq!(slider.height, slider_widget_height());
        let add = host.dag.fixture.nodes.iter().find(|n| n.id == "add").expect("add");
        assert!(matches!(add.kind, DagNodeKind::Computation { .. }));
        assert_eq!(slider.width, add.width, "all components should share one width");
        assert_eq!(slider.width, computation_node_width(&slider.name, &[], &[]));
        let preview = host.dag.fixture.nodes.iter().find(|n| n.id == "preview").expect("preview");
        assert!(matches!(preview.kind, DagNodeKind::Preview { .. }));
    }

    #[test]
    fn default_fixture_evaluates_add_preview() {
        let host = host_with_test_bridge();
        assert_eq!(host.preview_text(), "3");
    }

    #[test]
    fn slider_updates_preview() {
        // 🧵️ Mutating a widget never auto-evaluates anymore (see `evaluate_step`'s doc comment) — an
        // off-main-thread ticker outside `flow` is responsible for that; this simulates one tick
        // with a direct `evaluate_internal` call.
        let mut host = host_with_test_bridge();
        host.set_slider_value("slider", 5.0);
        host.evaluate_internal();
        assert_eq!(host.preview_text(), "5");
    }

    #[test]
    fn evaluate_skips_unchanged_tree_after_move_widget() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_for_bridge = calls.clone();
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(move |kind, input| {
            calls_for_bridge.fetch_add(1, Ordering::Relaxed);
            test_math_bridge(kind, input)
        }));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        host.evaluate_internal();
        let baseline = calls.load(Ordering::Relaxed);
        host.move_widget("slider", -120.0, 20.0).unwrap();
        host.evaluate_internal();
        assert_eq!(calls.load(Ordering::Relaxed), baseline);
    }

    #[test]
    fn pending_eval_widget_ids_reports_without_computing() {
        let mut host = host_with_test_bridge();
        let before = host.preview_text();
        host.set_slider_value("slider", 9.0);
        let pending = host.pending_eval_widget_ids();
        assert!(pending.contains(&"add".to_string()), "the widget downstream of the changed slider is pending");
        assert!(!pending.contains(&"slider".to_string()), "the seed slider itself is not a pending neuron");
        assert_eq!(host.preview_text(), before, "a probe must never actually compute anything");
    }

    #[test]
    fn set_slider_value_marks_downstream_computing_chrome() {
        let mut host = host_with_test_bridge();
        host.set_slider_value("slider", 7.0);
        let pending = host.pending_eval_widget_ids();
        assert!(!pending.is_empty(), "slider change must flag downstream nodes as pending");
        host.refresh_computing_chrome_from_pending();
        let remaining = host.pending_eval_widget_ids();
        assert_eq!(remaining.first().map(String::as_str), pending.first().map(String::as_str));
    }

    #[test]
    fn apply_eval_outputs_json_establishes_baseline_for_dirty_probe() {
        let mut host = host_with_test_bridge();
        let eval_json = host.last_eval_json.clone();
        let mut fresh = FlowHost::default();
        fresh.set_eval_bridge_fn(Box::new(test_math_bridge));
        fresh.set_neuron_kind_infos_json(&test_kind_infos_json());
        fresh.apply_eval_outputs_json(&eval_json);
        fresh.set_slider_value("slider", 4.0);
        let pending = fresh.pending_eval_widget_ids();
        assert!(pending.contains(&"add".to_string()));
        assert!(!pending.contains(&"slider".to_string()));
    }

    #[test]
    fn apply_eval_outputs_json_skips_baseline_when_outputs_stale_for_seeds() {
        let mut host = host_with_test_bridge();
        let stale_eval_json = host.last_eval_json.clone();
        host.set_slider_value("slider", 9.0);
        host.apply_eval_outputs_json(&stale_eval_json);
        let pending = host.pending_eval_widget_ids();
        assert!(pending.contains(&"add".to_string()), "stale channel outputs must not converge the baseline after a seed change");
        assert!(!pending.contains(&"slider".to_string()));
        host.evaluate_internal();
        assert_eq!(host.preview_text(), "9");
        let fresh_eval_json = host.last_eval_json.clone();
        host.apply_eval_outputs_json(&fresh_eval_json);
        assert!(host.pending_eval_widget_ids().is_empty(), "fresh eval for the current seeds must establish a converged baseline");
    }

    #[test]
    fn flow_eval_session_retains_baseline_across_ephemeral_hosts() {
        let mut session = FlowEvalSession::new();
        let mut host = host_with_test_bridge();
        session.capture_baseline_from(&host);
        host.set_slider_value("slider", 8.0);
        let mut replay = FlowHost::default();
        replay.set_eval_bridge_fn(Box::new(test_math_bridge));
        replay.set_neuron_kind_infos_json(&test_kind_infos_json());
        replay.replace_fixture(host.fixture.clone());
        session.install_baseline_into(&mut replay);
        let pending = replay.pending_eval_widget_ids();
        assert!(pending.contains(&"add".to_string()));
        assert!(!pending.contains(&"slider".to_string()));
    }

    #[test]
    fn flow_eval_session_seeds_its_retained_neural_cache() {
        let session = FlowEvalSession::new();
        let expected = Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(42.0)));
        let output_json = serde_json::to_string(&expected).unwrap();
        session.seed_node_cache(17, &output_json).unwrap();
        assert_eq!(session.neural_cache().get(17), Some(expected));
    }

    /// 🧵️ Builds a two-computable-node chain (`add` -> `pass`, replacing `add`'s direct link to
    /// `preview`) on top of the default fixture, for tests that need more than one node to step
    /// through with a budgeted `evaluate_step`.
    fn host_with_two_node_chain() -> (FlowHost, String) {
        let mut host = host_with_test_bridge();
        let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 240.0, 0.0).unwrap();
        host.connect_ports("add", "sum", &pass_id, "number").unwrap();
        host.connect_ports(&pass_id, "number", "preview", "").unwrap();
        let stale_link = host.fixture.synapses.iter().find(|s| s.from == "add" && s.to == "preview").map(|s| s.id.clone());
        if let Some(id) = stale_link {
            host.disconnect(&id).unwrap();
        }
        host.evaluate_internal();
        (host, pass_id)
    }

    #[test]
    fn evaluate_step_budget_one_converges_over_multiple_calls() {
        let (mut host, _pass_id) = host_with_two_node_chain();
        assert_eq!(host.preview_text(), "3", "chain settles to the same value as the direct add->preview link");
        host.set_slider_value("slider", 6.0);
        // ⏳️ Nothing evaluates until stepped — mirrors a mutation with no tick chain run yet.
        assert_eq!(host.preview_text(), "3");
        // ⏱️ Tick 1: budget for one cache-missed node — computes "add" for free-riding boundary nodes
        // plus that one dispatch, then stops right before the next miss ("pass"). `remaining[0]` is
        // the blocking node; anything after it (here, "preview") is just downstream-and-untouched.
        let remaining_after_tick1 = host.evaluate_step(1);
        assert_eq!(remaining_after_tick1.first(), Some(&"pass".to_string()), "pass is the next node blocking completion");
        assert_eq!(host.preview_text(), "3", "the chain hasn't reached \"pass\" (and thus \"preview\") yet");
        // ⏱️ Tick 2: "add" is now cached, so this reaches and computes "pass".
        let remaining_after_tick2 = host.evaluate_step(1);
        assert!(remaining_after_tick2.is_empty(), "the walk reached the end of the topo order");
        assert_eq!(host.preview_text(), "6", "converged to the dragged value after both ticks");
    }

    #[test]
    fn flow_eval_session_sync_and_tick_state_machine() {
        let (mut host, _pass_id) = host_with_two_node_chain();
        let mut session = FlowEvalSession::new();
        session.capture_baseline_from(&host);
        assert!(!session.pending());
        assert!(!session.sync(&host));
        assert!(!session.pending());
        host.set_slider_value("slider", 12.0);
        assert!(session.sync(&host), "a changed slider arms the chain");
        assert!(session.pending());
        assert!(session.status_json().contains("computing"), "the immediate dependent is reported as computing");
        assert!(!session.sync(&host));
        while session.tick(&mut host) {}
        assert!(!session.pending());
        assert_eq!(host.preview_text(), "12");
        host.set_slider_value("slider", 20.0);
        assert!(session.sync(&host));
        assert!(session.pending());
        host.set_slider_value("slider", 30.0);
        assert!(!session.sync(&host), "a chain is already scheduled — sync must not arm a redundant second one");
        assert!(session.pending(), "the in-flight chain is still the one that will pick up 30");
        while session.tick(&mut host) {}
        assert_eq!(host.preview_text(), "30", "converges on the latest value, not the superseded intermediate one");
    }

    #[test]
    fn connect_ports_allows_fan_out_from_same_output() {
        let mut host = host_with_test_bridge();
        let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 120.0, 120.0).unwrap();
        host.connect_ports("add", "sum", &pass_id, "number").unwrap();
        let fan_out: Vec<_> = host.fixture.synapses.iter().filter(|s| s.from == "add" && s.from_port == "sum").collect();
        assert_eq!(fan_out.len(), 2);
        assert!(fan_out.iter().any(|s| s.to == "preview"));
        assert!(fan_out.iter().any(|s| s.to == pass_id));
    }

    #[test]
    fn connect_ports_replaces_existing_incoming_on_same_input() {
        let mut host = host_with_test_bridge();
        assert!(host.fixture.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
        let note_id = host.add_widget(r#"{"kind":"inputNote","id":"note","text":"2"}"#, -120.0, 0.0).unwrap();
        host.connect_ports(&note_id, "text", "add", "a").unwrap();
        let incoming_a: Vec<_> = host.fixture.synapses.iter().filter(|s| s.to == "add" && s.to_port == "a").collect();
        assert_eq!(incoming_a.len(), 1);
        assert_eq!(incoming_a[0].from, note_id);
        assert!(!host.fixture.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
    }

    #[test]
    fn evaluate_runs_after_tree_change() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_for_bridge = calls.clone();
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(move |kind, input| {
            calls_for_bridge.fetch_add(1, Ordering::Relaxed);
            test_math_bridge(kind, input)
        }));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        host.evaluate_internal();
        let baseline = calls.load(Ordering::Relaxed);
        host.set_slider_value("slider", 5.0);
        host.evaluate_internal();
        let after_slider = calls.load(Ordering::Relaxed);
        assert!(after_slider > baseline);
        host.disconnect("s1").unwrap();
        host.connect_ports("slider", "number", "add", "b").unwrap();
        host.evaluate_internal();
        assert!(calls.load(Ordering::Relaxed) > after_slider);
    }

    #[test]
    fn dirty_propagation_only_dispatches_affected_branch() {
        use std::sync::Arc;
        use std::sync::Mutex as StdMutex;
        // Branch A (default fixture): slider -> add -> preview. Branch B (added here): a second,
        // disconnected slider -> passThrough, sharing no synapse with branch A.
        let calls: Arc<StdMutex<Vec<String>>> = Arc::new(StdMutex::new(Vec::new()));
        let calls_for_bridge = calls.clone();
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(move |kind, input| {
            calls_for_bridge.lock().unwrap().push(kind.to_string());
            test_math_bridge(kind, input)
        }));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        let slider_b_id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":1.0}"#, 400.0, 0.0).unwrap();
        let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 600.0, 0.0).unwrap();
        host.connect_ports(&slider_b_id, "number", &pass_id, "number").unwrap();
        host.evaluate_internal();
        calls.lock().unwrap().clear();

        host.set_slider_value("slider", 5.0);
        host.evaluate_internal();

        let dispatched = calls.lock().unwrap().clone();
        assert!(dispatched.iter().any(|kind| kind == "math.add"), "branch A (add) should re-dispatch after its slider changed");
        assert!(!dispatched.iter().any(|kind| kind == "math.passThrough"), "branch B (pass) must stay clean when only branch A changed");
    }

    #[test]
    fn neural_cache_persists_across_evaluations() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_for_bridge = calls.clone();
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(move |kind, input| {
            calls_for_bridge.fetch_add(1, Ordering::Relaxed);
            test_math_bridge(kind, input)
        }));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        host.evaluate_internal();
        let baseline = calls.load(Ordering::Relaxed);
        assert!(baseline > 0, "first evaluation is a cache miss and must dispatch to the bridge");
        host.evaluate_internal();
        assert_eq!(calls.load(Ordering::Relaxed), baseline, "an unchanged tree must be served entirely from the cache");
        host.set_slider_value("slider", 4.0);
        host.evaluate_internal();
        assert_eq!(calls.load(Ordering::Relaxed), baseline + 1, "only the node downstream of the changed slider should re-dispatch");
    }

    #[test]
    fn collect_live_geometry_handles_includes_input_channels() {
        let mut outputs = BTreeMap::new();
        outputs.insert("box".into(), Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-box".into()))).insert("kind", NeuralValue::Atom(Atom::String("solid".into()))));
        outputs.insert("volume".into(), Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(12.0))));
        let mut inputs = BTreeMap::new();
        inputs.insert("volume".into(), Dictionary::new().insert("geometry", NeuralValue::Dictionary(Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-box".into()))))));
        let channels = EvalChannels { outputs, inputs };
        let handles = collect_live_geometry_handles_from_channels(&channels);
        assert_eq!(handles, vec![String::from("solid-box")]);
    }

    #[test]
    fn apply_eval_outputs_json_preserves_state_on_global_error() {
        let mut host = host_with_test_bridge();
        let good = host.last_eval_json.clone();
        host.apply_eval_outputs_json(r#"{"error":"missing input: geometry"}"#);
        assert_eq!(host.last_eval_json, good);
        assert!(!host.outputs.is_empty());
    }

    fn collect_live_geometry_handles(outputs: &BTreeMap<String, Dictionary>) -> Vec<String> {
        let mut handles = Vec::new();
        for dict in outputs.values() {
            collect_geometry_handles_from_dictionary(dict, &mut handles);
        }
        handles.sort();
        handles.dedup();
        handles
    }

    #[test]
    fn collect_live_geometry_handles_traverses_nested_dictionaries() {
        let mut outputs = BTreeMap::new();
        outputs.insert("box".into(), Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-1".into()))).insert("kind", NeuralValue::Atom(Atom::String("solid".into()))));
        outputs.insert("nested".into(), Dictionary::new().insert("child", NeuralValue::Dictionary(Dictionary::with_schema("face").insert("handle", NeuralValue::Atom(Atom::String("face-2".into()))))));
        let handles = collect_live_geometry_handles(&outputs);
        assert_eq!(handles, vec![String::from("face-2"), String::from("solid-1")]);
    }

    #[test]
    fn collect_live_drawing_handles_traverses_list_values() {
        let mut outputs = BTreeMap::new();
        outputs.insert(
            "get".into(),
            Dictionary::new()
                .insert("value", NeuralValue::Dictionary(Dictionary::with_schema("list").insert("0", NeuralValue::Dictionary(Dictionary::with_schema("draw.drawing").insert("handle", NeuralValue::Atom(Atom::String("drawing-2".into()))))))),
        );
        let channels = EvalChannels { outputs, inputs: BTreeMap::new() };
        assert_eq!(collect_live_drawing_handles_from_channels(&channels), vec![String::from("drawing-2")]);
    }

    #[test]
    fn evaluate_emits_channel_structured_json() {
        let host = host_with_test_bridge();
        let parsed: serde_json::Value = serde_json::from_str(&host.last_eval_json).expect("json");
        let add = parsed.get("add").and_then(|value| value.as_object()).expect("add channels");
        assert!(add.get("in").and_then(|value| value.as_object()).is_some());
        let out = add.get("out").and_then(|value| value.as_object()).expect("add out");
        assert!(out.get("sum").is_some());
    }

    #[test]
    fn preview_text_formats_geometry_as_tree_summary() {
        let dict = Dictionary::new().insert("geometry", NeuralValue::Atom(Atom::String("solid-3".into())));
        let content = dag_preview_content_from_dict(&dict);
        assert!(matches!(content, DagPreviewContent::Tree { .. }));
        assert_eq!(preview_content_summary(&content), "{1 keys}");
    }

    #[test]
    fn preview_scalar_content_from_number_dict() {
        let dict = Dictionary::new().insert("number", NeuralValue::Atom(Atom::Decimal(3.0)));
        assert!(matches!(
            dag_preview_content_from_dict(&dict),
            DagPreviewContent::Scalar { text } if text == "3"
        ));
    }

    #[test]
    fn image_input_seed_and_preview_content() {
        let mut host = host_with_test_bridge();
        let png = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
        host.fixture.widgets.push(Widget::InputImage { id: "image".into(), src: png.into() });
        host.rebuild_dag();
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == "image").expect("image node");
        assert!(matches!(node.kind, DagNodeKind::Image { .. }));
        let seeds = host.build_seeds();
        assert_eq!(seeds.get("image").and_then(|d| d.get("image")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("dataUrl")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some(png));
    }

    #[test]
    fn slider_drag_does_not_evaluate_until_explicit_evaluate() {
        // 🧵️ A live drag firing many pointer-move ticks used to re-evaluate the whole graph on every
        // one of them (fine for cheap graphs, a repeated multi-second stall for a heavy one, e.g. a
        // brep boolean). Dragging alone must never evaluate now — the off-main-thread ticker (outside
        // `flow`) picks up the changed slider value at its own pace; an explicit `evaluate`
        // (simulated here) still updates the preview once it runs.
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
        assert_eq!(host.preview_text(), "3");
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx + 80.0, sy, false, false, false);
        assert_eq!(host.preview_text(), "3", "a live drag must not synchronously re-evaluate the graph");
        host.evaluate_internal();
        assert_ne!(host.preview_text(), "3", "an explicit evaluate still picks up the dragged value");
        host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    }

    #[test]
    fn dag_slider_drag_syncs_fixture_value() {
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        let slider_node = host.dag.fixture.nodes.iter().find(|n| n.id == "slider").expect("slider").clone();
        let DagNodeKind::Slider { .. } = slider_node.kind else {
            panic!("expected slider kind");
        };
        let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx + 80.0, sy, false, false, false);
        host.pointer_up_screen(sx + 80.0, sy, false, false, false);
        let value = host
            .fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::InputSlider { id, value, .. } if id == "slider" => Some(*value),
                _ => None,
            })
            .unwrap();
        assert!(value > 3.0);
    }

    #[test]
    fn default_fixture_does_not_auto_layout() {
        let host = host_with_test_bridge();
        let slider = host.fixture.layout.get("slider").expect("slider");
        let add = host.fixture.layout.get("add").expect("add");
        let preview = host.fixture.layout.get("preview").expect("preview");
        assert_eq!(slider.x, 0.0);
        assert_eq!(add.x, 200.0);
        assert_eq!(preview.x, 400.0);
    }

    #[test]
    fn canvas_slider_hit_adjusts_value_playground_viewport() {
        let mut host = host_with_test_bridge();
        host.set_viewport(1259, 706, 1.0);
        let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx + 90.0, sy, false, false, false);
        host.pointer_up_screen(sx + 90.0, sy, false, false, false);
        let slider = host
            .fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::InputSlider { id, value, .. } if id == "slider" => Some(*value),
                _ => None,
            })
            .unwrap();
        assert!(slider > 3.0);
    }

    #[test]
    fn canvas_slider_hit_adjusts_value() {
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx + 80.0, sy, false, false, false);
        host.pointer_up_screen(sx + 80.0, sy, false, false, false);
        let slider = host
            .fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::InputSlider { id, value, .. } if id == "slider" => Some(*value),
                _ => None,
            })
            .unwrap();
        assert!(slider > 3.0);
    }

    #[test]
    fn reorganize_overwrites_saved_layout_left_to_right() {
        let mut host = host_with_test_bridge();
        host.fixture.layout.insert("slider".into(), WidgetLayout { x: -900.0, y: -900.0 });
        host.fixture.layout.insert("add".into(), WidgetLayout { x: -900.0, y: -900.0 });
        host.fixture.layout.insert("preview".into(), WidgetLayout { x: -900.0, y: -900.0 });
        host.rebuild_dag();
        host.reorganize("").unwrap();
        let slider = host.fixture.layout.get("slider").expect("slider layout");
        let add = host.fixture.layout.get("add").expect("add layout");
        let preview = host.fixture.layout.get("preview").expect("preview layout");
        assert!(add.x > slider.x);
        assert!(preview.x > add.x);
    }

    #[test]
    fn fixture_json_round_trip() {
        let host = FlowHost::default();
        let json = host.fixture_json().unwrap();
        let parsed = FlowHost::parse_fixture_json(&json).unwrap();
        assert_eq!(parsed.schema, "flow.fixture");
    }

    #[test]
    fn flow_document_tree_is_shakable() {
        let host = host_with_test_bridge();
        let document = host.document();
        assert_eq!(document.schema, "flow.artifact");
        assert!(!document.tree.neurons.is_empty());
        let registry = neural::Registry::new();
        let evaluator = Evaluator::new(&registry);
        let dispatch = |kind: &str, input: &Dictionary| test_math_bridge(kind, input);
        let mut seeds = HashMap::new();
        seeds.insert("slider".into(), channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(3.0)))));
        let channels = evaluator.evaluate_channels_with(&document.tree, &seeds, &host.kind_infos, &dispatch).unwrap();
        assert_eq!(channels.outputs.get("add").and_then(|d| d.get("sum")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn rebuild_dag_preserves_canvas_theme() {
        use canvas::Color;
        let mut host = FlowHost::default();
        host.dag.canvas_theme.node_fill = Color::from_rgba8(12, 34, 56, 255);
        host.rebuild_dag();
        assert_eq!(host.dag.canvas_theme.node_fill.to_rgba8(), Color::from_rgba8(12, 34, 56, 255).to_rgba8());
    }

    #[test]
    fn set_canvas_theme_dark_applies_board_dark_strokes() {
        let mut host = FlowHost::default();
        host.set_canvas_theme_dark(true);
        let stroke = host.dag.canvas_theme.node_stroke.to_rgba8();
        assert!(stroke.r > 80 || stroke.g > 80);
        host.set_canvas_theme_dark(false);
        let light_stroke = host.dag.canvas_theme.node_stroke.to_rgba8();
        assert!(light_stroke.r < 80);
    }

    #[test]
    fn paint_scene_dark_theme_paints_edges_and_nodes() {
        let mut host = host_with_test_bridge();
        host.set_viewport(1280, 800, 1.0);
        host.set_canvas_theme_dark(true);
        let mut scene = canvas::Scene::new();
        host.paint_scene(&mut scene, 1280, 800, 1.0);
        assert!(scene.path_count() > 8, "populated fixture should paint edges, handles, and node bodies under dark board theme");
    }

    #[test]
    fn flow_host_enables_minimap_widget_on_dag() {
        let mut host = host_with_test_bridge();
        host.set_viewport(1280, 800, 1.0);
        host.dag.set_camera(200.0, 120.0, 0.65);
        let raw: serde_json::Value = serde_json::from_str(&host.dag.label_overlay_paint_state_json().unwrap()).unwrap();
        assert!(raw.get("minimapWidget").is_some());
    }

    #[test]
    fn replace_fixture_preserves_kind_infos_and_named_input_ports() {
        let mut host = host_with_test_bridge();
        host.replace_fixture(FlowFixture {
            schema: "flow.fixture".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![Widget::Neuron { id: "add".into(), neuron_kind: "math.add".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: true }],
            synapses: vec![],
            layout: crate::OrderedMap::new(),
        });
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == "add").expect("add node");
        let input_ids: Vec<&str> = node.inputs().iter().map(|port| port.id.as_str()).collect();
        assert_eq!(input_ids, vec!["a", "b"]);
    }

    #[test]
    fn catalogue_nested_groups_round_trip() {
        let host_json = serde_json::to_string(&[CatalogueSection {
            id: "brep".into(),
            title: "Brep".into(),
            items: vec![],
            groups: vec![CatalogueGroup {
                id: "brep.primitives-3d".into(),
                title: "Primitives 3D".into(),
                items: vec![CatalogueItem {
                    kind: "neuron".into(),
                    neuron_kind: Some("brep.prim3d.box".into()),
                    action: None,
                    format: None,
                    name: "Box".into(),
                    abbreviation: "Box".into(),
                    icon: "emoji:📦️".into(),
                    summary: "Axis-aligned box".into(),
                }],
                groups: vec![],
            }],
        }])
        .unwrap();
        let sections = merge_catalogue_sections(&host_json).unwrap();
        let brep = sections.iter().find(|section| section.id == "brep").expect("brep section");
        let prim3d = brep.groups.iter().find(|group| group.title == "Primitives 3D").expect("prim3d group");
        assert_eq!(prim3d.items[0].neuron_kind.as_deref(), Some("brep.prim3d.box"));
    }

    #[test]
    fn catalogue_has_module_sections() {
        let host = host_with_test_bridge();
        let json = host.catalogue_json().unwrap();
        assert!(json.contains("math"));
        assert!(json.contains("math.add"));
        assert!(json.contains("Inputs"));
        assert!(json.contains("Outputs"));
    }

    #[test]
    fn flow_backed_node_graph_extras_include_fixture_and_flow_engine() {
        install_first_party_light_flow_extensions_for_tests();
        let host = host_with_test_bridge();
        let extras = flow_backed_node_graph_extras(&host.fixture, FLOW_LOD_MODE_AUTOMATIC, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, None);
        assert!(extras.fixture_json.as_ref().is_some_and(|json| json.contains("flow.fixture")));
        assert!(extras.operators.iter().any(|info| info.id == "math.add"));
        assert!(extras.capabilities_json.as_ref().is_some_and(|json| json.contains(r#""engine":"flow""#)));
        assert!(extras.catalogue_json.as_ref().is_some_and(|json| json.contains("brep") || json.contains("math")));
        assert!(extras.lod_json.as_ref().is_some_and(|json| json.contains(r#""automatic":true"#)));
    }

    #[test]
    fn contributed_extension_manifest_installs_catalogue_operator() {
        let manifest = r#"{"schema":"flow.extension","id":"stubext","name":"Stub","version":"0.0.1","activationEvents":[],"contributes":{"schemas":[],"operators":[{"id":"stubext.echo","extension":"stubext","name":"Echo","abbreviation":"Echo","icon":"emoji:📣️","summary":"Echo","inputs":[],"outputs":[]}],"widgets":[],"commands":[],"settings":[]}}"#;
        install_flow_extension_manifest("stub-plugin", manifest).expect("stub extension admission");
        assert!(flow_extension_registry().operator_info("stubext.echo").is_some());
        let sections = flow_catalogue_sections();
        assert!(sections.iter().any(|section| section.id == "stubext"));
        uninstall_flow_extension("stubext").expect("stub extension uninstall admission");
    }

    #[test]
    fn flow_fixture_with_synapses_builds_dag_edges_and_ports() {
        install_first_party_light_flow_extensions_for_tests();
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
        host.replace_fixture(<FlowFixture as crate::os_store::ArtifactDsl>::parse_dsl(include_str!("../📚️examples/🌊️default.flow.dsl.semio")).expect("fixture"));
        assert!(!host.dag.fixture.edges.is_empty(), "synapses should become dag edges");
        let add = host.dag.fixture.nodes.iter().find(|node| node.id == "add").expect("add node");
        assert_eq!(add.inputs().len(), 2);
        assert_eq!(add.outputs().len(), 1);
        let mut scene = canvas::Scene::new();
        host.set_canvas_theme_dark(true);
        host.paint_scene(&mut scene, 1280, 800, 1.0);
        assert!(scene.path_count() > 8, "rich flow graph should paint edges and handles");
    }

    #[test]
    fn add_widget_and_connect() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.passThrough"}"#, 100.0, 50.0).unwrap();
        host.connect_ports("slider", "number", &id, "number").unwrap();
        host.connect_ports(&id, "number", "preview", "").unwrap();
        host.set_slider_value("slider", 4.0);
        host.evaluate_internal();
        assert_eq!(host.preview_text(), "4");
    }

    #[test]
    fn output_export_widget_catalogue_descriptor_and_payload() {
        let mut host = host_with_test_bridge();
        let sections = merge_catalogue_sections("").unwrap();
        let exports: Vec<_> = sections.iter().flat_map(|section| section.items.iter()).filter(|item| item.kind == "outputExport").collect();
        assert_eq!(exports.len(), 4);
        assert!(exports.iter().any(|item| item.format.as_deref() == Some("svg")));
        let id = host.add_widget(r#"{"kind":"outputExport","format":"png"}"#, 120.0, 80.0).unwrap();
        host.connect_ports("add", "sum", &id, "").unwrap();
        host.set_slider_value("slider", 4.0);
        host.evaluate_internal();
        let payload_json = host.export_payload_json(&id).expect("export payload");
        assert_ne!(payload_json, "{}");
        assert!(payload_json.contains("4") || payload_json.contains("value") || payload_json.contains("sum"));
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == id).expect("export node");
        assert!(matches!(node.kind, DagNodeKind::Export { .. }));
    }

    /// ↩️ Exercises the standard `crate::os_store::ArtifactStore<FlowFixture, FlowMutation>` undo/redo
    /// mechanism directly (the same one `FlowHost::undo`/`redo` are built on) — add a widget, undo,
    /// confirm it's gone, redo, confirm it's back — in place of the old test's direct assertions on a
    /// hand-rolled `Vec<FlowFixture>` snapshot stack.
    #[semio_framework_async_macros::async_test]
    async fn undo_redo_add_widget() {
        let mut host = host_with_test_bridge();
        let fixture_before = host.fixture.clone();
        let count_before = fixture_before.widgets.len();
        let id = host.add_widget(r#"{"kind":"inputNote","text":"undo me"}"#, 42.0, 42.0).unwrap();
        assert_eq!(host.fixture.widgets.len(), count_before + 1);

        let operations = flow_fixture_operations(&fixture_before, &host.fixture).expect("wire-representable flow fixture");
        assert!(!operations.is_empty(), "add_widget must diff into vcs operations");

        let envelope: FlowEnvelope = create_document_envelope(FLOW_DOCUMENT_SCHEMA, "test", fixture_before, None);
        let mut store = FlowStore::new(envelope).await.expect("valid flow store fixture");
        store.dispatch(ArtifactCommand::Apply { mutations: operations, description: None }).await.expect("apply add-widget operations");
        assert_eq!(store.snapshot().expect("projection").widgets.len(), count_before + 1);

        store.dispatch(ArtifactCommand::Undo).await.expect("undo");
        let after_undo = store.snapshot().expect("projection");
        assert_eq!(after_undo.widgets.len(), count_before);
        assert!(!after_undo.widgets.iter().any(|w| widget_id_for(w) == id));

        store.dispatch(ArtifactCommand::Redo).await.expect("redo");
        let after_redo = store.snapshot().expect("projection");
        assert!(after_redo.widgets.iter().any(|w| widget_id_for(w) == id));
    }

    #[test]
    fn camera_change_does_not_create_undo_step() {
        let mut host = host_with_test_bridge();
        let camera_before = host.fixture.camera.clone();
        host.set_camera(camera_before.x + 50.0, camera_before.y - 30.0, camera_before.zoom * 1.5);
        assert!(!host.can_undo());
        let id = host.add_widget(r#"{"kind":"inputNote","text":"x"}"#, 0.0, 0.0).unwrap();
        assert!(host.can_undo());
        assert!(host.undo());
        assert_eq!(host.fixture.camera.x, camera_before.x + 50.0);
        assert_eq!(host.fixture.camera.y, camera_before.y - 30.0);
        assert!((host.fixture.camera.zoom - camera_before.zoom * 1.5).abs() < 1e-9);
        assert!(!host.fixture.widgets.iter().any(|w| widget_id_for(w) == id));
    }

    #[test]
    fn replace_fixture_preserves_live_camera() {
        let mut host = host_with_test_bridge();
        host.set_camera(120.0, -45.0, 1.75);
        host.replace_fixture(FlowFixture { schema: "flow.fixture".into(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, widgets: vec![Widget::InputNote { id: "note".into(), text: "hello".into() }], synapses: vec![], layout: crate::OrderedMap::new() });
        assert_eq!(host.fixture.camera.x, 120.0);
        assert_eq!(host.fixture.camera.y, -45.0);
        assert!((host.fixture.camera.zoom - 1.75).abs() < 1e-9);
        assert!(host.fixture.widgets.iter().any(|w| widget_id_for(w) == "note"));
    }

    fn test_dictionary_merge_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if kind == "core.number" {
            let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(value))));
        }
        if kind != "dictionary.merge" {
            return Err(EvalError::UnknownKind(kind.into()));
        }
        let items = input.get("items").and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput("items".into()))?;
        let mut indices: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
        indices.sort_unstable();
        if indices.len() < 2 {
            return Err(EvalError::MissingInput("items".into()));
        }
        let mut merged = Dictionary::with_schema("dictionary");
        for index in indices {
            let slot = items.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(index.to_string()))?;
            merged = merged.merge(slot);
        }
        Ok(channel_output("dictionary", merged))
    }

    #[test]
    fn variadic_merge_evaluates_port_routed_inputs() {
        let mut host = FlowHost::from_fixture(FlowFixture {
            schema: "flow.fixture".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![
                Widget::InputSlider { id: "a".into(), label: "A".into(), value: 1.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
                Widget::InputSlider { id: "b".into(), label: "B".into(), value: 2.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
                Widget::Neuron { id: "merge".into(), neuron_kind: "dictionary.merge".into(), params: Dictionary::new(), input_ports: vec!["0".into(), "1".into()], output_ports: vec![], preview: true },
                Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: crate::OrderedSet::new() },
            ],
            synapses: vec![
                SynapseSpec { id: "s1".into(), from: "a".into(), to: "merge".into(), from_port: "number".into(), to_port: "0".into() },
                SynapseSpec { id: "s2".into(), from: "b".into(), to: "merge".into(), from_port: "number".into(), to_port: "1".into() },
                SynapseSpec { id: "s3".into(), from: "merge".into(), to: "preview".into(), from_port: "dictionary".into(), to_port: String::new() },
            ],
            layout: crate::OrderedMap::new(),
        });
        host.set_eval_bridge_fn(Box::new(test_dictionary_merge_bridge));
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "dictionary.merge".into(),
                extension: "dictionary".into(),
                name: "Merge".into(),
                abbreviation: "Merge".into(),
                icon: "emoji:🔀️".into(),
                summary: "Merge".into(),
                inputs: vec![],
                outputs: vec![InputSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
                variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
                ..Default::default()
            }])
            .unwrap(),
        );
        host.previous_snapshot = None;
        host.outputs.clear();
        host.evaluate_internal();
        let preview = host
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::OutputPreview { preview, .. } => Some(preview),
                _ => None,
            })
            .expect("preview");
        assert_eq!(preview.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(2.0));
    }

    #[test]
    fn widget_to_dag_node_carries_display_meta() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 0.0, 0.0).unwrap();
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == id).expect("node");
        assert_eq!(node.name, "Add");
        assert_eq!(node.abbreviation, "Add");
        assert_eq!(node.icon, "emoji:➕️");
    }

    #[test]
    fn add_slider_widget_with_explicit_range() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":10.2,"min":10.2,"max":15.0,"step":0.1}"#, 0.0, 0.0).unwrap();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputSlider { value, min, max, step, .. } = widget else {
            panic!("expected slider widget");
        };
        assert!((value - 10.2).abs() < 1e-6);
        assert!((min - 10.2).abs() < 1e-6);
        assert!((max - 15.0).abs() < 1e-6);
        assert!((step - 0.1).abs() < 1e-6);
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let DagNodeKind::Slider { min: dag_min, max: dag_max, step: dag_step, value: dag_value, .. } = &node.kind else {
            panic!("expected slider node");
        };
        assert!((dag_min - 10.2).abs() < 1e-6);
        assert!((dag_max - 15.0).abs() < 1e-6);
        assert!((dag_step - 0.1).abs() < 1e-6);
        assert!((dag_value - 10.2).abs() < 1e-6);
    }

    #[test]
    fn add_note_widget_with_text() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputNote","text":"some text"}"#, 0.0, 0.0).unwrap();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputNote { text, .. } = widget else {
            panic!("expected note widget");
        };
        assert_eq!(text, "some text");
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let DagNodeKind::Note { text: dag_text, .. } = &node.kind else {
            panic!("expected note node");
        };
        assert_eq!(dag_text, "some text");
        assert!(node.width >= 40.0);
        assert_eq!(node.height, dag::DAG_CHANNEL_ROW_HEIGHT);
    }

    #[test]
    fn begin_note_edit_groups_undo_into_single_gesture() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 0.0).unwrap();
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let origin_x = node.x - node.width * 0.5 + 4.0;
        host.begin_note_edit(&id, origin_x + 40.0, node.y);
        host.note_insert_text("!");
        host.note_commit_edit();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputNote { text, .. } = widget else {
            panic!("expected note widget");
        };
        assert_eq!(text, "hi!");
        assert!(host.undo());
        let Widget::InputNote { text: restored, .. } = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget") else {
            panic!("expected note widget");
        };
        assert_eq!(restored, "hi");
    }

    #[test]
    fn wheel_screen_zoom_gesture_changes_zoom() {
        let mut host = host_with_test_bridge();
        let z0 = host.fixture.camera.zoom;
        host.wheel_screen(400.0, 300.0, 0.0, -10.0, true);
        assert_ne!(host.fixture.camera.zoom, z0);
    }

    #[test]
    fn wheel_plan_matches_direct_and_rejects_stale_revision() {
        let mut direct = host_with_test_bridge();
        let mut planned = host_with_test_bridge();
        direct.set_viewport(800, 600, 1.0);
        planned.set_viewport(800, 600, 1.0);
        direct.wheel_screen(320.0, 240.0, 0.0, -10.0, true);
        let plan = planned.plan_wheel(320.0, 240.0, 0.0, -10.0, true);
        assert!(planned.commit_wheel(plan));
        assert_eq!(direct.fixture.camera, planned.fixture.camera);

        let stale = planned.plan_wheel(320.0, 240.0, 0.0, -10.0, true);
        planned.pointer_down_screen(10.0, 10.0, 0, false, false, false, true);
        let replacement = planned.fixture.camera.clone();
        assert!(!planned.commit_wheel(stale));
        assert_eq!(planned.fixture.camera, replacement);
    }

    #[test]
    fn set_note_text_keeps_uniform_component_width() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 0.0).unwrap();
        let short_w = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node").width;
        host.set_note_text(&id, "a much longer note string");
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let DagNodeKind::Note { text, .. } = &node.kind else {
            panic!("expected note node");
        };
        assert_eq!(text, "a much longer note string");
        assert_eq!(node.width, short_w);
    }

    #[test]
    fn add_slider_widget_with_single_value_uses_sensible_range() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":5.0}"#, 0.0, 0.0).unwrap();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputSlider { value, min, max, step, .. } = widget else {
            panic!("expected slider widget");
        };
        assert!((value - 5.0).abs() < 1e-6);
        assert!((min - 0.0).abs() < 1e-6);
        assert!((max - 10.0).abs() < 1e-6);
        assert!((step - 1.0).abs() < 1e-6);
    }

    #[test]
    fn add_slider_widget_with_decimal_value_uses_matching_step() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":1.3}"#, 0.0, 0.0).unwrap();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputSlider { value, min, max, step, .. } = widget else {
            panic!("expected slider widget");
        };
        assert!((value - 1.3).abs() < 1e-6);
        assert!((min - 0.0).abs() < 1e-6);
        assert!((max - 10.0).abs() < 1e-6);
        assert!((step - 0.1).abs() < 1e-6);
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let DagNodeKind::Slider { min: dag_min, max: dag_max, step: dag_step, value: dag_value, .. } = &node.kind else {
            panic!("expected slider node");
        };
        assert!((dag_min - 0.0).abs() < 1e-6);
        assert!((dag_max - 10.0).abs() < 1e-6);
        assert!((dag_step - 0.1).abs() < 1e-6);
        assert!((dag_value - 1.3).abs() < 1e-6);
    }

    #[test]
    fn add_slider_widget_with_two_decimal_places_uses_finer_step() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":1.25}"#, 0.0, 0.0).unwrap();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputSlider { step, .. } = widget else {
            panic!("expected slider widget");
        };
        assert!((step - 0.01).abs() < 1e-6);
    }

    #[test]
    fn set_slider_value_expands_bounds_when_out_of_range() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":3.0,"min":0.0,"max":10.0,"step":1.0}"#, 0.0, 0.0).unwrap();
        host.set_slider_value(&id, 12.0);
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputSlider { value, min, max, .. } = widget else {
            panic!("expected slider widget");
        };
        assert!((value - 12.0).abs() < 1e-6);
        assert!((min - 0.0).abs() < 1e-6);
        assert!((max - 20.0).abs() < 1e-6);
    }

    #[test]
    fn ghost_widget_matches_placed_neuron_size() {
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "brep.sketch2d.circle".into(),
                extension: "brep".into(),
                name: "Sketch Circle".into(),
                abbreviation: "Circle".into(),
                icon: "emoji:⚪️".into(),
                summary: "Sketched circle profile".into(),
                inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
                outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
                ..Default::default()
            }])
            .unwrap(),
        );
        let descriptor = r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#;
        host.set_ghost_widget(descriptor, 40.0, 40.0).unwrap();
        let ghost_width = host.ghost_node.as_ref().expect("ghost").width;
        let placed_id = host.add_widget(descriptor, 80.0, 80.0).unwrap();
        let placed_width = host.dag.fixture.nodes.iter().find(|node| node.id == placed_id).expect("placed").width;
        assert!((ghost_width - placed_width).abs() < 1e-6, "ghost width {ghost_width} != placed {placed_width}");
    }

    #[test]
    fn ghost_widget_preview_and_clear() {
        let mut host = host_with_test_bridge();
        host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 42.0, 24.0).unwrap();
        let ghost = host.ghost_node.as_ref().expect("ghost");
        assert!((ghost.x - 42.0).abs() < 1e-6);
        assert!((ghost.y - 24.0).abs() < 1e-6);
        assert_eq!(ghost.name, "Add");
        assert_eq!(ghost.abbreviation, "Add");
        assert_eq!(ghost.icon, "emoji:➕️");
        host.clear_ghost_widget();
        assert!(host.ghost_node.is_none());
    }

    #[test]
    fn ghost_widget_label_overlay_matches_placed_at_micro() {
        let mut host = host_with_test_bridge();
        host.set_viewport(1280, 800, 1.0);
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "brep.sketch2d.circle".into(),
                extension: "brep".into(),
                name: "Sketch Circle".into(),
                abbreviation: "Circle".into(),
                icon: "emoji:⚪️".into(),
                summary: "Sketched circle profile".into(),
                inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
                outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
                ..Default::default()
            }])
            .unwrap(),
        );
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label("micro");
        let descriptor = r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#;
        host.set_ghost_widget(descriptor, 40.0, 40.0).unwrap();
        let ghost = host.dag.ghost_node().expect("ghost");
        assert_eq!(host.draw_lod_label(), "micro");
        assert_eq!(dag::DagDrawLod::Micro.node_label(), dag::DagNodeLabel::Name);
        assert!(dag::DagDrawLod::Micro.shows_port_labels());
        assert!(dag::DagDrawLod::Micro.shows_handles());
        let ghost_overlay_rows = host.dag.label_overlay_rows_for_node_spec(ghost, true);
        assert_eq!(ghost_overlay_rows.len(), 3);
        let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        let overlay_ghost_rows: Vec<_> = overlay["labels"].as_array().unwrap().iter().filter(|row| row["ghost"] == true).collect();
        assert_eq!(overlay_ghost_rows.len(), 3);
        let placed_node = {
            let widget = widget_from_descriptor(&serde_json::from_str::<WidgetDescriptor>(descriptor).unwrap(), "placed".into(), &host.kind_infos);
            let mut layout = crate::OrderedMap::new();
            layout.insert("placed".into(), WidgetLayout { x: 80.0, y: 80.0 });
            let mut node = widget_to_dag_node(&widget, 0, &layout, &[], &host.kind_infos);
            widget.retire_cold();
            let mut retirement = crate::retained::FlowRetirement::default();
            retirement.push(crate::retained::FlowOwner::Layouts(layout));
            retirement.retire_cold();
            fit_node_size(&mut node);
            node
        };
        let placed_rows = host.dag.label_overlay_rows_for_node_spec(&placed_node, false);
        assert_eq!(ghost_overlay_rows.len(), placed_rows.len());
        for (ghost_row, placed_row) in ghost_overlay_rows.iter().zip(placed_rows.iter()) {
            assert_eq!(ghost_row["text"], placed_row["text"]);
            assert_eq!(ghost_row["layout"], placed_row["layout"]);
            assert_eq!(ghost_row["align"], placed_row["align"]);
        }
        let mut scene = canvas::Scene::new();
        host.paint_scene(&mut scene, 1280, 800, 1.0);
    }

    #[test]
    fn rebuild_dag_preserves_ghost_overlay_at_micro() {
        let mut host = host_with_test_bridge();
        host.set_viewport(1280, 800, 1.0);
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "brep.sketch2d.circle".into(),
                extension: "brep".into(),
                name: "Sketch Circle".into(),
                abbreviation: "Circle".into(),
                icon: "emoji:⚪️".into(),
                summary: "Sketched circle profile".into(),
                inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
                outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
                ..Default::default()
            }])
            .unwrap(),
        );
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label("micro");
        host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#, 12.0, 18.0).unwrap();
        host.rebuild_dag();
        assert!(host.dag.ghost_node().is_some());
        assert_eq!(host.draw_lod_label(), "micro");
        let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        let ghost_rows: Vec<_> = overlay["labels"].as_array().unwrap().iter().filter(|row| row["ghost"] == true).collect();
        assert_eq!(ghost_rows.len(), 3);
    }

    #[test]
    fn ghost_widget_paint_scene_smoke() {
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 10.0, 20.0).unwrap();
        let mut scene = canvas::Scene::new();
        host.paint_scene(&mut scene, 800, 600, 1.0);
    }

    #[test]
    fn selection_and_preview_state_round_trip() {
        let mut host = FlowHost::default();
        host.set_selection_json(r#"["slider","add"]"#);
        let selected: Vec<String> = serde_json::from_str(&host.selected_widget_ids_json()).unwrap();
        assert_eq!(selected, vec!["slider", "add"]);
        host.set_hover(Some("add"));
        assert_eq!(host.hovered_widget_id().as_deref(), Some("add"));
        host.set_preview_off_json(r#"["add"]"#);
        assert_eq!(host.preview_off_widget_ids(), vec!["add"]);
        host.toggle_preview("add").unwrap();
        assert!(host.preview_off_widget_ids().is_empty());
    }

    #[test]
    fn channel_hover_and_selection_round_trip_at_detail_lod() {
        let mut host = host_with_test_bridge();
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label("detail");
        host.set_hover_channel(Some("add"), Some("a"));
        let hovered: dag::DagChannelRef = serde_json::from_str(&host.hovered_channel_json()).unwrap();
        assert_eq!(hovered.widget_id, "add");
        assert_eq!(hovered.port, "a");
        assert_eq!(hovered.direction, "in");
        host.set_selected_channels_json(r#"[{"widgetId":"add","port":"a","direction":"in"}]"#);
        let selected: Vec<dag::DagChannelRef> = serde_json::from_str(&host.selected_channels_json()).unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].widget_id, "add");
        assert_eq!(selected[0].port, "a");
    }

    #[test]
    fn drag_merge_node_preserves_single_fixture_widget() {
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "dictionary.merge".into(),
                extension: "dictionary".into(),
                name: "Merge".into(),
                abbreviation: "Merge".into(),
                icon: "emoji:🔀️".into(),
                summary: "Merge".into(),
                inputs: vec![],
                outputs: vec![InputSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
                variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
                ..Default::default()
            }])
            .unwrap(),
        );
        let merge_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 120.0, 80.0).unwrap();
        host.set_viewport(800, 600, 1.0);
        let merge = host.dag.fixture.nodes.iter().find(|n| n.id == merge_id).expect("merge").clone();
        let grab = Point::new(merge.x, merge.y);
        let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
        let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
        let screen = world_to_screen(&cam, &viewport, grab);
        host.pointer_down_screen(screen.x, screen.y, 0, false, false, false, false);
        host.pointer_move_screen(screen.x + 80.0, screen.y + 40.0, false, false, false);
        host.pointer_up_screen(screen.x + 80.0, screen.y + 40.0, false, false, false);
        assert_eq!(host.fixture.widgets.iter().filter(|w| widget_id_for(w) == merge_id).count(), 1);
        assert_eq!(host.dag.fixture.nodes.iter().filter(|n| n.id == merge_id).count(), 1);
        let moved = host.fixture.layout.get(&merge_id).expect("merge layout");
        assert!((moved.x - merge.x).abs() > 1.0);
    }

    #[test]
    fn ghost_widget_cleared_on_pointer_down_and_add_widget() {
        let mut host = host_with_test_bridge();
        host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 12.0, 18.0).unwrap();
        host.set_viewport(800, 600, 1.0);
        host.pointer_down_screen(120.0, 120.0, 0, false, false, false, false);
        assert!(host.ghost_node.is_none());
        host.set_ghost_widget(r#"{"kind":"inputSlider","label":"Number"}"#, 0.0, 0.0).unwrap();
        let _ = host.add_widget(r#"{"kind":"inputSlider","label":"Number"}"#, 40.0, 40.0).unwrap();
        assert!(host.ghost_node.is_none());
        assert_eq!(host.fixture.widgets.iter().filter(|w| widget_id_for(w).starts_with("slider")).count(), 2);
        assert_eq!(host.dag.fixture.nodes.iter().filter(|n| n.id == "slider").count(), 1);
    }

    #[test]
    fn delete_selection_removes_widget_from_fixture() {
        let mut host = host_with_test_bridge();
        host.dag.set_selection(&["slider".into()]);
        host.delete_selection().unwrap();
        assert!(host.fixture.widgets.iter().all(|w| widget_id_for(w) != "slider"));
        assert!(host.dag.fixture.nodes.iter().all(|n| n.id != "slider"));
    }

    #[test]
    fn node_drag_proximity_skips_wired_cut_inputs_in_flow() {
        use canvas::camera::{world_to_screen, Camera, Viewport};
        use canvas::Point;
        let mut host = FlowHost::default();
        host.set_viewport(1280, 800, 1.0);
        host.fixture.widgets = vec![
            Widget::Neuron { id: "sphere".into(), neuron_kind: "brep.prim3d.sphere".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
            Widget::Neuron { id: "torus".into(), neuron_kind: "brep.prim3d.torus".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
            Widget::Neuron { id: "cut".into(), neuron_kind: "brep.bool.cut".into(), params: Dictionary::new(), input_ports: vec!["a".into(), "b".into()], output_ports: vec![], preview: true },
        ];
        host.fixture.synapses = vec![
            SynapseSpec { id: "e1".into(), from: "sphere".into(), to: "cut".into(), from_port: "solid".into(), to_port: "a".into() },
            SynapseSpec { id: "e2".into(), from: "torus".into(), to: "cut".into(), from_port: "solid".into(), to_port: "b".into() },
        ];
        host.fixture.layout.insert("sphere".into(), WidgetLayout { x: 0.0, y: -60.0 });
        host.fixture.layout.insert("torus".into(), WidgetLayout { x: 0.0, y: 60.0 });
        host.fixture.layout.insert("cut".into(), WidgetLayout { x: 240.0, y: 0.0 });
        let solid_out = vec![InputSpec::named("S", "Sld", "solid", "Solid")];
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[
                NeuronKindInfo {
                    id: "brep.prim3d.sphere".into(),
                    extension: "brep".into(),
                    name: "Sphere".into(),
                    abbreviation: "Sphere".into(),
                    icon: "emoji:⚪️".into(),
                    summary: "Sphere".into(),
                    inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
                    outputs: solid_out.clone(),
                    ..Default::default()
                },
                NeuronKindInfo {
                    id: "brep.prim3d.torus".into(),
                    extension: "brep".into(),
                    name: "Torus".into(),
                    abbreviation: "Torus".into(),
                    icon: "emoji:🛢️".into(),
                    summary: "Torus".into(),
                    inputs: vec![InputSpec::number_default("major", 2.0, NUMBER_OPS), InputSpec::number_default("minor", 0.5, NUMBER_OPS)],
                    outputs: solid_out.clone(),
                    ..Default::default()
                },
                NeuronKindInfo {
                    id: "brep.bool.cut".into(),
                    extension: "brep".into(),
                    name: "Cut".into(),
                    abbreviation: "Cut".into(),
                    icon: "emoji:🔗️".into(),
                    summary: "Cut".into(),
                    inputs: vec![InputSpec::requires("a", &["geometry"]), InputSpec::requires("b", &["geometry"])],
                    outputs: solid_out,
                    ..Default::default()
                },
            ])
            .unwrap(),
        );
        host.rebuild_dag();
        host.dag.set_proximity_distance(160.0);
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label("normal");
        assert_eq!(host.dag.engine.edges.len(), 2, "synapses should load as engine edges");
        let cut = host.dag.fixture.nodes.iter().find(|node| node.id == "cut").expect("cut");
        let grab = Point::new(cut.x, cut.y);
        let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
        let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
        let screen = world_to_screen(&cam, &viewport, grab);
        host.pointer_down_screen(screen.x, screen.y, 0, false, false, false, false);
        host.pointer_move_screen(screen.x - 180.0, screen.y, false, false, false);
        assert!(host.dag.engine.render_snapshot().pending_edge.is_none(), "dragging wired cut near sources must not preview proximity edges");
        host.pointer_up_screen(screen.x - 180.0, screen.y, false, false, false);
        assert_eq!(host.dag.engine.edges.len(), 2);
        assert_eq!(host.fixture.synapses.len(), 2);
    }

    #[test]
    fn dag_bridge_keeps_same_named_brep_input_and_output_distinct() {
        let mut host = FlowHost::default();
        host.fixture.widgets = vec![
            Widget::Neuron { id: "extrude".into(), neuron_kind: "brep.solid.extrude".into(), params: Dictionary::new(), input_ports: vec!["wire".into(), "vector".into()], output_ports: vec![], preview: true },
            Widget::Neuron { id: "brep".into(), neuron_kind: "brep.brep".into(), params: Dictionary::new(), input_ports: vec!["brep".into(), "vertex".into(), "edge".into(), "face".into()], output_ports: vec![], preview: true },
            Widget::Neuron { id: "get".into(), neuron_kind: "list.get".into(), params: Dictionary::new(), input_ports: vec!["list".into(), "index".into(), "wrap".into()], output_ports: vec!["0".into()], preview: true },
        ];
        host.fixture.synapses = vec![
            SynapseSpec { id: "e112".into(), from: "extrude".into(), to: "brep".into(), from_port: "solid".into(), to_port: "brep".into() },
            SynapseSpec { id: "e113".into(), from: "brep".into(), to: "get".into(), from_port: "brep".into(), to_port: "list".into() },
        ];
        host.fixture.layout.insert("extrude".into(), WidgetLayout { x: 0.0, y: 0.0 });
        host.fixture.layout.insert("brep".into(), WidgetLayout { x: 200.0, y: 0.0 });
        host.fixture.layout.insert("get".into(), WidgetLayout { x: 400.0, y: 0.0 });
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[
                NeuronKindInfo {
                    id: "brep.solid.extrude".into(),
                    extension: "brep".into(),
                    name: "Extrude".into(),
                    abbreviation: "Extr".into(),
                    icon: "emoji:⬆️".into(),
                    summary: "Extrude".into(),
                    inputs: vec![InputSpec::requires("wire", &["geometry"]), InputSpec::requires("vector", &["vector"])],
                    outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
                    ..Default::default()
                },
                NeuronKindInfo {
                    id: "brep.brep".into(),
                    extension: "brep".into(),
                    name: "Brep".into(),
                    abbreviation: "Brep".into(),
                    icon: "emoji:🧊️".into(),
                    summary: "Brep".into(),
                    inputs: vec![InputSpec::requires("brep", &["brep.brep"]), InputSpec::list("vertex", &["brep.brep"]), InputSpec::list("edge", &["brep.brep"]), InputSpec::list("face", &["brep.brep"])],
                    outputs: vec![InputSpec::named("B", "Brp", "brep", "Brep")],
                    ..Default::default()
                },
                NeuronKindInfo {
                    id: "list.get".into(),
                    extension: "list".into(),
                    name: "Get".into(),
                    abbreviation: "Get".into(),
                    icon: "emoji:📋️".into(),
                    summary: "Get".into(),
                    inputs: vec![InputSpec::list("list", &["list.get"]), InputSpec::number_default("index", 0.0, &["list.get"]), InputSpec::boolean_default("wrap", false, &["list.get"])],
                    outputs: vec![InputSpec::named("V", "Val", "value", "ListValue")],
                    ..Default::default()
                },
            ])
            .unwrap(),
        );
        host.rebuild_dag();
        let incoming = host.dag.engine.edges.get(&112).expect("incoming brep edge");
        let outgoing = host.dag.engine.edges.get(&113).expect("outgoing brep edge");
        let incoming_target = host.dag.engine.handles.get(&incoming.target).expect("incoming target");
        let outgoing_source = host.dag.engine.handles.get(&outgoing.source).expect("outgoing source");
        assert_eq!(incoming_target.role, HandleRole::Target);
        assert_eq!(outgoing_source.role, HandleRole::Source);
    }

    #[test]
    fn delete_selection_removes_selected_edge_from_fixture() {
        let mut host = host_with_test_bridge();
        let synapse_count_before = host.fixture.synapses.len();
        assert!(synapse_count_before > 0);
        let edge_id = *host.dag.engine.edges.keys().next().expect("edge");
        host.dag.engine.selection.edge_ids.insert(edge_id);
        assert!(host.has_selection());
        host.delete_selection().unwrap();
        assert!(host.fixture.synapses.len() < synapse_count_before);
        assert!(!host.has_selection());
    }

    #[test]
    fn delete_selection_removes_edge_selected_by_synapse_id_domain() {
        let mut host = host_with_test_bridge();
        let before = host.fixture.synapses.len();
        host.dag.set_selection_domains_json(r#"{"nodes":[],"edges":["s1"],"handles":[]}"#);
        assert!(host.has_selection(), "synapse id s1 must map into engine edge selection");
        host.delete_selection().unwrap();
        assert!(host.fixture.synapses.len() < before);
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.id == "s1"));
    }

    #[test]
    fn align_selection_left_aligns_selected_widget_layout() {
        let mut host = host_with_test_bridge();
        host.move_widget("slider", -120.0, 20.0).unwrap();
        host.move_widget("add", 180.0, -40.0).unwrap();
        host.dag.set_selection(&["slider".into(), "add".into()]);
        host.align_selection("alignLeft").unwrap();
        let slider = host.dag.fixture.nodes.iter().find(|node| node.id == "slider").expect("slider");
        let add = host.dag.fixture.nodes.iter().find(|node| node.id == "add").expect("add");
        let slider_left = slider.x - slider.width * 0.5;
        let add_left = add.x - add.width * 0.5;
        assert!((slider_left - add_left).abs() < 1e-6, "left edges should match after alignLeft");
        assert!(host.fixture.layout.contains_key("slider"));
        assert!(host.fixture.layout.contains_key("add"));
    }

    #[test]
    fn add_input_port_inserts_variadic_slot() {
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "dictionary.merge".into(),
                extension: "dictionary".into(),
                name: "Merge".into(),
                abbreviation: "Merge".into(),
                icon: "emoji:🔀️".into(),
                summary: "Merge".into(),
                inputs: vec![],
                outputs: vec![InputSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
                variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
                ..Default::default()
            }])
            .unwrap(),
        );
        let merge_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 0.0, 0.0).unwrap();
        host.add_input_port(&merge_id, 1).unwrap();
        let widget = host.fixture.widgets.iter().find(|widget| widget_id_for(widget) == merge_id).expect("merge");
        let Widget::Neuron { input_ports, .. } = widget else { panic!("neuron") };
        assert_eq!(input_ports.len(), 3);
    }

    #[test]
    fn add_output_port_inserts_variadic_get_slot() {
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "list.get".into(),
                extension: "list".into(),
                name: "Get".into(),
                abbreviation: "Get".into(),
                icon: "emoji:📋️".into(),
                summary: "Reads consecutive values by index".into(),
                inputs: vec![InputSpec::list("list", &["list.get"]), InputSpec::number_default("index", 0.0, &["list.get"]), InputSpec::boolean_default("wrap", false, &["list.get"])],
                outputs: vec![InputSpec::named("V", "Val", "value", "ListValue")],
                variadic_output: Some(neural::VariadicSpec { slot_key: "value".into(), min: 1, max: None }),
                ..Default::default()
            }])
            .unwrap(),
        );
        let get_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"list.get"}"#, 0.0, 0.0).unwrap();
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == get_id).expect("get");
        let labels: Vec<&str> = node.outputs().iter().map(|port| port.label.as_str()).collect();
        assert_eq!(labels, vec!["i"]);
        host.add_output_port(&get_id, 1).unwrap();
        let widget = host.fixture.widgets.iter().find(|widget| widget_id_for(widget) == get_id).expect("get");
        let Widget::Neuron { output_ports, .. } = widget else { panic!("neuron") };
        assert_eq!(output_ports.len(), 2);
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == get_id).expect("get");
        let labels: Vec<&str> = node.outputs().iter().map(|port| port.label.as_str()).collect();
        assert_eq!(labels, vec!["i", "i+1"]);
    }

    #[test]
    fn add_widget_with_explicit_id() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","id":"custom_slider","value":2.0}"#, 0.0, 0.0).unwrap();
        assert_eq!(id, "custom_slider");
    }

    #[test]
    fn insert_between_rewires_downstream_and_connects_anchor() {
        let mut host = host_with_test_bridge();
        let mid = host.add_widget(r#"{"kind":"neuron","id":"mid","neuronKind":"math.passThrough"}"#, 120.0, 0.0).unwrap();
        host.insert_between("slider", "number", &mid, "number", "number").unwrap();
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "mid"));
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "mid" && synapse.to == "add"));
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "add" && synapse.to == "preview"));
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "add"));
    }

    #[test]
    fn insert_between_preserves_existing_mid_inputs() {
        let mut host = host_with_test_bridge();
        let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 120.0, 0.0).unwrap();
        host.connect_ports("slider", "number", &variable_id, "width").unwrap();
        host.insert_between("slider", "number", &variable_id, "width", "width").unwrap();
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == variable_id && synapse.to_port == "width"));
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.from == variable_id && synapse.to == variable_id));
    }

    #[test]
    fn make_space_shifts_widgets_right_of_anchor() {
        let mut host = host_with_test_bridge();
        host.fixture.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
        host.fixture.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
        host.fixture.layout.insert("preview".into(), WidgetLayout { x: 400.0, y: 0.0 });
        host.rebuild_dag();
        host.make_space("slider", 100.0, 0.0).unwrap();
        assert!((host.fixture.layout.get("slider").expect("slider").x - 0.0).abs() < 1e-6);
        assert!((host.fixture.layout.get("add").expect("add").x - 300.0).abs() < 1e-6);
        assert!((host.fixture.layout.get("preview").expect("preview").x - 500.0).abs() < 1e-6);
    }

    #[test]
    fn set_neuron_params_merges_into_eval_input() {
        let mut host = host_with_test_bridge();
        let preview_synapse = host.fixture.synapses.iter().find(|synapse| synapse.from == "add" && synapse.to == "preview").map(|synapse| synapse.id.clone()).expect("preview synapse");
        host.disconnect(&preview_synapse).unwrap();
        let id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough"}"#, 100.0, 0.0).unwrap();
        host.connect_ports(&id, "number", "preview", "").unwrap();
        host.set_neuron_params(&id, r#"{"number":{"$schema":"number","value":7.5}}"#).unwrap();
        host.evaluate_internal();
        assert_eq!(host.preview_text(), "7.5");
    }

    #[test]
    fn cluster_ports_from_contract() {
        let inner = Tree {
            neurons: vec![
                Neuron::with_kind("in_a", INPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("a".into()))).insert("operators", NeuralValue::Atom(Atom::String("core.number".into())))),
                Neuron::with_kind("out_sum", OUTPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("sum".into()))).insert("operators", NeuralValue::Atom(Atom::String("core.number".into())))),
            ],
            synapses: vec![],
        };
        let widget = Widget::Cluster { id: "cluster".into(), name: "Add cluster".into(), tree: inner, flow: FlowGui::default() };
        let (inputs, outputs, _, _) = widget_io_ports(&widget, &[], &HashMap::new());
        assert_eq!(inputs.len(), 1);
        assert_eq!(outputs.len(), 1);
        assert_eq!(inputs[0].id, "a");
        assert_eq!(outputs[0].id, "sum");
    }

    #[test]
    fn variable_relay_evaluates_through_flow_host() {
        let mut host = host_with_test_bridge();
        let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 0.0, 0.0).unwrap();
        let slider_id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":4.0}"#, -200.0, 0.0).unwrap();
        host.connect_ports(&slider_id, "number", &variable_id, "width").unwrap();
        let eval_json = host.evaluate().expect("evaluate");
        let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
        let width = parsed.get(&variable_id).and_then(|entry| entry.get("out")).and_then(|out| out.get("width")).expect("variable width output");
        assert_eq!(width.get("$schema").and_then(|value| value.as_str()), Some("number"));
    }

    #[test]
    fn collapse_uses_variable_name_as_cluster_input_port() {
        let mut host = host_with_test_bridge();
        host.fixture.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
        let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 100.0, 0.0).unwrap();
        host.fixture.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
        host.fixture.synapses.retain(|synapse| synapse.from != "slider" || synapse.to != "add");
        host.connect_ports("slider", "number", &variable_id, "width").unwrap();
        host.connect_ports(&variable_id, "width", "add", "a").unwrap();
        host.rebuild_dag();
        let cluster_id = host.collapse_selection(&[variable_id.clone(), "add".into()]).unwrap();
        let cluster = host
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Cluster { id, tree, .. } if id == &cluster_id => Some(tree.clone()),
                _ => None,
            })
            .expect("cluster");
        let (inputs, _) = cluster.contract();
        assert!(inputs.iter().any(|port| port.name == "width"));
        host.explode_cluster(&cluster_id).unwrap();
        assert!(host.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Variable { name, .. } if name == "width")));
    }

    #[test]
    fn collapse_then_explode_round_trips() {
        let mut host = host_with_test_bridge();
        host.fixture.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
        host.fixture.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
        host.rebuild_dag();
        let cluster_id = host.collapse_selection(&["slider".into(), "add".into()]).unwrap();
        assert!(host.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Cluster { id, .. } if id == &cluster_id)));
        host.explode_cluster(&cluster_id).unwrap();
        assert!(host.fixture.widgets.iter().any(|widget| widget_id_for(widget).starts_with(&format!("{cluster_id}/"))));
        assert!(!host.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Cluster { .. })));
    }

    #[test]
    fn rectangle_extrude_fixture_port_labels_follow_draw_lod() {
        let _guard = RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|error| error.into_inner());
        // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
        // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
        // parses (`FlowHost::parse_fixture_json`), decoupled from procedural's document format.
        let json = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 140, "y": -60, "zoom": 2.2 },
  "widgets": [
    { "kind": "inputSlider", "id": "width", "label": "Width", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "height", "label": "Height", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "distance", "label": "Distance", "value": 3, "min": 0.1, "max": 10, "step": 0.1 },
    {
      "kind": "neuron",
      "id": "rect",
      "neuronKind": "brep.curve.rectangle",
      "params": {},
      "input_ports": ["width", "height"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "vector",
      "neuronKind": "math.vector",
      "params": {},
      "input_ports": ["x", "y", "z"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "extrude",
      "neuronKind": "brep.solid.extrude",
      "params": {},
      "input_ports": ["wire", "vector"],
      "preview": true
    },
    {
      "kind": "neuron",
      "id": "volume",
      "neuronKind": "brep.measure.volume",
      "params": {},
      "input_ports": ["geometry"],
      "preview": false
    }
  ],
  "synapses": [
    { "id": "e1", "from": "width", "to": "rect", "fromPort": "number", "toPort": "width" },
    { "id": "e2", "from": "height", "to": "rect", "fromPort": "number", "toPort": "height" },
    { "id": "e3", "from": "rect", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e4", "from": "distance", "to": "vector", "fromPort": "number", "toPort": "z" },
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "volume", "fromPort": "solid", "toPort": "geometry" }
  ],
  "layout": {
    "rect": { "x": 120, "y": -40 },
    "vector": { "x": 200, "y": 20 },
    "extrude": { "x": 280, "y": -40 },
    "volume": { "x": 360, "y": -40 },
    "width": { "x": 40, "y": -60 },
    "height": { "x": 40, "y": -20 },
    "distance": { "x": 120, "y": 20 }
  }
}
"#;
        let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
        let mut host = FlowHost::from_fixture(fixture);
        host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
        host.set_viewport(1280, 800, 1.0);
        host.fixture.camera.zoom = 1.0;
        host.rebuild_dag();
        let mut port_texts = |lod: &str| -> Vec<String> {
            host.dag.set_automatic_lod(false);
            host.dag.set_forced_draw_lod_label(lod);
            let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
            raw["labels"].as_array().expect("labels").iter().filter(|row| row["kind"] == "port").filter_map(|row| row["text"].as_str().map(str::to_string)).collect()
        };
        let normal = port_texts("normal");
        assert!(normal.iter().any(|text| text.ends_with("wid")), "normal ports: {normal:?}");
        assert!(normal.iter().any(|text| text.ends_with("wir")), "normal ports: {normal:?}");
        let detail = port_texts("detail");
        assert!(detail.iter().any(|text| text.ends_with("width")), "detail ports: {detail:?}");
        assert!(detail.iter().any(|text| text.ends_with("wire")), "detail ports: {detail:?}");
        let micro = port_texts("micro");
        assert!(micro.iter().any(|text| text.ends_with("RectangleWire")), "micro ports: {micro:?}");
        assert!(micro.iter().any(|text| text.ends_with("ExtrudedSolid")), "micro ports: {micro:?}");
    }

    #[test]
    fn rectangle_extrude_fixture_evaluates_solid_output() {
        let _guard = RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|error| error.into_inner());
        // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
        // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
        // parses (`FlowHost::parse_fixture_json`), decoupled from procedural's document format.
        let json = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 140, "y": -60, "zoom": 2.2 },
  "widgets": [
    { "kind": "inputSlider", "id": "width", "label": "Width", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "height", "label": "Height", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "distance", "label": "Distance", "value": 3, "min": 0.1, "max": 10, "step": 0.1 },
    {
      "kind": "neuron",
      "id": "rect",
      "neuronKind": "brep.curve.rectangle",
      "params": {},
      "input_ports": ["width", "height"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "vector",
      "neuronKind": "math.vector",
      "params": {},
      "input_ports": ["x", "y", "z"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "extrude",
      "neuronKind": "brep.solid.extrude",
      "params": {},
      "input_ports": ["wire", "vector"],
      "preview": true
    },
    {
      "kind": "neuron",
      "id": "volume",
      "neuronKind": "brep.measure.volume",
      "params": {},
      "input_ports": ["geometry"],
      "preview": false
    }
  ],
  "synapses": [
    { "id": "e1", "from": "width", "to": "rect", "fromPort": "number", "toPort": "width" },
    { "id": "e2", "from": "height", "to": "rect", "fromPort": "number", "toPort": "height" },
    { "id": "e3", "from": "rect", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e4", "from": "distance", "to": "vector", "fromPort": "number", "toPort": "z" },
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "volume", "fromPort": "solid", "toPort": "geometry" }
  ],
  "layout": {
    "rect": { "x": 120, "y": -40 },
    "vector": { "x": 200, "y": 20 },
    "extrude": { "x": 280, "y": -40 },
    "volume": { "x": 360, "y": -40 },
    "width": { "x": 40, "y": -60 },
    "height": { "x": 40, "y": -20 },
    "distance": { "x": 120, "y": 20 }
  }
}
"#;
        let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
        let mut host = FlowHost::from_fixture(fixture);
        host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
        let eval_json = host.evaluate().expect("evaluate");
        let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
        let solid = parsed.get("extrude").and_then(|entry| entry.get("out")).and_then(|out| out.get("solid").or_else(|| out.get("S"))).expect("extrude solid output");
        assert_eq!(solid.get("$schema").and_then(|v| v.as_str()), Some("geometry"));
        assert_eq!(solid.get("kind").and_then(|v| v.as_str()), Some("solid"));
    }

    #[test]
    fn hexagonal_mushroom_fixture_reports_extruded_solid_output() {
        // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
        // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
        // parses (`FlowHost::parse_fixture_json`), decoupled from procedural's document format.
        let json = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 94.75581571737445, "y": -97.50833134679668, "zoom": 1.7844325616011099 },
  "widgets": [
    { "kind": "inputSlider", "id": "height", "label": "Column Height", "value": 6.0, "min": 0.0, "max": 10.0, "step": 0.5, "unit": "m" },
    { "kind": "inputSlider", "id": "radius", "label": "Profile Radius", "value": 0.5, "min": 0.1, "max": 2.0, "step": 0.05, "unit": "m" },
    { "kind": "inputSlider", "id": "sides", "label": "Side Count", "value": 6.0, "min": 3.0, "max": 12.0, "step": 1.0 },
    { "kind": "neuron", "id": "profile", "neuronKind": "brep.curve.polygon", "params": {}, "input_ports": ["radius", "sides"], "preview": false },
    { "kind": "neuron", "id": "extrusion-axis", "neuronKind": "math.vector", "params": {}, "input_ports": ["x", "y", "z"], "preview": false },
    { "kind": "neuron", "id": "extrude", "neuronKind": "brep.solid.extrude", "params": {}, "input_ports": ["wire", "vector"], "preview": true },
    { "kind": "outputPreview", "id": "column-preview", "preview": {}, "expanded": [] }
  ],
  "synapses": [
    { "id": "e1", "from": "height", "to": "extrusion-axis", "fromPort": "number", "toPort": "z" },
    { "id": "e2", "from": "radius", "to": "profile", "fromPort": "number", "toPort": "radius" },
    { "id": "e3", "from": "sides", "to": "profile", "fromPort": "number", "toPort": "sides" },
    { "id": "e4", "from": "profile", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e5", "from": "extrusion-axis", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "column-preview", "fromPort": "solid", "toPort": "" }
  ],
  "layout": {
    "height": { "x": -197.1913555449187, "y": -102.70789997839545 },
    "radius": { "x": -156.03796288966, "y": -177.3373596163105 },
    "sides": { "x": -156.43467044109153, "y": -155.28679730672846 },
    "profile": { "x": -64.49671116929301, "y": -163.40310309861746 },
    "extrusion-axis": { "x": -65.26327021036892, "y": -116.45687403531778 },
    "extrude": { "x": 34.842068675720895, "y": -154.18083645790136 },
    "column-preview": { "x": 237.4197774877085, "y": -103.14518978933415 }
  }
}
"#;
        let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
        let mut host = FlowHost::from_fixture(fixture);
        host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
        let eval_json = host.evaluate().expect("evaluate");
        let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
        let solid = parsed.get("extrude").and_then(|entry| entry.get("out")).and_then(|out| out.get("solid").or_else(|| out.get("S"))).expect("extrude solid output");
        assert_eq!(solid.get("$schema").and_then(serde_json::Value::as_str), Some("geometry"));
        assert_eq!(solid.get("kind").and_then(serde_json::Value::as_str), Some("solid"));
        let handle = solid.get("handle").and_then(serde_json::Value::as_str).expect("solid handle");
        assert!(handle.starts_with("solid-"));
        let mesh = crate::tessellate_geometry(handle, 0.05).expect("solid mesh");
        assert!(!mesh.positions.is_empty());
        assert!(mesh.indices.len() >= 3);
    }

    #[test]
    fn compiled_wire_literal_includes_operator_kinds() {
        let host = host_with_test_bridge();
        let text = host.compiled_wire_literal();
        assert!(text.contains("core.number"));
        assert!(text.contains("math.add"));
    }

    #[test]
    fn flow_fixture_to_form_spec_maps_input_widgets() {
        use self::forms_bridge::flow_fixture_to_form_spec;
        let fixture = FlowFixture::default();
        let spec = flow_fixture_to_form_spec(&fixture);
        let kinds: Vec<&str> = spec.steps[0].blocks.iter().map(|question| question.kind.as_str()).collect();
        assert!(kinds.contains(&"slider"));
    }

    #[test]
    fn apply_generation_values_to_fixture_patches_slider_value() {
        use self::forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec};
        let fixture = FlowFixture::default();
        let spec = flow_fixture_to_form_spec(&fixture);
        let slider_id = spec.steps[0].blocks.iter().find(|question| question.kind == "slider").map(|question| question.id.clone()).expect("slider question");
        let fixture_json = serde_json::to_string(&fixture).expect("fixture json");
        let mut values = serde_json::Map::new();
        values.insert(slider_id.clone(), serde_json::json!(8.0));
        let patched = apply_generation_values_to_fixture(&fixture_json, &values);
        let reparsed: serde_json::Value = serde_json::from_str(&patched).expect("patched json");
        let slider = reparsed.get("widgets").and_then(|widgets| widgets.as_array()).and_then(|widgets| widgets.iter().find(|widget| widget.get("id").and_then(|id| id.as_str()) == Some(slider_id.as_str()))).expect("slider widget");
        assert_eq!(slider.get("value").and_then(|value| value.as_f64()), Some(8.0));
    }
}
// #endregion 🔖️Tests
