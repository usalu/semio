//! 🖥️ Flow host: canvas editing, evaluation session, and host errors.

use crate::infinite::board::ports::directed_dag as dag;
use crate::infinite::canvas;
use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

use dag::{fit_node_size, would_create_cycle, DagHost, DagLayoutOptions};
use semio_framework_artifact_infinite_dag::{dag_fixture_execution_rows, dag_fixture_to_wire_literal, DagFixture, DagFixtureEdge, DagNodeKind, DagNodeSpec, EdgeRouteStyle, IoPortSpec};
use semio_framework_artifact_flow_flow::{widget_id_for, FlowMutation, FlowStore, ReplaceFlowFixture, FLOW_DOCUMENT_SCHEMA};
use graph::dsl::{WireEdge, WireNode};
use graph::manifest::{PropertyBag, PropertyValue};
use neural::{
    channel_output, compute_dirty_set, Atom, BudgetedEval, ColdRetire, Dictionary, EvalChannels, EvalError, EvalStepBudget, Evaluator, NeuralCache, Neuron, OperatorInfo, Synapse, Tree, TreeSnapshot,
    Value as NeuralValue, CLUSTER_KIND, INPUT_KIND, OUTPUT_KIND,
};
use serde::{Deserialize, Serialize};

use crate::artifact::*;
use crate::bridge::*;
use crate::catalogue::*;
use crate::drawing::*;
use crate::os_store::{create_document_envelope, ArtifactCommand, MemberStoreOwner, SnapshotRetirementStep, SpaceMember};
use crate::registry::*;
use semio_framework::io::resolve_ready;

// #region ⚠️ Errors
/// 🧯️ `FlowHost`'s error type — wraps JSON codec failures, the `dag` crate's own `DagError`, and
/// this crate's own graph-editing validation failures. Every variant's Display text is byte-for-byte
/// identical to the `String` it replaces, so downstream `.to_string()` call sites and JSON error
/// envelopes are unaffected.
#[derive(Debug)]
pub enum FlowCoreError {
    /// 🌉️ Holds the formatted message rather than a codec error type directly — `pack::json`'s
    /// `JsonError` (syntax) and the value derive's `ValueError` (shape) both fold into this,
    /// matching the Display text `serde_json::Error` used to produce.
    Json(String),
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
    MinInputPorts {
        widget: String,
        min: usize,
    },
    MinOutputPorts {
        widget: String,
        min: usize,
    },
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
            Self::Json(error) => formatter.write_str(error),
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
            Self::Dag(error) => Some(error),
            _ => None,
        }
    }
}

impl From<crate::os_pack::json::JsonError> for FlowCoreError {
    fn from(error: crate::os_pack::json::JsonError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<crate::os_dsl::ValueError> for FlowCoreError {
    fn from(error: crate::os_dsl::ValueError) -> Self {
        Self::Json(error.to_string())
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
    /// 🧠️ The operator catalogue this host indexes, SHARED with every other live host: it is a pure
    /// projection of the extension registry, ~108 kB of it, and an evaluation tick that rebuilt its
    /// own copy paid 11-26 ms per tick for a map nobody had changed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    kind_infos: Arc<HashMap<String, OperatorInfo>>,
    neural_cache: Arc<NeuralCache>,
    previous_snapshot: Option<TreeSnapshot>,
    previous_channels: Option<EvalChannels>,
    /// 🔢 Which replacement of the process-wide flow extension registry
    /// `previous_snapshot`/`previous_channels` were computed against. An unchanged TREE is not an
    /// unchanged EVALUATION: the operator table the tree is dispatched through is process-wide
    /// state that a contribution install replaces underneath a host, so the incremental fast path's
    /// premise ("nothing that could change the answer has changed") is false the moment the
    /// generation moves — a node that faulted `unknown kind` against the empty registry would
    /// otherwise keep its fault forever, because its tree never changed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    baseline_registry_generation: u64,
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
    /// 🧹️ Cold owner for neural values this host DISPLACES while it is live — the previous tick's
    /// `outputs` map and `previous_channels`, and the parameter bag a `set_neuron_params` merge
    /// replaces. Those roots are SHARED with [`NeuralCache`] and with the current tick's own
    /// evaluation, so they may not be retired at the moment they are displaced; they wait here and
    /// are drained at the START of the next evaluation (`drain_displaced`) or, if none follows, when
    /// the host itself closes ([`FlowHostRetirement::new`] takes this frontier over). The frontier
    /// therefore holds at most one tick's displacement and never outlives the host
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️unit-suite-3d-2026-09-09.md` §5).
    displaced: neural::ValueRetirement,
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
    pub fn from_fixture_with_cache(fixture: FlowFixture, neural_cache: Arc<NeuralCache>) -> Self {
        Self::from_fixture_with_cache_and_infos(fixture, neural_cache, Arc::default())
    }

    /// 🏠️ Builds a host that ALREADY indexes `kind_infos`. The ONE construction path for a caller
    /// that would set the operator catalogue immediately afterwards: the bare constructor builds its
    /// dag against an empty catalogue, and the setter's own `rebuild_dag` then throws that work away
    /// — twice per evaluation tick (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn from_fixture_with_cache_and_infos(mut fixture: FlowFixture, neural_cache: Arc<NeuralCache>, kind_infos: Arc<HashMap<String, OperatorInfo>>) -> Self {
        dedupe_fixture_widgets(&mut fixture);
        let mut host = Self {
            fixture,
            dag: DagHost::from_fixture(DagFixture { schema: "dag.fixture".into(), camera: semio_framework_artifact_infinite_dag::DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            outputs: BTreeMap::new(),
            export_payloads: BTreeMap::new(),
            last_eval_json: String::new(),
            eval_bridge: None,
            host_catalogue_json: String::new(),
            kind_infos,
            neural_cache,
            previous_snapshot: None,
            previous_channels: None,
            baseline_registry_generation: flow_extension_registry_generation(),
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
            displaced: neural::ValueRetirement::default(),
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
            self.displace_eval_state();
            self.last_eval_json.clear();
        }
        self.pan_anchor = None;
        self.ghost_node = None;
        self.rebuild_dag();
        self.refresh_interaction_projection();
        if reset_history {
            if let Some(store) = self.history_store.as_mut() {
                let envelope = create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow-host", self.fixture.clone(), None);
                resolve_ready(store.reset(envelope, Vec::new(), Vec::new())).expect("failed to reset flow history store");
                store.install_document_store_owners_exact(FlowFixture::member_store_owners());
            }
            self.pending_history_baseline = None;
            self.pending_change = false;
            self.gesture_active = false;
        }
    }

    pub fn parse_fixture_json(json: &str) -> Result<FlowFixture, FlowCoreError> {
        Ok(crate::os_pack::json::from_json_str(json)?)
    }

    pub fn fixture_json(&self) -> Result<String, FlowCoreError> {
        Ok(crate::os_pack::json::to_json_string(&self.fixture))
    }

    pub fn document(&self) -> FlowArtifact {
        self.fixture.to_artifact()
    }

    pub fn catalogue_json(&self) -> Result<String, FlowCoreError> {
        let sections = merge_catalogue_sections(&self.host_catalogue_json)?;
        Ok(crate::os_pack::json::to_json_string(&sections))
    }

    pub fn set_host_catalogue_json(&mut self, json: &str) {
        self.host_catalogue_json = json.to_string();
    }

    /// 🧹️ Installs a new operator catalogue and retires the one it displaces — but ONLY when this
    /// host was its last owner. The catalogue is shared by every live host
    /// ([`FlowHost::kind_infos`]), and its `ChannelSpec::default` values are `Value`s that fail
    /// closed on a bare drop, so a plain assignment aborted the process the moment a host swapped
    /// a uniquely-owned catalogue out (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn install_kind_infos(&mut self, kind_infos: Arc<HashMap<String, OperatorInfo>>) {
        if let Some(displaced) = Arc::into_inner(std::mem::replace(&mut self.kind_infos, kind_infos)) {
            displaced.retire_cold();
        }
        self.rebuild_dag();
    }

    pub fn set_neuron_kind_infos_json(&mut self, json: &str) {
        self.install_kind_infos(Arc::new(if json.trim().is_empty() { HashMap::new() } else { crate::os_pack::json::from_json_str::<Vec<OperatorInfo>>(json).map(|items| items.into_iter().map(|info| (info.id.clone(), info)).collect()).unwrap_or_default() }));
    }

    /// 🧠️ Same as `set_neuron_kind_infos_json` but over the already-built id-keyed map — the ONE
    /// in-process path, because the JSON form of this catalogue is ~108 kB and an evaluation tick
    /// that serialized and re-parsed it spent 11-26 ms per tick doing nothing else
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn set_neuron_kind_info_map(&mut self, infos: Arc<HashMap<String, OperatorInfo>>) {
        self.install_kind_infos(infos);
    }

    /// 🧠️ Same as `set_neuron_kind_infos_json` but over the typed `NodeGraphScene.operators` records.
    pub fn set_neuron_kind_infos(&mut self, infos: &[ui_wgpu::wgpu::NodeGraphOperatorRecord]) {
        self.install_kind_infos(Arc::new(infos.iter().map(|record| (record.id.clone(), node_graph_operator_record_to_operator_info(record))).collect()));
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
        let evaluated_generation = flow_extension_registry_generation();
        let converged = self.probe_eval_outputs_converged(&tree, &seeds, &dirty, &channels);
        tree.retire_cold();
        seeds.retire_cold();
        self.last_eval_json = json.to_string();
        if converged {
            let displaced_outputs = std::mem::replace(&mut self.outputs, channels.outputs.clone());
            self.displaced.push_dictionaries(displaced_outputs);
            self.apply_preview_outputs(&channels.outputs);
            self.apply_export_outputs(&channels.outputs);
            self.baseline_registry_generation = evaluated_generation;
            self.previous_snapshot = Some(snapshot);
            if let Some(displaced_channels) = self.previous_channels.replace(channels) {
                self.displaced.push_channels(displaced_channels);
            }
            self.dag.clear_computing();
        } else {
            channels.retire_cold();
            self.refresh_computing_chrome_from_pending();
        }
    }

    fn probe_eval_outputs_converged(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>, dirty: &HashSet<String>, channels: &EvalChannels) -> bool {
        let registry = flow_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        let mut probe_never_dispatches = |kind: &str, _: &Dictionary| -> Result<Dictionary, EvalError> { Err(EvalError::InvalidInput(format!("apply_eval_outputs_json probed a dispatch for {kind}"))) };
        match evaluator.evaluate_channels_budgeted(tree, seeds, &self.kind_infos, &mut probe_never_dispatches, &self.neural_cache, dirty, Some(channels), EvalStepBudget::PROBE) {
            Ok(BudgetedEval { remaining, channels, .. }) => {
                channels.retire_cold();
                remaining.is_empty()
            }
            Err(_) => false,
        }
    }

    /// 🧵️ Installs a durable eval baseline from an off-thread driver onto this ephemeral host.
    ///
    /// `registry_generation` is the flow extension registry replacement the driver's baseline was
    /// computed against, and it travels WITH the baseline rather than being read from the live
    /// registry here: an ephemeral host is rebuilt after the install that bumped the generation, so
    /// reading "now" would tell every such host its inherited baseline is current when it is
    /// precisely the one that is not (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn install_eval_baseline(&mut self, snapshot: Option<TreeSnapshot>, channels: Option<EvalChannels>, registry_generation: u64) {
        self.baseline_registry_generation = registry_generation;
        self.previous_snapshot = snapshot;
        if let Some(displaced_channels) = std::mem::replace(&mut self.previous_channels, channels) {
            self.displaced.push_channels(displaced_channels);
        }
        if let Some(restored) = self.previous_channels.as_ref().map(|channels| channels.outputs.clone()) {
            let displaced_outputs = std::mem::replace(&mut self.outputs, restored);
            self.displaced.push_dictionaries(displaced_outputs);
        }
    }

    /// 🧹️ Hands every value displaced since the last drain to the artifact's bounded retirement
    /// ladder. Called at the START of an evaluation tick, so a root the CURRENT tick's `NeuralCache`
    /// or `previous_channels` still reads is never torn down under it — the frontier therefore holds
    /// at most one tick's displacement, and what is left when the host closes is taken over by
    /// [`FlowHostRetirement::new`].
    fn drain_displaced(&mut self) {
        while !matches!(self.displaced.close_step(64, 65_536), neural::ValueRetirementStep::Complete) {}
    }

    /// 🧹️ Displaces the whole evaluation baseline — `outputs`, `export_payloads`, the tree snapshot
    /// and the channel pair — onto the frontier. `BTreeMap<String, Dictionary>::clear` is a bare drop
    /// of every `Dictionary` in it, which aborts the worker as soon as one of them is a final owner.
    fn displace_eval_state(&mut self) {
        let outputs = std::mem::take(&mut self.outputs);
        self.displaced.push_dictionaries(outputs);
        let export_payloads = std::mem::take(&mut self.export_payloads);
        self.displaced.push_dictionaries(export_payloads);
        if let Some(snapshot) = self.previous_snapshot.take() {
            self.displaced.push_snapshot(snapshot);
        }
        if let Some(channels) = self.previous_channels.take() {
            self.displaced.push_channels(channels);
        }
    }

    /// 🧵️ Captures this host's eval baseline for persistence on a durable driver.
    pub fn eval_baseline(&self) -> (Option<TreeSnapshot>, Option<EvalChannels>) {
        (self.previous_snapshot.clone(), self.previous_channels.clone())
    }

    /// 🔢 The flow extension registry replacement this host's eval baseline was computed against.
    pub fn eval_baseline_registry_generation(&self) -> u64 {
        self.baseline_registry_generation
    }

    /// 🔢 Whether the incremental baseline may still be believed — i.e. whether the operator table
    /// it was dispatched through is the one installed right now.
    ///
    /// `compute_dirty_set` diffs the TREE, and a contributed operator arriving in a registry
    /// replacement changes no tree at all: the operator table is process-wide state swapped out
    /// underneath a live host. So a stale generation must make the WHOLE tree dirty and drop the
    /// previous channels, not merely refuse the skip — a refused skip that still diffs against the
    /// old snapshot computes an empty dirty set and dispatches nothing, which is the same stale
    /// answer by a longer road (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn baseline_is_current(&self) -> bool {
        self.baseline_registry_generation == flow_extension_registry_generation()
    }

    /// ⏮️ The snapshot to diff this evaluation against — `None` once the registry it was computed
    /// against has been replaced, which makes every node dirty.
    fn current_baseline_snapshot(&self) -> Option<&TreeSnapshot> {
        self.previous_snapshot.as_ref().filter(|_| self.baseline_is_current())
    }

    /// ⏮️ The channels this evaluation may free-ride on, under the same condition.
    fn current_baseline_channels(&self) -> Option<&EvalChannels> {
        self.previous_channels.as_ref().filter(|_| self.baseline_is_current())
    }

    /// ⚡️ Whether the incremental fast path may skip this evaluation entirely: nothing is dirty, a
    /// full channel baseline is held, and outputs are published.
    ///
    /// One predicate, read by every caller that owns a fast path ([`FlowHost::evaluate_step`] and
    /// [`FlowHost::pending_eval_widget_ids`]), so the two can never drift apart.
    fn baseline_answers_everything(&self, dirty: &HashSet<String>) -> bool {
        dirty.is_empty() && self.current_baseline_channels().is_some() && !self.outputs.is_empty()
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
        let descriptor: WidgetDescriptor = crate::os_pack::json::from_json_str(descriptor_json)?;
        let id: String = "__ghost__".into();
        let widget = widget_from_descriptor(&descriptor, id.clone(), &self.kind_infos);
        let mut layout = crate::OrderedMap::new();
        layout.insert(id, WidgetLayout { x: world_x, y: world_y });
        let mut node = widget_to_dag_node(&widget, 0, &layout, &[], &self.kind_infos, widget_node_size(&widget, &[], &self.kind_infos));
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
        let descriptor: WidgetDescriptor = crate::os_pack::json::from_json_str(descriptor_json)?;
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
        let patch: Dictionary = crate::os_pack::json::from_json_str(params_json)?;
        let merged = match self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id) {
            Some(Widget::Neuron { params, .. }) => Ok(std::mem::replace(params, params.merge(&patch))),
            Some(_) => Err(FlowCoreError::NotNeuron(widget_id.to_string())),
            None => Err(FlowCoreError::UnknownWidget(widget_id.to_string())),
        };
        self.displaced.push_dictionary(patch);
        self.displaced.push_dictionary(merged?);
        self.sync_dag_display_from_widgets();
        Ok(())
    }
    // #endregion GumballEditing

    /// 🌳️ Recomputes widget positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts_json: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let opts: DagLayoutOptions = if opts_json.trim().is_empty() { DagLayoutOptions::default() } else { crate::os_pack::json::from_json_str(opts_json)? };
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
        crate::os_pack::json::to_string(&crate::os_pack::json::object([
            ("ids".to_string(), crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&self.dag.preselect_widget_ids()))),
            ("removedIds".to_string(), crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&self.dag.preselect_removed_widget_ids()))),
        ]))
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
        self.evaluate_step(EvalStepBudget::UNBOUNDED);
    }

    /// ⏳️🧵️ Evaluates at most `budget.dispatches` cache-missed (dirty) nodes, yielding early once
    /// `budget.deadline` passes, and returns the not-yet-computed widget ids in topo order —
    /// `remaining[0]` is the node currently blocking, `remaining[1..]` are downstream widgets
    /// waiting behind it. An off-main-thread caller (a plugin worker) resumes with another
    /// `evaluate_step` call until `remaining` is empty; a single
    /// `evaluate_step(EvalStepBudget::UNBOUNDED)` call (via
    /// [`FlowHost::evaluate`]/`evaluate_internal`) still evaluates everything synchronously in one
    /// shot for callers that don't need to spread the work across ticks (tests, explicit
    /// worker-side `evaluate` actions that already run off the caller's main thread).
    ///
    /// `begin_epoch`/`sweep` bracket the *whole run* (every tick up to and including the completing
    /// one), not each tick: `begin_epoch` is cheap to call repeatedly (just bumps a counter), while
    /// `sweep` evicts anything not touched since — calling it before the run completes would discard
    /// earlier ticks' results. A run interleaved with another unrelated evaluation sharing the same
    /// [`NeuralCache`] (e.g. a generation-preview eval firing mid-chain) may have its in-progress
    /// entries swept early by that other call's completion; the next tick simply recomputes them —
    /// extra work, never a wrong result.
    pub fn evaluate_step(&mut self, budget: EvalStepBudget) -> Vec<String> {
        self.drain_displaced();
        self.pending_extension_eval = None;
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.current_baseline_snapshot(), &snapshot);
        if self.baseline_answers_everything(&dirty) {
            tree.retire_cold();
            seeds.retire_cold();
            return Vec::new();
        }
        // 🔢 Read BEFORE the registry it describes: a replacement landing between the two reads
        // then stamps the baseline with the OLDER generation, which over-dirties the next step
        // (extra work, never a stale answer). Reading it after could stamp a generation the
        // evaluation never used.
        let evaluated_generation = flow_extension_registry_generation();
        let registry = flow_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        self.neural_cache.begin_epoch();
        let previous = self.current_baseline_channels();
        let budgeted = if let Some(bridge) = self.eval_bridge.as_ref() {
            let mut dispatch = |kind: &str, input: &Dictionary| bridge.evaluate(kind, input);
            evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut dispatch, &self.neural_cache, &dirty, previous, budget)
        } else {
            let mut dispatch = |kind: &str, input: &Dictionary| registry.as_ref().dispatch(kind, input);
            evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut dispatch, &self.neural_cache, &dirty, previous, budget)
        };
        tree.retire_cold();
        seeds.retire_cold();
        match budgeted {
            Ok(BudgetedEval { channels, remaining, pending_extension }) => {
                self.pending_extension_eval = pending_extension;
                let displaced_outputs = std::mem::replace(&mut self.outputs, channels.outputs.clone());
                self.displaced.push_dictionaries(displaced_outputs);
                self.apply_preview_outputs(&channels.outputs);
                self.apply_export_outputs(&channels.outputs);
                self.last_eval_json = build_channel_eval_json(&self.fixture, &channels, &self.kind_infos);
                if !remaining.is_empty() {
                    channels.retire_cold();
                    return remaining;
                }
                self.neural_cache.sweep();
                let live_handles = collect_live_geometry_handles_from_channels(&channels);
                crate::retain_geometry_handles(&live_handles);
                let live_drawing_handles = collect_live_drawing_handles_from_channels(&channels);
                retain_drawing_handles(&live_drawing_handles);
                // 🔒️ Only advance the snapshot/channels/generation triple together, and only on
                // success — a failed evaluation keeps diffing against the last known-good state
                // next time, which is always a safe (never under-dirty) baseline.
                self.baseline_registry_generation = evaluated_generation;
                self.previous_snapshot = Some(snapshot);
                if let Some(displaced_channels) = self.previous_channels.replace(channels) {
                    self.displaced.push_channels(displaced_channels);
                }
                Vec::new()
            }
            Err(err) => {
                self.neural_cache.sweep();
                if self.last_eval_json.is_empty() || is_global_eval_error_json(&self.last_eval_json) {
                    self.last_eval_json = crate::os_pack::json::to_string(&crate::os_pack::json::object([("error".to_string(), crate::os_pack::json::Value::String(err.to_string()))]));
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
        // 🧹️ `self.outputs` republishes every SEEDED widget's channel too, so a plain `extend`
        // displaces those seed dictionaries and lets `HashMap::insert`'s returned `Option<Dictionary>`
        // drop — and `Dictionary` fail-closes on a bare drop
        // (`🧠️neural/⚙️engine/🦀️.rs`'s `Drop`). Retire what is displaced
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        for (widget_id, channels) in self.outputs.clone() {
            if let Some(displaced) = outputs.insert(widget_id, channels) {
                displaced.retire_cold();
            }
        }
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
        tree.retire_cold();
        outputs.retire_cold();
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
        let dirty = compute_dirty_set(self.current_baseline_snapshot(), &snapshot);
        if self.baseline_answers_everything(&dirty) {
            tree.retire_cold();
            seeds.retire_cold();
            return Vec::new();
        }
        let registry = flow_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        let previous = self.current_baseline_channels();
        let mut probe_never_dispatches = |kind: &str, _: &Dictionary| -> Result<Dictionary, EvalError> { Err(EvalError::InvalidInput(format!("pending_eval_widget_ids probed a dispatch for {kind}"))) };
        let pending = match evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut probe_never_dispatches, &self.neural_cache, &dirty, previous, EvalStepBudget::PROBE) {
            Ok(BudgetedEval { remaining, channels, .. }) => {
                channels.retire_cold();
                remaining
            }
            Err(_) => Vec::new(),
        };
        tree.retire_cold();
        seeds.retire_cold();
        pending
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
        crate::os_pack::json::from_json_str(json).ok()
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
                    std::mem::replace(preview, out.clone()).retire_cold();
                } else if let Some(syn) = self.fixture.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        std::mem::replace(preview, preview_dict_from_connection(src, &syn.from_port, &syn.to_port)).retire_cold();
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
        Ok(crate::os_pack::json::to_json_string(&payload))
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

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
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
        crate::os_pack::json::to_json_string(&self.dag.selected_node_ids())
    }

    /// 🎯️ Full selection snapshot as JSON (`nodes`, `edges`, `🐙️handles`).
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
        let json = crate::os_pack::json::to_string(&crate::os_pack::json::object([
            ("nodes".to_string(), crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&ids.to_vec()))),
            ("edges".to_string(), crate::os_pack::json::Value::Array(vec![])),
            ("handles".to_string(), crate::os_pack::json::Value::Array(vec![])),
        ]));
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
        let ids: Vec<String> = crate::os_pack::json::from_json_str(json).unwrap_or_default();
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
            self.fixture.widgets.iter().enumerate().filter(|(_, widget)| seen.insert(widget_id_for(widget).to_string())).map(|(i, w)| widget_to_dag_node(w, i, &self.fixture.layout, &self.fixture.synapses, &self.kind_infos, widget_node_size(w, &self.fixture.synapses, &self.kind_infos))).collect();
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        let edges: Vec<DagFixtureEdge> = self
            .fixture
            .synapses
            .iter()
            .filter(|syn| !would_create_cycle(&existing.iter().filter(|(a, b)| !(a == &syn.from && b == &syn.to)).cloned().collect::<Vec<_>>(), &syn.from, &syn.to))
            .map(|syn| DagFixtureEdge { id: syn.id.clone(), source: format!("{}@{}", syn.from, syn.from_port), target: format!("{}@{}", syn.to, syn.to_port), route_style: EdgeRouteStyle::default(), properties: PropertyBag::new() })
            .collect();
        DagFixture { schema: "dag.fixture".into(), camera: semio_framework_artifact_infinite_dag::DagCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom }, nodes, edges }
    }

    fn screen_to_world_point(&self, sx: f64, sy: f64) -> canvas::Point {
        use canvas::camera::{screen_to_world, Camera, Viewport};
        use canvas::Point;
        let cam = Camera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn next_widget_id(&mut self, descriptor: &WidgetDescriptor) -> String {
        let (id, serial) = generated_widget_id(descriptor, self.fixture.widgets.iter().map(widget_id_for));
        self.next_widget_serial = serial;
        id
    }

    pub fn set_slider_value(&mut self, widget_id: &str, value: f64) {
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::InputSlider { id, .. } = widget {
                if id == widget_id {
                    set_widget_slider_value(widget, value);
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
        Ok(crate::os_pack::json::to_json_string(&refs))
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
                    self.fixture.synapses.iter().find(|entry| entry.from == synapse.to && selected.contains(&entry.to)).map_or_else(|| (synapse.to.clone(), synapse.to_port.clone()), |entry| (entry.to.clone(), entry.to_port.clone()))
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
                    self.fixture.synapses.iter().find(|entry| entry.to == synapse.from && selected.contains(&entry.from)).map_or_else(|| (synapse.from.clone(), synapse.from_port.clone()), |entry| (entry.from.clone(), entry.from_port.clone()))
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
        // 🧹️ The working copy is BORROWED from one retired clone, not destructured out of it: a
        // cluster's `Tree` params and its `FlowUi` node map both fail closed on a bare drop, so
        // owning `tree`/`flow` as loose locals aborted the process at the end of this function
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        let exploded = self.fixture.widgets[cluster_index].clone();
        let Widget::Cluster { tree, flow, .. } = &exploded else {
            exploded.retire_cold();
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
                let layout = flow.nodes.get(&neuron.id).map_or(WidgetLayout { x: 0.0, y: 0.0 }, |node| node.layout.clone());
                self.fixture.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
                restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
                continue;
            }
            let mut widget = neuron_to_exploded_widget(neuron);
            match &mut widget {
                Widget::Neuron { id, .. } | Widget::InputSlider { id, .. } | Widget::InputNote { id, .. } | Widget::InputImage { id, .. } | Widget::Variable { id, .. } => *id = namespaced_id.clone(),
                _ => {}
            }
            let layout = flow.nodes.get(&neuron.id).map_or(WidgetLayout { x: 0.0, y: 0.0 }, |node| node.layout.clone());
            self.fixture.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
            restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
        }
        let id_map: HashMap<String, String> = restored_widgets.iter().map(|(namespaced, original, _)| (original.clone(), namespaced.clone())).collect();
        // 🧹️ The cluster widget carries a `FlowUi` whose `OrderedMap<FlowNodeGui>` and a `Tree`
        // whose `Dictionary` params both fail closed on a bare drop, so the exploded shell is
        // RETIRED rather than dropped (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        self.fixture.widgets.remove(cluster_index).retire_cold();
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
                .and_then(|neuron| neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str())).map_or_else(|| synapse.from_port.clone(), str::to_string);
            let to_port = tree
                .neurons
                .iter()
                .find(|neuron| neuron.id == synapse.to && neuron.kind == OUTPUT_KIND)
                .and_then(|neuron| neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str())).map_or_else(|| synapse.to_port.clone(), str::to_string);
            self.next_synapse_serial += 1;
            next_synapses.push(SynapseSpec { id: format!("s{}", self.next_synapse_serial), from: from.clone(), to: to.clone(), from_port, to_port });
        }
        self.fixture.synapses = next_synapses;
        exploded.retire_cold();
        self.rebuild_dag();
        Ok(())
    }

    // #region History
    fn content_changed(a: &FlowFixture, b: &FlowFixture) -> bool {
        a.widgets != b.widgets || a.synapses != b.synapses || a.layout != b.layout
    }

    /// 🧾️ Lazily seeds the undo/redo store from `baseline`.
    ///
    /// ⚠️ `baseline` is CONSUMED only on the first call — the store is seeded once, and every later
    /// `flush_pending_change` hands in a fresh `FlowFixture` clone this function does not need. A
    /// `FlowFixture` owns the fail-closed `OrderedMap<WidgetLayout>` root, so that surplus clone is
    /// RETIRED through the artifact's own bounded frontier instead of dropped; the bare drop aborted
    /// the pool worker on the second discrete edit of any session
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️unit-suite-3d-2026-09-09.md` §5).
    fn history_store_from_baseline(&mut self, baseline: FlowFixture) -> Option<&mut FlowStore> {
        if self.history_store.is_some() {
            let mut retirement = crate::retained::FlowRetirement::default();
            retirement.push(crate::retained::FlowOwner::Fixture(baseline));
            retirement.retire_cold();
            return self.history_store.as_mut();
        }
        let mut store = resolve_ready(FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow-host", baseline, None))).ok()?;
        store.install_document_store_owners_exact(FlowFixture::member_store_owners());
        self.history_store = Some(store);
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
    kind_infos: Arc<HashMap<String, OperatorInfo>>,
    neural_cache: Option<neural::NeuralCacheRetirement>,
    previous_snapshot: Option<TreeSnapshot>,
    previous_channels: Option<EvalChannels>,
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
pub struct FlowHostRetirement {
    state: std::mem::ManuallyDrop<FlowHostRetirementState>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowHostRetirementFault {
    NoCredit,
    Failed,
}
impl std::ops::Deref for FlowHostRetirement {
    type Target = FlowHostRetirementState;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl std::ops::DerefMut for FlowHostRetirement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

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
            baseline_registry_generation: _,
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
            displaced,
        } = host;
        let mut dag = dag::DagHostRetirement::new(dag);
        if let Some(node) = ghost_node {
            dag.retain_node_payload(node);
        }
        Self {
            state: std::mem::ManuallyDrop::new(FlowHostRetirementState {
                fixture,
                dag: Some(dag),
                outputs,
                export_payloads,
                last_eval_json,
                eval_bridge,
                host_catalogue_json,
                kind_infos,
                neural_cache: Some(neural::NeuralCacheRetirement::new(neural_cache)),
                previous_snapshot,
                previous_channels,
                history_store,
                pending_history_baseline,
                pending_extension_eval,
                interaction_projection,
                domain: crate::retained::FlowRetirement::default(),
                neural: displaced,
                terminal: false,
                faulted: false,
            }),
        }
    }

    pub fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        let complete = self.close_page(1, 4096).unwrap_or(false);
        context.consume_fuel(1);
        complete
    }

    /// 📏️ Advances one host owner with caller byte credit for its byte-backed retirement cursors.
    pub fn close_page(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<bool, FlowHostRetirementFault> {
        use crate::os_store::ErasedSnapshotRetirement;
        use crate::retained::FlowOwner;
        let state = &mut *self.state;
        if maximum_items == 0 || maximum_bytes == 0 {
            return Err(FlowHostRetirementFault::NoCredit);
        }
        if state.faulted {
            return Err(FlowHostRetirementFault::Failed);
        }
        if let Some(dag) = state.dag.as_mut() {
            match dag.close_step(maximum_items, maximum_bytes) {
                dag::DagRetirementStep::Blocked | dag::DagRetirementStep::Pending { .. } => return Ok(false),
                dag::DagRetirementStep::Complete => {
                    if !dag.terminal_is_empty() {
                        state.faulted = true;
                        return Err(FlowHostRetirementFault::Failed);
                    }
                    state.dag = None;
                }
            }
        } else if !state.domain.is_empty() {
            if state.domain.close_step(1, maximum_bytes).is_err() {
                state.faulted = true;
            }
        } else if !state.neural.terminal_is_empty() {
            state.neural.close_step(1, maximum_bytes);
        } else if let Some(widget) = state.fixture.widgets.pop() {
            state.domain.push(FlowOwner::Widget(widget));
        } else if let Some(synapse) = state.fixture.synapses.pop() {
            state.domain.push(FlowOwner::Specs(vec![synapse]));
        } else if !state.fixture.layout.is_empty() {
            state.domain.push(FlowOwner::Layouts(std::mem::take(&mut state.fixture.layout)));
        } else if !state.fixture.schema.is_empty() {
            state.domain.text(std::mem::take(&mut state.fixture.schema));
        } else if let Some((key, value)) = state.outputs.pop_first() {
            state.neural.text(key);
            state.neural.push_dictionary(value);
        } else if let Some((key, value)) = state.export_payloads.pop_first() {
            state.neural.text(key);
            state.neural.push_dictionary(value);
        } else if state.last_eval_json.capacity() != 0 {
            state.domain.text(std::mem::take(&mut state.last_eval_json));
        } else if state.host_catalogue_json.capacity() != 0 {
            state.domain.text(std::mem::take(&mut state.host_catalogue_json));
        } else if !state.kind_infos.is_empty() {
            match Arc::get_mut(&mut state.kind_infos).and_then(|infos| infos.extract_if(|_, _| true).next()) {
                Some((key, value)) => {
                    state.neural.text(key);
                    state.neural.push_operator(value);
                }
                None => state.kind_infos = Arc::default(),
            }
        } else if let Some(snapshot) = state.previous_snapshot.take() {
            state.neural.push_snapshot(snapshot);
        } else if let Some(channels) = state.previous_channels.take() {
            state.neural.push_channels(channels);
        } else if let Some(fixture) = state.pending_history_baseline.take() {
            state.domain.push(FlowOwner::Fixture(fixture));
        } else if let Some(pending) = state.pending_extension_eval.take() {
            state.neural.text(pending.extension_id);
            state.neural.text(pending.operator_id);
            state.neural.text(pending.input_json);
        } else if state.eval_bridge.take().is_some() || state.interaction_projection.take().is_some() {
        } else if let Some(cache) = state.neural_cache.as_mut() {
            if matches!(cache.close_step(1, maximum_bytes), neural::ValueRetirementStep::Complete) {
                if !cache.terminal_nonopaque_is_empty() {
                    state.faulted = true;
                    return Err(FlowHostRetirementFault::Failed);
                }
                state.neural_cache = None;
            }
        } else if let Some(store) = state.history_store.as_mut() {
            match store.close_owned_step(1, maximum_bytes) {
                Ok(SnapshotRetirementStep::Complete) if store.close_owned_terminal_is_empty() => state.history_store = None,
                Ok(_) => {}
                Err(_) => state.faulted = true,
            }
        } else {
            state.terminal = true;
            return Ok(true);
        }
        if state.faulted {
            Err(FlowHostRetirementFault::Failed)
        } else {
            Ok(false)
        }
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
        if !self.terminal_nonopaque_is_empty() {
            assert!(std::thread::panicking(), "FlowHostRetirement must reach terminal-empty before release");
            return;
        }
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.state);
        }
    }
}

impl FlowHost {
    /// 🧊️ Explicit cold-only disposal of a detached host — the twin of [`FlowFixture::retire_cold`].
    /// A `FlowHost` owns a `FlowFixture`, whose `layout: OrderedMap<WidgetLayout>` panics on a bare
    /// drop (`ordered-map root must be explicitly retired before drop`), so a host is CLOSED, never
    /// dropped. Retained callers drive [`FlowHostRetirement::close_step`] under their own grant
    /// instead; this drains the same ladder in one uninterrupted cold pass.
    pub fn retire_cold(self) {
        let mut retirement = FlowHostRetirement::new(self);
        while !retirement.close_page(1, 4096).expect("cold flow host retirement") {}
    }

    /// 🏠️ Runs `body` against a host built from `fixture`, then retires that host — the ONE shape a
    /// caller that only needs a host for the length of an expression should use.
    pub fn with_fixture<R>(fixture: &FlowFixture, body: impl FnOnce(&mut FlowHost) -> R) -> R {
        let mut host = Self::from_fixture(fixture.clone());
        let result = body(&mut host);
        host.retire_cold();
        result
    }
}

// #region 🔖️EvalSession
#[cfg(test)]
#[path = "🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs"]
mod session_retirement_tests;

use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

/// ⏱️ Max cache-missed neuron dispatches per `flowEvalTick` — keeps one dispatch from blocking the worker while still converging small graphs in a single tick.
pub const FLOW_EVAL_TICK_STEP_BUDGET: usize = 512;

/// ⏱️ Wall-clock ceiling for ONE `flowEvalTick` dag walk, in microseconds.
///
/// 🚨️ [`FLOW_EVAL_TICK_STEP_BUDGET`] alone cannot bound a tick: it counts NODES, and a seven-node
/// graph is far under 512 however expensive each node is. The reactor's own 8 ms budget cannot
/// bound it either — `run_until_deadline` checks its deadline BETWEEN two tasks' `step()` calls,
/// never inside one, and `flowEvalTick` is a plain synchronous `fn`, so once the executor enters it
/// nothing can interrupt it. That is how one browser turn ran for 3.5-18.4 s against an 8 ms budget
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-guest-tick-cost-2026-09-12.md` §1.3).
///
/// 🎚️ Sized at three quarters of [`semio_framework_job::INTERACTIVE_STEP_CEILING_US`] so the rest of
/// the turn — publication, retirement, the effect the tick returns — still fits the same 8 ms the
/// dag walk is being held to, and so a tick that yields here has not ALREADY broken the contract it
/// exists to keep. A walk always dispatches at least one node before the deadline can stop it
/// ([`neural::EvalStepBudget::exhausted`]), so a graph whose single cheapest node exceeds the
/// ceiling still converges — one node per tick — instead of re-arming forever with no progress.
pub const FLOW_EVAL_TICK_ELAPSED_CEILING_US: u64 = semio_framework_job::INTERACTIVE_STEP_CEILING_US / 4 * 3;

/// ⏱️ The budget ONE `flowEvalTick` dag walk runs under: [`FLOW_EVAL_TICK_STEP_BUDGET`] nodes,
/// preempted after [`FLOW_EVAL_TICK_ELAPSED_CEILING_US`] on the process clock. Falls back to the
/// node count alone when no clock is installed (bare wasm), which is the pre-existing behaviour.
pub fn flow_eval_tick_budget() -> EvalStepBudget {
    match semio_framework_job::default_now_us() {
        Some(now_us) => EvalStepBudget::until(FLOW_EVAL_TICK_STEP_BUDGET, semio_framework_job::default_now_us, now_us.saturating_add(FLOW_EVAL_TICK_ELAPSED_CEILING_US)),
        None => EvalStepBudget::dispatches(FLOW_EVAL_TICK_STEP_BUDGET),
    }
}

static FLOW_SESSION_GEOMETRY: LazyLock<Mutex<HashMap<u64, BTreeSet<String>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static NEXT_FLOW_SESSION_ID: AtomicU64 = AtomicU64::new(1);

fn sync_flow_geometry_retention() {
    let merged: HashSet<String> = FLOW_SESSION_GEOMETRY.lock().map(|entries| entries.values().flat_map(|set| set.iter().cloned()).collect()).unwrap_or_default();
    let live: Vec<String> = merged.into_iter().collect();
    crate::retain_geometry_handles(&live);
}

/// 🚦 Per-widget evaluation state for flow graph chrome (not persisted in config).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(tag = "status", rename_all = "camelCase")]
#[value(tag = "status", rename_all = "camelCase")]
pub enum NodeEvalStatus {
    Ok,
    Stale,
    Queued,
    Computing,
    Error { message: String },
    Blocked { ports: Vec<String> },
}

/// 🔒️ What ONE preview window's evaluation chain currently owes, as the retained session sees it.
///
/// ⏱️ `armed` is the latch itself: exactly one `flowEvalTick` may be outstanding for a window at a
/// time, no matter how many independent sources ask for one (a host refresh poll, a
/// `setContributions` install, an example switch, a view command, the tick's own continuation).
/// `in_flight` counts the [`ExtensionInvocation`]s that window's last tick parked — a window waiting
/// on an extension answer must NOT be ticked again, because the tick would recompute the identical
/// pending request and park a duplicate. `owed` remembers that an answer asked for a continuation
/// while its siblings were still outstanding, so the LAST answer arms exactly one tick instead of
/// every answer arming its own. `unfinished` is what the window's own last tick reported, so a
/// refresh poll can tell "this evaluation is still running and nothing is chasing it" from "this
/// evaluation is done" and from "this evaluation gave up" — PER WINDOW, because generate-mode
/// previews evaluate a patched fixture of their own and finish on their own schedule
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FlowEvalWindowTickLatch {
    armed: bool,
    in_flight: u32,
    owed: bool,
    unfinished: bool,
}

/// 🔑️ The latch key for one preview window instance id. Hashed rather than retained: the latch map
/// must cost nothing to retire, and a window id is the caller's string, never the session's.
fn flow_eval_window_key(window_id: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    window_id.hash(&mut hasher);
    hasher.finish()
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
    /// 🔒️ One arming latch per preview window this session publishes into, keyed by
    /// [`flow_eval_window_key`]. See [`FlowEvalWindowTickLatch`] — this is what makes "at most ONE
    /// pending `flowEvalTick` per (instance, window)" a fact the session owns rather than a
    /// convention every caller has to re-derive. Plain `Copy` rows keyed by hash, so retirement is
    /// a single `clear` (the `tessellate_progress_by_hash` precedent) and no window id is ever
    /// retained here.
    window_tick_latches: BTreeMap<u64, FlowEvalWindowTickLatch>,
    live_geometry_handles: BTreeSet<String>,
    /// 🧊 Tessellated preview meshes keyed by geometry handle, each one a base64 `pack` record body
    /// (see `brep_geometry::encode_mesh_pack`) — filled via extension `tessellate` because
    /// runtime-installable brep owns the kernel that minted the handles. Binary, not a JSON number
    /// array: the render path decodes typed arrays instead of parsing millions of JSON tokens.
    preview_mesh_pack_by_handle: BTreeMap<String, String>,
    /// ⏳ In-flight tessellate requests keyed by `nodeHash` forwarded through `InvokeExtension`. A
    /// request is removed the moment its answer is folded, even a partial one — the continuation
    /// re-admits it on the next tick.
    pending_tessellate_by_hash: BTreeMap<u64, String>,
    /// 🔗 The handle every admitted `nodeHash` belongs to, kept for as long as the tessellation is
    /// unfinished. This is what makes a MULTI-STEP tessellation survive `retain_preview_meshes`:
    /// its progress row and its half-received mesh body are keyed by hash, and pruning them by the
    /// (already emptied) pending table would restart the transfer from chunk zero forever.
    tessellate_handle_by_hash: BTreeMap<u64, String>,
    /// 📈 Progress of every tessellation this session has admitted, keyed by `nodeHash`. Plain
    /// `Copy` rows — no heap, so retirement is a single take.
    tessellate_progress_by_hash: BTreeMap<u64, PreviewTessellateProgress>,
    /// 🧱 Partially received mesh bodies keyed by `nodeHash`, accumulated one intake-sized base64
    /// chunk per round trip until `next_chunk == chunks`.
    tessellate_chunks_by_hash: BTreeMap<u64, String>,
    /// 🩺 Blocking validate-gate findings keyed by geometry handle, as a JSON array string.
    preview_diagnostics_by_handle: BTreeMap<String, String>,
    /// 🔢 Which replacement of the process-wide flow extension registry every result this session
    /// still holds was computed against. A contributed operator that no plugin had contributed yet
    /// evaluates to a fault, and the fault is CACHED — in the neural cache, in the incremental
    /// baseline and in the tessellation ledger — so a later contribution install has to be able to
    /// say "everything in here predates the registry you can now address"
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    flow_extension_generation: u64,
    /// 💥 The LAST `evaluate` answer this session was handed that faulted, so the surface publishes
    /// the fault it is actually living with instead of the addressing miss that preceded it. Cleared
    /// the moment a contribution install moves `flow_extension_generation`, and the moment any
    /// answer folds — a stale fault outranking a live evaluation is the same defect as an
    /// `extension-not-contributed` card surviving its own install
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    extension_evaluate_fault: Option<ExtensionEvaluateFault>,
    /// 🛑 Whether the user's explicit `cancelPreviewEval` gesture is the LAST thing that happened to
    /// this session's evaluation. It is not derivable from the tessellation ledger: a cancel raised
    /// while the chain was still in its `evaluate` round trips has no progress row to stamp, and a
    /// cancel of a never-admitted tessellation leaves an all-`Idle` ledger — both of which would
    /// publish `phase: "idle"` and silently swallow the gesture. Cleared by the next tick that
    /// actually begins ([`FlowEvalSession::begin_window_tick`]), so a later gesture resumes with a
    /// clean status rather than a frozen `cancelled` banner
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    preview_cancelled: bool,
    /// ⏱️ Per-node progress of the BUDGETED `evaluate` round trips — the half of the preview's work
    /// the tessellation ledger cannot see at all, because a long operator (a boolean) admits no
    /// tessellation until it has finished. Keyed by `nodeHash`, the same identity the extension
    /// resumes its retained job by (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️extension-evaluate-budget-2026-09-12.md`).
    eval_progress_by_hash: BTreeMap<u64, PreviewEvalProgress>,
    retiring_cache: Option<neural::NeuralCacheRetirement>,
    retirement: neural::ValueRetirement,
    retiring_collections: std::collections::LinkedList<SessionCollectionOwner>,
    closing: bool,
}

/// 💥 One faulted `evaluate` round trip, as the surface publishes it: the addressed extension, the
/// capability that failed, and the fault the SDK decoded from the host's completion. Distinct from
/// [`FlowExtensionAddressMiss`] on purpose — a miss means NOTHING was invoked, this means the
/// extension answered and refused (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExtensionEvaluateFault {
    pub extension_id: String,
    pub capability: String,
    pub code: String,
    pub message: String,
}

impl ExtensionEvaluateFault {
    /// 🪪️ The stable wire code the surface publishes for this fault family.
    pub const CODE: &'static str = "flow.extension-evaluate-failed";

    /// 🌍 English and German headline, with no default language — the message itself stays verbatim.
    pub fn labels(&self) -> (String, String) {
        (format!("Geometry extension '{}' could not evaluate", self.extension_id), format!("Geometrie-Erweiterung '{}' konnte nicht auswerten", self.extension_id))
    }
}

/// 📊️ What the preview-eval publication gate saw and what it let through, since the last reset.
/// `considered`/`considered_bytes` is exactly what an ungated tick chain WOULD have published (one
/// full serialization per tick), so one measured boot reports both the before and the after.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlowEvalPublicationLedger {
    pub considered: u64,
    pub considered_bytes: u64,
    pub published: u64,
    pub published_bytes: u64,
}

/// 📊️ Four relaxed atomic counters, no formatting and no allocation — "how many bytes did this boot
/// publish" is the quantity the hot-path budget is written in, and a log line cannot be asserted on.
/// Process-wide, not thread-local: a retained command's work step runs on whatever thread the host
/// pumps it from, which is never the thread that reads the ledger back.
static FLOW_EVAL_PUBLICATION_COUNTERS: [AtomicU64; 4] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];

/// 📊️ The preview-eval publication ledger since the last reset.
pub fn flow_eval_publication_ledger() -> FlowEvalPublicationLedger {
    FlowEvalPublicationLedger {
        considered: FLOW_EVAL_PUBLICATION_COUNTERS[0].load(AtomicOrdering::Relaxed),
        considered_bytes: FLOW_EVAL_PUBLICATION_COUNTERS[1].load(AtomicOrdering::Relaxed),
        published: FLOW_EVAL_PUBLICATION_COUNTERS[2].load(AtomicOrdering::Relaxed),
        published_bytes: FLOW_EVAL_PUBLICATION_COUNTERS[3].load(AtomicOrdering::Relaxed),
    }
}

/// 📊️ Zeroes the publication ledger, so one measurement starts from a known point.
pub fn reset_flow_eval_publication_ledger() {
    for counter in &FLOW_EVAL_PUBLICATION_COUNTERS {
        counter.store(0, AtomicOrdering::Relaxed);
    }
}

/// 📊️ One evaluation tick's measured wall cost, recorded only while the runtime-diagnostics switch
/// is armed (`semio_framework_job::set_runtime_diagnostics`) — the interactive-step observable for a
/// perf run, absent from a normal boot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlowEvalStepLedger {
    pub steps: u64,
    /// ⏱️ The CHEAPEST tick measured. Wall time on a machine that is also compiling only ever grows,
    /// so the minimum is this machine's real capability and the only sample a budget law can assert
    /// on without asserting the scheduler.
    pub best_us: u64,
    pub worst_us: u64,
    pub total_us: u64,
}

static FLOW_EVAL_STEP_COUNTERS: [AtomicU64; 4] = [AtomicU64::new(0), AtomicU64::new(u64::MAX), AtomicU64::new(0), AtomicU64::new(0)];

/// 📊️ The evaluation-step ledger since the last reset.
pub fn flow_eval_step_ledger() -> FlowEvalStepLedger {
    FlowEvalStepLedger {
        steps: FLOW_EVAL_STEP_COUNTERS[0].load(AtomicOrdering::Relaxed),
        best_us: FLOW_EVAL_STEP_COUNTERS[1].load(AtomicOrdering::Relaxed),
        worst_us: FLOW_EVAL_STEP_COUNTERS[2].load(AtomicOrdering::Relaxed),
        total_us: FLOW_EVAL_STEP_COUNTERS[3].load(AtomicOrdering::Relaxed),
    }
}

/// 📊️ Zeroes the evaluation-step ledger.
pub fn reset_flow_eval_step_ledger() {
    FLOW_EVAL_STEP_COUNTERS[0].store(0, AtomicOrdering::Relaxed);
    FLOW_EVAL_STEP_COUNTERS[1].store(u64::MAX, AtomicOrdering::Relaxed);
    FLOW_EVAL_STEP_COUNTERS[2].store(0, AtomicOrdering::Relaxed);
    FLOW_EVAL_STEP_COUNTERS[3].store(0, AtomicOrdering::Relaxed);
}

/// 📊️ Records one evaluation tick's wall cost.
pub fn record_flow_eval_step(elapsed_us: u64) {
    FLOW_EVAL_STEP_COUNTERS[0].fetch_add(1, AtomicOrdering::Relaxed);
    FLOW_EVAL_STEP_COUNTERS[1].fetch_min(elapsed_us, AtomicOrdering::Relaxed);
    FLOW_EVAL_STEP_COUNTERS[2].fetch_max(elapsed_us, AtomicOrdering::Relaxed);
    FLOW_EVAL_STEP_COUNTERS[3].fetch_add(elapsed_us, AtomicOrdering::Relaxed);
}

/// 📤️ What one tick owes the surface's retained preview-eval publication.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlowEvalPublication {
    /// ♻️ The retained publication already carries exactly these bytes — publish nothing this tick.
    Retained,
    /// 📤️ The eval JSON changed; `None` clears the preview because nothing is evaluated yet.
    Changed(Option<String>),
}

enum SessionCollectionOwner {
    Handles(BTreeSet<String>),
    Meshes(BTreeMap<String, String>),
    Pending(BTreeMap<u64, String>),
}

/// 🔒️ Evaluation ownership stays guarded until every collection, domain, cache, and byte frontier is empty.
pub struct FlowEvalSession {
    state: std::mem::ManuallyDrop<FlowEvalSessionState>,
}
impl std::ops::Deref for FlowEvalSession {
    type Target = FlowEvalSessionState;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl std::ops::DerefMut for FlowEvalSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl Default for FlowEvalSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FlowEvalSession {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            assert!(std::thread::panicking(), "FlowEvalSession must finish explicit close before drop");
            return;
        }
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.state);
        }
    }
}

impl FlowEvalSession {
    pub fn new() -> Self {
        Self {
            state: std::mem::ManuallyDrop::new(FlowEvalSessionState {
                session_id: NEXT_FLOW_SESSION_ID.fetch_add(1, AtomicOrdering::Relaxed),
                neural_cache: Some(Arc::new(NeuralCache::new())),
                previous_snapshot: None,
                previous_channels: None,
                eval_json: String::new(),
                status_json: "{}".into(),
                tick_scheduled: false,
                window_tick_latches: BTreeMap::new(),
                live_geometry_handles: BTreeSet::new(),
                preview_mesh_pack_by_handle: BTreeMap::new(),
                pending_tessellate_by_hash: BTreeMap::new(),
                tessellate_handle_by_hash: BTreeMap::new(),
                tessellate_progress_by_hash: BTreeMap::new(),
                eval_progress_by_hash: BTreeMap::new(),
                tessellate_chunks_by_hash: BTreeMap::new(),
                preview_diagnostics_by_handle: BTreeMap::new(),
                retiring_cache: None,
                retirement: neural::ValueRetirement::default(),
                flow_extension_generation: flow_extension_registry_generation(),
                extension_evaluate_fault: None,
                preview_cancelled: false,
                retiring_collections: std::collections::LinkedList::new(),
                closing: false,
            }),
        }
    }

    pub fn neural_cache(&self) -> Arc<NeuralCache> {
        self.neural_cache.as_ref().expect("live Flow evaluation session owns its neural cache").clone()
    }

    pub fn install_baseline_into(&self, host: &mut FlowHost) {
        host.install_eval_baseline(self.previous_snapshot.clone(), self.previous_channels.clone(), self.state.flow_extension_generation);
    }

    /// 🧵️ Takes over a host's eval baseline, INCLUDING which registry replacement it was computed
    /// against.
    ///
    /// The generation carry-back is monotonic and gated on the host actually holding a snapshot: a
    /// host that has evaluated against a newer registry owns results that are current, and leaving
    /// the session pinned to the older generation would make every ephemeral host it seeds refuse
    /// the incremental fast path forever. It can never walk BACKWARDS, so it cannot cancel a
    /// pending [`FlowEvalSession::invalidate_for_flow_extension_registry`]
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn capture_baseline_from(&mut self, host: &FlowHost) {
        let (snapshot, channels) = host.eval_baseline();
        let evaluated_generation = host.eval_baseline_registry_generation();
        let state = &mut *self.state;
        if snapshot.is_some() && evaluated_generation > state.flow_extension_generation {
            state.flow_extension_generation = evaluated_generation;
        }
        if let Some(previous) = std::mem::replace(&mut state.previous_snapshot, snapshot) {
            state.retirement.push_snapshot(previous);
        }
        if let Some(previous) = std::mem::replace(&mut state.previous_channels, channels) {
            state.retirement.push_channels(previous);
        }
        if let Some(channels) = state.previous_channels.as_ref() {
            let next = collect_live_geometry_handles_from_channels(channels).into_iter().collect();
            state.retiring_collections.push_back(SessionCollectionOwner::Handles(std::mem::replace(&mut state.live_geometry_handles, next)));
            if let Ok(mut map) = FLOW_SESSION_GEOMETRY.lock() {
                if let Some(previous) = map.insert(state.session_id, state.live_geometry_handles.clone()) {
                    state.retiring_collections.push_back(SessionCollectionOwner::Handles(previous));
                }
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
        let remaining = host.evaluate_step(flow_eval_tick_budget());
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

    /// 📤️ What this session owes ONE surface whose retained preview publication currently holds
    /// `retained`. See [`flow_eval_publication_for`] — the decision is per PUBLICATION TARGET, never
    /// per session: two preview windows on one instance each own their own retained bytes.
    pub fn eval_publication_for(&self, retained: Option<&str>) -> FlowEvalPublication {
        flow_eval_publication_for(self, retained)
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
        if let Some(previous) = state.previous_snapshot.take() {
            state.retirement.push_snapshot(previous);
        }
        if let Some(previous) = state.previous_channels.take() {
            state.retirement.push_channels(previous);
        }
        state.retirement.text(std::mem::replace(&mut state.status_json, "{}".into()));
        state.retiring_collections.push_back(SessionCollectionOwner::Handles(std::mem::take(&mut state.live_geometry_handles)));
        state.retiring_collections.push_back(SessionCollectionOwner::Meshes(std::mem::take(&mut state.preview_mesh_pack_by_handle)));
        state.retiring_collections.push_back(SessionCollectionOwner::Meshes(std::mem::take(&mut state.preview_diagnostics_by_handle)));
        state.retiring_collections.push_back(SessionCollectionOwner::Pending(std::mem::take(&mut state.pending_tessellate_by_hash)));
        state.retiring_collections.push_back(SessionCollectionOwner::Pending(std::mem::take(&mut state.tessellate_handle_by_hash)));
        state.retiring_collections.push_back(SessionCollectionOwner::Pending(std::mem::take(&mut state.tessellate_chunks_by_hash)));
        state.tessellate_progress_by_hash.clear();
        state.eval_progress_by_hash.clear();
        if let Ok(mut map) = FLOW_SESSION_GEOMETRY.lock() {
            if let Some(previous) = map.remove(&state.session_id) {
                state.retiring_collections.push_back(SessionCollectionOwner::Handles(previous));
            }
        }
        sync_flow_geometry_retention();
    }

    pub fn pending(&self) -> bool {
        self.tick_scheduled
    }

    //#region 🔒️TickLatch
    /// 🔒️ Arms `window_id`'s `flowEvalTick` if — and only if — nothing already owes one.
    ///
    /// This is the ONE gate every arming source goes through: the host refresh poll
    /// (`ArtifactApp::pending_effects`), the `setContributions` install, `setActiveExample`, every
    /// view command, and the chain's own continuations. Answers whether the CALLER owes the effect,
    /// so a refused arm emits nothing rather than duplicating a tick that is already in flight.
    ///
    /// ⏳️ A window waiting on an extension answer is not armed at all — it is marked `owed` instead,
    /// and [`FlowEvalSession::settle_window_extension`] hands the arm to whichever answer lands
    /// last. Ticking a window whose invocation is outstanding recomputes the identical pending
    /// request, which is how one graph turned into 298 invocations against 111 settles in 80 s of
    /// live console (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn arm_window_tick(&mut self, window_id: &str) -> bool {
        let latch = self.state.window_tick_latches.entry(flow_eval_window_key(window_id)).or_default();
        if latch.armed {
            return false;
        }
        if latch.in_flight > 0 {
            latch.owed = true;
            return false;
        }
        latch.armed = true;
        latch.owed = false;
        true
    }

    /// 🔎️ Whether `window_id` owes a tick that nothing has armed — a window that has never ticked
    /// owes its first one, and a window whose own last tick reported unfinished work owes another
    /// only while no tick and no extension answer is already chasing it. The REFRESH poll's whole
    /// question, and the reason it costs no evaluation at all.
    pub fn window_tick_owed(&self, window_id: &str) -> bool {
        match self.window_tick_latches.get(&flow_eval_window_key(window_id)) {
            None => true,
            Some(latch) => latch.unfinished && !latch.armed && latch.in_flight == 0,
        }
    }

    /// 🔒️ Arms the tick `window_id` actually owes — [`FlowEvalSession::window_tick_owed`] and
    /// [`FlowEvalSession::arm_window_tick`] as the single question a refresh poll asks.
    pub fn arm_owed_window_tick(&mut self, window_id: &str) -> bool {
        self.window_tick_owed(window_id) && self.arm_window_tick(window_id)
    }

    /// ▶️ Marks `window_id`'s armed tick as RUNNING: the effect has been delivered, so the latch is
    /// free for whatever this tick's own outcome decides to arm next.
    /// 🛑 A tick that actually begins is work RESUMING, which is the one thing that retires the
    /// `cancelled` banner an explicit gesture raised — see [`FlowEvalSession::preview_cancelled`].
    pub fn begin_window_tick(&mut self, window_id: &str) {
        self.state.preview_cancelled = false;
        self.state.window_tick_latches.entry(flow_eval_window_key(window_id)).or_default().armed = false;
    }

    /// ⏳️ Records that `window_id`'s tick parked `count` extension invocations. Those answers own
    /// the continuation from here — the tick that parked them owes no re-arm.
    pub fn note_window_extensions_in_flight(&mut self, window_id: &str, count: usize) {
        let latch = self.state.window_tick_latches.entry(flow_eval_window_key(window_id)).or_default();
        latch.in_flight = latch.in_flight.saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
    }

    /// 📝️ Records what `window_id`'s tick reported: `unfinished` is the tick's own "there is more to
    /// compute". Only this makes a refresh poll able to answer [`FlowEvalSession::window_tick_owed`]
    /// without evaluating anything.
    pub fn note_window_tick_outcome(&mut self, window_id: &str, unfinished: bool) {
        self.state.window_tick_latches.entry(flow_eval_window_key(window_id)).or_default().unfinished = unfinished;
    }

    /// 🚧️ Marks `window_id`'s chain as GIVEN UP: an extension answer this process cannot fold (a
    /// faulted `invokeExtension`, an answer that seeds no cache entry) is not slow work, and the
    /// next tick would park the identical request and fault again at the host's own cadence. Nothing
    /// but a gesture — a contributions install, an example switch, an edit — may resume it, and each
    /// of those arms through [`FlowEvalSession::arm_window_tick`] directly.
    pub fn abandon_window_tick(&mut self, window_id: &str) {
        let latch = self.state.window_tick_latches.entry(flow_eval_window_key(window_id)).or_default();
        latch.unfinished = false;
        latch.owed = false;
    }

    /// 🧹️ Drops the latch of every window that is no longer attached. A detached window's latch can
    /// never be discharged by a tick — the retained route refuses a window that has left the
    /// roster — so keeping it would make a window that comes back unarmable forever.
    pub fn retain_window_tick_latches(&mut self, window_ids: &[&str]) {
        if self.state.window_tick_latches.is_empty() {
            return;
        }
        let live: BTreeSet<u64> = window_ids.iter().map(|window_id| flow_eval_window_key(window_id)).collect();
        self.state.window_tick_latches.retain(|key, _| live.contains(key));
    }

    /// ✅️ Folds ONE extension answer back into `window_id`'s latch. Answers whether that settle
    /// discharged an arm a sibling answer had already asked for, so the LAST answer of a fan-out
    /// emits exactly one re-arm and the earlier ones emit none.
    pub fn settle_window_extension(&mut self, window_id: &str) -> bool {
        let latch = self.state.window_tick_latches.entry(flow_eval_window_key(window_id)).or_default();
        latch.in_flight = latch.in_flight.saturating_sub(1);
        if latch.in_flight == 0 && latch.owed && !latch.armed {
            latch.armed = true;
            latch.owed = false;
            return true;
        }
        false
    }

    /// 🔎️ How many extension answers `window_id` is still waiting for — readable so a law can state
    /// the in-flight rule rather than infer it.
    pub fn window_extensions_in_flight(&self, window_id: &str) -> u32 {
        self.window_tick_latches.get(&flow_eval_window_key(window_id)).map(|latch| latch.in_flight).unwrap_or(0)
    }

    /// 🔎️ Whether a `flowEvalTick` is currently pending for `window_id`.
    pub fn window_tick_is_armed(&self, window_id: &str) -> bool {
        self.window_tick_latches.get(&flow_eval_window_key(window_id)).is_some_and(|latch| latch.armed)
    }

    /// 🧹️ Abandons every window's latch — what a chain RESTART owes itself. The results the old
    /// chain was chasing have just been swept (a registry replacement, a fresh document), so the
    /// invocations it parked can no longer resume anything and their windows must be free to be
    /// armed again from scratch.
    pub fn clear_window_tick_latches(&mut self) {
        self.state.window_tick_latches.clear();
    }
    //#endregion 🔒️TickLatch

    /// 🔢 Which flow extension registry replacement this session's held results were computed
    /// against — the invalidation key, readable so a surface can state it rather than infer it.
    pub fn flow_extension_generation(&self) -> u64 {
        self.flow_extension_generation
    }

    /// 🔄️ Re-arms this session against the flow extension registry as it is NOW.
    ///
    /// A node whose operator no plugin had contributed yet does not merely stall: its miss is
    /// retained everywhere a result is retained — the neural cache entry, the incremental
    /// baseline (`previous_snapshot`/`previous_channels`), the published `eval_json`/`status_json`,
    /// and the tessellation ledger whose `phase` a surface projects as
    /// `PreviewTessellatePhase::Faulted`. Installing the contribution AFTERWARDS therefore changes
    /// nothing a surface can see until somebody says those results predate the registry, and
    /// `tick_scheduled` is still `true` from the chain that gave up, so `sync` refuses to arm a new
    /// one. This is that statement, and the REGISTRY GENERATION is its key: any later contribution
    /// change bumps it too, so a re-push of an unchanged closure (whose generation is unmoved)
    /// invalidates nothing (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// Answers whether anything was actually invalidated, so the caller only re-arms a chain it
    /// owes.
    pub fn invalidate_for_flow_extension_registry(&mut self, generation: u64) -> bool {
        if self.state.flow_extension_generation == generation {
            return false;
        }
        self.state.flow_extension_generation = generation;
        self.state.extension_evaluate_fault = None;
        self.state.preview_cancelled = false;
        self.state.window_tick_latches.clear();
        if let Some(cache) = self.state.neural_cache.as_ref() {
            cache.begin_epoch();
            cache.sweep();
        }
        self.set_eval_json(String::new());
        true
    }

    /// 💥 Records the fault an `evaluate` answer came back with. The extension id and the decoded
    /// message are the SDK's own (`reactor::extension_response_args` echoes `faultCode`/
    /// `faultMessage` onto the response action) — nothing here re-words them.
    pub fn note_extension_evaluate_fault(&mut self, fault: ExtensionEvaluateFault) {
        self.state.extension_evaluate_fault = Some(fault);
    }

    /// ✅️ Forgets the last evaluate fault — an answer that folded supersedes it.
    pub fn clear_extension_evaluate_fault(&mut self) {
        self.state.extension_evaluate_fault = None;
    }

    /// 💥 The evaluate fault this session is currently living with, if any.
    pub fn extension_evaluate_fault(&self) -> Option<&ExtensionEvaluateFault> {
        self.extension_evaluate_fault.as_ref()
    }

    pub fn seed_node_cache(&self, node_hash: u64, output_json: &str) -> Result<(), FlowCoreError> {
        let cache = self.neural_cache.as_deref().expect("live Flow evaluation session owns its neural cache");
        seed_flow_eval_node_cache(cache, node_hash, output_json)
    }

    /// 🧊 Preview mesh body (base64 `pack` record body) previously resolved through the owning
    /// geometry extension.
    pub fn preview_mesh_pack(&self, handle: &str) -> Option<&str> {
        self.preview_mesh_pack_by_handle.get(handle).map(String::as_str)
    }

    /// 🩺 Blocking validate-gate findings for `handle`, as a JSON array string.
    pub fn preview_diagnostics(&self, handle: &str) -> Option<&str> {
        self.preview_diagnostics_by_handle.get(handle).map(String::as_str)
    }

    /// 🩺 Every handle the validate gate rejected, paired with its JSON issue array.
    pub fn preview_diagnostic_entries(&self) -> Vec<(&str, &str)> {
        self.preview_diagnostics_by_handle.iter().map(|(handle, issues)| (handle.as_str(), issues.as_str())).collect()
    }

    /// 🧹 Drops preview meshes/pending tessellates whose handles are no longer live. A tessellation
    /// still in progress keeps its row and its half-received body — liveness is judged by HANDLE
    /// (`tessellate_handle_by_hash`), never by the pending table, which a partial answer empties.
    pub fn retain_preview_meshes(&mut self, live_handles: &HashSet<String>) {
        self.preview_mesh_pack_by_handle.retain(|handle, _| live_handles.contains(handle));
        self.preview_diagnostics_by_handle.retain(|handle, _| live_handles.contains(handle));
        self.tessellate_handle_by_hash.retain(|_, handle| live_handles.contains(handle));
        self.pending_tessellate_by_hash.retain(|_, handle| live_handles.contains(handle));
        let live_hashes: BTreeSet<u64> = self.tessellate_handle_by_hash.keys().copied().collect();
        self.tessellate_progress_by_hash.retain(|hash, _| live_hashes.contains(hash));
        self.tessellate_chunks_by_hash.retain(|hash, _| live_hashes.contains(hash));
    }

    /// 📨 Notes an in-flight tessellate; returns true when the caller should emit `InvokeExtension`.
    pub fn note_pending_tessellate(&mut self, node_hash: u64, handle: String) -> bool {
        if let Some(pack) = self.preview_mesh_pack_by_handle.get(&handle) {
            if preview_mesh_pack_has_geometry(pack) {
                return false;
            }
            self.preview_mesh_pack_by_handle.remove(&handle);
        }
        if self.preview_diagnostics_by_handle.contains_key(&handle) {
            return false;
        }
        if self.pending_tessellate_by_hash.values().any(|pending| pending == &handle) {
            return false;
        }
        self.tessellate_handle_by_hash.insert(node_hash, handle.clone());
        self.pending_tessellate_by_hash.insert(node_hash, handle);
        true
    }

    /// 🧱 The mesh-body chunk index the next `tessellate` round trip for `node_hash` must ask for.
    pub fn next_tessellate_chunk(&self, node_hash: u64) -> u32 {
        self.tessellate_progress_by_hash.get(&node_hash).map_or(0, |progress| progress.next_chunk)
    }

    /// 📈 Aggregate progress of every tessellation this session has admitted and not yet finished —
    /// what the preview window's status object reports.
    pub fn preview_tessellate_status(&self) -> PreviewTessellateStatus {
        let mut status = PreviewTessellateStatus::default();
        status.in_flight = self.pending_tessellate_by_hash.len() as u32;
        status.diagnostics = self.preview_diagnostics_by_handle.len() as u32;
        for progress in self.tessellate_progress_by_hash.values() {
            status.units_done = status.units_done.saturating_add(progress.units_done);
            status.units_total = status.units_total.saturating_add(progress.units_total);
            status.faces_done = status.faces_done.saturating_add(progress.faces_done);
            status.faces_total = status.faces_total.saturating_add(progress.faces_total);
            // 🧱 A kernel-COMPLETE job whose mesh body has not finished crossing is not idle — it is
            // `Transferring`, the one phase the enum declares that no kernel ever reports. Without
            // this the whole chunked transfer of a large mesh published `phase: "idle"` with a
            // full-looking ratio, so a surface could neither label it nor offer to stop it
            // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
            if progress.phase == PreviewTessellatePhase::Complete {
                if progress.next_chunk < progress.chunks {
                    status.phase = PreviewTessellatePhase::Transferring;
                }
            } else {
                status.phase = progress.phase;
            }
            status.chunks_done = status.chunks_done.saturating_add(progress.next_chunk.min(progress.chunks));
            status.chunks_total = status.chunks_total.saturating_add(progress.chunks);
        }
        if status.units_total == 0 && status.in_flight > 0 {
            status.phase = PreviewTessellatePhase::SamplingEdges;
        }
        status
    }

    /// 🛑 Retires every in-flight preview evaluation and tessellation: the kernel jobs this process
    /// owns are cancelled in place, every pending request is forgotten, every partial mesh body is
    /// dropped, every window's arming latch falls back to QUIESCENT (`window_id`'s is CREATED if it
    /// does not exist yet) and the session remembers that the gesture happened. Returns how many in-flight tessellations were retired. Idempotent — a
    /// second cancel is a no-op.
    ///
    /// 🧱 The mesh-body chunk CURSOR is reset alongside the half-received body it addresses. The two
    /// are one fact in two fields: `tessellate_chunks_by_hash` holds the base64 assembled so far and
    /// `PreviewTessellateProgress::next_chunk` says which chunk continues it. Dropping the body
    /// while keeping the cursor made the NEXT evaluation of the same handle ask the kernel for chunk
    /// `n` and concatenate it onto nothing — a silently truncated `pack` body, i.e. exactly the
    /// "stale mesh from the cancelled run" a cancel exists to prevent
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🔒️ The latches are reset to [`FlowEvalWindowTickLatch::default`] rather than REMOVED: a
    /// missing latch reads as "this window never ticked" and [`FlowEvalSession::window_tick_owed`]
    /// answers `true` for it, so `clear()` here would have the host refresh poll re-arm the very
    /// chain the user just stopped. A defaulted latch owes nothing and admits the next gesture.
    ///
    /// 🚪️ This cancels what THIS process holds. The kernel jobs the geometry extension retains live
    /// in the extension actor's own instance and are only reachable through its `tessellateCancel`
    /// capability — see [`FlowEvalSession::preview_cancel_invocation_request_json`], which the
    /// cancelling command emits alongside this call.
    pub fn cancel_preview_evaluation(&mut self, window_id: &str) -> usize {
        let retired = self.pending_tessellate_by_hash.len();
        crate::brep_geometry::cancel_all_tessellations();
        let state = &mut *self.state;
        state.retiring_collections.push_back(SessionCollectionOwner::Pending(std::mem::take(&mut state.pending_tessellate_by_hash)));
        state.retiring_collections.push_back(SessionCollectionOwner::Pending(std::mem::take(&mut state.tessellate_handle_by_hash)));
        state.retiring_collections.push_back(SessionCollectionOwner::Pending(std::mem::take(&mut state.tessellate_chunks_by_hash)));
        for progress in state.tessellate_progress_by_hash.values_mut() {
            if progress.phase != PreviewTessellatePhase::Complete {
                progress.phase = PreviewTessellatePhase::Cancelled;
            }
            progress.next_chunk = 0;
            progress.chunks = 0;
        }
        // ⏱️ The budgeted-evaluation ledger is EMPTIED rather than stamped: unlike a tessellation,
        // whose frozen counters are what the surface keeps showing beside `phase: "cancelled"`, a
        // parked evaluation that survives here would make the very next `preview_eval_status()`
        // report work in flight that nothing will ever answer.
        state.eval_progress_by_hash.clear();
        for latch in state.window_tick_latches.values_mut() {
            *latch = FlowEvalWindowTickLatch::default();
        }
        // 🔒️ The ADDRESSED window's latch is created if it does not exist yet. A window with no
        // latch reads as "never ticked", which `window_tick_owed` answers `true` for — so a cancel
        // that left the addressed window latchless would be undone by the very next host refresh
        // poll (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        if !window_id.is_empty() {
            state.window_tick_latches.insert(flow_eval_window_key(window_id), FlowEvalWindowTickLatch::default());
        }
        state.preview_cancelled = true;
        state.tick_scheduled = false;
        retired
    }

    /// 🛑 The `tessellateCancel` request body the cancelling command sends to the geometry extension.
    /// Deliberately addresses NO handle: the session keys its ledger by `nodeHash`, which is a
    /// one-way digest of `(handle, tolerance bits)`, so the tolerance half of the extension's job key
    /// is not recoverable here — and a cancel that can only retire SOME of the jobs it means to
    /// retire is worse than the whole-registry branch the capability already offers
    /// (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`'s `tessellateCancel` handler).
    pub fn preview_cancel_invocation_request_json(window_id: &str, window_kind_id: &str) -> String {
        format!("{{\"windowId\":{},\"windowKindId\":{}}}", crate::os_pack::json::to_string(&crate::os_pack::json::Value::String(window_id.to_string())), crate::os_pack::json::to_string(&crate::os_pack::json::Value::String(window_kind_id.to_string())))
    }

    /// ✅️ Folds ONE budgeted `evaluate` envelope (`{done, phase, unitsDone, unitsTotal, outputJson}`)
    /// into the session's evaluation ledger and says what the chain owes next. A `done` envelope
    /// retires the progress row; a working one keeps it so the surface can paint the phase and
    /// ratio of the ONE part of a boolean preview the tessellation ledger is blind to.
    ///
    /// 🧾️ A body that is not an envelope at all is read as a finished evaluation whose output IS
    /// that body — that is precisely what an extension built before this contract answers, and
    /// treating it as complete is the only reading that cannot lose an answer.
    pub fn resolve_preview_eval(&mut self, node_hash: u64, envelope_json: &str) -> PreviewEvalOutcome {
        let Ok(envelope) = crate::os_pack::json::parse(envelope_json) else {
            self.eval_progress_by_hash.remove(&node_hash);
            return PreviewEvalOutcome::Complete { output_json: envelope_json.to_string() };
        };
        let Some(done) = envelope.get("done").and_then(crate::os_pack::json::Value::as_bool) else {
            self.eval_progress_by_hash.remove(&node_hash);
            return PreviewEvalOutcome::Complete { output_json: envelope_json.to_string() };
        };
        let phase = PreviewEvalPhase::from_job_tag(envelope.get("phase").and_then(crate::os_pack::json::Value::as_str).unwrap_or("computing"));
        let units_done = envelope.get("unitsDone").and_then(crate::os_pack::json::Value::as_f64).unwrap_or(0.0).max(0.0) as u32;
        let units_total = envelope.get("unitsTotal").and_then(crate::os_pack::json::Value::as_f64).unwrap_or(0.0).max(0.0) as u32;
        if !done {
            self.eval_progress_by_hash.insert(node_hash, PreviewEvalProgress { units_done, units_total, phase });
            return PreviewEvalOutcome::Working;
        }
        self.eval_progress_by_hash.remove(&node_hash);
        if matches!(phase, PreviewEvalPhase::Cancelled) {
            return PreviewEvalOutcome::Cancelled;
        }
        PreviewEvalOutcome::Complete { output_json: envelope.get("outputJson").and_then(crate::os_pack::json::Value::as_str).unwrap_or_default().to_string() }
    }

    /// 📈 Aggregate progress of every budgeted evaluation this session has admitted and not yet
    /// finished — what the preview window's status object reports for the `evaluate` half of the
    /// work.
    pub fn preview_eval_status(&self) -> PreviewEvalStatus {
        let mut status = PreviewEvalStatus { in_flight: self.eval_progress_by_hash.len() as u32, ..PreviewEvalStatus::default() };
        for progress in self.eval_progress_by_hash.values() {
            status.units_done = status.units_done.saturating_add(progress.units_done);
            status.units_total = status.units_total.saturating_add(progress.units_total);
            status.phase = progress.phase;
        }
        status
    }

    /// 🛑 The `evaluateCancel` request body the cancelling command sends to the geometry extension.
    /// Whole-registry, for the same reason [`FlowEvalSession::preview_cancel_invocation_request_json`]
    /// is: a session cancels its preview, not one named operator hop.
    pub fn preview_eval_cancel_invocation_request_json(window_id: &str, window_kind_id: &str) -> String {
        Self::preview_cancel_invocation_request_json(window_id, window_kind_id)
    }

    /// 🛑 Whether the user's explicit cancel is the last thing that happened to this evaluation —
    /// what makes a surface able to publish `phase: "cancelled"` even when the gesture landed before
    /// any tessellation had been admitted.
    pub fn preview_cancelled(&self) -> bool {
        self.preview_cancelled
    }

    /// ⏳️ How many extension answers this session is waiting for across EVERY window it publishes
    /// into — the session-wide half of "is there work in flight", which the per-window latch can only
    /// answer one window at a time. A surface's `cancellable` is this OR a tessellation in flight: an
    /// `evaluate` round trip admits no tessellation at all, so the tessellation ledger alone reports
    /// `idle` through the whole (slowest) part of a boolean preview.
    pub fn extensions_in_flight(&self) -> u32 {
        self.window_tick_latches.values().map(|latch| latch.in_flight).sum()
    }

    /// 🧹 Retires one tessellation's transfer state (its handle binding and any half-received mesh
    /// body) without touching the progress row the status object still reports.
    fn retire_tessellate_transfer(&mut self, node_hash: u64) {
        let state = &mut *self.state;
        state.tessellate_handle_by_hash.remove(&node_hash);
        if let Some(partial) = state.tessellate_chunks_by_hash.remove(&node_hash) {
            state.retirement.text(partial);
        }
    }

    /// ✅ Folds one `tessellate` response envelope into this session. The mesh only lands once every
    /// chunk of its `pack` body has arrived; until then the caller re-arms another round trip.
    pub fn resolve_preview_tessellate(&mut self, node_hash: u64, output_json: &str) -> PreviewTessellateOutcome {
        let Some(handle) = self.pending_tessellate_by_hash.remove(&node_hash) else {
            return PreviewTessellateOutcome::Unknown;
        };
        let Ok(envelope) = crate::os_pack::json::parse(output_json) else {
            return PreviewTessellateOutcome::Failed;
        };
        let phase = envelope.get("phase").and_then(|value| value.as_str()).unwrap_or_default();
        let uint = |key: &str| envelope.get(key).and_then(|value| value.as_f64()).unwrap_or_default().max(0.0) as u32;
        let mut progress = PreviewTessellateProgress { units_done: uint("unitsDone"), units_total: uint("unitsTotal"), faces_done: uint("facesDone"), faces_total: uint("facesTotal"), phase: PreviewTessellatePhase::from_tag(phase), next_chunk: 0, chunks: uint("chunks") };
        match progress.phase {
            PreviewTessellatePhase::Invalid => {
                let diagnostics = envelope.get("diagnostics").cloned().unwrap_or(crate::os_pack::json::Value::Array(Vec::new()));
                self.preview_diagnostics_by_handle.insert(handle, crate::os_pack::json::to_string(&diagnostics));
                self.retire_tessellate_transfer(node_hash);
                self.tessellate_progress_by_hash.insert(node_hash, progress);
                PreviewTessellateOutcome::Invalid
            }
            PreviewTessellatePhase::Failed => {
                self.retire_tessellate_transfer(node_hash);
                self.tessellate_progress_by_hash.insert(node_hash, progress);
                PreviewTessellateOutcome::Failed
            }
            PreviewTessellatePhase::Cancelled => {
                self.retire_tessellate_transfer(node_hash);
                self.tessellate_progress_by_hash.insert(node_hash, progress);
                PreviewTessellateOutcome::Cancelled
            }
            PreviewTessellatePhase::Complete => {
                let chunk = envelope.get("meshPack").and_then(|value| value.as_str()).unwrap_or_default();
                let index = uint("chunk");
                let assembled = self.tessellate_chunks_by_hash.entry(node_hash).or_default();
                assembled.push_str(chunk);
                progress.next_chunk = index.saturating_add(1);
                if progress.next_chunk < progress.chunks {
                    self.tessellate_progress_by_hash.insert(node_hash, progress);
                    return PreviewTessellateOutcome::Working;
                }
                let Some(body) = self.tessellate_chunks_by_hash.remove(&node_hash) else {
                    return PreviewTessellateOutcome::Failed;
                };
                if !preview_mesh_pack_has_geometry(&body) {
                    self.tessellate_progress_by_hash.insert(node_hash, progress);
                    return PreviewTessellateOutcome::Failed;
                }
                self.preview_mesh_pack_by_handle.insert(handle, body);
                self.tessellate_handle_by_hash.remove(&node_hash);
                self.tessellate_progress_by_hash.insert(node_hash, progress);
                PreviewTessellateOutcome::Ready
            }
            _ => {
                self.tessellate_progress_by_hash.insert(node_hash, progress);
                PreviewTessellateOutcome::Working
            }
        }
    }

    /// 🧹 Begins exact incremental retirement of this instance-owned evaluation session.
    pub fn begin_close(&mut self) {
        self.closing = true;
        self.tick_scheduled = false;
        self.window_tick_latches.clear();
        if let Ok(mut map) = FLOW_SESSION_GEOMETRY.lock() {
            if let Some(previous) = map.remove(&self.session_id) {
                self.retiring_collections.push_back(SessionCollectionOwner::Handles(previous));
            }
        }
        sync_flow_geometry_retention();
    }

    /// 📄 Releases at most one retained owner under the caller's close-page grant.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep as Step;
        let state = &mut *self.state;
        if !state.closing || maximum_items == 0 || maximum_bytes == 0 {
            return Step::Blocked;
        }
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
                    if let Some(value) = values.pop_first() {
                        state.retirement.text(value);
                    }
                    if !values.is_empty() {
                        state.retiring_collections.push_front(SessionCollectionOwner::Handles(values));
                    }
                }
                SessionCollectionOwner::Meshes(mut values) => {
                    if let Some((key, value)) = values.pop_first() {
                        state.retirement.text(key);
                        state.retirement.text(value);
                    }
                    if !values.is_empty() {
                        state.retiring_collections.push_front(SessionCollectionOwner::Meshes(values));
                    }
                }
                SessionCollectionOwner::Pending(mut values) => {
                    if let Some((_, value)) = values.pop_first() {
                        state.retirement.text(value);
                    }
                    if !values.is_empty() {
                        state.retiring_collections.push_front(SessionCollectionOwner::Pending(values));
                    }
                }
            }
            return Step::Pending { released_items: 1, released_bytes: 0 };
        }
        if !state.preview_mesh_pack_by_handle.is_empty() {
            state.retiring_collections.push_back(SessionCollectionOwner::Meshes(std::mem::take(&mut state.preview_mesh_pack_by_handle)));
        } else if !state.preview_diagnostics_by_handle.is_empty() {
            state.retiring_collections.push_back(SessionCollectionOwner::Meshes(std::mem::take(&mut state.preview_diagnostics_by_handle)));
        } else if !state.tessellate_chunks_by_hash.is_empty() {
            state.retiring_collections.push_back(SessionCollectionOwner::Pending(std::mem::take(&mut state.tessellate_chunks_by_hash)));
        } else if !state.tessellate_handle_by_hash.is_empty() {
            state.retiring_collections.push_back(SessionCollectionOwner::Pending(std::mem::take(&mut state.tessellate_handle_by_hash)));
        } else if !state.tessellate_progress_by_hash.is_empty() {
            state.tessellate_progress_by_hash.clear();
        } else if !state.eval_progress_by_hash.is_empty() {
            state.eval_progress_by_hash.clear();
        } else if !state.window_tick_latches.is_empty() {
            state.window_tick_latches.clear();
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
                neural::ValueRetirementStep::Complete => {
                    assert!(cache.terminal_nonopaque_is_empty());
                    state.retiring_cache = None;
                }
            }
        } else {
            return Step::Complete;
        }
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
            && self.preview_mesh_pack_by_handle.is_empty()
            && self.preview_diagnostics_by_handle.is_empty()
            && self.tessellate_chunks_by_hash.is_empty()
            && self.tessellate_handle_by_hash.is_empty()
            && self.tessellate_progress_by_hash.is_empty()
            && self.eval_progress_by_hash.is_empty()
            && self.window_tick_latches.is_empty()
            && self.pending_tessellate_by_hash.is_empty()
            && self.retiring_cache.is_none()
            && self.retirement.terminal_is_empty()
            && self.retiring_collections.is_empty()
    }
}

/// 🧊 True when a base64 `pack` mesh body carries something paintable — decoded once, structurally,
/// never by re-parsing prose.
fn preview_mesh_pack_has_geometry(base64_body: &str) -> bool {
    let Ok(bytes) = crate::brep_geometry::decode_base64(base64_body) else {
        return false;
    };
    let Ok(mesh) = crate::brep_geometry::decode_mesh_pack(&bytes) else {
        return false;
    };
    (!mesh.indices.is_empty() && mesh.positions.len() >= 9) || mesh.edge_positions.len() >= 6 || (mesh.positions.len() >= 3 && mesh.indices.is_empty())
}

/// ⏱️ Where one preview tessellation currently is — the wire-stable mirror of the kernel's own
/// `TessellationPhase`, widened by the two boundary-only outcomes (`invalid`, `failed`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PreviewTessellatePhase {
    #[default]
    Idle,
    SamplingEdges,
    MeshingFaces,
    PackingEdges,
    Transferring,
    Complete,
    Cancelled,
    Invalid,
    /// 💥️ The kernel itself answered with a failure.
    Failed,
    /// 🪪️ No kernel could even be addressed — the geometry extension this preview needs is not
    /// contributed by any loaded plugin, so no invocation was ever emitted. Boundary-only, like
    /// `Invalid`/`Failed`: a kernel never reports it, the producing app raises it from its own
    /// `flow_extension_invocation_address` miss (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    Faulted,
}

impl PreviewTessellatePhase {
    /// 🏷️ The stable wire tag, shared by the extension envelope, the status object and the UI.
    pub fn tag(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::SamplingEdges => "samplingEdges",
            Self::MeshingFaces => "meshingFaces",
            Self::PackingEdges => "packingEdges",
            Self::Transferring => "transferring",
            Self::Complete => "complete",
            Self::Cancelled => "cancelled",
            Self::Invalid => "invalid",
            Self::Failed => "failed",
            Self::Faulted => "faulted",
        }
    }

    /// 🏷️ Inverse of [`PreviewTessellatePhase::tag`]; an unknown tag reads as `Idle`.
    pub fn from_tag(tag: &str) -> Self {
        match tag {
            "samplingEdges" => Self::SamplingEdges,
            "meshingFaces" => Self::MeshingFaces,
            "packingEdges" => Self::PackingEdges,
            "transferring" => Self::Transferring,
            "complete" => Self::Complete,
            "cancelled" => Self::Cancelled,
            "invalid" => Self::Invalid,
            "failed" => Self::Failed,
            "faulted" => Self::Faulted,
            _ => Self::Idle,
        }
    }

    /// 🛑 True while a cancel can still retire the job.
    pub fn is_cancellable(self) -> bool {
        matches!(self, Self::SamplingEdges | Self::MeshingFaces | Self::PackingEdges | Self::Transferring)
    }

    /// 🌍 English and German labels — the UI carries both, with no default language.
    pub fn labels(self) -> (&'static str, &'static str) {
        match self {
            Self::Idle => ("Idle", "Bereit"),
            Self::SamplingEdges => ("Sampling edges", "Kanten werden abgetastet"),
            Self::MeshingFaces => ("Meshing faces", "Flächen werden vernetzt"),
            Self::PackingEdges => ("Packing edges", "Kanten werden gepackt"),
            Self::Transferring => ("Transferring mesh", "Netz wird übertragen"),
            Self::Complete => ("Complete", "Fertig"),
            Self::Cancelled => ("Cancelled", "Abgebrochen"),
            Self::Invalid => ("Invalid geometry", "Ungültige Geometrie"),
            Self::Failed => ("Failed", "Fehlgeschlagen"),
            Self::Faulted => ("Geometry extension unavailable", "Geometrie-Erweiterung nicht verfügbar"),
        }
    }
}

/// ⏱️ Where one BUDGETED `evaluate` round trip currently is. The extension's job reports its own
/// domain phase tag (`imprint`, `classifyA`, `validate`, …); this enum is the wire-stable,
/// localizable vocabulary the surface paints, and `Computing` is the honest answer for any tag a
/// future operator invents (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PreviewEvalPhase {
    #[default]
    Idle,
    /// 🧮️ A stepped operator is working and named a phase this vocabulary does not know.
    Computing,
    /// ✂️ Imprinting intersection curves onto the operands' faces.
    Imprinting,
    /// 🧩 Splitting imprinted faces into pieces.
    Splitting,
    /// 🎯 Classifying each piece against the other operand.
    Classifying,
    /// 🧵 Stitching the kept faces into shells and solids.
    Stitching,
    /// 🩺 Validating the result.
    Validating,
    Complete,
    Cancelled,
}

impl PreviewEvalPhase {
    /// 🏷️ The stable wire tag the status object publishes.
    pub fn tag(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Computing => "computing",
            Self::Imprinting => "imprinting",
            Self::Splitting => "splitting",
            Self::Classifying => "classifying",
            Self::Stitching => "stitching",
            Self::Validating => "validating",
            Self::Complete => "complete",
            Self::Cancelled => "cancelled",
        }
    }

    /// 🏷️ Reads the extension job's own phase tag. An unknown tag is `Computing`, never `Idle`: the
    /// envelope that carried it said the job is still working, and publishing `idle` for live work
    /// is the exact defect this ledger exists to close.
    pub fn from_job_tag(tag: &str) -> Self {
        match tag {
            "imprint" => Self::Imprinting,
            "applyA" | "applyB" => Self::Splitting,
            "classifyA" | "classifyB" => Self::Classifying,
            "stitch" => Self::Stitching,
            "validate" => Self::Validating,
            "complete" => Self::Complete,
            "cancelled" => Self::Cancelled,
            "idle" => Self::Idle,
            _ => Self::Computing,
        }
    }

    /// 🛑 True while a cancel can still retire the evaluation.
    pub fn is_cancellable(self) -> bool {
        matches!(self, Self::Computing | Self::Imprinting | Self::Splitting | Self::Classifying | Self::Stitching | Self::Validating)
    }

    /// 🌍 English and German labels — the UI carries both, with no default language.
    pub fn labels(self) -> (&'static str, &'static str) {
        match self {
            Self::Idle => ("Idle", "Bereit"),
            Self::Computing => ("Computing", "Berechnen"),
            Self::Imprinting => ("Imprinting intersections", "Schnittkurven werden eingeprägt"),
            Self::Splitting => ("Splitting faces", "Flächen werden geteilt"),
            Self::Classifying => ("Classifying faces", "Flächen werden klassifiziert"),
            Self::Stitching => ("Stitching shells", "Schalen werden vernäht"),
            Self::Validating => ("Validating solid", "Körper wird geprüft"),
            Self::Complete => ("Complete", "Fertig"),
            Self::Cancelled => ("Cancelled", "Abgebrochen"),
        }
    }
}

/// 📈 One budgeted evaluation's monotone progress.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreviewEvalProgress {
    pub units_done: u32,
    pub units_total: u32,
    pub phase: PreviewEvalPhase,
}

/// 📈 The whole session's budgeted-evaluation state, as the status object reports it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreviewEvalStatus {
    pub units_done: u32,
    pub units_total: u32,
    pub in_flight: u32,
    pub phase: PreviewEvalPhase,
}

impl PreviewEvalStatus {
    /// 📈 Fraction of the admitted evaluation work already done, in `[0, 1]`.
    pub fn ratio(&self) -> f64 {
        if self.units_total == 0 {
            return if self.in_flight == 0 { 1.0 } else { 0.0 };
        }
        (f64::from(self.units_done) / f64::from(self.units_total)).clamp(0.0, 1.0)
    }

    /// 🛑 True while an explicit cancel would still retire something.
    pub fn is_cancellable(&self) -> bool {
        self.in_flight > 0 || self.phase.is_cancellable()
    }
}

/// ✅️ What folding one budgeted `evaluate` envelope achieved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreviewEvalOutcome {
    /// 🔁 The extension's job is still working — arm another round trip for the SAME request.
    Working,
    /// ✅ The operator answered; `output_json` is the out dictionary to seed.
    Complete { output_json: String },
    /// 🛑 The job was retired; nothing is produced and nothing is owed.
    Cancelled,
}

impl PreviewEvalOutcome {
    /// 🔁 Whether the chain owes another identical `evaluate` round trip.
    pub fn needs_another_round_trip(&self) -> bool {
        matches!(self, Self::Working)
    }
}

/// 📈 One tessellation's monotone progress plus its mesh-body chunk cursor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreviewTessellateProgress {
    pub units_done: u32,
    pub units_total: u32,
    pub faces_done: u32,
    pub faces_total: u32,
    pub phase: PreviewTessellatePhase,
    pub next_chunk: u32,
    pub chunks: u32,
}

/// 📈 The whole session's preview tessellation state, as the status object reports it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreviewTessellateStatus {
    pub units_done: u32,
    pub units_total: u32,
    pub faces_done: u32,
    pub faces_total: u32,
    /// 🧱 Mesh-body chunks already folded, and how many the bodies in flight carry in all — the
    /// SECOND stage of the work a preview owes, which [`PreviewTessellateStatus::ratio`] counts
    /// alongside the kernel's units so a body still crossing can never publish `ratio: 1`.
    pub chunks_done: u32,
    pub chunks_total: u32,
    pub in_flight: u32,
    pub diagnostics: u32,
    pub phase: PreviewTessellatePhase,
}

impl PreviewTessellateStatus {
    /// 📈 Fraction of the admitted work already done, in `[0, 1]`.
    /// ⚖️ A preview owes TWO stages of work — tessellating the kernel's units and carrying the mesh
    /// body across in chunks — so the ratio is the fraction of BOTH. Counting units alone published
    /// `ratio: 1` for the whole of a ten-chunk transfer, i.e. the surface said "done" while nothing
    /// had been painted (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn ratio(&self) -> f64 {
        let done = u64::from(self.units_done).saturating_add(u64::from(self.chunks_done));
        let total = u64::from(self.units_total).saturating_add(u64::from(self.chunks_total));
        if total == 0 {
            return if self.in_flight == 0 { 1.0 } else { 0.0 };
        }
        ((done as f64) / (total as f64)).clamp(0.0, 1.0)
    }

    /// 🛑 True while an explicit cancel would still retire something.
    pub fn is_cancellable(&self) -> bool {
        self.in_flight > 0 || self.phase.is_cancellable()
    }
}

/// ✅ What one folded `tessellate` response did to the session.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewTessellateOutcome {
    /// 🔁 More units or more mesh-body chunks remain — re-arm another round trip.
    Working,
    /// ✅ The complete mesh landed under its handle.
    Ready,
    /// 🩺 The validate gate rejected the topology; a typed diagnostic is stored for the handle.
    Invalid,
    /// 🛑 The job was cancelled.
    Cancelled,
    /// 💥 The extension faulted or sent an undecodable body.
    Failed,
    /// ❓ No pending request carried this `nodeHash` — a stale or superseded response.
    Unknown,
}

impl PreviewTessellateOutcome {
    /// 🔁 True when the caller must dispatch another tick to continue this tessellation.
    pub fn needs_another_round_trip(self) -> bool {
        matches!(self, Self::Working)
    }
}

/// 🧬 Stable `nodeHash` for an extension tessellate request (mirrored through ShellHost).
/// 📤️ The publication one surface owes, given the evaluation bytes it ALREADY retains.
///
/// Every caller that used to write `session.eval_json().to_string()` unconditionally goes through
/// here. A `flowEvalTick` self-redispatches until the graph settles, and most of those ticks move no
/// node at all — republishing identical bytes costs a fresh `String` now and a 4096-bytes-per-
/// rotation retirement later, and since the retirement cursor is byte-proportional while the
/// maintenance rotation revisits the transient store only once per 24 turns, an unconditional
/// republication queues retirement faster than the rotation can drain it (the multi-millisecond
/// stage-22 outliers in `📓️runtime-hotpath-audit-2026-09-10.md` §2).
pub fn flow_eval_publication_for(session: &FlowEvalSession, retained: Option<&str>) -> FlowEvalPublication {
    let evaluated = (!session.eval_json().is_empty()).then(|| session.eval_json());
    let bytes = evaluated.map_or(0, str::len) as u64;
    let unchanged = retained == evaluated;
    FLOW_EVAL_PUBLICATION_COUNTERS[0].fetch_add(1, AtomicOrdering::Relaxed);
    FLOW_EVAL_PUBLICATION_COUNTERS[1].fetch_add(bytes, AtomicOrdering::Relaxed);
    if !unchanged {
        FLOW_EVAL_PUBLICATION_COUNTERS[2].fetch_add(1, AtomicOrdering::Relaxed);
        FLOW_EVAL_PUBLICATION_COUNTERS[3].fetch_add(bytes, AtomicOrdering::Relaxed);
    }
    if unchanged {
        return FlowEvalPublication::Retained;
    }
    FlowEvalPublication::Changed(evaluated.map(str::to_string))
}

pub fn preview_tessellate_node_hash(handle: &str, tolerance_bits: u64) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    handle.hash(&mut hasher);
    tolerance_bits.hash(&mut hasher);
    hasher.finish()
}

/// 🏠 Builds a host wired to `session`'s shared cache and converged baseline.
pub fn flow_host_with_session(fixture: &FlowFixture, session: &FlowEvalSession) -> FlowHost {
    let mut host = FlowHost::from_fixture_with_cache_and_infos(fixture.clone(), session.neural_cache(), flow_neuron_kind_info_map());
    session.install_baseline_into(&mut host);
    if !session.eval_json().is_empty() {
        host.last_eval_json = session.eval_json().to_string();
    }
    host
}

/// 🚧️ The operator kinds `fixture` needs that the live flow extension registry cannot serve — the
/// typed, up-front form of the evaluator's per-node `unknown kind: …`.
///
/// A miss here is NOT something an evaluation tick can work off: `Evaluator::dispatch` answers
/// `EvalError::UnknownKind` for an unregistered kind, the node publishes an error dictionary, and
/// the very next tick recomputes the same miss because an error is never cached. Only a REGISTRY
/// change — a host `setContributions` push, which bumps
/// [`flow_extension_registry_generation`] and re-arms every attached surface through
/// [`FlowEvalSession::invalidate_for_flow_extension_registry`] — can move it. A surface that arms a
/// chain anyway spins forever at its own refresh cadence, which is exactly what the served
/// generation3d editor did for the whole 45 s of a live boot with no contributions installed
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// Scope is deliberately the fixture's own `Widget::Neuron` kinds: those are the kinds a contributed
/// extension pack registers and the ones a host push makes resolvable. A cluster's inner tree is not
/// walked — its boundary kinds are structural (`core.input`/`core.output`), never contributed — so
/// this answer never invents a block that a contribution could not lift.
pub fn unserved_flow_operator_kinds(fixture: &FlowFixture) -> Vec<String> {
    let registry = flow_registry();
    let mut unserved = BTreeSet::new();
    for widget in &fixture.widgets {
        if let Widget::Neuron { neuron_kind, .. } = widget {
            if registry.as_ref().operator(neuron_kind).is_none() {
                unserved.insert(neuron_kind.clone());
            }
        }
    }
    unserved.into_iter().collect()
}

fn node_eval_status_json(status: &NodeEvalStatus) -> crate::os_pack::json::Value {
    crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(status))
}

fn build_flow_status_json(host: &FlowHost, remaining: &[String]) -> String {
    let eval = crate::os_pack::json::parse(&host.last_eval_json).unwrap_or_else(|_| crate::os_pack::json::Value::Object(crate::os_pack::json::Object::new()));
    let tree = host.build_tree_for_status();
    let seeds = host.build_seeds_for_status();
    let snapshot = TreeSnapshot::capture(&tree, &seeds);
    let dirty = compute_dirty_set(host.eval_baseline_snapshot(), &snapshot);
    let active = remaining.first().map(String::as_str);
    let mut widgets = crate::os_pack::json::Object::new();
    for widget in &host.fixture.widgets {
        let id = widget_id_for(widget);
        if matches!(widget, Widget::InputSlider { .. } | Widget::InputNote { .. } | Widget::InputImage { .. } | Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } | Widget::Cluster { .. }) {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Ok));
            continue;
        }
        if let Some(entry) = eval.get(id) {
            if let Some(message) = entry.get("error").and_then(crate::os_pack::json::Value::as_str) {
                widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Error { message: message.to_string() }));
                continue;
            }
        }
        let blocked = host.widget_blocked_ports(id);
        if !blocked.is_empty() {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Blocked { ports: blocked }));
            continue;
        }
        if active == Some(id) {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Computing));
            continue;
        }
        if remaining.iter().any(|entry| entry == id) {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Queued));
            continue;
        }
        if dirty.contains(id) && !remaining.is_empty() {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Stale));
            continue;
        }
        widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Ok));
    }
    tree.retire_cold();
    seeds.retire_cold();
    crate::os_pack::json::to_string(&crate::os_pack::json::Value::Object(widgets))
}
// #endregion 🔖️EvalSession

fn dedupe_fixture_widgets(fixture: &mut FlowFixture) {
    let mut seen = BTreeSet::new();
    fixture.widgets.retain(|widget| seen.insert(widget_id_for(widget).to_string()));
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

fn widget_node_size(widget: &Widget, synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> (f64, f64) {
    let label = widget_label(widget);
    match widget {
        Widget::InputSlider { .. } => {
            let output = IoPortSpec::named("N", "Num", "number", "Number");
            (dag::slider_widget_width(&label, &output), dag::slider_widget_height())
        }
        Widget::InputNote { text, .. } => dag::note_widget_size(text),
        Widget::OutputAction { .. } | Widget::OutputExport { .. } => (dag::io_widget_width(&label), dag::io_widget_height(&label)),
        Widget::InputImage { src, .. } => dag::image_widget_size(src),
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = semio_framework_artifact_infinite_dag::normalize_node_display(&display_name, &abbreviation);
            (dag::computation_node_width(&normalized_name, &inputs, &outputs), dag::computation_node_height(1, 1, false, false))
        }
        Widget::OutputPreview { preview, expanded, .. } => dag::preview_widget_size(&dag_preview_content_from_dict(preview), &expanded.iter().cloned().collect()),
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, .. } => {
            let (inputs, outputs, variadic_inputs, variadic_outputs) = neuron_io_layout(id, neuron_kind, input_ports, output_ports, params, synapses, kind_infos);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = semio_framework_artifact_infinite_dag::normalize_node_display(&display_name, &abbreviation);
            (dag::computation_node_width(&normalized_name, &inputs, &outputs), dag::computation_node_height(inputs.len(), outputs.len(), variadic_inputs, variadic_outputs))
        }
        Widget::Cluster { id, name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(id, name, tree, synapses);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = semio_framework_artifact_infinite_dag::normalize_node_display(&display_name, &abbreviation);
            (dag::computation_node_width(&normalized_name, &inputs, &outputs), dag::computation_node_height(inputs.len(), outputs.len(), false, false))
        }
    }
}
