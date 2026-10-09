//! 🖥️ Flow host: canvas editing, evaluation session, and host errors.

use crate::infinite::board::schema::dag_input::{DagSelectionDomains,DagNodeStatuses};
use crate::infinite::board::ports::directed_dag as dag;
use semio_framework_canvas as canvas;
use neural_engine as neural;
use semio_framework_artifact_infinite_dag::io::text::snapshot::dag_host_snapshot_to_wire_literal;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};
use protocol::causal::transition::HistoryFoldIndex;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{controlled::ControlledRetirement,queue::RetirementQueue}};

use semio_framework_os_infinite::board::schema::layout::{DagLayoutOptions};
use dag::{fit_node_size, would_create_cycle, DagHost};
use semio_framework_artifact_infinite_dag::{dag_host_snapshot_execution_rows, DagHostSnapshot, DagHostSnapshotEdge, DagNodeKind, DagNodeSpec, EdgeRouteStyle, IoPortSpec};
use semio_framework_artifact_flow_flow::{retire_flow_mutation, widget_id_for, AddSynapse, AddWidget, ChangeLayout, ChangeSynapse, ChangeWidget, FlowLayoutEntry, FlowMutation, FlowStore, RemoveSynapse, RemoveWidget, FLOW_DOCUMENT_SCHEMA};
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
use crate::os_store::{create_document_envelope, ArtifactCommand, MemberStoreOwner, SpaceMember};
use crate::registry::*;
#[path="🧹️retirement/🦀️.rs"]
mod original_retirement;
use original_retirement::FlowHostPayload;
#[path="📥️evaluation-source/🦀️.rs"]
mod evaluation_source;
use evaluation_source::FlowEvaluationSource;


// #region ⚠️ Errors
/// 🧯️ `FlowHost`'s error type — wraps JSON codec failures, the `dag` crate's own `DagError`, and
/// this crate's own graph-editing validation failures. Every variant's Display text is byte-for-byte
/// identical to the `String` it replaces, so downstream `.to_string()` call sites and JSON error
/// envelopes are unaffected.
#[derive(Debug,semio_framework_value::RetireOwned)]
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
    /// 🔌️ The drawn wire would carry a value the target port does not accept — the source port's
    /// declared value schemas and the target's are disjoint. Both sides are carried so a surface can
    /// name them in the user's own language.
    IncompatiblePortTypes {
        source: String,
        source_type: String,
        target: String,
        target_type: String,
    },
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
    /// 📦️ A content-addressed payload referenced a digest this guest process does not hold, so the
    /// sender must carry the body again — see [`resolve_flow_shared_payload`].
    UnknownSharedPayload(String),
    /// 🧾️ The history store refused a gesture's leaves. No partial history row survives: the history was reset to the live
    /// content, so earlier undo steps are gone, and the carried text names the refusal.
    HistoryRefused(String),
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
            Self::IncompatiblePortTypes { source, source_type, target, target_type } => write!(formatter, "{source} carries {source_type}, {target} accepts {target_type}"),
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
            Self::UnknownSharedPayload(digest) => write!(formatter, "unknown shared payload: {digest}"),
            Self::HistoryRefused(reason) => write!(formatter, "history refused the edit: {reason}"),
        }
    }
}

//#region 📦️SharedPayloads
/// 📦️ Marks a content-addressed flow payload. `@<digest>\n<body>` CARRIES a body and registers it
/// under that digest; `@<digest>` alone REFERENCES a body this guest process already holds; anything
/// else is a bare body.
///
/// The app-static operator catalogue is the same bytes for every board in a page — 88 438 B of
/// `setNeuronKindInfosJson` and 22 849 B of `setCatalogueJson` — and every flow session used to
/// re-cross both when its surface attached (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️flow-scroll-render-perf-2026-09-15.md` §9). Every session in a page lives in ONE wasm module
/// with one linear memory, so a second board can name what the first delivered instead of moving it
/// again, and a generation that really did change carries a new digest and therefore a new body.
pub const FLOW_SHARED_PAYLOAD_PREFIX: char = '@';

/// 🧩️ Separates the PARTS of one payload. A payload the host composes from several sources — the
/// operator table is the app-static catalogue plus whatever records the scene itself derived — is
/// carried part by part, so appending 1 805 B of scene operators names the 98 642 B of app operators
/// the guest already holds instead of re-crossing them (measured, `🐍️flow-surface-followup-probe.mjs`).
pub const FLOW_SHARED_PAYLOAD_PART_SEPARATOR: char = '\u{1e}';

thread_local! {
    /// 🗂️ The bodies this guest process holds, by content digest. Plain `String`s rather than the
    /// parsed catalogue: a parsed `OperatorInfo` map carries `Value`s that fail closed on a bare
    /// drop, and a process-wide registry has no owner to retire it.
    static FLOW_SHARED_PAYLOAD_BODIES: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
}

/// 📮️ Resolves a content-addressed payload to its body, registering a carried one under its digest.
///
/// Answers [`FlowCoreError::UnknownSharedPayload`] for a reference this process cannot resolve, so a
/// sender whose note of what the guest holds is wrong learns it and carries the body again instead of
/// installing an empty catalogue.
pub fn resolve_flow_shared_payload(payload: &str) -> Result<String, FlowCoreError> {
    let Some(reference) = payload.strip_prefix(FLOW_SHARED_PAYLOAD_PREFIX) else {
        return Ok(payload.to_string());
    };
    match reference.split_once('\n') {
        Some((digest, body)) => {
            FLOW_SHARED_PAYLOAD_BODIES.with(|bodies| bodies.borrow_mut().insert(digest.to_string(), body.to_string()));
            Ok(body.to_string())
        }
        None => FLOW_SHARED_PAYLOAD_BODIES.with(|bodies| bodies.borrow().get(reference).cloned()).ok_or_else(|| FlowCoreError::UnknownSharedPayload(reference.to_string())),
    }
}

/// 🧩️ Resolves every part of a composed payload, in the order the sender wrote them.
pub fn resolve_flow_shared_payload_parts(payload: &str) -> Result<Vec<String>, FlowCoreError> {
    payload.split(FLOW_SHARED_PAYLOAD_PART_SEPARATOR).map(resolve_flow_shared_payload).collect()
}

/// 🧹️ Forgets every registered shared payload — the retirement seam a test needs to prove that a
/// reference to an unheld digest fails closed.
pub fn retire_flow_shared_payloads() {
    FLOW_SHARED_PAYLOAD_BODIES.with(|bodies| bodies.borrow_mut().clear());
}
//#endregion 📦️SharedPayloads

impl std::error::Error for FlowCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Dag(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework_pack_json::JsonError> for FlowCoreError {
    fn from(error: semio_framework_pack_json::JsonError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<semio_framework_value::ValueError> for FlowCoreError {
    fn from(error: semio_framework_value::ValueError) -> Self {
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

struct FlowPublicationShared<T:RetireOwned+Sync>(Arc<T>);
impl<T:RetireOwned+Sync> RetireOwned for FlowPublicationShared<T> {
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(semio_framework_value::retirement::shared::SharedControlledRetirement::lease(self.0))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<T>())}
    fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}

#[derive(semio_framework_value::RetireOwned)]
struct FlowBaselinePublication {
    snapshot:Option<TreeSnapshot>,
    channels:Option<EvalChannels>,
    shared_snapshot:Option<FlowPublicationShared<TreeSnapshot>>,
    shared_channels:Option<FlowPublicationShared<EvalChannels>>,
    generation:u64,
    converged:bool,
}

struct FlowBaselineLeases {
    snapshot:Option<Arc<TreeSnapshot>>,
    channels:Option<Arc<EvalChannels>>,
    current:Option<Arc<EvalChannels>>,
}

#[derive(semio_framework_value::RetireOwned)]
struct FlowPublicationDisplaced {
    exports:Option<HistoryFoldIndex<String,Dictionary>>,
    snapshot:Option<TreeSnapshot>,
    leases:FlowBaselineLeases,
}

impl semio_framework_value::retirement::RetireOwned for FlowBaselineLeases {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{
        use semio_framework_value::retirement::{RetireOwned,sequence,shared::SharedControlledRetirement};
        let snapshot:Box<dyn semio_framework_value::retirement::RetirementCursor>=match self.snapshot{Some(source)=>Box::new(SharedControlledRetirement::lease(source)),None=>().retirement()};
        let channels:Box<dyn semio_framework_value::retirement::RetirementCursor>=match self.channels{Some(source)=>Box::new(SharedControlledRetirement::lease(source)),None=>().retirement()};
        let current:Box<dyn semio_framework_value::retirement::RetirementCursor>=match self.current{Some(source)=>Box::new(SharedControlledRetirement::lease(source)),None=>().retirement()};
        sequence(vec![snapshot,channels,current])
    }
    fn retirement_birth_bytes(&self)->Option<usize>{
        use semio_framework_value::retirement::{sequence_birth_bytes,leaf_birth_bytes,shared::shared_retirement_birth_bytes};
        sequence_birth_bytes(&[if self.snapshot.is_some(){shared_retirement_birth_bytes::<TreeSnapshot>()}else{leaf_birth_bytes::<()>()},if self.channels.is_some(){shared_retirement_birth_bytes::<EvalChannels>()}else{leaf_birth_bytes::<()>()},if self.current.is_some(){shared_retirement_birth_bytes::<EvalChannels>()}else{leaf_birth_bytes::<()>()}])
    }
    fn controlled_retirement_supported()->bool{true}
}

/// 🏠️ Retained flow host: host_snapshot, dag scene, evaluation cache.
pub struct FlowHost {
    geometry_port: Option<Box<dyn crate::geometry::GeometryPort>>,
    operator_registry: Option<neural::SharedRegistry>,
    pub host_snapshot: FlowHostSnapshot,
    pub dag: DagHost,
    current_channels: Option<Arc<EvalChannels>>,
    export_payloads: HistoryFoldIndex<String, Dictionary>,
    pub last_eval_json: String,
    host_catalogue_json: String,
    /// 🧠️ The operator catalogue this host indexes, SHARED with every other live host: it is a pure
    /// projection of the extension registry, ~108 kB of it, and an evaluation tick that rebuilt its
    /// own copy paid 11-26 ms per tick for a map nobody had changed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    kind_infos: Arc<HistoryFoldIndex<String, OperatorInfo>>,
    neural_cache: Arc<NeuralCache>,
    previous_snapshot: Option<Arc<TreeSnapshot>>,
    previous_channels: Option<Arc<EvalChannels>>,
    pending_baseline_publication:Option<FlowBaselinePublication>,
    baseline_retirement:Option<ControlledRetirement<FlowPublicationDisplaced>>,
    baseline_publication_progress:RetainedCloneProgress,
    pending_evaluation:Option<neural::BudgetedEvalState>,
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
    /// ↩️ Undo/redo, backed by the standard `crate::os_store::ArtifactStore<FlowHostSnapshot, FlowMutation>`
    /// mechanism (see the `impl FlowHost`'s `🔖️History` region) instead of a hand-rolled snapshot stack.
    history_store: Option<FlowStore>,
    pending_history_baseline: Option<FlowHostSnapshot>,
    /// 🚩️ Armed by `begin_change`/`begin_gesture` for a discrete mutation not yet flushed into
    /// `history_store` — lets `can_undo` reflect it immediately, mirroring how the old snapshot stack's
    /// `begin_change` pushed synchronously instead of lazily.
    pending_change: bool,
    /// 🧾️ The concrete flow leaves the edit in progress emitted at its gesture sites, in application order; recorded as ONE
    /// store transaction by `record_history_edit`, retired cold when discarded.
    pending_leaves: Vec<FlowMutation>,
    /// 🧯️ The refusal of the last history row, kept until [`FlowHost::take_history_fault`] hands it over.
    history_fault: Option<FlowCoreError>,
    /// 🧪️ The leaves of every recorded edit, in order, so tests read what each gesture emitted.
    #[cfg(test)]
    recorded: Vec<Vec<FlowMutation>>,
    /// 🔗️ How many dag journal rows predated the open gesture, so only the gesture's own rows become leaves.
    journal_mark: usize,
    /// ✏️ The note widget an inline note edit is changing, recorded as one `ChangeWidget` at `note_commit_edit`.
    edited_note: Option<String>,
    /// 🖐️ `true` while a coalescing gesture (drag, inline note edit) is in progress — guards
    /// `begin_change` from checkpointing mid-gesture; see `begin_gesture`/`commit_gesture_history`.
    gesture_active: bool,
    /// 🌊️ The contributed-operator requests the last budgeted step parked — ONE topological wave,
    /// never one node: every member's inputs were ready in the same walk, so they are independent by
    /// construction and all cross to their plugins on the same hop.
    pending_extension_evals: Vec<neural::PendingExtensionEval>,
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
        Self::from_host_snapshot(FlowHostSnapshot::default())
    }
}

impl FlowHost {
    /// 📔️ Supplies immutable operator implementations owned by this host's composition.
    pub fn with_operator_registry(mut self, registry: neural::SharedRegistry) -> Self {
        assert!(self.operator_registry.is_none(), "operator registry is supplied once");
        self.set_neuron_kind_info_map(Arc::new(registry.operator_infos().map(|info| (info.id.clone(),info.clone())).collect()));
        self.operator_registry = Some(registry);
        self.baseline_registry_generation = 0;
        self
    }
    fn operator_registry(&self) -> neural::SharedRegistry { self.operator_registry.clone().unwrap_or_else(flow_operator_registry) }
    fn operator_registry_generation(&self) -> u64 { if self.operator_registry.is_some() { 0 } else { flow_extension_registry_generation() } }
    /// 🔌️ Supplies one geometry authority that this owner retains and explicitly closes.
    pub fn with_geometry_port(mut self, port: Box<dyn crate::geometry::GeometryPort>) -> Self {
        assert!(self.geometry_port.is_none(), "geometry authority is supplied once");
        self.geometry_port = Some(port);
        self
    }
    pub fn geometry_port(&self) -> Result<&dyn crate::geometry::GeometryPort, String> {
        self.geometry_port.as_deref().ok_or_else(|| "flow.geometry-port-missing".into())
    }
    pub fn from_host_snapshot(host_snapshot: FlowHostSnapshot) -> Self {
        Self::from_host_snapshot_with_cache(host_snapshot, Arc::new(NeuralCache::new()))
    }

    /// 🧠️ Builds a host sharing an existing [`NeuralCache`] — lets a long-lived caller (e.g. a
    /// stateless request/response program boundary that reconstructs `FlowHost` on every call)
    /// keep per-node memoization alive across those reconstructions instead of discarding it.
    pub fn from_host_snapshot_with_cache(host_snapshot: FlowHostSnapshot, neural_cache: Arc<NeuralCache>) -> Self {
        Self::from_host_snapshot_with_cache_and_infos(host_snapshot, neural_cache, Arc::default())
    }

    /// 🏠️ Builds a host that ALREADY indexes `kind_infos`. The ONE construction path for a caller
    /// that would set the operator catalogue immediately afterwards: the bare constructor builds its
    /// dag against an empty catalogue, and the setter's own `rebuild_dag` then throws that work away
    /// — twice per evaluation tick (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn from_host_snapshot_with_cache_and_infos(mut host_snapshot: FlowHostSnapshot, neural_cache: Arc<NeuralCache>, kind_infos: Arc<HistoryFoldIndex<String, OperatorInfo>>) -> Self {
        dedupe_host_snapshot_widgets(&mut host_snapshot);
        let mut host = Self {
            geometry_port: None,
            operator_registry: None,
            host_snapshot: host_snapshot,
            dag: DagHost::from_host_snapshot(DagHostSnapshot { schema: "dag.hostDocument".into(), camera: semio_framework_artifact_infinite_dag::DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            current_channels: None,
            export_payloads: HistoryFoldIndex::new(),
            last_eval_json: String::new(),
            host_catalogue_json: String::new(),
            kind_infos,
            neural_cache,
            previous_snapshot: None,
            previous_channels: None,
            pending_baseline_publication:None,
            baseline_retirement:None,
            baseline_publication_progress:Default::default(),
            pending_evaluation:None,
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
            pending_leaves: Vec::new(),
            history_fault: None,
            #[cfg(test)]
            recorded: Vec::new(),
            journal_mark: 0,
            edited_note: None,
            gesture_active: false,
            pending_extension_evals: Vec::new(),
            interaction_revision: 0,
            interaction_projection: None,
            displaced: neural::ValueRetirement::default(),
        };
        host.rebuild_dag();
        host.refresh_interaction_projection();
        host
    }

    /// 📥️ Loads fixture content while keeping catalogue, operator metadata, eval bridge, and the live camera. A load is a history
    /// reset: no edit survives it.
    pub fn replace_host_snapshot(&mut self, host_snapshot: FlowHostSnapshot) {
        self.apply_host_snapshot(host_snapshot, true, false);
    }

    /// 📥️ Scene resync: reloads fixture layout/content without discarding eval baseline or cached outputs. A scene that merely
    /// echoes the live content leaves the history alone; a scene whose content differs is a load and resets the history, because
    /// a content change may only enter the history as concrete leaves.
    pub fn resync_host_snapshot_from_scene(&mut self, host_snapshot: FlowHostSnapshot) {
        self.apply_host_snapshot(host_snapshot, false, true);
    }

    fn apply_host_snapshot(&mut self, mut host_snapshot: FlowHostSnapshot, force_reset: bool, preserve_eval: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        dedupe_host_snapshot_widgets(&mut host_snapshot);
        let load = force_reset || self.host_snapshot.widgets != host_snapshot.widgets || self.host_snapshot.synapses != host_snapshot.synapses || self.host_snapshot.layout != host_snapshot.layout;
        // 🎥️ Camera is ephemeral view state (same as undo/redo) — never snap the live pan/zoom when a
        // scene resync reloads fixture content (hover, eval tick, remote operations, …).
        let camera = self.host_snapshot.camera.clone();
        host_snapshot.camera = camera;
        std::mem::replace(&mut self.host_snapshot, host_snapshot).retire_cold();
        if !preserve_eval {
            self.displace_eval_state().expect("cold fixture load requires settled original baseline custody");
            self.last_eval_json.clear();
        }
        self.pan_anchor = None;
        self.ghost_node = None;
        self.rebuild_dag();
        self.refresh_interaction_projection();
        if load {
            self.reset_history_store();
            self.pending_change = false;
            self.gesture_active = false;
            self.edited_note = None;
            self.discard_pending_leaves();
        }
    }

    /// 🧹️ Restarts the history at the live content: the store is reset to it (when one exists) and the armed baseline is
    /// retired. Used by every load and by a refused history row, since the store can no longer represent the live content.
    fn reset_history_store(&mut self) {
        if let Some(store) = self.history_store.as_mut() {
            let envelope = create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow-host", self.host_snapshot.clone(), None);
            ::semio_framework_async::poll::resolve_ready(store.reset(envelope)).expect("failed to reset flow history store");
            store.install_document_store_owners_exact(FlowHostSnapshot::member_store_owners());
        }
        if let Some(stale) = self.pending_history_baseline.take() {
            stale.retire_cold();
        }
    }

    pub fn parse_host_snapshot_json(json: &str) -> Result<FlowHostSnapshot, FlowCoreError> {
        Ok(semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?)
    }

    pub fn host_snapshot_json(&self) -> Result<String, FlowCoreError> {
        Ok(semio_framework_pack_json::to_json_string(&self.host_snapshot))
    }

    pub fn document(&self) -> FlowArtifact {
        self.host_snapshot.to_artifact()
    }

    pub fn catalogue_json(&self) -> Result<String, FlowCoreError> {
        let sections = merge_catalogue_sections(&self.host_catalogue_json)?;
        Ok(semio_framework_pack_json::to_json_string(&sections))
    }

    pub fn set_host_catalogue_json(&mut self, json: &str) {
        self.host_catalogue_json = json.to_string();
    }

    /// 📦️ Installs the palette sections from a content-addressed payload — see
    /// [`resolve_flow_shared_payload`]. Several parts are concatenated section by section.
    pub fn set_host_catalogue_payload(&mut self, payload: &str) -> Result<(), FlowCoreError> {
        let parts = resolve_flow_shared_payload_parts(payload)?;
        if let [single] = parts.as_slice() {
            self.set_host_catalogue_json(single);
            return Ok(());
        }
        let mut sections: Vec<CatalogueSection> = Vec::new();
        for part in &parts {
            if part.trim().is_empty() {
                continue;
            }
            sections.extend(semio_framework_pack_json::from_json_str::<Vec<CatalogueSection>>(part, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default());
        }
        self.set_host_catalogue_json(&semio_framework_pack_json::to_json_string(&sections));
        Ok(())
    }

    /// 🔎️ The operator kind ids this host indexes, sorted — the public read-back a law composes a
    /// table against without reaching into `kind_infos`.
    pub fn neuron_kind_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.kind_infos.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// 📦️ Installs the operator kind infos from a content-addressed payload — see
    /// [`resolve_flow_shared_payload`].
    ///
    /// The table is composed of its parts in order (the app-static catalogue, then whatever records
    /// the scene itself derived), so a later id wins over an earlier one exactly as it did when the
    /// host concatenated the two sources into one body itself.
    pub fn set_neuron_kind_infos_payload(&mut self, payload: &str) -> Result<(), FlowCoreError> {
        let parts = resolve_flow_shared_payload_parts(payload)?;
        let mut infos: HistoryFoldIndex<String, OperatorInfo> = HistoryFoldIndex::new();
        for part in &parts {
            if part.trim().is_empty() {
                continue;
            }
            for info in semio_framework_pack_json::from_json_str::<Vec<OperatorInfo>>(part, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default() {
                infos.insert(info.id.clone(), info);
            }
        }
        self.install_kind_infos(Arc::new(infos));
        Ok(())
    }

    /// 🧹️ Installs a new operator catalogue and retires the one it displaces — but ONLY when this
    /// host was its last owner. The catalogue is shared by every live host
    /// ([`FlowHost::kind_infos`]), and its `ChannelSpec::default` values are `Value`s that fail
    /// closed on a bare drop, so a plain assignment aborted the process the moment a host swapped
    /// a uniquely-owned catalogue out (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn install_kind_infos(&mut self, kind_infos: Arc<HistoryFoldIndex<String, OperatorInfo>>) {
        if let Some(displaced) = Arc::into_inner(std::mem::replace(&mut self.kind_infos, kind_infos)) {
            displaced.retire_cold();
        }
        self.rebuild_dag();
    }

    pub fn set_neuron_kind_infos_json(&mut self, json: &str) {
        self.install_kind_infos(Arc::new(if json.trim().is_empty() { HistoryFoldIndex::new() } else { semio_framework_pack_json::from_json_str::<Vec<OperatorInfo>>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|items| items.into_iter().map(|info| (info.id.clone(), info)).collect()).unwrap_or_default() }));
    }

    /// 🧠️ Same as `set_neuron_kind_infos_json` but over the already-built id-keyed map — the ONE
    /// in-process path, because the JSON form of this catalogue is ~108 kB and an evaluation tick
    /// that serialized and re-parsed it spent 11-26 ms per tick doing nothing else
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn set_neuron_kind_info_map(&mut self, infos: Arc<HistoryFoldIndex<String, OperatorInfo>>) {
        self.install_kind_infos(infos);
    }

    /// 🧠️ Same as `set_neuron_kind_infos_json` but over the typed `NodeGraphScene.operators` records.
    pub fn set_neuron_kind_infos(&mut self, infos: &[ui_wgpu::wgpu::NodeGraphOperatorRecord]) {
        self.install_kind_infos(Arc::new(infos.iter().map(|record| (record.id.clone(), node_graph_operator_record_to_operator_info(record))).collect()));
    }

    pub fn evaluate(&mut self) -> Result<String, FlowCoreError> {
        self.evaluate_internal();
        Ok(self.last_eval_json.clone())
    }

    /// 📥️ Applies channel-structured eval JSON from an off-thread worker without re-running operators.
    pub fn apply_eval_outputs_json(&mut self, json: &str)->Result<(),ValueError> {
        if self.pending_baseline_publication.is_some(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline publication must advance before another intake"))}
        if is_global_eval_error_json(json) {
            self.dag.clear_computing();
            return Ok(());
        }
        let outputs = outputs_from_channel_eval_json(json);
        let inputs = inputs_from_channel_eval_json(json);
        let channels = EvalChannels { outputs, inputs };
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.previous_snapshot.as_deref(), &snapshot);
        let evaluated_generation = self.operator_registry_generation();
        let converged = self.probe_eval_outputs_converged(&tree, &seeds, &dirty, &channels);
        tree.retire_cold();
        seeds.retire_cold();
        self.last_eval_json = json.to_string();
        if converged {
            self.apply_preview_outputs(&channels.outputs);
            self.apply_export_outputs(&channels.outputs);
            self.begin_baseline_publication(snapshot,channels,evaluated_generation).unwrap_or_else(|_|unreachable!("original baseline intake was checked before constructing owners"));
            self.dag.clear_computing();
        } else {
            channels.retire_cold();
            self.refresh_computing_chrome_from_pending();
        }
        Ok(())
    }

    fn probe_eval_outputs_converged(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>, dirty: &HashSet<String>, channels: &EvalChannels) -> bool {
        let registry = self.operator_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        let mut probe_never_dispatches = |kind: &str, _: &Dictionary| -> Result<Dictionary, EvalError> { Err(EvalError::InvalidInput(format!("apply_eval_outputs_json probed a dispatch for {kind}"))) };
        match evaluator.evaluate_channels_budgeted(tree, seeds, &self.kind_infos, &mut probe_never_dispatches, &self.neural_cache, dirty, Some(channels), EvalStepBudget::PROBE,&|_|true) {
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
    pub fn install_eval_baseline(&mut self,snapshot:Option<Arc<TreeSnapshot>>,channels:Option<Arc<EvalChannels>>,registry_generation:u64,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Option<Arc<TreeSnapshot>>,Option<Arc<EvalChannels>>)>{
        let refusal=if grant.maximum_items==0{Some((ValueRefusalKind::WorkLimit,"baseline lease handoff requires one admitted item"))}
            else if grant.maximum_depth==0{Some((ValueRefusalKind::DepthLimit,"baseline lease handoff requires source depth"))}
            else if !self.baseline_publication_terminal_is_empty(){Some((ValueRefusalKind::WorkLimit,"baseline publication must complete before installing another lease"))}
            else if self.current_channels.is_some(){Some((ValueRefusalKind::UnsupportedOwner,"baseline installation requires original output lease handoff"))}else{None};
        if let Some((kind,message))=refusal{return Err((ValueError::literal(kind,message),snapshot,channels))}
        let original=FlowBaselineLeases{snapshot:self.previous_snapshot.take(),channels:self.previous_channels.take(),current:self.current_channels.take()};
        if original.snapshot.is_some()||original.channels.is_some()||original.current.is_some(){self.baseline_retirement=Some(ControlledRetirement::new(FlowPublicationDisplaced{snapshot:None,leases:original}).unwrap_or_else(|_|unreachable!("original baseline leases are supported")));}
        self.previous_snapshot=snapshot;self.current_channels=channels.clone();self.previous_channels=channels;self.baseline_registry_generation=registry_generation;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🔎️ Reads the original active output owner or its immutable installed baseline lease.
    pub fn output_channels(&self,widget_id:&str)->Option<&Dictionary>{
        self.current_channels.as_deref().or(self.previous_channels.as_deref())?.outputs.get(widget_id)
    }
    /// 📖️ Borrows actual output rows without restoring a copied map.
    pub fn output_entries(&self)->impl Iterator<Item=(&String,&Dictionary)>{
        self.current_channels.as_deref().or(self.previous_channels.as_deref()).into_iter().flat_map(|channels|channels.outputs.iter())
    }

    /// 🧹️ Hands every value displaced since the last drain to the artifact's bounded retirement
    /// ladder. Called at the START of an evaluation tick, so a root the CURRENT tick's `NeuralCache`
    /// or `previous_channels` still reads is never torn down under it — the frontier therefore holds
    /// at most one tick's displacement, and what is left when the host closes is taken over by
    /// [`FlowHostRetirement::new`].
    fn drain_displaced(&mut self) {
        while !matches!(self.displaced.close_step(64, 65_536), neural::ValueRetirementStep::Complete) {}
    }

    /// 📥️ Retains the original baseline and export arenas inline before funded retirement.
    fn displace_eval_state(&mut self)->Result<(),ValueError>{
        if self.pending_baseline_publication.is_some()||self.baseline_retirement.is_some(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline publication must finish before displacement"))}
        let original=FlowPublicationDisplaced{exports:Some(std::mem::take(&mut self.export_payloads)),snapshot:None,leases:FlowBaselineLeases{snapshot:self.previous_snapshot.take(),channels:self.previous_channels.take(),current:self.current_channels.take()}};
        self.baseline_retirement=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original baseline fields declare typed retirement")));Ok(())
    }

    /// 🧵️ Captures this host's eval baseline for persistence on a durable driver.
    pub fn eval_baseline(&self) -> (Option<Arc<TreeSnapshot>>, Option<Arc<EvalChannels>>) {
        (self.previous_snapshot.clone(), self.previous_channels.clone())
    }

    /// 📥️ Retains the evaluator's genuine computed owners before admitting shared publication.
    pub fn begin_baseline_publication(&mut self,snapshot:TreeSnapshot,channels:EvalChannels,generation:u64)->Result<(),(ValueError,TreeSnapshot,EvalChannels)>{
        self.begin_eval_publication(snapshot,channels,generation,true)
    }

    fn begin_eval_publication(&mut self,snapshot:TreeSnapshot,channels:EvalChannels,generation:u64,converged:bool)->Result<(),(ValueError,TreeSnapshot,EvalChannels)>{
        if !self.baseline_publication_terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original evaluation publication is already pending"),snapshot,channels))}
        self.pending_baseline_publication=Some(FlowBaselinePublication{snapshot:Some(snapshot),channels:Some(channels),shared_snapshot:None,shared_channels:None,generation,converged});Ok(())
    }
    /// 📏️ Borrows each independent currency of the actual publication owner.
    pub fn next_baseline_publication_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.baseline_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
        let Some(owner)=self.pending_baseline_publication.as_ref()else{return Ok(Default::default())};
        let capacity_bytes=if owner.converged&&owner.snapshot.is_some(){semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<TreeSnapshot>()}else if owner.channels.is_some(){semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<EvalChannels>()}else{0};
        Ok(RetirementDemand{capacity_bytes,depth:1,..Default::default()})
    }
    /// 🎟️ Creates one admitted Arc header or transfers the same original baseline owners.
    pub fn baseline_publication_step_progress(&self)->RetainedCloneProgress{self.baseline_publication_progress}

    pub fn baseline_publication_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.baseline_publication_progress=Default::default();
        let empty=RetainedCloneProgress::default();
        if self.baseline_publication_terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        let demand=self.next_baseline_publication_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original baseline publication exceeds admitted depth"))}
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(owner)=self.baseline_retirement.as_mut(){
            let result=owner.step(grant);self.baseline_publication_progress=owner.step_progress();let step=result.map_err(|error|error.with_retained_progress(self.baseline_publication_progress))?;
            let progress=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,owner.terminal_is_empty(),"Flow original baseline source")?.progress();
            if owner.terminal_is_empty(){self.baseline_retirement=None;}
            return Ok(if self.baseline_publication_terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
        }
        let owner=self.pending_baseline_publication.as_mut().unwrap();
        if owner.converged{if let Some(source)=owner.snapshot.take(){owner.shared_snapshot=Some(FlowPublicationShared(Arc::new(source)));self.baseline_publication_progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..empty};return Ok(RetainedCloneStep::Progress(self.baseline_publication_progress))}}
        if let Some(source)=owner.channels.take(){owner.shared_channels=Some(FlowPublicationShared(Arc::new(source)));self.baseline_publication_progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..empty};return Ok(RetainedCloneStep::Progress(self.baseline_publication_progress))}
        let original=FlowPublicationDisplaced{exports:None,snapshot:owner.snapshot.take(),leases:FlowBaselineLeases{
            snapshot:if owner.converged{self.previous_snapshot.take()}else{None},channels:if owner.converged{self.previous_channels.take()}else{None},current:self.current_channels.take(),
        }};
        if original.snapshot.is_some()||original.leases.snapshot.is_some()||original.leases.channels.is_some()||original.leases.current.is_some(){self.baseline_retirement=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original publication sources are supported")));}
        self.current_channels=owner.shared_channels.take().map(|lease|lease.0);
        if owner.converged{self.previous_snapshot=owner.shared_snapshot.take().map(|lease|lease.0);self.previous_channels=self.current_channels.clone();self.baseline_registry_generation=owner.generation;}
        self.pending_baseline_publication=None;
        let progress=RetainedCloneProgress{copied_items:1,..empty};self.baseline_publication_progress=progress;
        Ok(if self.baseline_publication_terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    /// 🏁️ Publication completes only after all displaced original leases close.
    pub fn baseline_publication_terminal_is_empty(&self)->bool{self.pending_baseline_publication.is_none()&&self.baseline_retirement.is_none()}

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
        self.baseline_registry_generation == self.operator_registry_generation()
    }

    /// ⏮️ The snapshot to diff this evaluation against — `None` once the registry it was computed
    /// against has been replaced, which makes every node dirty.
    fn current_baseline_snapshot(&self) -> Option<&TreeSnapshot> {
        self.previous_snapshot.as_deref().filter(|_| self.baseline_is_current())
    }

    /// ⏮️ The channels this evaluation may free-ride on, under the same condition.
    fn current_baseline_channels(&self) -> Option<&EvalChannels> {
        self.previous_channels.as_deref().filter(|_| self.baseline_is_current())
    }

    /// ⚡️ Whether the incremental fast path may skip this evaluation entirely: nothing is dirty, a
    /// full channel baseline is held, and outputs are published.
    ///
    /// One predicate, read by every caller that owns a fast path ([`FlowHost::evaluate_step`] and
    /// [`FlowHost::pending_eval_widget_ids`]), so the two can never drift apart.
    fn baseline_answers_everything(&self, dirty: &HashSet<String>) -> bool {
        dirty.is_empty() && self.current_baseline_channels().is_some() && self.output_entries().next().is_some()
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
    pub fn set_node_statuses(&mut self,statuses:&DagNodeStatuses){self.dag.set_node_statuses(statuses);}

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
        self.host_snapshot.camera = CameraJson { x, y, zoom: zoom.clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX) };
        self.dag.set_camera(x, y, self.host_snapshot.camera.zoom);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    /// 📷️ The ONE camera of a flow surface. `self.host_snapshot.camera` is the authority — it is what
    /// `screen_to_world_point` projects with, what `build_dag_host_snapshot_v1` re-seeds the dag copy from
    /// on every rebuild, and what the renderer publishes as `nodeGraphViewport`. The dag's own
    /// `host_snapshot.camera` is a derived paint copy; reading it instead is how a fit got published stale.
    pub fn camera(&self) -> [f64; 3] {
        [self.host_snapshot.camera.x, self.host_snapshot.camera.y, self.host_snapshot.camera.zoom]
    }

    /// 📷️ Adopts a camera the DAG computed for itself (a fit, a refit, an opening decision) into the
    /// authority, so the published, projected and painted cameras cannot drift apart.
    fn adopt_dag_camera(&mut self) {
        self.host_snapshot.camera = CameraJson { x: self.dag.host_snapshot.camera.x, y: self.dag.host_snapshot.camera.y, zoom: self.dag.host_snapshot.camera.zoom };
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    /// 🖼️ Frames the whole graph and keeps the authority on the fitted camera.
    ///
    /// @see `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagHost::fit_camera_to_content`
    pub fn fit_camera_to_content(&mut self) -> bool {
        let fitted = self.dag.fit_camera_to_content();
        if fitted {
            self.adopt_dag_camera();
        }
        fitted
    }

    /// 🖼️ The opening-camera decision, with the authority kept on whichever camera won.
    ///
    /// @see `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagHost::adopt_camera_or_fit`
    pub fn adopt_camera_or_fit(&mut self, x: f64, y: f64, zoom: f64) -> bool {
        let fitted = self.dag.adopt_camera_or_fit(x, y, zoom);
        self.adopt_dag_camera();
        fitted
    }

    /// 🖼️ Re-fits a graph that changed under a live camera, with the authority kept on the result.
    ///
    /// @see `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagHost::refit_camera_if_content_left_view`
    pub fn refit_camera_if_content_left_view(&mut self) -> bool {
        let refitted = self.dag.refit_camera_if_content_left_view();
        if refitted {
            self.adopt_dag_camera();
        }
        refitted
    }

    fn refresh_interaction_projection(&mut self) {
        self.interaction_projection = self.dag.bounded_interaction_projection(self.interaction_revision).ok();
    }

    /// 🖱️ Re-seeds the bounded pointer-plan projection after a SCREEN-path gesture mutated this host
    /// directly (drawing a wire, dragging the minimap viewport, dragging an inline widget) — without
    /// it `plan_pointer` answers `Unsupported` for the rest of the session, because its stored
    /// projection is a revision behind and nothing else on that path bumps it back into step.
    pub fn resync_interaction_projection(&mut self) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    pub fn plan_wheel(&self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) -> FlowWheelPlan {
        let camera = &self.host_snapshot.camera;
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
        let camera = &self.host_snapshot.camera;
        if self.interaction_revision != plan.revision || [camera.x.to_bits(), camera.y.to_bits(), camera.zoom.to_bits()] != [plan.expected[0].to_bits(), plan.expected[1].to_bits(), plan.expected[2].to_bits()] {
            return false;
        }
        self.host_snapshot.camera = CameraJson { x: plan.next[0], y: plan.next[1], zoom: plan.next[2] };
        self.dag.set_camera(plan.next[0], plan.next[1], plan.next[2]);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
        true
    }

    /// 🫳️ Whether this host is mid-way through a BOUNDED gesture — a node drag, a marquee or a pan
    /// started by `commit_pointer` and not yet released. A gesture belongs to the path that started it.
    pub fn bounded_pointer_gesture_active(&self) -> bool {
        self.interaction_projection.is_some_and(|projection| projection.gesture_active())
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
            if self.host_snapshot.layout.contains_key(id) {
                self.host_snapshot.layout.insert(id.to_owned(), WidgetLayout { x, y });
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
        let zoom = (self.host_snapshot.camera.zoom * factor).clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
        self.host_snapshot.camera.zoom = zoom;
        self.dag.set_camera(self.host_snapshot.camera.x, self.host_snapshot.camera.y, zoom);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        let after = self.screen_to_world_point(sx, sy);
        self.host_snapshot.camera.x += before.x - after.x;
        self.host_snapshot.camera.y += before.y - after.y;
        self.dag.set_camera(self.host_snapshot.camera.x, self.host_snapshot.camera.y, zoom);
    }

    pub fn wheel_pan_screen(&mut self, delta_x: f64, delta_y: f64) {
        let zoom = self.host_snapshot.camera.zoom;
        let x = self.host_snapshot.camera.x - delta_x / zoom;
        let y = self.host_snapshot.camera.y - delta_y / zoom;
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
        let descriptor: WidgetDescriptor = semio_framework_pack_json::from_json_str(descriptor_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
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
        let descriptor: WidgetDescriptor = semio_framework_pack_json::from_json_str(descriptor_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        let id = descriptor_explicit_id(&descriptor).unwrap_or_else(|| self.next_widget_id(&descriptor));
        if self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id) {
            return Err(FlowCoreError::WidgetIdExists(id));
        }
        let widget = widget_from_descriptor(&descriptor, id.clone(), &self.kind_infos);
        let added = FlowMutation::AddWidget(AddWidget { index: u32::try_from(self.host_snapshot.widgets.len()).unwrap_or(u32::MAX), widget: widget.clone() });
        self.host_snapshot.widgets.push(widget);
        self.host_snapshot.layout.insert(id.clone(), WidgetLayout { x: world_x, y: world_y });
        self.note_leaves([added].into_iter().chain(self.layout_leaf([id.as_str()])));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(id)
    }

    pub fn remove_widget(&mut self, widget_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let before = self.host_snapshot.widgets.len();
        let touching: Vec<String> = self.host_snapshot.synapses.iter().filter(|s| s.from == widget_id || s.to == widget_id).map(|s| s.id.clone()).collect();
        let had_layout = self.host_snapshot.layout.contains_key(widget_id);
        self.host_snapshot.widgets.retain(|w| widget_id_for(w) != widget_id);
        if self.host_snapshot.widgets.len() == before {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        }
        self.host_snapshot.layout.remove(widget_id);
        self.host_snapshot.synapses.retain(|s| s.from != widget_id && s.to != widget_id);
        let unplaced = had_layout.then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: widget_id.to_string(), layout: None }] }));
        self.note_leaves(touching.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain([FlowMutation::RemoveWidget(RemoveWidget { id: widget_id.to_string() })]));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn move_widget(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), FlowCoreError> {
        if !self.host_snapshot.widgets.iter().any(|w| widget_id_for(w) == widget_id) {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        }
        self.begin_change();
        self.host_snapshot.layout.insert(widget_id.to_string(), WidgetLayout { x, y });
        let moved = self.layout_leaf([widget_id]);
        self.note_leaves(moved);
        self.dag.set_widget_position(widget_id, x, y)?;
        self.checked_change()?;
        Ok(())
    }

    pub fn connect(&mut self, from_id: &str, to_id: &str) -> Result<String, FlowCoreError> {
        let from_port = first_output_port(from_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        let to_port = first_input_port(to_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        self.connect_ports(from_id, &from_port, to_id, &to_port)
    }

    fn next_synapse_id(&mut self) -> String {
        loop {
            self.next_synapse_serial = self.next_synapse_serial.wrapping_add(1);
            let id = format!("s{}", self.next_synapse_serial);
            if self.host_snapshot.synapses.iter().all(|synapse| synapse.id != id) {
                return id;
            }
        }
    }

    pub fn connect_ports(&mut self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, FlowCoreError> {
        self.begin_change();
        if from_id == to_id {
            return Err(FlowCoreError::SelfConnection);
        }
        if !widget_has_output(from_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(from_id.to_string()));
        }
        if !widget_has_input(to_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoInputPort(to_id.to_string()));
        }
        if !widget_has_declared_port(from_id, from_port, PortSide::Output, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::UnknownOutputPort(from_port.to_string()));
        }
        if !widget_has_declared_port(to_id, to_port, PortSide::Input, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::UnknownInputPort(to_port.to_string()));
        }
        let existing: Vec<(String, String)> = self.host_snapshot.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err(FlowCoreError::CycleWouldBeCreated);
        }
        if self.host_snapshot.synapses.iter().any(|s| s.from == from_id && s.from_port == from_port && s.to == to_id && s.to_port == to_port) {
            return Err(FlowCoreError::ConnectionAlreadyExists);
        }
        let source_types = widget_port_value_types(from_id, from_port, PortSide::Output, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        let target_types = widget_port_value_types(to_id, to_port, PortSide::Input, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        if !port_value_types_compatible(&source_types, &target_types) {
            return Err(FlowCoreError::IncompatiblePortTypes {
                source: format!("{from_id}@{from_port}"),
                source_type: source_types.join(","),
                target: format!("{to_id}@{to_port}"),
                target_type: target_types.join(","),
            });
        }
        let displaced: Vec<String> = self.host_snapshot.synapses.iter().filter(|s| s.to == to_id && s.to_port == to_port).map(|s| s.id.clone()).collect();
        self.host_snapshot.synapses.retain(|s| !(s.to == to_id && s.to_port == to_port));
        let synapse_id = self.next_synapse_id();
        let synapse = SynapseSpec { id: synapse_id.clone(), from: from_id.to_string(), to: to_id.to_string(), from_port: from_port.to_string(), to_port: to_port.to_string() };
        let added = FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(self.host_snapshot.synapses.len()).unwrap_or(u32::MAX), synapse: synapse.clone() });
        self.host_snapshot.synapses.push(synapse);
        self.note_leaves(displaced.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain([added]));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(synapse_id)
    }

    pub fn add_input_port(&mut self, widget_id: &str, index: usize) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_input.clone()).ok_or_else(|| FlowCoreError::NotVariadicInput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
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
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.to != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.to_port.parse::<usize>() {
                if old_index >= insert_at {
                    synapse.to_port = (old_index + 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        *input_ports = (0..ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse })).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn remove_input_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_input.clone()).ok_or_else(|| FlowCoreError::NotVariadicInput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
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
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| synapse.to == widget_id && synapse.to_port == port_id).map(|synapse| synapse.id.clone()).collect();
        self.host_snapshot.synapses.retain(|synapse| !(synapse.to == widget_id && synapse.to_port == port_id));
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.to != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.to_port.parse::<usize>() {
                if old_index > remove_index {
                    synapse.to_port = (old_index - 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        let mut next_ports = ports;
        next_ports.remove(remove_index);
        *input_ports = (0..next_ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse }))).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn add_output_port(&mut self, widget_id: &str, index: usize) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_output.clone()).ok_or_else(|| FlowCoreError::NotVariadicOutput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
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
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.from != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.from_port.parse::<usize>() {
                if old_index >= insert_at {
                    synapse.from_port = (old_index + 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        *output_ports = (0..ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse })).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn remove_output_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_output.clone()).ok_or_else(|| FlowCoreError::NotVariadicOutput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
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
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| synapse.from == widget_id && synapse.from_port == port_id).map(|synapse| synapse.id.clone()).collect();
        self.host_snapshot.synapses.retain(|synapse| !(synapse.from == widget_id && synapse.from_port == port_id));
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.from != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.from_port.parse::<usize>() {
                if old_index > remove_index {
                    synapse.from_port = (old_index - 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        let mut next_ports = ports;
        next_ports.remove(remove_index);
        *output_ports = (0..next_ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse }))).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn disconnect(&mut self, synapse_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let before = self.host_snapshot.synapses.len();
        self.host_snapshot.synapses.retain(|s| s.id != synapse_id);
        if self.host_snapshot.synapses.len() == before {
            return Err(FlowCoreError::UnknownSynapse(synapse_id.to_string()));
        }
        self.note_leaves([FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse_id.to_string() })]);
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    // #region GumballEditing
    /// 🔀️ Splices `mid_id` between `anchor_id` and its downstream consumers on `anchor_out_port`.
    pub fn insert_between(&mut self, anchor_id: &str, anchor_out_port: &str, mid_id: &str, mid_in_port: &str, mid_out_port: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        if !self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == anchor_id) {
            return Err(FlowCoreError::UnknownWidget(anchor_id.to_string()));
        }
        if !self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == mid_id) {
            return Err(FlowCoreError::UnknownWidget(mid_id.to_string()));
        }
        if anchor_id == mid_id {
            return Err(FlowCoreError::SelfInsertion);
        }
        if !widget_has_output(anchor_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(anchor_id.to_string()));
        }
        if !widget_has_input(mid_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoInputPort(mid_id.to_string()));
        }
        if !widget_has_output(mid_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(mid_id.to_string()));
        }
        let existing: Vec<(String, String)> = self.host_snapshot.synapses.iter().map(|synapse| (synapse.from.clone(), synapse.to.clone())).collect();
        if would_create_cycle(&existing, anchor_id, mid_id) {
            return Err(FlowCoreError::CycleWouldBeCreated);
        }
        let mid_has_input = self.host_snapshot.synapses.iter().any(|synapse| synapse.to == mid_id);
        let mut rerouted = Vec::new();
        if !mid_has_input {
            for synapse in &mut self.host_snapshot.synapses {
                if synapse.from == anchor_id && synapse.from_port == anchor_out_port {
                    synapse.from = mid_id.to_string();
                    synapse.from_port = mid_out_port.to_string();
                    rerouted.push(synapse.clone());
                }
            }
        }
        self.note_leaves(rerouted.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse })));
        if self.host_snapshot.synapses.iter().any(|synapse| synapse.from == anchor_id && synapse.from_port == anchor_out_port && synapse.to == mid_id && synapse.to_port == mid_in_port) {
            self.rebuild_dag();
            self.checked_change()?;
            return Ok(());
        }
        let synapse_id = self.next_synapse_id();
        let synapse = SynapseSpec { id: synapse_id, from: anchor_id.to_string(), to: mid_id.to_string(), from_port: anchor_out_port.to_string(), to_port: mid_in_port.to_string() };
        let added = FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(self.host_snapshot.synapses.len()).unwrap_or(u32::MAX), synapse: synapse.clone() });
        self.host_snapshot.synapses.push(synapse);
        self.note_leaves([added]);
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    /// ↔ Shifts widgets to the right of `anchor_id` to open layout space for inserted nodes.
    pub fn make_space(&mut self, anchor_id: &str, dx: f64, dy: f64) -> Result<(), FlowCoreError> {
        self.begin_change();
        let anchor_x = self.host_snapshot.layout.get(anchor_id).map(|layout| layout.x).ok_or_else(|| FlowCoreError::UnknownWidgetLayout(anchor_id.to_string()))?;
        let previous = std::mem::take(&mut self.host_snapshot.layout);
        let mut shifted = Vec::new();
        for (widget_id, layout) in &previous {
            let mut layout = layout.clone();
            if layout.x > anchor_x {
                layout.x += dx;
                layout.y += dy;
                shifted.push(widget_id.clone());
            }
            let _ = self.dag.set_widget_position(widget_id, layout.x, layout.y);
            self.host_snapshot.layout.insert(widget_id.clone(), layout);
        }
        let mut retirement = crate::retained::FlowRetirement::default();
        retirement.push(crate::retained::FlowOwner::Layouts(previous));
        retirement.retire_cold();
        let moved = self.layout_leaf(shifted.iter().map(String::as_str));
        self.note_leaves(moved);
        self.checked_change()?;
        Ok(())
    }

    /// 🧬️ Merges JSON params into a neuron widget for compact transform values.
    pub fn set_neuron_params(&mut self, widget_id: &str, params_json: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let patch: Dictionary = semio_framework_pack_json::from_json_str(params_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        let merged = match self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id) {
            Some(Widget::Neuron { params, .. }) => Ok(std::mem::replace(params, params.merge(&patch))),
            Some(_) => Err(FlowCoreError::NotNeuron(widget_id.to_string())),
            None => Err(FlowCoreError::UnknownWidget(widget_id.to_string())),
        };
        self.displaced.push_dictionary(patch);
        self.displaced.push_dictionary(merged?);
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.checked_change()?;
        Ok(())
    }
    // #endregion GumballEditing

    /// 🌳️ Recomputes widget positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self,opts:&DagLayoutOptions,control:&mut semio_framework_os_infinite::board::schema::layout::LayoutControl<'_>)->Result<(),FlowCoreError>{
        let mut dag=DagHost::from_host_snapshot_without_layout(self.build_dag_host_snapshot_v1());dag.canvas_theme=self.dag.canvas_theme;
        dag.reorganize(opts,control)?;self.begin_change();self.dag=dag;self.sync_from_dag();
        let placed: Vec<String> = self.host_snapshot.layout.keys().cloned().collect();
        let moved = self.layout_leaf(placed.iter().map(String::as_str));
        self.note_leaves(moved);
        self.checked_change()?;
        Ok(())
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "pointer-event handler mirroring this file's other screen-space input methods (pointer_move_screen/pointer_up_screen/wheel_screen) — position + button + modifier-key flags is the natural shape for this UI event, not a bundling candidate on its own without also restructuring its siblings"
    )]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        if pan {
            self.pan_anchor = Some((sx, sy, self.host_snapshot.camera.x, self.host_snapshot.camera.y));
            return;
        }
        self.clear_ghost_widget();
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.begin_gesture();
        self.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, false);
        if let Some((side, widget_id, index)) = self.dag.take_pending_port_insert() {
            let inserted = match side {
                dag::DagPortSide::Input => self.add_input_port(&widget_id, index),
                dag::DagPortSide::Output => self.add_output_port(&widget_id, index),
            };
            if inserted.is_ok() {
                self.dag.journal_port_insert(widget_id, side, index);
            }
            return;
        }
        self.sync_from_dag();
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.pan_anchor {
            let zoom = self.host_snapshot.camera.zoom;
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

    pub fn pointer_cancel_screen(&mut self) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.pan_anchor = None;
        self.dag.pointer_cancel_screen();
        if !self.gesture_active {
            return;
        }
        self.gesture_active = false;
        self.discard_pending_leaves();
        self.edited_note = None;
        let Some(mut baseline) = self.pending_history_baseline.take() else {
            return;
        };
        baseline.camera = self.host_snapshot.camera.clone();
        let stale = std::mem::replace(&mut self.host_snapshot, baseline);
        stale.retire_cold();
        self.rebuild_dag();
    }

    /// 🔗️ Drains the graph edits the last gesture performed as the `nodeGraphEdit` arguments the renderer dispatches
    /// (`dag::dag_graph_edit_rows_json`: `connect`, `disconnect`, `move` — the node-graph gesture record of design §13.3 —
    /// `setSlider`, `insertPort`). The journal narrates EVERY content change a gesture makes, so the renderer never
    /// re-publishes the whole fixture; a plain click, a marquee, a pan and a press that grabbed nothing journal nothing
    /// and are owed no dispatch at all.
    pub fn take_graph_edits_json(&mut self) -> String {
        let edits = self.dag.take_graph_edits();
        let refusal = self.dag.take_journal_refusal();
        dag::dag_graph_edit_rows_json(&edits, refusal)
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
        semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("ids".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&self.dag.preselect_widget_ids()))),
            ("removedIds".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&self.dag.preselect_removed_widget_ids()))),
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
        let nodes: Vec<String> = self.dag.selected_node_ids().into_iter().filter(|id| self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id)).collect();
        let edges = self.dag.selected_edge_ids();
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| edges.contains(&synapse.id) || nodes.contains(&synapse.from) || nodes.contains(&synapse.to)).map(|synapse| synapse.id.clone()).collect();
        let unplaced: Vec<FlowLayoutEntry> = nodes.iter().filter(|id| self.host_snapshot.layout.contains_key(id.as_str())).map(|id| FlowLayoutEntry { id: id.clone(), layout: None }).collect();
        self.dag.delete_selected();
        self.sync_from_dag();
        let unplaced = (!unplaced.is_empty()).then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: unplaced }));
        self.note_leaves(severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain(nodes.into_iter().map(|id| FlowMutation::RemoveWidget(RemoveWidget { id }))));
        self.checked_change()?;
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
        self.evaluate_step(EvalStepBudget::UNBOUNDED,&|_|true);
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
    pub fn evaluate_step(&mut self, budget: EvalStepBudget,source_required:&dyn Fn(u64)->bool) -> Result<Vec<String>,ValueError> {
        if !self.baseline_publication_terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline publication must advance before another evaluation"))}
        self.drain_displaced();
        self.pending_extension_evals.clear();
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.current_baseline_snapshot(), &snapshot);
        if self.baseline_answers_everything(&dirty) {
            tree.retire_cold();
            seeds.retire_cold();
            return Ok(Vec::new());
        }
        // 🔢 Read BEFORE the registry it describes: a replacement landing between the two reads
        // then stamps the baseline with the OLDER generation, which over-dirties the next step
        // (extra work, never a stale answer). Reading it after could stamp a generation the
        // evaluation never used.
        let evaluated_generation = self.operator_registry_generation();
        let registry = self.operator_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        self.neural_cache.begin_epoch();
        let previous = self.current_baseline_channels();
        let mut dispatch = |kind: &str, input: &Dictionary| registry.as_ref().dispatch(kind, input);
        let budgeted = evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut dispatch, &self.neural_cache, &dirty, previous, budget,source_required);
        tree.retire_cold();
        seeds.retire_cold();
        match budgeted {
            Ok(BudgetedEval { channels, remaining, pending_extensions }) => {
                self.pending_extension_evals = pending_extensions;
                self.apply_preview_outputs(&channels.outputs);
                self.apply_export_outputs(&channels.outputs);
                self.last_eval_json = build_channel_eval_json(&self.host_snapshot, &channels, &self.kind_infos);
                if !remaining.is_empty(){
                    self.begin_eval_publication(snapshot,channels,evaluated_generation,false).unwrap_or_else(|_|unreachable!("original evaluation publication was checked before intake"));
                    return Ok(remaining);
                }
                self.neural_cache.sweep();
                // 🧹️ The converged channel set is this host's CLAIM on the process-wide geometry
                // store, never its authority over it. Retiring straight from here pruned the kernel
                // to ONE host's handles, so any other live evaluation — a second preview window's
                // session, a viewer instance, a headless oracle — lost the shapes its own cached
                // nodes still name, and the very next evaluation that reused one of those cached
                // dictionaries faulted `missing handle: <digest>` on a node whose inputs had not
                // moved at all. That is what made a moved slider settle on an empty payload while
                // the census reported the chain done. The one authority is
                // the supplied geometry authority, over the MERGED claim of every live session, and
                // [`FlowEvalSession::capture_baseline_from`] publishes this host's claim into it on
                // exactly this convergence. A session-free host owns no claim and therefore retires
                // nothing (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
                // `📓️slider-reevaluation-correctness-2026-09-15.md`).
                // 🔒️ Only advance the snapshot/channels/generation triple together, and only on
                // success — a failed evaluation keeps diffing against the last known-good state
                // next time, which is always a safe (never under-dirty) baseline.
                self.begin_baseline_publication(snapshot,channels,evaluated_generation).unwrap_or_else(|_|unreachable!("original evaluation intake was checked before constructing owners"));
                Ok(Vec::new())
            }
            Err(err) => {
                self.neural_cache.sweep();
                if self.last_eval_json.is_empty() || is_global_eval_error_json(&self.last_eval_json) {
                    self.last_eval_json = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".to_string(), semio_framework_pack_json::Value::String(err.to_string()))]));
                }
                Ok(Vec::new())
            }
        }
    }

    /// 🔌️ Consumes the wave of contributed-extension eval requests the last budgeted step parked.
    /// Empty when nothing is owed; otherwise every member may be invoked at once.
    pub fn take_pending_extension_evals(&mut self) -> Vec<neural::PendingExtensionEval> {
        std::mem::take(&mut self.pending_extension_evals)
    }

    /// 👀️ Probes which widget ids still need evaluation without computing anything (`budget = 0`) —
    /// used to decide whether a tick chain must be (re)armed and what to mark as computing/stale.
    pub fn eval_baseline_snapshot(&self) -> Option<&TreeSnapshot> {
        self.previous_snapshot.as_deref()
    }

    pub fn widget_blocked_ports(&self, widget_id: &str) -> Vec<String> {
        let Some(operator_info) = self.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == widget_id).and_then(|widget| widget_operator_info(widget, &self.kind_infos)) else {
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
        for (widget_id, channels) in self.output_entries() {
            if let Some(displaced) = outputs.insert(widget_id.clone(), channels.clone()) {
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

    /// 🧭️ Which widgets an edit from `previous` to this document leaves DIRTY, in declaration order —
    /// the incremental contract as a number a law can read, without a contributed operator table and
    /// without running a single dispatch.
    ///
    /// ⚖️ This is exactly the set [`FlowHost::evaluate_step`] recomputes and its complement is exactly
    /// the set that free-rides on the previous channels: one `compute_dirty_set` over the same two
    /// [`TreeSnapshot`]s, so a law stating "moving `height` re-evaluates the extrude branch and
    /// nothing else" is stating what the evaluator does rather than a parallel rule that could drift
    /// from it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn dirty_widget_ids_since(&self, previous: &FlowHost) -> Vec<String> {
        let (before_tree, before_seeds) = (previous.build_tree(), previous.build_seeds());
        let baseline = TreeSnapshot::capture(&before_tree, &before_seeds);
        before_tree.retire_cold();
        before_seeds.retire_cold();
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let dirty = compute_dirty_set(Some(&baseline), &TreeSnapshot::capture(&tree, &seeds));
        tree.retire_cold();
        seeds.retire_cold();
        self.host_snapshot.widgets.iter().map(widget_id_for).filter(|id| dirty.contains(*id)).map(str::to_string).collect()
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
        let registry = self.operator_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        let previous = self.current_baseline_channels();
        let mut probe_never_dispatches = |kind: &str, _: &Dictionary| -> Result<Dictionary, EvalError> { Err(EvalError::InvalidInput(format!("pending_eval_widget_ids probed a dispatch for {kind}"))) };
        let pending = match evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut probe_never_dispatches, &self.neural_cache, &dirty, previous, EvalStepBudget::PROBE,&|_|true) {
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
        let fixture = self.build_dag_host_snapshot_v1();
        let (nodes, edges) = dag_host_snapshot_execution_rows(&fixture);
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
        semio_framework_artifact_flow_flow::cluster_tree_from_property(node.properties.get("clusterTree")?).ok()
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
        dag_host_snapshot_to_wire_literal(&self.build_dag_host_snapshot_v1())
    }

    fn build_seeds(&self) -> HashMap<String, Dictionary> {
        let mut seeds = HashMap::new();
        for widget in &self.host_snapshot.widgets {
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

    fn apply_preview_outputs(&mut self, outputs: &HistoryFoldIndex<String, Dictionary>) {
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::OutputPreview { id, preview, .. } = widget {
                if let Some(out) = outputs.get(id) {
                    std::mem::replace(preview, out.clone()).retire_cold();
                } else if let Some(syn) = self.host_snapshot.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        std::mem::replace(preview, preview_dict_from_connection(src, &syn.from_port, &syn.to_port)).retire_cold();
                    }
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.dag.fit_preview_sizes();
    }

    fn apply_export_outputs(&mut self, outputs: &HistoryFoldIndex<String, Dictionary>) {
        for widget in &self.host_snapshot.widgets {
            if let Widget::OutputExport { id, .. } = widget {
                if let Some(out) = outputs.get(id) {
                    self.export_payloads.insert(id.clone(), out.clone());
                } else if let Some(syn) = self.host_snapshot.synapses.iter().find(|s| s.to == *id) {
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
        Ok(semio_framework_pack_json::to_json_string(&payload))
    }

    /// 📤️ Returns and clears a pending export control click from the last pointer hit.
    pub fn take_pending_export_click(&mut self) -> Option<String> {
        self.dag.take_pending_export_click()
    }

    fn sync_dag_display_from_widgets(&mut self) {
        for widget in &self.host_snapshot.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.host_snapshot.nodes.iter_mut().find(|n| n.id == *id) else {
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
        let fixture = self.build_dag_host_snapshot_v1();
        if self.pending_change || self.gesture_active {
            let kept: BTreeSet<&str> = fixture.edges.iter().map(|edge| edge.id.as_str()).collect();
            let dropped: Vec<FlowMutation> = self.host_snapshot.synapses.iter().filter(|synapse| !kept.contains(synapse.id.as_str())).map(|synapse| FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse.id.clone() })).collect();
            self.pending_leaves.extend(dropped);
        }
        let theme = self.dag.canvas_theme;
        let automatic_lod = self.dag.automatic_lod();
        let forced_draw_lod = self.dag.forced_draw_lod_label().map(str::to_string);
        let ghost = self.ghost_node.clone();
        self.dag.replace_host_snapshot_without_layout(fixture);
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

    /// 🎯️ Projects selected semantic widget identities.
    pub fn selected_widget_ids(&self)->Vec<String>{self.dag.selected_node_ids()}

    /// 🎯️ Projects typed node, edge and handle selection.
    pub fn selection_domains(&self)->DagSelectionDomains{self.dag.selection_domains()}

    /// 🖱️ Hovered widget id when the pointer is over a node or port handle.
    pub fn hovered_widget_id(&self) -> Option<String> {
        self.dag.hovered_node_id()
    }

    /// 🎯️ All pick targets under a screen point as JSON for DOM disambiguation menus.
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        self.dag.pick_targets_at_screen_json(sx, sy)
    }

    /// 🎯️ Screen-space geometry for a live entity (`domain`/`id` in the pick-target grammar) —
    /// see `DagHost::entity_screen_json`. Powers introduction-demonstration semantic targeting.
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        self.dag.entity_screen_json(domain, id)
    }

    /// 🖱️ Projects admitted widget hover and live wire refusal facts.
    pub fn hover_facts(&self)->crate::infinite::board::schema::dag_input::DagHoverFacts{self.dag.hover_facts()}

    /// 🔌️ Projects selected typed widget channels.
    pub fn selected_channels(&self)->Vec<dag::DagChannelRef>{self.dag.selected_channels()}

    /// ✅️ Replaces selection from admitted semantic domains.
    pub fn set_selection_domains(&mut self,domains:&DagSelectionDomains){self.dag.set_selection_domains(domains);}

    /// ✅️ Replaces the node domain selection directly.
    pub fn set_selection(&mut self,ids:&[String]){self.dag.set_selection(ids);}

    /// 📦️ Screen-space union bounds of the current selection for DOM overlays.
    pub fn selection_union_bounds_screen_json(&self) -> String {
        self.dag.selection_union_bounds_screen_json()
    }

    /// 📐️ Aligns or distributes the selection and journals what it moved as node-graph gesture records (design §13.3,
    /// one per distinct offset), which the renderer drains ([`Self::take_graph_edits_json`]) and dispatches; an align whose
    /// records outgrow one dispatch is refused whole and every node stays where it was.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        let baseline = self.dag.node_positions();
        self.dag.align_selection(mode)?;
        let refused = self.dag.journal_moves_since(&dag::dag_drag_gesture_id(self.interaction_revision), &baseline).is_err();
        if refused {
            self.dag.restore_node_positions(&baseline);
        }
        self.sync_from_dag();
        if !refused {
            let aligned = self.dag.selected_node_ids();
            let moved = self.layout_leaf(aligned.iter().map(String::as_str));
            self.note_leaves(moved);
        }
        self.checked_change()?;
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

    /// 🔌️ Replaces admitted typed channel selection.
    pub fn set_selected_channels(&mut self,channels:&[dag::DagChannelRef]){self.dag.set_selected_channels(channels);}

    /// 🌫️ Widget ids with preview disabled.
    pub fn preview_off_widget_ids(&self) -> Vec<String> {
        self.host_snapshot
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
        let ids: Vec<String> = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::Neuron { id, preview, .. } = widget {
                *preview = !ids.contains(id);
            }
        }
        self.sync_preview_dimmed();
    }

    /// 👁️ Toggles preview on a neuron widget.
    pub fn toggle_preview(&mut self, widget_id: &str) -> Result<(), FlowCoreError> {
        let Some(widget) = self.host_snapshot.widgets.iter_mut().find(|w| widget_id_for(w) == widget_id) else {
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
        let dag_ids: BTreeSet<String> = self.dag.host_snapshot.nodes.iter().map(|node| node.id.clone()).collect();
        self.host_snapshot.widgets.retain(|widget| dag_ids.contains(widget_id_for(widget)));
        for node in &self.dag.host_snapshot.nodes {
            self.host_snapshot.layout.insert(node.id.clone(), WidgetLayout { x: node.x, y: node.y });
        }
        for widget in &mut self.host_snapshot.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.host_snapshot.nodes.iter().find(|n| n.id == *id) else {
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
        self.host_snapshot.synapses = self
            .dag
            .host_snapshot
            .edges
            .iter()
            .map(|edge| {
                let (from, from_port) = parse_port_endpoint(&edge.source, "");
                let (to, to_port) = parse_port_endpoint(&edge.target, "");
                SynapseSpec { id: edge.id.clone(), from, to, from_port, to_port }
            })
            .collect();
        self.host_snapshot.camera = CameraJson { x: self.dag.host_snapshot.camera.x, y: self.dag.host_snapshot.camera.y, zoom: self.dag.host_snapshot.camera.zoom };
    }

    fn build_dag_host_snapshot_v1(&self) -> DagHostSnapshot {
        let mut seen = BTreeSet::new();
        let nodes: Vec<DagNodeSpec> =
            self.host_snapshot.widgets.iter().enumerate().filter(|(_, widget)| seen.insert(widget_id_for(widget).to_string())).map(|(i, w)| widget_to_dag_node(w, i, &self.host_snapshot.layout, &self.host_snapshot.synapses, &self.kind_infos, widget_node_size(w, &self.host_snapshot.synapses, &self.kind_infos))).collect();
        let existing: Vec<(String, String)> = self.host_snapshot.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        let edges: Vec<DagHostSnapshotEdge> = self
            .host_snapshot
            .synapses
            .iter()
            .filter_map(|syn| {
                if would_create_cycle(&existing.iter().filter(|(a, b)| !(a == &syn.from && b == &syn.to)).cloned().collect::<Vec<_>>(), &syn.from, &syn.to) {
                    return None;
                }
                let from_port = resolve_synapse_port(&syn.from, &syn.from_port, PortSide::Output, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos).unwrap_or_else(|| syn.from_port.clone());
                let to_port = resolve_synapse_port(&syn.to, &syn.to_port, PortSide::Input, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos).unwrap_or_else(|| syn.to_port.clone());
                Some(DagHostSnapshotEdge { id: syn.id.clone(), source: format!("{}@{}", syn.from, from_port), target: format!("{}@{}", syn.to, to_port), route_style: EdgeRouteStyle::default(), properties: PropertyBag::new() })
            })
            .collect();
        DagHostSnapshot { schema: "dag.hostDocument".into(), camera: semio_framework_artifact_infinite_dag::DagCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom }, nodes, edges }
    }

    fn screen_to_world_point(&self, sx: f64, sy: f64) -> canvas::Point {
        use canvas::camera::{screen_to_world, Camera, Viewport};
        use canvas::Point;
        let cam = Camera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn next_widget_id(&mut self, descriptor: &WidgetDescriptor) -> String {
        let (id, serial) = generated_widget_id(descriptor, self.host_snapshot.widgets.iter().map(widget_id_for));
        self.next_widget_serial = serial;
        id
    }

    pub fn set_slider_value(&mut self, widget_id: &str, value: f64) {
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::InputSlider { id, .. } = widget {
                if id == widget_id {
                    set_widget_slider_value(widget, value);
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.refresh_computing_chrome_from_pending();
        self.finish_change();
    }

    pub fn slider_overlay_state_json(&self) -> Result<String, FlowCoreError> {
        Ok(self.dag.slider_overlay_state_json()?)
    }

    pub fn set_note_text(&mut self, widget_id: &str, text: &str) {
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::InputNote { id, text: note } = widget {
                if id == widget_id {
                    *note = text.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.dag.fit_note_sizes();
        self.refresh_computing_chrome_from_pending();
        self.finish_change();
    }

    /// ✏️ Begins inline note editing for a widget at a world-space click.
    pub fn begin_note_edit(&mut self, widget_id: &str, world_x: f64, world_y: f64) {
        self.begin_gesture();
        self.edited_note = Some(widget_id.to_string());
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
        let refs = self.operator_registry().schema_refs();
        Ok(semio_framework_pack_json::to_json_string(&refs))
    }

    pub fn set_variable_name(&mut self, widget_id: &str, name: &str) {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::Variable { id, name: variable_name, .. } = widget {
                if id == widget_id {
                    *variable_name = trimmed.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.rebuild_dag();
        self.finish_change();
    }

    pub fn set_variable_schema(&mut self, widget_id: &str, schema: &str) {
        let trimmed = schema.trim();
        if trimmed.is_empty() {
            return;
        }
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::Variable { id, schema: variable_schema, .. } = widget {
                if id == widget_id {
                    *variable_schema = trimmed.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.rebuild_dag();
        self.finish_change();
    }

    pub fn set_image_src(&mut self, widget_id: &str, src: &str) {
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::InputImage { id, src: image } = widget {
                if id == widget_id {
                    *image = src.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.dag.fit_preview_sizes();
        self.refresh_computing_chrome_from_pending();
        self.finish_change();
    }

    pub fn preview_text(&self) -> String {
        self.host_snapshot
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
        if !selected.iter().all(|id| self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id)) {
            return Err(FlowCoreError::CollapseUnknownWidgets);
        }
        if selected.iter().any(|id| self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id && matches!(widget, Widget::Cluster { .. }))) {
            return Err(FlowCoreError::CollapseContainsClusters);
        }
        self.begin_change();
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| selected.contains(&synapse.from) || selected.contains(&synapse.to)).map(|synapse| synapse.id.clone()).collect();
        let unplaced: Vec<FlowLayoutEntry> = selected.iter().filter(|id| self.host_snapshot.layout.contains_key(id.as_str())).map(|id| FlowLayoutEntry { id: id.clone(), layout: None }).collect();
        let mut crossing_external = Vec::new();
        for synapse in &self.host_snapshot.synapses {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if (from_selected || to_selected) && !(from_selected && to_selected) {
                crossing_external.push(synapse.clone());
            }
        }
        let boundary_variables = boundary_variable_widget_ids(&selected, &crossing_external, &self.host_snapshot.widgets);
        let mut inner_neurons = Vec::new();
        let mut inner_layout = BTreeMap::new();
        for widget in &self.host_snapshot.widgets {
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
            if let Some(layout) = self.host_snapshot.layout.get(&id) {
                inner_layout.insert(id, layout.clone());
            }
        }
        let mut inner_synapses = Vec::new();
        let mut retained_external = Vec::new();
        for synapse in &self.host_snapshot.synapses {
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
        let channels=self.current_channels.clone().or_else(||self.previous_channels.clone());
        let empty_outputs=HistoryFoldIndex::new();
        let outputs=channels.as_deref().map_or(&empty_outputs,|channels|&channels.outputs);
        let kind_infos = self.kind_infos.clone();
        let widgets = self.host_snapshot.widgets.clone();
        let synapses_snapshot = self.host_snapshot.synapses.clone();
        for synapse in crossing_external {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if to_selected && !from_selected {
                let inner_target = if boundary_variables.contains(&synapse.to) {
                    self.host_snapshot.synapses.iter().find(|entry| entry.from == synapse.to && selected.contains(&entry.to)).map_or_else(|| (synapse.to.clone(), synapse.to_port.clone()), |entry| (entry.to.clone(), entry.to_port.clone()))
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
                    self.host_snapshot.synapses.iter().find(|entry| entry.to == synapse.from && selected.contains(&entry.from)).map_or_else(|| (synapse.from.clone(), synapse.from_port.clone()), |entry| (entry.from.clone(), entry.from_port.clone()))
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
        let (sum_x, sum_y, layout_count) = selected.iter().filter_map(|id| self.host_snapshot.layout.get(id)).fold((0.0, 0.0, 0usize), |(sx, sy, count), layout| (sx + layout.x, sy + layout.y, count + 1));
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
        self.host_snapshot.widgets.retain(|widget| !selected.contains(widget_id_for(widget)));
        let clustered = FlowMutation::AddWidget(AddWidget { index: u32::try_from(self.host_snapshot.widgets.len()).unwrap_or(u32::MAX), widget: cluster.clone() });
        self.host_snapshot.widgets.push(cluster);
        for id in &selected {
            self.host_snapshot.layout.remove(id);
        }
        self.host_snapshot.layout.insert(cluster_id.clone(), WidgetLayout { x: cluster_x, y: cluster_y });
        let retained_count = retained_external.len();
        self.host_snapshot.synapses = retained_external;
        for synapse in cluster_external {
            if synapse.to.is_empty() {
                self.host_snapshot.synapses.push(SynapseSpec { id: synapse.id, from: synapse.from, to: cluster_id.clone(), from_port: synapse.from_port, to_port: synapse.to_port });
            } else {
                self.host_snapshot.synapses.push(SynapseSpec { id: synapse.id, from: cluster_id.clone(), to: synapse.to, from_port: synapse.from_port, to_port: synapse.to_port });
            }
        }
        let unplaced = (!unplaced.is_empty()).then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: unplaced }));
        let rewired: Vec<FlowMutation> = self.host_snapshot.synapses[retained_count..].iter().enumerate().map(|(offset, synapse)| FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(retained_count + offset).unwrap_or(u32::MAX), synapse: synapse.clone() })).collect();
        let placed = self.layout_leaf([cluster_id.as_str()]);
        self.note_leaves(
            severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain(selected.iter().map(|id| FlowMutation::RemoveWidget(RemoveWidget { id: id.clone() }))).chain([clustered]).chain(placed).chain(rewired),
        );
        self.rebuild_dag();
        self.checked_change()?;
        Ok(cluster_id)
    }

    /// 💥️ Explodes a cluster back into its inner widgets.
    pub fn explode_cluster(&mut self, cluster_id: &str) -> Result<(), FlowCoreError> {
        let cluster_index = self.host_snapshot.widgets.iter().position(|widget| matches!(widget, Widget::Cluster { id, .. } if id == cluster_id)).ok_or_else(|| FlowCoreError::UnknownCluster(cluster_id.to_string()))?;
        // 🧹️ The working copy is BORROWED from one retired clone, not destructured out of it: a
        // cluster's `Tree` params and its `FlowUi` node map both fail closed on a bare drop, so
        // owning `tree`/`flow` as loose locals aborted the process at the end of this function
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        let exploded = self.host_snapshot.widgets[cluster_index].clone();
        let Widget::Cluster { tree, flow, .. } = &exploded else {
            exploded.retire_cold();
            return Err(FlowCoreError::WidgetNotCluster(cluster_id.to_string()));
        };
        let cluster_layout = self.host_snapshot.layout.get(cluster_id).cloned().unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
        self.begin_change();
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| synapse.from == cluster_id || synapse.to == cluster_id).map(|synapse| synapse.id.clone()).collect();
        let had_layout = self.host_snapshot.layout.contains_key(cluster_id);
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
                self.host_snapshot.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
                restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
                continue;
            }
            let mut widget = neuron_to_exploded_widget(neuron);
            match &mut widget {
                Widget::Neuron { id, .. } | Widget::InputSlider { id, .. } | Widget::InputNote { id, .. } | Widget::InputImage { id, .. } | Widget::Variable { id, .. } => *id = namespaced_id.clone(),
                _ => {}
            }
            let layout = flow.nodes.get(&neuron.id).map_or(WidgetLayout { x: 0.0, y: 0.0 }, |node| node.layout.clone());
            self.host_snapshot.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
            restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
        }
        let id_map: HashMap<String, String> = restored_widgets.iter().map(|(namespaced, original, _)| (original.clone(), namespaced.clone())).collect();
        // 🧹️ The cluster widget carries a `FlowUi` whose `OrderedMap<FlowNodeGui>` and a `Tree`
        // whose `Dictionary` params both fail closed on a bare drop, so the exploded shell is
        // RETIRED rather than dropped (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        self.host_snapshot.widgets.remove(cluster_index).retire_cold();
        self.host_snapshot.layout.remove(cluster_id);
        let mut restored_ids = Vec::with_capacity(restored_widgets.len());
        let mut restored_leaves = Vec::with_capacity(restored_widgets.len());
        for (namespaced_id, _, widget) in restored_widgets {
            restored_leaves.push(FlowMutation::AddWidget(AddWidget { index: u32::try_from(self.host_snapshot.widgets.len()).unwrap_or(u32::MAX), widget: widget.clone() }));
            restored_ids.push(namespaced_id);
            self.host_snapshot.widgets.push(widget);
        }
        let mut next_synapses = Vec::new();
        let mut rewired = Vec::new();
        for synapse in &self.host_snapshot.synapses {
            if synapse.to == cluster_id {
                if let Some((variable_id, variable_port)) = boundary_channels.get(&synapse.to_port) {
                    rewired.push(next_synapses.len());
                    next_synapses.push(SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: variable_id.clone(), from_port: synapse.from_port.clone(), to_port: variable_port.clone() });
                    continue;
                }
            } else if synapse.from == cluster_id {
                if let Some((variable_id, variable_port)) = boundary_channels.get(&synapse.from_port) {
                    rewired.push(next_synapses.len());
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
            let id = self.next_synapse_id();
            rewired.push(next_synapses.len());
            next_synapses.push(SynapseSpec { id, from: from.clone(), to: to.clone(), from_port, to_port });
        }
        let reconnected: Vec<FlowMutation> = rewired.into_iter().map(|index| FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(index).unwrap_or(u32::MAX), synapse: next_synapses[index].clone() })).collect();
        self.host_snapshot.synapses = next_synapses;
        exploded.retire_cold();
        let unplaced = had_layout.then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: cluster_id.to_string(), layout: None }] }));
        let placed = self.layout_leaf(restored_ids.iter().map(String::as_str));
        self.note_leaves(
            severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain([FlowMutation::RemoveWidget(RemoveWidget { id: cluster_id.to_string() })]).chain(restored_leaves).chain(placed).chain(reconnected),
        );
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    // #region History
    /// 🧾️ Lazily seeds the undo/redo store from `baseline`.
    ///
    /// ⚠️ `baseline` is CONSUMED only on the first call — the store is seeded once, and every later
    /// `flush_pending_change` hands in a fresh `FlowHostSnapshot` clone this function does not need. A
    /// `FlowHostSnapshot` owns the fail-closed `OrderedMap<WidgetLayout>` root, so that surplus clone is
    /// RETIRED through the artifact's own bounded frontier instead of dropped; the bare drop aborted
    /// the pool worker on the second discrete edit of any session
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️unit-suite-3d-2026-09-09.md` §5).
    fn history_store_from_baseline(&mut self, baseline: FlowHostSnapshot) -> Option<&mut FlowStore> {
        if self.history_store.is_some() {
            let mut retirement = crate::retained::FlowRetirement::default();
            retirement.push(crate::retained::FlowOwner::HostSnapshot(baseline));
            retirement.retire_cold();
            return self.history_store.as_mut();
        }
        let mut store = ::semio_framework_async::poll::resolve_ready(FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow-host", baseline, None), crate::os_spr::ActorId(crate::os_spr::LOCAL_ACTOR_ID.into()))).ok()?;
        store.install_document_store_owners_exact(FlowHostSnapshot::member_store_owners());
        self.history_store = Some(store);
        self.history_store.as_mut()
    }

    /// 🧾️ Appends the concrete leaves a gesture site just emitted to the edit in progress.
    fn note_leaves(&mut self, leaves: impl IntoIterator<Item = FlowMutation>) {
        self.pending_leaves.extend(leaves);
    }

    /// 🗑️ Retires the leaves of an edit that is not recorded; a widget leaf owns fail-closed roots that refuse a bare drop.
    fn discard_pending_leaves(&mut self) {
        std::mem::take(&mut self.pending_leaves).into_iter().for_each(retire_flow_mutation);
    }

    /// 🩹 The `change-widget` leaf carrying `widget_id`'s current content, when the widget exists.
    fn changed_widget_leaf(&self, widget_id: &str) -> Option<FlowMutation> {
        self.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == widget_id).map(|widget| FlowMutation::ChangeWidget(ChangeWidget { id: widget_id.to_string(), widget: widget.clone() }))
    }

    /// 📐️ The `change-layout` leaf assigning each of `ids` its current layout; an id without a layout is skipped.
    fn layout_leaf<'a>(&self, ids: impl IntoIterator<Item = &'a str>) -> Option<FlowMutation> {
        let entries: Vec<FlowLayoutEntry> = ids.into_iter().filter_map(|id| self.host_snapshot.layout.get(id).map(|layout| FlowLayoutEntry { id: id.to_string(), layout: Some(layout.clone()) })).collect();
        (!entries.is_empty()).then(|| FlowMutation::ChangeLayout(ChangeLayout { entries }))
    }

    /// 🔗️ The leaves of the wire, slider and move edits the open pointer gesture journalled in the dag (the gesture's own
    /// narration, read without draining); a port insert is recorded where `add_*_port` runs, and a connect that displaced
    /// the wire already feeding its target removes it first.
    fn gesture_journal_leaves(&self, baseline: &FlowHostSnapshot) -> Vec<FlowMutation> {
        let edits = self.dag.graph_edits();
        self.journal_leaves(edits.get(self.journal_mark..).unwrap_or(edits), baseline)
    }

    /// 🔗️ The leaves one gesture's journal `rows` stand for, given the `baseline` before the gesture and the live content after it.
    fn journal_leaves(&self, rows: &[dag::DagGraphEdit], baseline: &FlowHostSnapshot) -> Vec<FlowMutation> {
        let mut leaves = Vec::new();
        for edit in rows {
            match edit {
                dag::DagGraphEdit::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                    let displaced = baseline.synapses.iter().filter(|synapse| synapse.to == *target_node_id && synapse.to_port == *target_port_id && self.host_snapshot.synapses.iter().all(|now| now.id != synapse.id));
                    leaves.extend(displaced.map(|synapse| FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse.id.clone() })));
                    let created = self.host_snapshot.synapses.iter().enumerate().find(|(_, synapse)| synapse.from == *source_node_id && synapse.from_port == *source_port_id && synapse.to == *target_node_id && synapse.to_port == *target_port_id);
                    leaves.extend(created.map(|(index, synapse)| FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(index).unwrap_or(u32::MAX), synapse: synapse.clone() })));
                }
                dag::DagGraphEdit::Disconnect { synapse_id } => {
                    if baseline.synapses.iter().any(|synapse| synapse.id == *synapse_id) {
                        leaves.push(FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse_id.clone() }));
                    }
                }
                dag::DagGraphEdit::Move { node_ids, .. } => leaves.extend(self.layout_leaf(node_ids.iter().map(String::as_str))),
                dag::DagGraphEdit::SetSlider { node_id, .. } => leaves.extend(self.changed_widget_leaf(node_id)),
                dag::DagGraphEdit::InsertPort { .. } => {}
            }
        }
        leaves
    }

    /// 🧾️ Records the leaves emitted since the edit was armed as ONE store transaction (seeding the store from `baseline` on
    /// first use), retiring them cold when there is nothing to record; an edit that emitted no leaf leaves no history row.
    /// A refused transaction is never ignored: the history restarts at the live content (no partial row survives) and the
    /// refusal is kept as [`FlowCoreError::HistoryRefused`] for [`FlowHost::take_history_fault`].
    fn record_history_edit(&mut self, baseline: FlowHostSnapshot) {
        let operations = std::mem::take(&mut self.pending_leaves);
        #[cfg(test)]
        {
            if !operations.is_empty() {
                self.recorded.push(operations.clone());
            }
        }
        let recorded = !operations.is_empty();
        let refusal = match self.history_store_from_baseline(baseline) {
            Some(store) if recorded => ::semio_framework_async::poll::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: operations, transaction: None })).err().map(|error| format!("{error:?}")),
            _ => {
                operations.into_iter().for_each(retire_flow_mutation);
                None
            }
        };
        if let Some(reason) = refusal {
            self.reset_history_store();
            self.history_fault = Some(FlowCoreError::HistoryRefused(reason));
        }
    }

    /// 🚦️ Closes a discrete edit: records its leaves now unless a gesture is still coalescing them.
    fn finish_change(&mut self) {
        if !self.gesture_active {
            self.flush_pending_change();
        }
    }

    /// 🚦️ [`Self::finish_change`] that hands a refused history row back to the gesture that caused it.
    fn checked_change(&mut self) -> Result<(), FlowCoreError> {
        self.finish_change();
        self.history_outcome()
    }

    /// 🧯️ Hands over the refusal of the last history row, if any; the gestures that cannot return it (slider and text setters,
    /// pointer release, note commit) leave it here for their caller to collect.
    pub fn take_history_fault(&mut self) -> Option<FlowCoreError> {
        self.history_fault.take()
    }

    /// 🧯️ [`Self::take_history_fault`] as a `Result`.
    pub fn history_outcome(&mut self) -> Result<(), FlowCoreError> {
        self.take_history_fault().map_or(Ok(()), Err)
    }

    /// 🧾️ Flushes an armed-but-not-yet-recorded discrete mutation into `history_store` as the concrete flow leaves its gesture site emitted — the standard `crate::os_store::ArtifactStore`/`Mutation`/
    /// `MutationDiff` mechanism (see `🔖️Mutations`) driving undo/redo here instead of the old
    /// hand-rolled `Vec<FlowHostSnapshot>` snapshot stack. An armed edit that emitted no leaf leaves no history row.
    fn flush_pending_change(&mut self) {
        if self.pending_change {
            self.pending_change = false;
            let baseline = self.pending_history_baseline.take().unwrap_or_else(|| self.host_snapshot.clone());
            self.record_history_edit(baseline);
        }
    }

    /// ↩️ Arms a checkpoint for the mutation about to happen, unless a gesture (`begin_gesture`) is
    /// currently coalescing several mutations into one.
    pub fn begin_change(&mut self) {
        if !self.gesture_active {
            self.flush_pending_change();
            self.arm_history_baseline();
            self.pending_change = true;
        }
    }

    /// 🧾️ Arms a fresh undo baseline, RETIRING the one it replaces.
    ///
    /// 🩸️ A baseline is a `FlowHostSnapshot`, which owns the fail-closed `OrderedMap<WidgetLayout>` root,
    /// and overwriting the field simply DROPPED the old one: `panicked at 🗂️ordered/🦀️.rs:81:
    /// ordered-map root must be explicitly retired before drop`, aborting the whole pool worker.
    /// Reached on 6118 by the FOURTH middle-button pan of one session — a gesture whose release never
    /// committed leaves its baseline armed, and the next gesture's `begin_gesture` overwrote it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-node-graph-gestures-2026-09-13.md` §4).
    /// The same hazard sat on `begin_change` and on the history reset inside `apply_fixture`.
    fn arm_history_baseline(&mut self) {
        if let Some(stale) = self.pending_history_baseline.take() {
            stale.retire_cold();
        }
        self.pending_history_baseline = Some(self.host_snapshot.clone());
    }

    /// 🖐️ Starts a coalescing gesture (drag, inline note edit): flushes anything already armed first,
    /// then suppresses further `begin_change` checkpoints until `commit_gesture_history`.
    fn begin_gesture(&mut self) {
        self.flush_pending_change();
        self.arm_history_baseline();
        self.journal_mark = self.dag.graph_edits().len();
        self.gesture_active = true;
    }

    /// 🧾️ Closes a coalescing gesture, recording ONE invertible edit when the gesture actually
    /// changed content.
    ///
    /// 🩸️ A gesture that changed nothing — every plain CLICK on the graph — still holds a
    /// `FlowHostSnapshot` baseline, and a `FlowHostSnapshot` owns the fail-closed `OrderedMap<WidgetLayout>`
    /// root: letting it fall out of scope aborted the whole pool worker with `ordered-map root must
    /// be explicitly retired before drop` on the FIRST click. It was unreachable from wgpu only
    /// because no pointer ever reached the graph; the retention fix reaches it on press one (ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-node-graph-surface-retention-2026-09-13.md`).
    /// A no-op baseline is therefore RETIRED, exactly as `history_store_from_baseline` retires the
    /// surplus clone it does not need.
    fn commit_gesture_history(&mut self) {
        if self.gesture_active {
            self.gesture_active = false;
            let mut baseline = self.pending_history_baseline.take().unwrap_or_else(|| self.host_snapshot.clone());
            if let Err(refusal) = self.journal_gesture_moves(&baseline) {
                self.discard_pending_leaves();
                baseline.camera = self.host_snapshot.camera.clone();
                std::mem::replace(&mut self.host_snapshot, baseline).retire_cold();
                self.rebuild_dag();
                self.dag.carry_journal_refusal(refusal);
                return;
            }
            if let Some(id) = self.edited_note.take() {
                let edited = self.changed_widget_leaf(&id).filter(|_| baseline.widgets.iter().find(|widget| widget_id_for(widget) == id).is_some_and(|before| self.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == id) != Some(before)));
                self.note_leaves(edited);
            }
            let journalled = self.gesture_journal_leaves(&baseline);
            self.note_leaves(journalled);
            if self.pending_leaves.is_empty() {
                baseline.retire_cold();
                return;
            }
            self.record_history_edit(baseline);
        }
    }

    /// ✋️ Narrates a released gesture that moved nodes as the node-graph gesture record of design §13.3 — the moved widgets
    /// (in widget order) and their ONE offset from the gesture's baseline, one record per distinct offset (a grid snap can
    /// land off-grid starts on different offsets) — so the guest commits the drag as relative leaves, exactly the record
    /// the wgpu bounded path writes (`DagHost::plan_graph_edits`). Wires, inline sliders and port inserts are journalled
    /// where they happen; all move rows are admitted together or the caller restores the baseline.
    fn journal_gesture_moves(&mut self, baseline: &FlowHostSnapshot) -> Result<(), dag::DagJournalRefusal> {
        let displacements = self.host_snapshot.widgets.iter().filter_map(|widget| {
            let id = widget_id_for(widget);
            let (landed, start) = (self.host_snapshot.layout.get(id)?, baseline.layout.get(id)?);
            Some((id.to_string(), landed.x - start.x, landed.y - start.y))
        });
        let gesture_id = dag::dag_drag_gesture_id(self.interaction_revision);
        self.dag.journal_moves(&gesture_id, displacements)
    }

    /// ↩️ Restores the previous fixture content snapshot, keeping the current camera.
    pub fn undo(&mut self) -> bool {
        self.flush_pending_change();
        let camera = self.host_snapshot.camera.clone();
        let Some(store) = self.history_store.as_mut() else {
            return false;
        };
        if ::semio_framework_async::poll::resolve_ready(store.dispatch(ArtifactCommand::Undo)).is_err() {
            return false;
        }
        let Ok(mut restored) = store.snapshot() else {
            return false;
        };
        restored.camera = camera;
        std::mem::replace(&mut self.host_snapshot, restored).retire_cold();
        self.rebuild_dag();
        true
    }

    /// ↪️ Re-applies a fixture content snapshot undone earlier, keeping the current camera.
    pub fn redo(&mut self) -> bool {
        let camera = self.host_snapshot.camera.clone();
        let Some(store) = self.history_store.as_mut() else {
            return false;
        };
        if ::semio_framework_async::poll::resolve_ready(store.dispatch(ArtifactCommand::Redo)).is_err() {
            return false;
        }
        let Ok(mut restored) = store.snapshot() else {
            return false;
        };
        restored.camera = camera;
        std::mem::replace(&mut self.host_snapshot, restored).retire_cold();
        self.rebuild_dag();
        true
    }

    /// ↩️ Whether a content undo step is available.
    pub fn can_undo(&self) -> bool {
        !self.pending_leaves.is_empty() || self.history_store.as_ref().is_some_and(|store| !store.applied_edit_ids().is_empty())
    }

    /// ↪️ Whether a content redo step is available.
    pub fn can_redo(&self) -> bool {
        self.history_store.as_ref().is_some_and(|store| !store.redo_edit_ids().is_empty())
    }
    // #endregion History
}

/// 🏠️ Retains the whole original host before any independently admitted close work.
#[doc(hidden)]
pub struct FlowHostRetirementState {
    source:Option<FlowHost>,
    geometry:Option<crate::geometry::GeometryPortRetirement>,
    payload:Option<ControlledRetirement<FlowHostPayload>>,
}

/// 🔒️ Every original host field remains owned until its declared physical close authority completes.
#[must_use="the original host must reach physical terminal emptiness"]
pub struct FlowHostRetirement {state:std::mem::ManuallyDrop<FlowHostRetirementState>}

/// 🪜️ Identifies the next original host ownership boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FlowHostClosePhase {Geometry,Dag,Domain,Neural,Widgets,Synapses,Layout,Schema,CurrentChannels,Exports,EvalJson,Catalogue,KindInfos,PreviousSnapshot,PreviousChannels,HistoryBaseline,PendingEvals,Bridges,Cache,HistoryStore,Faulted,Complete}
impl FlowHostClosePhase {pub fn is_backing(self)->bool{matches!(self,Self::Domain|Self::Neural)}}

impl FlowHostRetirement {
    /// 🪹️ Transfers the exact original source without decomposition, allocation, release or cancellation work.
    pub fn new(host:FlowHost)->Self{Self{state:std::mem::ManuallyDrop::new(FlowHostRetirementState{source:Some(host),geometry:None,payload:None})}}
    pub fn terminal_is_empty(&self)->bool{self.state.source.is_none()&&self.state.geometry.is_none()&&self.state.payload.is_none()}
    pub fn close_phase(&self)->FlowHostClosePhase{if self.state.geometry.is_some()||self.state.source.as_ref().is_some_and(|host|host.geometry_port.is_some()){FlowHostClosePhase::Geometry}else if self.state.source.as_ref().is_some_and(|host|host.history_store.is_some()){FlowHostClosePhase::HistoryStore}else if self.state.source.is_some(){FlowHostClosePhase::Dag}else if self.state.payload.is_some(){FlowHostClosePhase::Domain}else{FlowHostClosePhase::Complete}}
    fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(RetirementDemand::default())}
        if let Some(payload)=self.state.payload.as_ref(){return Ok(if payload.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:payload.next_copy_byte_demand()?,capacity_bytes:payload.next_capacity_byte_demand(copy)?,release_bytes:payload.next_release_byte_demand()?,depth:payload.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"host payload close depth overflow"))?}})}
        if let Some(port)=self.state.geometry.as_ref(){return Ok(RetirementDemand{copy_bytes:port.next_copy_byte_demand()?,capacity_bytes:port.next_capacity_byte_demand(copy)?,release_bytes:port.next_release_byte_demand()?,depth:port.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"host geometry close depth overflow"))?})}
        let source=self.state.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"host close lost original custody"))?;
        if source.geometry_port.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(store)=source.history_store.as_ref(){if !store.close_owned_store_terminal_is_empty(){let mut demand=store.close_owned_demands(copy)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"host store close depth overflow"))?;return Ok(demand)}}
        Ok(RetirementDemand{depth:1,..Default::default()})
    }
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.close_demands(copy)?.capacity_bytes)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    /// 🎟️ Uses the original supplied currencies and preserves the full actual child receipt.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"host close exceeds admitted depth"))}
        if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(port)=self.state.geometry.as_mut(){
            if port.terminal_is_empty(){self.state.geometry=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
            let step=port.close_step(child)?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,port.terminal_is_empty(),"original host geometry retirement")?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        if let Some(payload)=self.state.payload.as_mut(){
            if payload.terminal_is_empty(){self.state.payload=None;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..empty}))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=payload.step(child)?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,payload.terminal_is_empty(),"original host payload retirement")?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        let source=self.state.source.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"host admitted close lost original source"))?;
        if let Some(port)=source.geometry_port.take(){self.state.geometry=Some(crate::geometry::GeometryPortRetirement::new(port));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
        if let Some(store)=source.history_store.as_mut(){
            if store.close_owned_store_terminal_is_empty(){source.history_store=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=store.close_owned_store_step(child)?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,store.close_owned_store_terminal_is_empty(),"original host history store retirement")?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        let payload=FlowHostPayload::from_host(self.state.source.take().unwrap());self.state.payload=Some(ControlledRetirement::new(payload).unwrap_or_else(|_|unreachable!("all original host payload fields declare typed retirement")));
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))
    }
}
impl Drop for FlowHostRetirement {
    fn drop(&mut self){if !self.terminal_is_empty(){assert!(std::thread::panicking(),"Flow host retirement abandoned its original source");return}unsafe{std::mem::ManuallyDrop::drop(&mut self.state)}}
}

impl FlowHost {
    /// 🧊️ Explicit cold-only disposal of a detached host — the twin of [`FlowHostSnapshot::retire_cold`].
    /// A `FlowHost` owns a `FlowHostSnapshot`, whose `layout: OrderedMap<WidgetLayout>` panics on a bare
    /// drop (`ordered-map root must be explicitly retired before drop`), so a host is CLOSED, never
    /// dropped. Retained callers drive [`FlowHostRetirement::close_step`] under their own grant
    /// instead; this drains the same ladder in one uninterrupted cold pass.
    pub fn retire_cold(self) {
        let mut retirement=FlowHostRetirement::new(self);
        while !retirement.terminal_is_empty(){let copy=retirement.next_close_copy_byte_demand().expect("cold host copy authority");let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:retirement.next_close_capacity_byte_demand(copy).expect("cold host capacity authority"),maximum_release_bytes:retirement.next_close_release_byte_demand().expect("cold host release authority"),maximum_depth:retirement.next_close_depth_demand().expect("cold host depth authority")};let step=retirement.close_step(grant).expect("cold original host close");assert!(step.progress().copied_items!=0,"cold original host close stalled");}
    }

    /// 🏠️ Runs `body` against a host built from `fixture`, then retires that host — the ONE shape a
    /// caller that only needs a host for the length of an expression should use.
    pub fn with_host_snapshot<R>(host_snapshot: &FlowHostSnapshot, body: impl FnOnce(&mut FlowHost) -> R) -> R {
        let mut host = Self::from_host_snapshot(host_snapshot.clone());
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

/// ⏱️ Wall a guest turn must still have UNSPENT of [`FLOW_EVAL_TICK_ELAPSED_CEILING_US`] before a
/// fold may run the next wave of its chain inline rather than park it for a host round trip.
///
/// 🔁️ A fold that continues inline saves a whole `flowEvalResolve` → `flowEvalTick` pair — measured
/// at 1 391 ms on React (`📓️react-hop-latency-2026-09-14.md` §2.1) — but it spends that saving
/// inside the answer's OWN turn, on top of whatever the fold already cost. Without a reserve the
/// two additions are exactly how an 8 ms hold becomes a 14 ms one. A third of the allowance is the
/// floor a walk needs to be worth starting at all: `EvalStepBudget::exhausted` always dispatches at
/// least one node before a deadline can stop it, so a continuation admitted with a sliver left
/// would overrun by one whole operator rather than yield (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US: u64 = FLOW_EVAL_TICK_ELAPSED_CEILING_US / 3;

/// ⏱️ The budget ONE `flowEvalTick` dag walk runs under: [`FLOW_EVAL_TICK_STEP_BUDGET`] nodes,
/// preempted after [`FLOW_EVAL_TICK_ELAPSED_CEILING_US`] on the process clock. Falls back to the
/// node count alone when no clock is installed (bare wasm), which is the pre-existing behaviour.
///
/// 🔁️ `turn_started_us` is when the GUEST TURN this walk belongs to began. A walk that opens its own
/// turn passes `None` and gets the whole allowance; one running inline inside a fold's turn passes
/// that turn's start and shares the SAME deadline, which is the entire reason an inline continuation
/// cannot stretch the interactive hold however many waves it chains.
pub fn flow_eval_tick_budget(turn_started_us: Option<u64>) -> EvalStepBudget {
    let started_us = turn_started_us.or_else(semio_framework_job::default_now_us);
    match started_us {
        Some(started_us) => EvalStepBudget::until(FLOW_EVAL_TICK_STEP_BUDGET, semio_framework_job::default_now_us, started_us.saturating_add(FLOW_EVAL_TICK_ELAPSED_CEILING_US)),
        None => EvalStepBudget::dispatches(FLOW_EVAL_TICK_STEP_BUDGET),
    }
}

/// ⏱️ Whether a turn that began at `turn_started_us` and is now at `now_us` still has
/// [`FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US`] of its evaluation allowance left — the WALL half of
/// [`FlowEvalSession::inline_continuation_admitted`], pure in both instants so a law can state the
/// park boundary in microseconds instead of racing a clock.
///
/// 🕰️ An uninstrumented process (no clock at all, bare wasm) admits the continuation: the node
/// budget still bounds the walk, and that is precisely the fallback [`flow_eval_tick_budget`] takes.
pub fn flow_eval_inline_continuation_fits(turn_started_us: Option<u64>, now_us: Option<u64>) -> bool {
    match (turn_started_us, now_us) {
        (Some(turn_started_us), Some(now_us)) => now_us.saturating_sub(turn_started_us).saturating_add(FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US) <= FLOW_EVAL_TICK_ELAPSED_CEILING_US,
        _ => true,
    }
}

/// 🚦 Per-widget evaluation state for flow graph chrome (not persisted in config).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(tag = "status", rename_all = "camelCase")]
#[value(tag = "status", rename_all = "camelCase")]
pub enum NodeEvalStatus {
    Ok,
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

semio_framework_value::artifact_retire_leaf!(FlowEvalWindowTickLatch);

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
    retiring_geometry_port: Option<crate::geometry::GeometryPortRetirement>,
    geometry_port: Option<Box<dyn crate::geometry::GeometryPort>>,
    operator_registry: Option<neural::SharedRegistry>,
    neural_cache: Option<Arc<NeuralCache>>,
    previous_snapshot: Option<Arc<TreeSnapshot>>,
    previous_channels: Option<Arc<EvalChannels>>,
    eval_json: Option<Arc<String>>,
    /// 🖼️ The evaluation a preview PAINTS while this one is still running: the live walk's own
    /// answer, with every node the live walk has not answered YET filled in from the last CONVERGED
    /// evaluation of the same node. Identical to [`FlowEvalSessionState::eval_json`] the moment the
    /// walk converges, so a settled evaluation is never dressed up by a stale one.
    ///
    /// 🐛️ A budgeted walk emits NO entry for a node whose extension request is parked, so a preview
    /// assembled from the live answer alone loses every branch that is recomputing. Measured on
    /// :6025 for ONE slider step of the hex column: the payload went from three meshes to one at
    /// +1 312 ms and the extruded solid was off screen for 1.4 s of a 2.7 s re-evaluation — the
    /// geometry vanished and came back rather than following the knob
    /// (`📓️slider-latency-incremental-eval-2026-09-15.md` §1).
    painted_eval_json: String,
    /// 🖼️ The last evaluation that CONVERGED — the only honest source for a node the live walk has
    /// not reached. Advanced with the incremental baseline, under the same `remaining.is_empty()`
    /// condition and never apart from it.
    converged_eval_json: String,
    status_json: String,
    tick_scheduled: bool,
    /// 🔒️ One arming latch per preview window this session publishes into, keyed by
    /// [`flow_eval_window_key`]. See [`FlowEvalWindowTickLatch`] — this is what makes "at most ONE
    /// pending `flowEvalTick` per (instance, window)" a fact the session owns rather than a
    /// convention every caller has to re-derive. Plain `Copy` rows keyed by hash, so retirement is
    /// a single `clear` (the `tessellate_progress_by_hash` precedent) and no window id is ever
    /// retained here.
    window_tick_latches: HistoryFoldIndex<u64, FlowEvalWindowTickLatch>,
    live_geometry_handles: HistoryFoldIndex<String, ()>,
    /// 🧊 Tessellated preview meshes keyed by geometry handle, each one a base64 `pack` record body
    /// (see `mesh::encode_mesh_pack`) — filled via extension `tessellate` because
    /// runtime-installable brep owns the kernel that minted the handles. Binary, not a JSON number
    /// array: the render path decodes typed arrays instead of parsing millions of JSON tokens.
    preview_mesh_pack_by_handle: HistoryFoldIndex<String, String>,
    /// ⏳ In-flight tessellate requests keyed by `nodeHash` forwarded through `InvokeExtension`. A
    /// request is removed the moment its answer is folded, even a partial one — the continuation
    /// re-admits it on the next tick.
    pending_tessellate_by_hash: HistoryFoldIndex<u64, String>,
    /// 🔗 The handle every admitted `nodeHash` belongs to, kept for as long as the tessellation is
    /// unfinished. This is what makes a MULTI-STEP tessellation survive `retain_preview_meshes`:
    /// its progress row and its half-received mesh body are keyed by hash, and pruning them by the
    /// (already emptied) pending table would restart the transfer from chunk zero forever.
    tessellate_handle_by_hash: HistoryFoldIndex<u64, String>,
    /// 📈 Progress of every tessellation this session has admitted, keyed by `nodeHash`. Plain
    /// `Copy` rows — no heap, so retirement is a single take.
    tessellate_progress_by_hash: HistoryFoldIndex<u64, PreviewTessellateProgress>,
    /// 🧱 Partially received mesh bodies keyed by `nodeHash`, accumulated one intake-sized base64
    /// chunk per round trip until `next_chunk == chunks`.
    tessellate_chunks_by_hash: HistoryFoldIndex<u64, String>,
    /// 🩺 Blocking validate-gate findings keyed by geometry handle, as a JSON array string.
    preview_diagnostics_by_handle: HistoryFoldIndex<String, String>,
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
    eval_progress_by_hash: HistoryFoldIndex<u64, PreviewEvalProgress>,
    /// 📏️ Fingerprint of the mesh packs the preview last published — lets tessellation land without
    /// moving the evaluation text while still owing a republication
    /// (`📓️slider-mesh-supersession-fallback-2026-09-15.md`).
    published_preview_mesh_digest: u64,
    baseline_geometry_pending:bool,
    pending_host:Option<FlowHost>,
    retiring_host:Option<FlowHostRetirement>,
    pending_host_cancelled:bool,
    preview_cancellation_cursor:Option<usize>,
    preview_cancellation_phase:u8,
    preview_cancellation_progress:RetainedCloneProgress,
    preview_retention:Option<SessionPreviewRetention>,
    preview_retention_progress:RetainedCloneProgress,
    retirement: SessionRetirement,
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

semio_framework_value::artifact_retire_struct!(ExtensionEvaluateFault{extension_id,capability,code,message});

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

struct SessionResetFields {
    painted: String,
    converged: String,
    status: String,
    handles: HistoryFoldIndex<String, ()>,
    meshes: HistoryFoldIndex<String, String>,
    diagnostics: HistoryFoldIndex<String, String>,
    pending: HistoryFoldIndex<u64, String>,
    tessellate_handles: HistoryFoldIndex<u64, String>,
    chunks: HistoryFoldIndex<u64, String>,
    tessellation: HistoryFoldIndex<u64, PreviewTessellateProgress>,
    evaluation: HistoryFoldIndex<u64, PreviewEvalProgress>,
    fault: Option<ExtensionEvaluateFault>,
    latches: Option<HistoryFoldIndex<u64, FlowEvalWindowTickLatch>>,
    retention:Option<SessionPreviewRetention>,
}

semio_framework_value::artifact_retire_struct!(SessionResetFields{painted,converged,status,handles,meshes,diagnostics,pending,tessellate_handles,chunks,tessellation,evaluation,fault,latches,retention});

struct SessionResetSources {
    eval: Option<Arc<String>>,
    baseline:FlowBaselineLeases,
    fields: SessionResetFields,
}

impl SessionResetSources {
    fn take(state:&mut FlowEvalSessionState,reset_latches:bool)->Self {
        Self{eval:state.eval_json.take(),baseline:FlowBaselineLeases{snapshot:state.previous_snapshot.take(),channels:state.previous_channels.take(),current:None},fields:SessionResetFields{
            painted:std::mem::take(&mut state.painted_eval_json),converged:std::mem::take(&mut state.converged_eval_json),status:std::mem::take(&mut state.status_json),
            handles:std::mem::take(&mut state.live_geometry_handles),meshes:std::mem::take(&mut state.preview_mesh_pack_by_handle),diagnostics:std::mem::take(&mut state.preview_diagnostics_by_handle),
            pending:std::mem::take(&mut state.pending_tessellate_by_hash),tessellate_handles:std::mem::take(&mut state.tessellate_handle_by_hash),chunks:std::mem::take(&mut state.tessellate_chunks_by_hash),
            tessellation:std::mem::take(&mut state.tessellate_progress_by_hash),evaluation:std::mem::take(&mut state.eval_progress_by_hash),fault:state.extension_evaluate_fault.take(),latches:reset_latches.then(||std::mem::take(&mut state.window_tick_latches)),retention:state.preview_retention.take(),
        }}
    }
    fn restore(self,state:&mut FlowEvalSessionState){
        let Self{eval,baseline:FlowBaselineLeases{snapshot,channels,current},fields:SessionResetFields{painted,converged,status,handles,meshes,diagnostics,pending,tessellate_handles,chunks,tessellation,evaluation,fault,latches,retention}}=self;
        assert!(current.is_none(),"original session reset has no active Host channel lease");
        state.eval_json=eval;state.previous_snapshot=snapshot;state.previous_channels=channels;state.preview_retention=retention;
        state.painted_eval_json=painted;state.converged_eval_json=converged;state.status_json=status;
        state.live_geometry_handles=handles;state.preview_mesh_pack_by_handle=meshes;state.preview_diagnostics_by_handle=diagnostics;
        state.pending_tessellate_by_hash=pending;state.tessellate_handle_by_hash=tessellate_handles;state.tessellate_chunks_by_hash=chunks;
        state.tessellate_progress_by_hash=tessellation;state.eval_progress_by_hash=evaluation;state.extension_evaluate_fault=fault;if let Some(latches)=latches{state.window_tick_latches=latches;}
    }
}

impl semio_framework_value::retirement::RetireOwned for SessionResetSources {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{
        use semio_framework_value::retirement::{RetireOwned,deferred,sequence,shared::SharedControlledRetirement};
        let eval:Box<dyn semio_framework_value::retirement::RetirementCursor>=match self.eval{Some(value)=>Box::new(SharedControlledRetirement::lease(value)),None=>().retirement()};
        sequence(vec![eval,deferred(self.baseline),deferred(self.fields)])
    }
    fn retirement_birth_bytes(&self)->Option<usize>{
        use semio_framework_value::retirement::{deferred_birth_bytes_for,leaf_birth_bytes,sequence_birth_bytes,shared::shared_retirement_birth_bytes};
        sequence_birth_bytes(&[if self.eval.is_some(){shared_retirement_birth_bytes::<String>()}else{leaf_birth_bytes::<()>()},deferred_birth_bytes_for(&self.baseline),deferred_birth_bytes_for(&self.fields)])
    }
    fn controlled_retirement_supported()->bool{true}
}

#[derive(semio_framework_value::RetireOwned)]
struct SessionPreviewCancellationSources {
    retention:Option<SessionPreviewRetention>,
    pending:HistoryFoldIndex<u64,String>,
    handles:HistoryFoldIndex<u64,String>,
    chunks:HistoryFoldIndex<u64,String>,
    evaluation:HistoryFoldIndex<u64,PreviewEvalProgress>,
}
impl SessionPreviewCancellationSources {
    fn take(state:&mut FlowEvalSessionState)->Self{Self{retention:state.preview_retention.take(),pending:std::mem::take(&mut state.pending_tessellate_by_hash),handles:std::mem::take(&mut state.tessellate_handle_by_hash),chunks:std::mem::take(&mut state.tessellate_chunks_by_hash),evaluation:std::mem::take(&mut state.eval_progress_by_hash)}}
    fn restore(self,state:&mut FlowEvalSessionState){state.preview_retention=self.retention;state.pending_tessellate_by_hash=self.pending;state.tessellate_handle_by_hash=self.handles;state.tessellate_chunks_by_hash=self.chunks;state.eval_progress_by_hash=self.evaluation;}
}

#[derive(semio_framework_value::RetireOwned)]
enum SessionRetentionRow {
    Mesh((String,String)),Pending((u64,String)),Progress((u64,PreviewTessellateProgress)),Roster(Vec<String>),
}
#[derive(semio_framework_value::RetireOwned)]
struct SessionPreviewRetention {
    roster:Option<Vec<String>>,phase:u8,slot:usize,comparison:usize,matched:bool,
    removed:Option<ControlledRetirement<SessionRetentionRow>>,
}

enum SessionCollectionOwner {
    Retention(SessionPreviewRetention),
    Cancellation(SessionPreviewCancellationSources),
    Reset(SessionResetSources),
    Baseline(FlowBaselineLeases),
    SourceLease(Arc<String>),
    Text(String),
    Handles(HistoryFoldIndex<String, ()>),
    Meshes(HistoryFoldIndex<String, String>),
    Pending(HistoryFoldIndex<u64, String>),
    Latches(HistoryFoldIndex<u64, FlowEvalWindowTickLatch>),
    Tessellation(HistoryFoldIndex<u64, PreviewTessellateProgress>),
    Evaluation(HistoryFoldIndex<u64, PreviewEvalProgress>),
    Snapshot(Arc<TreeSnapshot>),
    Channels(Arc<EvalChannels>),
    Registry(neural::SharedRegistry),
    Cache(Arc<NeuralCache>),
    Fault(ExtensionEvaluateFault),
}

impl semio_framework_value::retirement::RetireOwned for SessionCollectionOwner {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{
        use semio_framework_value::retirement::{RetireOwned,shared::SharedControlledRetirement};
        match self{
            Self::Retention(value)=>value.retirement(),
            Self::Cancellation(value)=>value.retirement(),
            Self::Reset(value)=>value.retirement(),
            Self::Baseline(value)=>value.retirement(),
            Self::SourceLease(value)=>Box::new(SharedControlledRetirement::lease(value)),
            Self::Snapshot(value)=>Box::new(SharedControlledRetirement::lease(value)),Self::Channels(value)=>Box::new(SharedControlledRetirement::lease(value)),
            Self::Cache(value)=>Box::new(SharedControlledRetirement::lease(value)),
            Self::Text(value)=>value.retirement(),Self::Handles(value)=>value.retirement(),Self::Meshes(value)=>value.retirement(),Self::Pending(value)=>value.retirement(),
            Self::Latches(value)=>value.retirement(),Self::Tessellation(value)=>value.retirement(),Self::Evaluation(value)=>value.retirement(),
            Self::Registry(value)=>value.retirement(),Self::Fault(value)=>value.retirement(),
        }
    }
    fn retirement_birth_bytes(&self)->Option<usize>{
        use semio_framework_value::retirement::{RetireOwned,shared::shared_retirement_birth_bytes};
        match self{
            Self::Retention(value)=>value.retirement_birth_bytes(),
            Self::Cancellation(value)=>value.retirement_birth_bytes(),
            Self::Reset(value)=>value.retirement_birth_bytes(),
            Self::Baseline(value)=>value.retirement_birth_bytes(),
            Self::SourceLease(_)=>Some(shared_retirement_birth_bytes::<String>()),Self::Cache(_)=>Some(shared_retirement_birth_bytes::<NeuralCache>()),
            Self::Snapshot(_)=>Some(shared_retirement_birth_bytes::<TreeSnapshot>()),Self::Channels(_)=>Some(shared_retirement_birth_bytes::<EvalChannels>()),
            Self::Text(value)=>value.retirement_birth_bytes(),Self::Handles(value)=>value.retirement_birth_bytes(),Self::Meshes(value)=>value.retirement_birth_bytes(),Self::Pending(value)=>value.retirement_birth_bytes(),
            Self::Latches(value)=>value.retirement_birth_bytes(),Self::Tessellation(value)=>value.retirement_birth_bytes(),Self::Evaluation(value)=>value.retirement_birth_bytes(),
            Self::Registry(value)=>value.retirement_birth_bytes(),Self::Fault(value)=>value.retirement_birth_bytes(),
        }
    }
    fn controlled_retirement_supported()->bool{true}
}

#[derive(Default)]
struct SessionRetirement {
    progress:RetainedCloneProgress,
    root: Option<ControlledRetirement<SessionCollectionOwner>>,
    owners: RetirementQueue,
}

impl SessionRetirement {
    fn terminal_is_empty(&self)->bool{self.root.is_none()&&self.owners.terminal_is_empty()}
    fn admission_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.root.is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if !self.owners.has_reserved_slot(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"session retirement requires an admitted original queue slot"))}
        Ok(RetirementDemand{capacity_bytes:RetirementQueue::frame_birth_bytes::<SessionCollectionOwner>(),depth:self.owners.len()+1,..Default::default()})
    }
    fn preflight(&self,grant:RetainedCloneGrant)->Result<(),ValueError>{
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"session mutation requires an admitted item"))}
        let demand=self.admission_demands()?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"session mutation requires original source depth"))}
        if grant.maximum_capacity_bytes<demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"session mutation requires its original retained frame capacity"))}
        Ok(())
    }
    fn admit(&mut self,value:SessionCollectionOwner,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,SessionCollectionOwner)>{
        if self.root.is_some(){return self.owners.admit_owned(value,grant)}
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"session handoff requires an admitted item"),value))}
        if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"session handoff requires admitted depth"),value))}
        match ControlledRetirement::new(value){Ok(owner)=>{self.root=Some(owner);Ok(RetainedCloneProgress{copied_items:1,..Default::default()})},Err(error)=>Err(error)}
    }
    fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.root.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
        Ok(RetirementDemand{copy_bytes:self.owners.next_copy_byte_demand()?,capacity_bytes:self.owners.next_capacity_byte_demand(copy)?,release_bytes:self.owners.next_release_byte_demand()?,depth:self.owners.next_depth_demand()?})
    }
    fn step_progress(&self)->RetainedCloneProgress{self.progress}
    fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.progress=Default::default();
        let Some(owner)=self.root.as_mut()else{let result=self.owners.step(grant);self.progress=self.owners.step_progress();return result};
        let result=owner.step(grant);self.progress=owner.step_progress();let step=result?;
        let progress=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,owner.terminal_is_empty(),"Flow session original root")?.progress();
        if owner.terminal_is_empty(){self.root=None;}
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
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

/// 🧹️ How many close turns one cold evaluation-session teardown pays before it declares the ladder stuck.
const FLOW_EVAL_SESSION_COLD_CLOSE_STEPS: usize = 1_000_000;

impl FlowEvalSession {
    /// 📔️ Shares the explicitly supplied registry with this session's evaluation hosts.
    pub fn with_operator_registry(mut self, registry: neural::SharedRegistry) -> Self {
        assert!(!self.closing, "closed session refuses an operator registry");
        assert!(self.state.operator_registry.is_none(), "operator registry is supplied once");
        self.state.operator_registry = Some(registry);
        self.state.flow_extension_generation = 0;
        self
    }
    /// 🔌️ Supplies one geometry authority that this owner retains and explicitly closes.
    pub fn with_geometry_port(mut self, port: Box<dyn crate::geometry::GeometryPort>) -> Self {
        assert!(!self.closing, "closed session refuses a geometry authority");
        assert!(self.state.geometry_port.is_none(), "geometry authority is supplied once");
        self.state.geometry_port = Some(port);
        self
    }
    /// 🌐️ Reads the explicitly supplied geometry authority for this evaluation session.
    pub fn geometry_port(&self) -> Option<&dyn crate::geometry::GeometryPort> { self.state.geometry_port.as_deref() }

    pub fn new() -> Self {
        Self {
            state: std::mem::ManuallyDrop::new(FlowEvalSessionState {
                retiring_geometry_port: None,
                geometry_port: None,
                operator_registry: None,
                neural_cache: Some(Arc::new(NeuralCache::new())),
                previous_snapshot: None,
                previous_channels: None,
                eval_json: Some(Arc::new(String::new())),
                painted_eval_json: String::new(),
                converged_eval_json: String::new(),
                status_json: "{}".into(),
                tick_scheduled: false,
                window_tick_latches: HistoryFoldIndex::new(),
                live_geometry_handles: HistoryFoldIndex::new(),
                preview_mesh_pack_by_handle: HistoryFoldIndex::new(),
                pending_tessellate_by_hash: HistoryFoldIndex::new(),
                tessellate_handle_by_hash: HistoryFoldIndex::new(),
                tessellate_progress_by_hash: HistoryFoldIndex::new(),
                eval_progress_by_hash: HistoryFoldIndex::new(),
                tessellate_chunks_by_hash: HistoryFoldIndex::new(),
                preview_diagnostics_by_handle: HistoryFoldIndex::new(),
                retirement: SessionRetirement::default(),
                flow_extension_generation: flow_extension_registry_generation(),
                extension_evaluate_fault: None,
                preview_cancelled: false,
                published_preview_mesh_digest: 0,
                baseline_geometry_pending:false,
                pending_host:None,
                retiring_host:None,
                pending_host_cancelled:false,
                preview_cancellation_cursor:None,
                preview_cancellation_phase:0,
                preview_cancellation_progress:Default::default(),
                preview_retention:None,
                preview_retention_progress:Default::default(),
                closing: false,
            }),
        }
    }

    pub fn neural_cache(&self) -> Arc<NeuralCache> {
        self.neural_cache.as_ref().expect("live Flow evaluation session owns its neural cache").clone()
    }

    /// 📥️ Retains the same admitted original Host across evaluation and publication turns.
    pub fn retain_tick_host(&mut self,host:FlowHost,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,FlowHost)>{
        if self.closing||self.state.pending_host.is_some()||self.state.retiring_host.is_some(){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original tick Host custody is occupied"),host))}
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original tick Host intake requires an admitted item"),host))}
        if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"original tick Host intake requires source depth"),host))}
        self.state.pending_host=Some(host);self.state.pending_host_cancelled=false;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🔎️ Borrows the exact original Host retained by the current tick.
    pub fn tick_host(&self)->Option<&FlowHost>{self.state.pending_host.as_ref()}

    pub fn install_baseline_into(&self,host:&mut FlowHost,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        match host.install_eval_baseline(self.previous_snapshot.clone(),self.previous_channels.clone(),self.state.flow_extension_generation,grant){
            Ok(progress)=>Ok(progress),Err((error,snapshot,channels))=>{drop(snapshot);drop(channels);Err(error)},
        }
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
    pub fn capture_baseline_from(&mut self,host:&FlowHost,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        if !host.baseline_publication_terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original host baseline publication must complete before capture"))}
        if self.state.baseline_geometry_pending{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original geometry membership must finish before another baseline capture"))}
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline capture requires an admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original baseline capture requires source depth"))}
        let displaced=self.state.previous_snapshot.is_some()||self.state.previous_channels.is_some();
        if displaced{self.state.retirement.preflight(grant)?;}
        let (snapshot,channels)=host.eval_baseline();
        let evaluated_generation = host.eval_baseline_registry_generation();
        let progress=if displaced{
            let original=FlowBaselineLeases{snapshot:self.state.previous_snapshot.take(),channels:self.state.previous_channels.take(),current:None};
            match self.state.retirement.admit(SessionCollectionOwner::Baseline(original),grant){
                Ok(progress)=>progress,
                Err((error,SessionCollectionOwner::Baseline(original)))=>{self.state.previous_snapshot=original.snapshot;self.state.previous_channels=original.channels;return Err(error)},
                Err(_)=>unreachable!("original session baseline admission kind"),
            }
        }else{RetainedCloneProgress{copied_items:1,..Default::default()}};
        if snapshot.is_some()&&evaluated_generation>self.state.flow_extension_generation{self.state.flow_extension_generation=evaluated_generation;}
        self.state.previous_snapshot=snapshot;self.state.previous_channels=channels;
        self.state.baseline_geometry_pending=self.state.previous_channels.is_some();
        Ok(progress)
    }

    /// 🌐️ Geometry claims remain pending until their original admitted roster operation finishes.
    pub fn baseline_capture_terminal_is_empty(&self)->bool{!self.state.baseline_geometry_pending}

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

    /// 🔁️ Checks the existing partial-response ledger before sending a compact extension resume.
    pub fn has_evaluation_progress(&self,node_hash:u64)->bool {self.eval_progress_by_hash.contains_key(&node_hash)}

    /// ⏱️ One budgeted dag walk. `turn_started_us` is when the guest turn this walk belongs to began
    /// — `None` for a walk that opens its own turn, `Some` for one running inline inside a fold's
    /// turn, which then shares that turn's single deadline (see [`flow_eval_tick_budget`]).
    pub fn tick(&mut self, host: &mut FlowHost, turn_started_us: Option<u64>) -> bool {
        let remaining = host.evaluate_step(flow_eval_tick_budget(turn_started_us),&|hash|!self.has_evaluation_progress(hash));
        let state = &mut *self.state;
        if let Some(previous) = state.eval_json.replace(Arc::new(host.last_eval_json.clone())) { state.retiring_collections.push_back(SessionCollectionOwner::SourceLease(semio_framework_value::retirement::shared_lease_retirement(previous))); }
        self.status_json = build_flow_status_json(host, &remaining);
        if remaining.is_empty() {
            self.capture_baseline_from(host);
        }
        self.repaint(host, remaining.is_empty());
        self.tick_scheduled = !remaining.is_empty();
        self.tick_scheduled
    }

    /// 🖼️ Advances what a preview PAINTS, and — on a converged walk only — what a later unconverged
    /// one may fall back on.
    ///
    /// 🪟️ The fallback is filtered through the host's CURRENT widget roster, so switching example
    /// replaces the graph instead of painting the previous example's leftovers over the new one: a
    /// node id the document no longer carries is not a node whose answer is merely late.
    fn repaint(&mut self, host: &FlowHost, converged: bool) {
        let painted = if converged { self.eval_json().to_owned() } else { merge_unanswered_eval_entries(self.eval_json(), &self.converged_eval_json, &host.host_snapshot) };
        let state = &mut *self.state;
        state.retirement.text(std::mem::replace(&mut state.painted_eval_json, painted));
        if converged {
            let converged_text = state.eval_json.as_ref().map_or_else(String::new, |text| text.as_str().to_owned());
            state.retirement.text(std::mem::replace(&mut state.converged_eval_json, converged_text));
        }
    }

    pub fn eval_json(&self) -> &str {
        self.eval_json.as_ref().map_or("", |text| text.as_str())
    }

    /// 🔗️ Leases this primary immutable evaluation allocation without copying its text.
    pub fn lease_eval_json(&self)->Option<Arc<String>> {self.eval_json.as_ref().map(Arc::clone)}

    /// 🔍️ Admits publication only while the current primary owner is the held source lease.
    pub fn owns_eval_json(&self,source:&Arc<String>)->bool {self.eval_json.as_ref().is_some_and(|current|Arc::ptr_eq(current,source))}

    /// 🖼️ The evaluation a preview paints — see [`FlowEvalSessionState::painted_eval_json`]. This is
    /// what every publication carries; `eval_json` stays the live walk's own answer, because that is
    /// what the next walk, the pending-handle probe and every law about convergence read.
    pub fn painted_eval_json(&self) -> &str {
        if self.painted_eval_json.is_empty() {
            return self.eval_json();
        }
        &self.painted_eval_json
    }

    /// 🏁️ The last evaluation that fully converged — what a preview may fall back to while a node's
    /// new brep handle is still crossing and its tessellation has not landed yet.
    pub fn converged_eval_json(&self) -> &str {
        &self.converged_eval_json
    }

    /// 📤️ What this session owes ONE surface whose retained preview publication currently holds
    /// `retained`. See [`flow_eval_publication_for`] — the decision is per PUBLICATION TARGET, never
    /// per session: two preview windows on one instance each own their own retained bytes.
    pub fn eval_publication_for(&self, retained: Option<&str>) -> FlowEvalPublication {
        flow_eval_publication_for(self, retained)
    }

    pub fn status_json(&self) -> &str {
        if self.status_json.is_empty(){"{}"}else{&self.status_json}
    }

    pub fn status_json_for_host(&self, host: &FlowHost) -> String {
        let remaining = host.pending_eval_widget_ids();
        build_flow_status_json(host, &remaining)
    }

    /// 📤️ Advances the original live evaluation lease while preserving progress and preview owners.
    pub fn publish_eval_json(&mut self,eval_json:Arc<String>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Arc<String>)>{
        if self.state.eval_json.as_ref().is_some_and(|current|Arc::ptr_eq(current,&eval_json)){return Ok(Default::default())}
        if let Err(error)=self.state.retirement.preflight(grant){return Err((error,eval_json))}
        if let Some(original)=self.state.eval_json.take(){
            match self.state.retirement.admit(SessionCollectionOwner::SourceLease(original),grant){
                Ok(progress)=>{self.state.eval_json=Some(eval_json);Ok(progress)},
                Err((error,SessionCollectionOwner::SourceLease(original)))=>{self.state.eval_json=Some(original);Err((error,eval_json))},
                Err(_)=>unreachable!("original live evaluation lease returned a different source family"),
            }
        }else{self.state.eval_json=Some(eval_json);Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}
    }

    /// 🪙️ Borrows the pending Host publication demand without creating an evaluation owner.
    pub fn next_tick_publication_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        self.state.pending_host.as_ref().map_or(Ok(Default::default()),|host|host.next_baseline_publication_demands(copy))
    }

    pub fn next_tick_publication_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_tick_publication_demands(0)?.copy_bytes)}
    pub fn next_tick_publication_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.next_tick_publication_demands(copy)?.capacity_bytes)}
    pub fn next_tick_publication_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_tick_publication_demands(0)?.release_bytes)}
    pub fn next_tick_publication_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_tick_publication_demands(0)?.depth)}

    /// 🎟️ Advances only the exact pending original Host publication under the incoming grant.
    pub fn tick_publication_step_progress(&self)->RetainedCloneProgress{self.state.pending_host.as_ref().map_or(Default::default(),FlowHost::baseline_publication_step_progress)}

    pub fn tick_publication_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let Some(host)=self.state.pending_host.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};
        let step=host.baseline_publication_step(grant)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,host.baseline_publication_terminal_is_empty(),"Flow original retained tick publication")
    }

    /// 🔗️ Captures the original retained tick's completed shared baseline, preserving refusal custody.
    pub fn capture_tick_baseline_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let Some(host)=self.state.pending_host.take()else{return Ok(Default::default())};
        let result=self.capture_baseline_from(&host,grant);self.state.pending_host=Some(host);result
    }

    /// 🎟️ Publishes one preborn immutable evaluation after admitting the original reset owners.
    pub fn set_eval_json(&mut self,eval_json:Arc<String>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Arc<String>)>{
        if let Err(error)=self.state.retirement.preflight(grant){return Err((error,eval_json))}
        let original=SessionResetSources::take(&mut self.state,false);
        match self.state.retirement.admit(SessionCollectionOwner::Reset(original),grant){
            Ok(progress)=>{self.state.eval_json=Some(eval_json);self.state.tick_scheduled=false;self.state.published_preview_mesh_digest=0;self.state.pending_host_cancelled=self.state.pending_host.is_some();Ok(progress)},
            Err((error,SessionCollectionOwner::Reset(original)))=>{original.restore(&mut self.state);Err((error,eval_json))},
            Err(_)=>unreachable!("original session reset returned a different source family"),
        }
    }

    pub fn pending(&self) -> bool {
        self.tick_scheduled
    }

    //#region 🔒️TickLatch
    /// 🪙️ Quotes the exact original latch arena without constructing a replacement row.
    pub fn next_window_tick_latch_copy_byte_demand(&self,window_id:&str)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_copy_byte_demand(&key)}
    }
    pub fn next_window_tick_latch_capacity_byte_demand(&self,window_id:&str,copy:usize)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_capacity_byte_demand(&key,copy)}
    }
    pub fn next_window_tick_latch_release_byte_demand(&self,window_id:&str)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_release_byte_demand(&key)}
    }
    pub fn next_window_tick_latch_depth_demand(&self,window_id:&str)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_depth_demand(&key)}
    }

    /// 🌱️ Admits original backing before placing the original Copy-keyed latch.
    pub fn prepare_window_tick_latch(&mut self,window_id:&str,grant:RetainedCloneGrant)->Result<RetainedCloneStep,(ValueError,RetainedCloneProgress)>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){return Ok(RetainedCloneStep::Complete(Default::default()))}
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original latch preparation requires one admitted item"),Default::default()))}
        if self.state.window_tick_latches.next_insert_capacity_byte_demand(&key,grant.maximum_copy_bytes).map_err(|error|(error,RetainedCloneProgress::default()))?>0{
            return self.state.window_tick_latches.reserve_insert_step(&key,grant).map(RetainedCloneStep::Progress);
        }
        match self.state.window_tick_latches.insert_reserved(key,FlowEvalWindowTickLatch::default(),grant){
            Ok((None,progress))=>Ok(RetainedCloneStep::Complete(progress)),
            Ok((Some(_),_))=>unreachable!("original latch preparation cannot displace an occupied row"),
            Err((error,_,_))=>Err((error,Default::default())),
        }
    }

    fn window_tick_latch_mut(&mut self,window_id:&str,grant:RetainedCloneGrant)->Result<&mut FlowEvalWindowTickLatch,ValueError>{
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original latch mutation requires one admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original latch mutation requires source depth"))}
        self.state.window_tick_latches.get_mut(&flow_eval_window_key(window_id)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original latch backing must be admitted before mutation"))
    }

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
    pub fn arm_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        if latch.armed {
            return Ok((false,progress));
        }
        if latch.in_flight > 0 {
            latch.owed = true;
            return Ok((false,progress));
        }
        latch.armed = true;
        latch.owed = false;
        Ok((true,progress))
    }

    /// 🔎️ Whether `window_id` owes a tick that nothing has armed — a window that has never ticked
    /// owes its first one, and a window whose own last tick reported unfinished work owes another
    /// only while no tick and no extension answer is already chasing it. The REFRESH poll's whole
    /// question, and the reason it costs no evaluation at all.
    pub fn window_tick_owed(&self, window_id: &str) -> bool {
        if self.preview_cancelled{return false}
        match self.window_tick_latches.get(&flow_eval_window_key(window_id)) {
            None => true,
            Some(latch) => latch.unfinished && !latch.armed && latch.in_flight == 0,
        }
    }

    /// 🔒️ Arms the tick `window_id` actually owes — [`FlowEvalSession::window_tick_owed`] and
    /// [`FlowEvalSession::arm_window_tick`] as the single question a refresh poll asks.
    pub fn arm_owed_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        if self.window_tick_owed(window_id){self.arm_window_tick(window_id,grant)}else{Ok((false,Default::default()))}
    }

    /// ▶️ Marks `window_id`'s armed tick as RUNNING: the effect has been delivered, so the latch is
    /// free for whatever this tick's own outcome decides to arm next.
    /// 🛑 A tick that actually begins is work RESUMING, which is the one thing that retires the
    /// `cancelled` banner an explicit gesture raised — see [`FlowEvalSession::preview_cancelled`].
    pub fn begin_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        if !self.preview_cancellation_terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original preview cancellation must finish before another tick"))}
        self.window_tick_latch_mut(window_id,grant)?.armed=false;
        self.state.preview_cancelled = false;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// ⏳️ Records that `window_id`'s tick parked `count` extension invocations. Those answers own
    /// the continuation from here — the tick that parked them owes no re-arm.
    pub fn note_window_extensions_in_flight(&mut self, window_id: &str, count: usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        latch.in_flight = latch.in_flight.saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 📝️ Records what `window_id`'s tick reported: `unfinished` is the tick's own "there is more to
    /// compute". Only this makes a refresh poll able to answer [`FlowEvalSession::window_tick_owed`]
    /// without evaluating anything.
    pub fn note_window_tick_outcome(&mut self, window_id: &str, unfinished: bool,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        self.window_tick_latch_mut(window_id,grant)?.unfinished=unfinished;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🚧️ Marks `window_id`'s chain as GIVEN UP: an extension answer this process cannot fold (a
    /// faulted `invokeExtension`, an answer that seeds no cache entry) is not slow work, and the
    /// next tick would park the identical request and fault again at the host's own cadence. Nothing
    /// but a gesture — a contributions install, an example switch, an edit — may resume it, and each
    /// of those arms through [`FlowEvalSession::arm_window_tick`] directly.
    pub fn abandon_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        latch.unfinished = false;
        latch.owed = false;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🧹️ Drops the latch of every window that is no longer attached. A detached window's latch can
    /// never be discharged by a tick — the retained route refuses a window that has left the
    /// roster — so keeping it would make a window that comes back unarmable forever.
    pub fn retain_window_tick_latches(&mut self,window_ids:&[&str],cursor:&mut usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if grant.maximum_depth<self.state.window_tick_latches.next_extract_slot_depth_demand(*cursor)?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"session latch pruning requires original arena depth"))}
        let slot=*cursor;
        if slot>=self.state.window_tick_latches.slot_count(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        self.state.window_tick_latches.extract_slot_if(slot,|key,_|window_ids.iter().any(|id|flow_eval_window_key(id)==*key));
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        *cursor+=1;
        Ok(if *cursor==self.state.window_tick_latches.slot_count(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    /// ✅️ Folds ONE extension answer back into `window_id`'s latch. Answers whether that settle
    /// discharged an arm a sibling answer had already asked for, so the LAST answer of a fan-out
    /// emits exactly one re-arm and the earlier ones emit none.
    pub fn settle_window_extension(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        latch.in_flight = latch.in_flight.saturating_sub(1);
        if latch.in_flight == 0 && latch.owed && !latch.armed {
            latch.armed = true;
            latch.owed = false;
            return Ok((true,RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }

    /// 🔁️ Whether the fold that just settled `window_id`'s answer may run that window's next wave
    /// INLINE, inside the answer's own guest turn, instead of leaving the `previewEval` run job to
    /// dispatch a `flowEvalTick` hop for it.
    ///
    /// 🪜️ The question is deliberately the run job's OWN scheduling question
    /// ([`FlowEvalSession::window_tick_owed`]): a fold may only take over a hop the scheduler would
    /// otherwise have dispatched, never invent one. So the hop count can fall but the chain's shape
    /// cannot change — and a fold that declines simply leaves the latch alone, which IS the park:
    /// the run job's next step reads the same debt and dispatches the round trip as before. There is
    /// no parked-continuation state anywhere, because "not inline" is the pre-existing behaviour.
    ///
    /// 🛑 A CANCELLED session is refused outright. `begin_window_tick` retires the `cancelled` banner
    /// (work resuming is the one thing that may), so an inline continuation on a cancelled chain
    /// would not merely compute one wave too many — it would un-cancel the run the user stopped. A
    /// cancel also defaults every latch, so `window_tick_owed` already answers `false`; the explicit
    /// guard is kept because the two facts must not be one accident apart
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// ⏱️ The wall half is [`flow_eval_inline_continuation_fits`]: a turn with less than
    /// [`FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US`] of its evaluation allowance left parks. BOTH
    /// instants are the caller's, so the rule is pure and a law states the park boundary in
    /// microseconds instead of racing a clock — and so a native test, which installs no clock at all,
    /// can still drive the branch a browser turn takes.
    pub fn inline_continuation_admitted(&self, window_id: &str, turn_started_us: Option<u64>, now_us: Option<u64>) -> bool {
        !self.preview_cancelled() && self.window_tick_owed(window_id) && flow_eval_inline_continuation_fits(turn_started_us, now_us)
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
    pub fn clear_window_tick_latches(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        if self.state.window_tick_latches.terminal_is_empty(){return Ok(Default::default())}
        self.state.retirement.preflight(grant)?;
        let source=SessionCollectionOwner::Latches(std::mem::take(&mut self.state.window_tick_latches));
        match self.state.retirement.admit(source,grant){
            Ok(progress)=>Ok(progress),
            Err((error,SessionCollectionOwner::Latches(original)))=>{self.state.window_tick_latches=original;Err(error)},
            Err(_)=>unreachable!("session original latch admission kind"),
        }
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
    pub fn invalidate_for_flow_extension_registry(&mut self,generation:u64,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{
        if self.operator_registry.is_some()||self.state.flow_extension_generation==generation{return Ok((false,Default::default()))}
        self.state.retirement.preflight(grant)?;
        let original=SessionResetSources::take(&mut self.state,true);
        match self.state.retirement.admit(SessionCollectionOwner::Reset(original),grant){
            Ok(progress)=>{self.state.flow_extension_generation=generation;self.state.preview_cancelled=false;self.state.tick_scheduled=false;self.state.published_preview_mesh_digest=0;self.state.pending_host_cancelled=self.state.pending_host.is_some();Ok((true,progress))},
            Err((error,SessionCollectionOwner::Reset(original)))=>{original.restore(&mut self.state);Err(error)},
            Err(_)=>unreachable!("original registry invalidation returned a different source family"),
        }
    }

    /// 💥 Records the fault an `evaluate` answer came back with. The extension id and the decoded
    /// message are the SDK's own (`reactor::extension_response_args` echoes `faultCode`/
    /// `faultMessage` onto the response action) — nothing here re-words them.
    pub fn note_extension_evaluate_fault(&mut self,fault:ExtensionEvaluateFault,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,ExtensionEvaluateFault)> {
        if self.state.extension_evaluate_fault.is_none(){
            if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"session fault handoff requires an admitted item"),fault))}
            if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"session fault handoff requires original source depth"),fault))}
            self.state.extension_evaluate_fault=Some(fault);return Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
        }
        if let Err(error)=self.state.retirement.preflight(grant){return Err((error,fault))}
        let original=self.state.extension_evaluate_fault.take().unwrap();
        match self.state.retirement.admit(SessionCollectionOwner::Fault(original),grant){
            Ok(progress)=>{self.state.extension_evaluate_fault=Some(fault);Ok(progress)},
            Err((error,SessionCollectionOwner::Fault(original)))=>{self.state.extension_evaluate_fault=Some(original);Err((error,fault))},
            Err(_)=>unreachable!("session original fault admission kind"),
        }
    }

    /// ✅️ Forgets the last evaluate fault — an answer that folded supersedes it.
    pub fn clear_extension_evaluate_fault(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        if self.state.extension_evaluate_fault.is_none(){return Ok(Default::default())}
        self.state.retirement.preflight(grant)?;
        let original=self.state.extension_evaluate_fault.take().unwrap();
        match self.state.retirement.admit(SessionCollectionOwner::Fault(original),grant){
            Ok(progress)=>Ok(progress),
            Err((error,SessionCollectionOwner::Fault(original)))=>{self.state.extension_evaluate_fault=Some(original);Err(error)},
            Err(_)=>unreachable!("session original fault admission kind"),
        }
    }

    /// 💥 The evaluate fault this session is currently living with, if any.
    pub fn extension_evaluate_fault(&self) -> Option<&ExtensionEvaluateFault> {
        self.extension_evaluate_fault.as_ref()
    }

    pub fn seed_node_cache(&self, node_hash: u64, output: Dictionary) {
        let cache = self.neural_cache.as_deref().expect("live Flow evaluation session owns its neural cache");
        seed_flow_eval_node_cache(cache, node_hash, output)
    }

    /// 🧊 Preview mesh body (base64 `pack` record body) previously resolved through the owning
    /// geometry extension.
    pub fn preview_mesh_pack(&self, handle: &str) -> Option<&str> {
        self.preview_mesh_pack_by_handle.get(handle).map(String::as_str)
    }

    /// 📏️ Mesh-residency fingerprint the preview last published — see [`FlowEvalSessionState::published_preview_mesh_digest`].
    pub fn published_preview_mesh_digest(&self) -> u64 {
        self.published_preview_mesh_digest
    }

    /// 📏️ Records the mesh-residency fingerprint a preview publication just carried.
    pub fn note_published_preview_mesh_digest(&mut self, digest: u64) {
        self.published_preview_mesh_digest = digest;
    }

    /// 🧊 Every brep handle that currently holds a resolved preview tessellation pack.
    pub fn preview_packed_handles(&self) -> Vec<String> {
        self.preview_mesh_pack_by_handle.keys().cloned().collect()
    }

    /// 🩺 Blocking validate-gate findings for `handle`, as a JSON array string.
    pub fn preview_diagnostics(&self, handle: &str) -> Option<&str> {
        self.preview_diagnostics_by_handle.get(handle).map(String::as_str)
    }

    /// 🩺 Every handle the validate gate rejected, paired with its JSON issue array.
    pub fn preview_diagnostic_entries(&self) -> Vec<(&str, &str)> {
        self.preview_diagnostics_by_handle.iter().map(|(handle, issues)| (handle.as_str(), issues.as_str())).collect()
    }

    /// 📥️ Retains the exact incoming roster before pruning any original preview owner.
    pub fn begin_retain_preview_meshes(&mut self,handles:Vec<String>)->Result<(),(ValueError,Vec<String>)>{
        if self.closing||self.state.preview_retention.is_some()||!self.preview_cancellation_terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original preview retention is already active or closing"),handles))}
        self.state.preview_retention=Some(SessionPreviewRetention{roster:Some(handles),phase:0,slot:0,comparison:0,matched:false,removed:None});
        self.state.preview_retention_progress=Default::default();Ok(())
    }
    pub fn preview_retention_terminal_is_empty(&self)->bool{self.state.preview_retention.is_none()}
    pub fn preview_retention_step_progress(&self)->RetainedCloneProgress{self.state.preview_retention_progress}
    fn preview_retention_handle(&self,owner:&SessionPreviewRetention)->Option<&str>{
        match owner.phase {
            0=>self.state.preview_mesh_pack_by_handle.slot_entry(owner.slot).map(|(handle,_)|handle.as_str()),
            1=>self.state.preview_diagnostics_by_handle.slot_entry(owner.slot).map(|(handle,_)|handle.as_str()),
            2=>self.state.pending_tessellate_by_hash.slot_entry(owner.slot).map(|(_,handle)|handle.as_str()),
            3=>self.state.tessellate_progress_by_hash.slot_entry(owner.slot).and_then(|(hash,_)|self.state.tessellate_handle_by_hash.get(hash)).map(String::as_str),
            4=>self.state.tessellate_chunks_by_hash.slot_entry(owner.slot).and_then(|(hash,_)|self.state.tessellate_handle_by_hash.get(hash)).map(String::as_str),
            5=>self.state.tessellate_handle_by_hash.slot_entry(owner.slot).map(|(_,handle)|handle.as_str()),
            _=>None,
        }
    }
    /// 🪙️ Borrows all five currencies from the actual removed row or actual geometry cursor.
    pub fn next_preview_retention_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        let Some(owner)=self.state.preview_retention.as_ref()else{return Ok(Default::default())};
        if let Some(removed)=owner.removed.as_ref(){return Ok(RetirementDemand{copy_bytes:removed.next_copy_byte_demand()?,capacity_bytes:removed.next_capacity_byte_demand(copy)?,release_bytes:removed.next_release_byte_demand()?,depth:removed.next_depth_demand()?})}
        if owner.phase==7 {let port=self.geometry_port.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original geometry retention source disappeared"))?;return Ok(RetirementDemand{copy_bytes:port.next_retain_copy_byte_demand()?,capacity_bytes:port.next_retain_capacity_byte_demand(copy)?,release_bytes:port.next_retain_release_byte_demand()?,depth:port.next_retain_depth_demand()?})}
        let depth=match owner.phase {0=>self.preview_mesh_pack_by_handle.next_extract_slot_depth_demand(owner.slot)?,1=>self.preview_diagnostics_by_handle.next_extract_slot_depth_demand(owner.slot)?,2=>self.pending_tessellate_by_hash.next_extract_slot_depth_demand(owner.slot)?,3=>self.tessellate_progress_by_hash.next_extract_slot_depth_demand(owner.slot)?,4=>self.tessellate_chunks_by_hash.next_extract_slot_depth_demand(owner.slot)?,5=>self.tessellate_handle_by_hash.next_extract_slot_depth_demand(owner.slot)?,_=>1};
        Ok(RetirementDemand{depth:depth.max(1),..Default::default()})
    }
    /// 🍂️ Compares one roster entry or closes one exact original removed row per turn.
    pub fn retain_preview_meshes_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.state.preview_retention_progress=Default::default();
        if self.preview_retention_terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let demand=self.next_preview_retention_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original preview retention exceeds admitted depth"))}
        if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let mut owner=self.state.preview_retention.take().unwrap();
        let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        if let Some(removed)=owner.removed.as_mut(){
            let result=removed.step(grant);self.state.preview_retention_progress=removed.step_progress();self.state.preview_retention=Some(owner);
            let step=result?;progress=step.progress();
            if self.state.preview_retention.as_ref().unwrap().removed.as_ref().unwrap().terminal_is_empty(){self.state.preview_retention.as_mut().unwrap().removed=None;}
            self.state.preview_retention_progress=progress;return Ok(RetainedCloneStep::Progress(progress))
        }
        if owner.phase==7 {
            let port=self.geometry_port.as_mut().unwrap();let result=port.retain_step(grant);progress=port.retain_step_progress();
            if port.retain_terminal_is_empty(){owner.phase=8;}
            self.state.preview_retention=Some(owner);self.state.preview_retention_progress=progress;
            let step=result.map_err(|error|error.with_retained_progress(progress))?;semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,self.geometry_port.as_ref().unwrap().retain_terminal_is_empty(),"Flow original preview geometry retention")?;
            return Ok(RetainedCloneStep::Progress(progress))
        }
        if owner.phase==8 {self.state.baseline_geometry_pending=false;self.state.preview_retention_progress=progress;return Ok(RetainedCloneStep::Complete(progress))}
        if owner.phase==6 {
            let handles=owner.roster.take().unwrap();
            if let Some(port)=self.geometry_port.as_mut(){if let Err((error,handles))=port.begin_retain(handles){owner.roster=Some(handles);self.state.preview_retention=Some(owner);return Err(error)}owner.phase=7;}
            else{owner.removed=Some(ControlledRetirement::new(SessionRetentionRow::Roster(handles)).unwrap_or_else(|_|unreachable!("original roster declares retirement")));owner.phase=8;}
        }else {
            let slots=match owner.phase {0=>self.preview_mesh_pack_by_handle.slot_count(),1=>self.preview_diagnostics_by_handle.slot_count(),2=>self.pending_tessellate_by_hash.slot_count(),3=>self.tessellate_progress_by_hash.slot_count(),4=>self.tessellate_chunks_by_hash.slot_count(),5=>self.tessellate_handle_by_hash.slot_count(),_=>unreachable!()};
            if owner.slot>=slots{owner.phase+=1;owner.slot=0;owner.comparison=0;owner.matched=false;}
            else if !owner.matched&&owner.comparison<owner.roster.as_ref().unwrap().len(){owner.matched=self.preview_retention_handle(&owner).is_some_and(|handle|handle==owner.roster.as_ref().unwrap()[owner.comparison]);owner.comparison+=1;}
            else{
                let keep=owner.matched;let row=match owner.phase {0=>self.preview_mesh_pack_by_handle.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Mesh),1=>self.preview_diagnostics_by_handle.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Mesh),2=>self.pending_tessellate_by_hash.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Pending),3=>self.tessellate_progress_by_hash.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Progress),4=>self.tessellate_chunks_by_hash.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Pending),5=>self.tessellate_handle_by_hash.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Pending),_=>unreachable!()};
                if let Some(row)=row{owner.removed=Some(ControlledRetirement::new(row).unwrap_or_else(|_|unreachable!("original preview row declares retirement")));}
                owner.slot+=1;owner.comparison=0;owner.matched=false;
            }
        }
        self.state.preview_retention=Some(owner);self.state.preview_retention_progress=progress;Ok(RetainedCloneStep::Progress(progress))
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

    /// 🛑️ Records cancellation while retaining every original source until its admitted turn.
    pub fn cancel_preview_evaluation(&mut self,_window_id:&str,grant:RetainedCloneGrant)->Result<(usize,RetainedCloneProgress),ValueError>{
        if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original preview session is closing"))}
        if self.preview_cancelled{return Ok((0,Default::default()))}
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"preview cancellation requires an admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"preview cancellation requires admitted depth"))}
        let count=self.state.pending_tessellate_by_hash.len();
        self.state.preview_cancelled=true;self.state.tick_scheduled=false;self.state.pending_host_cancelled=self.state.pending_host.is_some();self.state.preview_cancellation_cursor=Some(0);self.state.preview_cancellation_phase=0;
        Ok((count,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }

    pub fn next_preview_cancellation_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.state.preview_cancellation_cursor.is_none(){return Ok(Default::default())}
        match self.state.preview_cancellation_phase{
            0=>self.retirement.admission_demands(),
            4=>self.geometry_port.as_ref().map_or(Ok(Default::default()),|port|Ok(RetirementDemand{copy_bytes:port.next_cancel_copy_byte_demand()?,capacity_bytes:port.next_cancel_capacity_byte_demand(copy)?,release_bytes:port.next_cancel_release_byte_demand()?,depth:port.next_cancel_depth_demand()?})),
            5=>self.retirement.demands(copy),
            _=>Ok(RetirementDemand{depth:1,..Default::default()}),
        }
    }

    /// ⏱️ Hands off exact sources, resets one actual row, or advances the original geometry cancellation.
    pub fn preview_cancellation_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.state.preview_cancellation_progress=Default::default();
        let Some(cursor)=self.state.preview_cancellation_cursor else{return Ok(RetainedCloneStep::Complete(Default::default()))};
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let demand=self.next_preview_cancellation_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"preview cancellation requires its original cursor depth"))}
        if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        match self.state.preview_cancellation_phase{
            0=>{
                self.retirement.preflight(grant)?;
                let original=SessionPreviewCancellationSources::take(&mut self.state);
                match self.retirement.admit(SessionCollectionOwner::Cancellation(original),grant){Ok(receipt)=>progress=receipt,Err((error,SessionCollectionOwner::Cancellation(original)))=>{original.restore(&mut self.state);return Err(error)},Err(_)=>unreachable!("original cancellation source changed")}
                self.state.preview_cancellation_phase=1;
            },
            1=>{
                if cursor<self.state.tessellate_progress_by_hash.slot_count(){if let Some((_,row))=self.state.tessellate_progress_by_hash.slot_entry_mut(cursor){if row.phase!=PreviewTessellatePhase::Complete{row.phase=PreviewTessellatePhase::Cancelled;}row.next_chunk=0;row.chunks=0;}self.state.preview_cancellation_cursor=Some(cursor+1);}
                else{self.state.preview_cancellation_phase=2;self.state.preview_cancellation_cursor=Some(0);}
            },
            2=>{
                if cursor<self.state.window_tick_latches.slot_count(){if let Some((_,row))=self.state.window_tick_latches.slot_entry_mut(cursor){*row=FlowEvalWindowTickLatch::default();}self.state.preview_cancellation_cursor=Some(cursor+1);}
                else{self.state.preview_cancellation_phase=3;}
            },
            3=>{if let Some(port)=self.geometry_port.as_mut(){port.begin_cancel()?;self.state.preview_cancellation_phase=4;}else{self.state.preview_cancellation_phase=5;}},
            4=>{
                let port=self.geometry_port.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original cancelling geometry source disappeared"))?;
                let result=port.cancel_step(grant);progress=port.cancel_step_progress();self.state.preview_cancellation_progress=progress;let step=result.map_err(|error|error.with_retained_progress(progress))?;
                semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,port.cancel_terminal_is_empty(),"Flow session original geometry cancellation")?;
                if port.cancel_terminal_is_empty(){self.state.preview_cancellation_phase=5;}
            },
            5=>{let result=self.retirement.step(grant);progress=self.retirement.step_progress();self.state.preview_cancellation_progress=progress;let step=result?;progress=step.progress();if self.retirement.terminal_is_empty(){self.state.preview_cancellation_cursor=None;}},
            _=>unreachable!("original preview cancellation phase"),
        }
        self.state.preview_cancellation_progress=progress;
        Ok(if self.preview_cancellation_terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    pub fn preview_cancellation_step_progress(&self)->RetainedCloneProgress{self.state.preview_cancellation_progress}

    pub fn preview_cancellation_terminal_is_empty(&self)->bool{self.state.preview_cancellation_cursor.is_none()}

    /// 🛑 The `tessellateCancel` request body the cancelling command sends to the geometry extension.
    /// Deliberately addresses NO handle: the session keys its ledger by `nodeHash`, which is a
    /// one-way digest of `(handle, tolerance bits)`, so the tolerance half of the extension's job key
    /// is not recoverable here — and a cancel that can only retire SOME of the jobs it means to
    /// retire is worse than the whole-registry branch the capability already offers
    /// (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`'s `tessellateCancel` handler).
    pub fn preview_cancel_invocation_request_json(window_id: &str, window_kind_id: &str) -> String {
        format!("{{\"windowId\":{},\"windowKindId\":{}}}", semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::String(window_id.to_string())), semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::String(window_kind_id.to_string())))
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
        let Ok(envelope) = semio_framework_pack_json::parse(envelope_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            self.eval_progress_by_hash.remove(&node_hash);
            return PreviewEvalOutcome::Complete { output_json: envelope_json.to_string() };
        };
        let Some(done) = envelope.get("done").and_then(semio_framework_pack_json::Value::as_bool) else {
            self.eval_progress_by_hash.remove(&node_hash);
            return PreviewEvalOutcome::Complete { output_json: envelope_json.to_string() };
        };
        let phase = PreviewEvalPhase::from_job_tag(envelope.get("phase").and_then(semio_framework_pack_json::Value::as_str).unwrap_or("computing"));
        let units_done = envelope.get("unitsDone").and_then(semio_framework_pack_json::Value::as_f64).unwrap_or(0.0).max(0.0) as u32;
        let units_total = envelope.get("unitsTotal").and_then(semio_framework_pack_json::Value::as_f64).unwrap_or(0.0).max(0.0) as u32;
        if !done {
            self.eval_progress_by_hash.insert(node_hash, PreviewEvalProgress { units_done, units_total, phase });
            return PreviewEvalOutcome::Working;
        }
        self.eval_progress_by_hash.remove(&node_hash);
        if matches!(phase, PreviewEvalPhase::Cancelled) {
            return PreviewEvalOutcome::Cancelled;
        }
        PreviewEvalOutcome::Complete { output_json: envelope.get("outputJson").and_then(semio_framework_pack_json::Value::as_str).unwrap_or_default().to_string() }
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

    /// ⛓️ The CHAIN's own progress — the ledger that is live for the WHOLE of an evaluation, which
    /// neither finer ledger is.
    ///
    /// 🩸️ [`FlowEvalSession::preview_tessellate_status`] reads `pending_tessellate_by_hash` and
    /// [`FlowEvalSession::preview_eval_status`] reads `eval_progress_by_hash`. Both are empty at
    /// every hop boundary of a `flowEvalTick` chain: a tessellation is admitted and answered inside
    /// one hop, and a budgeted evaluation only lands in its ledger once an extension has already
    /// answered `done: false`. An `evaluate` round trip parked at the geometry extension — which is
    /// where a boolean preview spends its whole slow half — is recorded in NEITHER. So every status
    /// a preview window published between hops was byte-identical `phase: "idle", inFlight: 0,
    /// ratio: 1.0` while the kernel was busy, on both renderers: 54 identical publications across a
    /// 23 s evaluation, no pill text, no progress, nothing to read
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md`).
    ///
    /// ⛓️ What IS live throughout is the per-window tick latch — `armed` (a hop is outstanding),
    /// `in_flight` (an extension answer is owed), `owed` (an answer asked for a continuation) and
    /// `unfinished` (the window's own last tick said there is more) — plus `tick_scheduled`. The
    /// monotone denominator is the session's own published per-node census: a node leaves
    /// `queued`/`computing` exactly once per chain and never returns to it, so
    /// `nodes_done / nodes_total` only ever grows while one evaluation runs.
    pub fn preview_chain_status(&self) -> PreviewChainStatus {
        let in_flight: u32 = self.window_tick_latches.values().map(|latch| latch.in_flight).sum();
        let working = self.tick_scheduled || self.window_tick_latches.values().any(|latch| latch.armed || latch.owed || latch.unfinished || latch.in_flight > 0);
        let mut status = PreviewChainStatus { in_flight, working, ..PreviewChainStatus::default() };
        if let Some(widgets) = semio_framework_pack_json::parse(&self.status_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_object().cloned()) {
            for (_, entry) in widgets.iter() {
                status.nodes_total = status.nodes_total.saturating_add(1);
                if !matches!(entry.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("queued" | "computing")) {
                    status.nodes_done = status.nodes_done.saturating_add(1);
                }
            }
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
        let Ok(envelope) = semio_framework_pack_json::parse(output_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            return PreviewTessellateOutcome::Failed;
        };
        let phase = envelope.get("phase").and_then(|value| value.as_str()).unwrap_or_default();
        let uint = |key: &str| envelope.get(key).and_then(|value| value.as_f64()).unwrap_or_default().max(0.0) as u32;
        let mut progress = PreviewTessellateProgress { units_done: uint("unitsDone"), units_total: uint("unitsTotal"), faces_done: uint("facesDone"), faces_total: uint("facesTotal"), phase: PreviewTessellatePhase::from_tag(phase), next_chunk: 0, chunks: uint("chunks") };
        match progress.phase {
            PreviewTessellatePhase::Invalid => {
                let diagnostics = envelope.get("diagnostics").cloned().unwrap_or(semio_framework_pack_json::Value::Array(Vec::new()));
                self.preview_diagnostics_by_handle.insert(handle, semio_framework_pack_json::to_string(&diagnostics));
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

    /// 🧹️ Marks the original session closed without releasing or replacing any source.
    pub fn begin_close(&mut self){self.closing=true;self.tick_scheduled=false;}

    fn owns_session_source(&self)->bool{
        let state=&*self.state;
        state.operator_registry.is_some()||state.neural_cache.is_some()||state.previous_snapshot.is_some()||state.previous_channels.is_some()||state.eval_json.is_some()
            ||state.painted_eval_json.capacity()!=0||state.converged_eval_json.capacity()!=0||state.status_json.capacity()!=0||state.extension_evaluate_fault.is_some()
            ||state.preview_retention.is_some()||!state.window_tick_latches.terminal_is_empty()||!state.live_geometry_handles.terminal_is_empty()||!state.preview_mesh_pack_by_handle.terminal_is_empty()
            ||!state.pending_tessellate_by_hash.terminal_is_empty()||!state.tessellate_handle_by_hash.terminal_is_empty()||!state.tessellate_progress_by_hash.terminal_is_empty()
            ||!state.eval_progress_by_hash.terminal_is_empty()||!state.tessellate_chunks_by_hash.terminal_is_empty()||!state.preview_diagnostics_by_handle.terminal_is_empty()
    }

    fn take_session_source(&mut self)->Option<SessionCollectionOwner>{
        let state=&mut*self.state;
        if let Some(value)=state.preview_retention.take(){return Some(SessionCollectionOwner::Retention(value))}
        if let Some(value)=state.operator_registry.take(){return Some(SessionCollectionOwner::Registry(value))}
        if !state.window_tick_latches.terminal_is_empty(){return Some(SessionCollectionOwner::Latches(std::mem::take(&mut state.window_tick_latches)))}
        if !state.preview_mesh_pack_by_handle.terminal_is_empty(){return Some(SessionCollectionOwner::Meshes(std::mem::take(&mut state.preview_mesh_pack_by_handle)))}
        if !state.preview_diagnostics_by_handle.terminal_is_empty(){return Some(SessionCollectionOwner::Meshes(std::mem::take(&mut state.preview_diagnostics_by_handle)))}
        if !state.tessellate_chunks_by_hash.terminal_is_empty(){return Some(SessionCollectionOwner::Pending(std::mem::take(&mut state.tessellate_chunks_by_hash)))}
        if !state.tessellate_handle_by_hash.terminal_is_empty(){return Some(SessionCollectionOwner::Pending(std::mem::take(&mut state.tessellate_handle_by_hash)))}
        if !state.tessellate_progress_by_hash.terminal_is_empty(){return Some(SessionCollectionOwner::Tessellation(std::mem::take(&mut state.tessellate_progress_by_hash)))}
        if !state.eval_progress_by_hash.terminal_is_empty(){return Some(SessionCollectionOwner::Evaluation(std::mem::take(&mut state.eval_progress_by_hash)))}
        if !state.pending_tessellate_by_hash.terminal_is_empty(){return Some(SessionCollectionOwner::Pending(std::mem::take(&mut state.pending_tessellate_by_hash)))}
        if !state.live_geometry_handles.terminal_is_empty(){return Some(SessionCollectionOwner::Handles(std::mem::take(&mut state.live_geometry_handles)))}
        if let Some(value)=state.previous_snapshot.take(){return Some(SessionCollectionOwner::Snapshot(value))}
        if let Some(value)=state.previous_channels.take(){return Some(SessionCollectionOwner::Channels(value))}
        if let Some(value)=state.eval_json.take(){return Some(SessionCollectionOwner::SourceLease(value))}
        for text in [&mut state.painted_eval_json,&mut state.converged_eval_json,&mut state.status_json]{if text.capacity()!=0{return Some(SessionCollectionOwner::Text(std::mem::take(text)))}}
        if let Some(value)=state.extension_evaluate_fault.take(){return Some(SessionCollectionOwner::Fault(value))}
        state.neural_cache.take().map(SessionCollectionOwner::Cache)
    }

    fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(host)=self.state.retiring_host.as_ref(){return Ok(RetirementDemand{copy_bytes:host.next_close_copy_byte_demand()?,capacity_bytes:host.next_close_capacity_byte_demand(copy)?,release_bytes:host.next_close_release_byte_demand()?,depth:host.next_close_depth_demand()?})}
        if self.state.pending_host.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(port)=self.retiring_geometry_port.as_ref(){return Ok(RetirementDemand{copy_bytes:port.next_copy_byte_demand()?,capacity_bytes:port.next_capacity_byte_demand(copy)?,release_bytes:port.next_release_byte_demand()?,depth:port.next_depth_demand()?})}
        if !self.retirement.terminal_is_empty(){return self.retirement.demands(copy)}
        Ok(RetirementDemand{depth:usize::from(self.geometry_port.is_some()||self.owns_session_source()),..Default::default()})
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.close_demands(copy)?.capacity_bytes)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}

    /// 🎟️ Quotes a new original retirement source handoff without allocating its queue or frame.
    pub fn next_retirement_admission_demands(&self)->Result<RetirementDemand,ValueError>{self.retirement.admission_demands()}
    pub fn next_retirement_reserve_capacity_byte_demand(&self)->Result<usize,ValueError>{self.retirement.owners.next_reserve_capacity_byte_demand()}
    pub fn reserve_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.retirement.owners.reserve_step(grant)}
    pub fn next_retirement_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement.demands(0)?.copy_bytes)}
    pub fn next_retirement_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.retirement.demands(copy)?.capacity_bytes)}
    pub fn next_retirement_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement.demands(0)?.release_bytes)}
    pub fn next_retirement_depth_demand(&self)->Result<usize,ValueError>{Ok(self.retirement.demands(0)?.depth)}
    pub fn retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.retirement.step(grant)}

    pub fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        let empty=RetainedCloneProgress::default();
        if !self.closing{return Step::Blocked}
        if grant.maximum_items==0{return Step::Pending{progress:empty}}
        let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(value)=>value,Err(error)=>return Step::Refused{kind:error.kind,progress:error.retained_progress()}};
        if grant.maximum_depth<demand.depth{return Step::Refused{kind:ValueRefusalKind::DepthLimit,progress:Default::default()}}
        if let Some(host)=self.state.retiring_host.as_mut(){
            let result=host.close_step(grant).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,host.terminal_is_empty(),"Flow session original pending Host"));
            if host.terminal_is_empty(){self.state.retiring_host=None;}
            return match result{Ok(step)=>if self.terminal_is_empty(){Step::Complete{progress:step.progress()}}else{Step::Pending{progress:step.progress()}},Err(error)=>Step::Refused{kind:error.kind,progress:error.retained_progress()}}
        }
        if let Some(host)=self.state.pending_host.take(){self.state.retiring_host=Some(FlowHostRetirement::new(host));return Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}}}
        if let Some(port)=self.retiring_geometry_port.as_mut(){
            let result=port.close_step(grant).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,port.terminal_is_empty(),"Flow session original geometry port"));
            if port.terminal_is_empty(){self.retiring_geometry_port=None;}
            return match result{Ok(step)=>Step::Pending{progress:step.progress()},Err(error)=>Step::Refused{kind:error.kind,progress:error.retained_progress()}}
        }
        if !self.retirement.terminal_is_empty(){
            return match self.retirement.step(grant){Ok(step)=>if self.terminal_is_empty(){Step::Complete{progress:step.progress()}}else{Step::Pending{progress:step.progress()}},Err(error)=>Step::Refused{kind:error.kind,progress:error.retained_progress()}}
        }
        if let Some(port)=self.geometry_port.take(){self.retiring_geometry_port=Some(crate::geometry::GeometryPortRetirement::new(port));return Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}}}
        if let Some(source)=self.take_session_source(){
            self.retirement.root=Some(ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!("declared original session source root supports controlled retirement")));
            return Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}}
        }
        Step::Complete{progress:empty}
    }

    /// 🧊️ Pays each original close currency explicitly at the existing cold disposal boundary.
    pub fn retire_cold(mut self){
        self.begin_close();
        for _ in 0..FLOW_EVAL_SESSION_COLD_CLOSE_STEPS{
            if self.terminal_is_empty(){return}
            let copy=self.next_close_copy_byte_demand().expect("cold session original copy demand").max(4096);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_close_capacity_byte_demand(copy).expect("cold session original capacity demand"),maximum_release_bytes:self.next_close_release_byte_demand().expect("cold session original release demand"),maximum_depth:self.next_close_depth_demand().expect("cold session original depth demand")};
            if let semio_framework_job::InteractiveJobCloseStep::Refused{kind,..}=self.close_step(grant){panic!("cold session original owner refused: {kind:?}")}
        }
        assert!(self.terminal_is_empty(),"cold session disposal did not reach original terminal ownership");
    }

    pub fn terminal_is_empty(&self)->bool{self.closing&&self.state.pending_host.is_none()&&self.state.retiring_host.is_none()&&self.geometry_port.is_none()&&self.retiring_geometry_port.is_none()&&!self.owns_session_source()&&self.retirement.terminal_is_empty()}
}

/// 🧊 True when a base64 `pack` mesh body carries something paintable — decoded once, structurally,
/// never by re-parsing prose.
fn preview_mesh_pack_has_geometry(base64_body: &str) -> bool {
    let Ok(bytes) = crate::mesh::decode_base64(base64_body) else {
        return false;
    };
    let Ok(mesh) = crate::mesh::decode_mesh_pack(&bytes) else {
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

semio_framework_value::artifact_retire_leaf!(PreviewEvalProgress);

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

/// ⛓️ The whole preview evaluation chain's state, as the status object reports it when no finer
/// ledger has anything to say — see [`FlowEvalSession::preview_chain_status`] for why the finer two
/// are structurally silent at every hop boundary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreviewChainStatus {
    /// 📈 Nodes this chain has settled (anything but `queued`/`computing`/`stale`).
    pub nodes_done: u32,
    /// 📈 Nodes the published census knows about at all.
    pub nodes_total: u32,
    /// ⏳️ Extension answers outstanding across every attached preview window.
    pub in_flight: u32,
    /// ⏳️ Whether ANY window still owes, awaits or is running a hop.
    pub working: bool,
}

impl PreviewChainStatus {
    /// 📈 The units this chain owes, as `(done, total)` — the pair a surface publishes and a pill
    /// prints as `n/m`.
    ///
    /// ⚖️ The denominator is the node census, raised to `done + 1` while the chain is still working:
    /// a census that has seen every node settle can still be waiting on the answer that will produce
    /// the mesh, and `4/4` there would say "done" about work a cancel could still stop — the same
    /// rule [`PreviewTessellateStatus::ratio`] applies to a mesh body still crossing in chunks.
    ///
    /// 🩸️ The outstanding round trips are deliberately NOT in the denominator. Counting them made it
    /// oscillate with every park and fold — `2/3` at a parked request, `2/4` the moment it settled —
    /// so the published ratio went BACKWARDS mid-evaluation, which is worse than no progress at all.
    /// The census is the one quantity that only ever grows inside one chain.
    pub fn units(&self) -> (u32, u32) {
        if !self.working {
            return (self.nodes_total, self.nodes_total);
        }
        (self.nodes_done, self.nodes_total.max(self.nodes_done.saturating_add(1)))
    }

    /// 🏁️ Whether this chain has SETTLED: it owes, awaits and runs nothing, and its own census
    /// accounts for every node it published.
    ///
    /// ⚖️ This is the guest's OWN answer to "is the evaluation over", and it outranks the run view on
    /// that question. A `ToolRunView` is a host artifact that reaches a surface on a render, so it
    /// LAGS — and a run whose job has finished but whose terminal state has not been rendered back
    /// yet kept `computing` and the `toolRunAbort` affordance raised for good: a spinner that
    /// outlives its work and a Cancel button for an evaluation with nothing left to cancel
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️flow-inline-continuation-2026-09-14.md`).
    ///
    /// 🚦️ A census of ZERO nodes is deliberately NOT settled. Before the first hop publishes one
    /// there is nothing to be done with, and that is exactly the window in which the run legitimately
    /// knows more than the chain — it was started by a gesture whose first tick has not run yet.
    pub fn settled(&self) -> bool {
        !self.working && self.nodes_total > 0 && self.nodes_done >= self.nodes_total
    }

    /// 📈 Fraction of the chain's units already done, in `[0, 1]`. A working chain never reports `1`:
    /// "done" is the one answer a live evaluation may not give.
    pub fn ratio(&self) -> f64 {
        let (done, total) = self.units();
        if total == 0 {
            return 1.0;
        }
        (f64::from(done) / f64::from(total)).clamp(0.0, 1.0)
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

semio_framework_value::artifact_retire_leaf!(PreviewTessellateProgress);

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
    let evaluated = (!session.painted_eval_json().is_empty()).then(|| session.painted_eval_json());
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
pub fn flow_host_with_session(mut host:FlowHost,session:&FlowEvalSession,grant:RetainedCloneGrant)->Result<(FlowHost,RetainedCloneProgress),(ValueError,FlowHost)>{
    match session.install_baseline_into(&mut host,grant){
        Ok(progress)=>{if let Some(registry)=session.operator_registry.as_ref(){host.operator_registry=Some(registry.clone());}Ok((host,progress))},
        Err(error)=>Err((error,host)),
    }
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
pub fn unserved_flow_operator_kinds(host_snapshot: &FlowHostSnapshot, registry: &neural::Registry) -> Vec<String> {
    let mut unserved = BTreeSet::new();
    for widget in &host_snapshot.widgets {
        if let Widget::Neuron { neuron_kind, .. } = widget {
            if registry.operator(neuron_kind).is_none() {
                unserved.insert(neuron_kind.clone());
            }
        }
    }
    unserved.into_iter().collect()
}

fn node_eval_status_json(status: &NodeEvalStatus) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(status))
}

/// 📊️ The per-node census one evaluation publishes — the ONE quantity that only ever grows inside a
/// chain, and therefore the only honest denominator a progress ratio may use
/// ([`FlowEvalSession::preview_chain_status`]).
///
/// 🌊️ `Computing` is EVERY node whose request is outstanding at its plugin right now, not the head
/// of `remaining`: a coalesced tick parks a whole topological wave, and naming one of them would
/// leave the rest of a live wave painted as merely queued.
///
/// 📈 A dirty node the walk has already passed is `Ok`, never `Stale`. `dirty` is measured against
/// the chain's FROZEN baseline — it advances only when the chain completes — so every node this
/// evaluation recomputed stays in it until the very last hop. Calling those nodes stale held
/// `nodes_done` at the count of untouched nodes for the whole evaluation: monotone, and flat at the
/// same fraction from the first hop to the last, which is a progress bar that never moves
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A node leaves `remaining` exactly once per chain and
/// never returns to it, which is what makes the census monotone.
/// 🖼️ `live` with every CURRENT widget it did not ANSWER filled in from `converged`.
///
/// ⚖️ An answered node has an output or a fault. `build_channel_eval_json` writes a row for every
/// widget of the document whether the walk reached it or not, so the mark of a node whose request is
/// still crossing is an EMPTY `out` with no `error` beside it — while the last converged walk did
/// produce one. A node the walk answered, including one it answered with an `error`, keeps its live
/// answer, so this can never paint over a real failure; and a node the document no longer declares is
/// never filled in at all, so switching example replaces the graph instead of painting the previous
/// example's leftovers over the new one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn merge_unanswered_eval_entries(live: &str, converged: &str, host_snapshot: &FlowHostSnapshot) -> String {
    if converged.is_empty() || live.is_empty() || is_global_eval_error_json(live) {
        return live.to_string();
    }
    let (Ok(live_value), Ok(converged_value)) = (semio_framework_pack_json::parse(live, semio_framework_pack_json::JsonMemberPolicy::Reject), semio_framework_pack_json::parse(converged, semio_framework_pack_json::JsonMemberPolicy::Reject)) else {
        return live.to_string();
    };
    let (Some(live_map), Some(converged_map)) = (live_value.as_object(), converged_value.as_object()) else {
        return live.to_string();
    };
    let mut merged = live_map.clone();
    let mut filled = false;
    for widget in &host_snapshot.widgets {
        let id = widget_id_for(widget);
        let Some(entry) = converged_map.get(id) else { continue };
        if !eval_entry_is_unanswered(live_map.get(id)) || eval_entry_is_unanswered(Some(entry)) {
            continue;
        }
        merged.insert(id.to_string(), entry.clone());
        filled = true;
    }
    if !filled {
        return live.to_string();
    }
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(merged))
}

/// 🖼️ Whether one evaluation row carries NO answer at all — no output channel and no fault. The row
/// a budgeted walk leaves behind for a node whose extension request it parked.
fn eval_entry_is_unanswered(entry: Option<&semio_framework_pack_json::Value>) -> bool {
    let Some(entry) = entry else { return true };
    if entry.get("error").is_some() {
        return false;
    }
    entry.get("out").and_then(semio_framework_pack_json::Value::as_object).is_none_or(|out| out.iter().next().is_none())
}

fn build_flow_status_json(host: &FlowHost, remaining: &[String]) -> String {
    let eval = semio_framework_pack_json::parse(&host.last_eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_pack_json::Value::Object(semio_framework_pack_json::Object::new()));
    let tree = host.build_tree_for_status();
    let seeds = host.build_seeds_for_status();
    let wave: BTreeSet<&str> = host.pending_extension_evals.iter().map(|pending| pending.neuron_id.as_str()).collect();
    let active = remaining.first().map(String::as_str);
    let mut widgets = semio_framework_pack_json::Object::new();
    for widget in &host.host_snapshot.widgets {
        let id = widget_id_for(widget);
        if matches!(widget, Widget::InputSlider { .. } | Widget::InputNote { .. } | Widget::InputImage { .. } | Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } | Widget::Cluster { .. }) {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Ok));
            continue;
        }
        if let Some(entry) = eval.get(id) {
            if let Some(message) = entry.get("error").and_then(semio_framework_pack_json::Value::as_str) {
                widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Error { message: message.to_string() }));
                continue;
            }
        }
        let blocked = host.widget_blocked_ports(id);
        if !blocked.is_empty() {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Blocked { ports: blocked }));
            continue;
        }
        if wave.contains(id) || (wave.is_empty() && active == Some(id)) {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Computing));
            continue;
        }
        if remaining.iter().any(|entry| entry == id) {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Queued));
            continue;
        }
        widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Ok));
    }
    tree.retire_cold();
    seeds.retire_cold();
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(widgets))
}
// #endregion 🔖️EvalSession

fn dedupe_host_snapshot_widgets(host_snapshot: &mut FlowHostSnapshot) {
    let mut seen = BTreeSet::new();
    host_snapshot.widgets.retain(|widget| seen.insert(widget_id_for(widget).to_string()));
}



/// 🔌️ Which side of a node a port sits on — the ONE thing that decides whether a port id is looked
/// up in a widget's inputs or its outputs, since `"{nodeId}@{portId}"` carries no direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSide {
    Input,
    Output,
}

/// 🔤️ The value schemas one endpoint declares, read straight off the port the operator catalogue
/// published — an untyped declared port answers with an empty list; connection admission separately checks exact endpoint identity.
pub fn widget_port_value_types(widget_id: &str, port_id: &str, side: PortSide, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> Vec<String> {
    let Some(widget) = widgets.iter().find(|widget| widget_id_for(widget) == widget_id) else {
        return Vec::new();
    };
    let (inputs, outputs, _, _) = widget_io_ports(widget, synapses, kind_infos);
    let ports = if side == PortSide::Output { outputs } else { inputs };
    ports.iter().find(|port| port.id == port_id).and_then(|port| port.value_type.clone()).map(|declared| declared.split(',').filter(|entry| !entry.is_empty()).map(str::to_string).collect()).unwrap_or_default()
}

/// 🔌️ The ONE port-compatibility rule the flow graph enforces, over the value schemas both ends
/// declare: a pair is refused only when both sides declare and the sets are disjoint.
///
/// @see `🧫️fixtures/🔌️port-types/🔣️.json` — the fixture that owns the pairs
/// @see `neural_engine::Registry::channel_compatible` — the same rule over two `ChannelSpec`s
pub fn port_value_types_compatible(source: &[String], target: &[String]) -> bool {
    if source.is_empty() || target.is_empty() {
        return true;
    }
    source.iter().any(|provided| target.iter().any(|accepted| accepted == provided))
}

fn widget_has_declared_port(widget_id: &str, port_id: &str, side: PortSide, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> bool {
    widgets.iter().find(|widget| widget_id_for(widget) == widget_id).is_some_and(|widget| {
        let (inputs, outputs, _, _) = widget_io_ports(widget, synapses, kind_infos);
        (if side == PortSide::Output { outputs } else { inputs }).iter().any(|port| port.id == port_id)
    })
}

fn widget_has_output(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> bool {
    widgets.iter().any(|w| widget_id_for(w) == widget_id && !widget_io_ports(w, synapses, kind_infos).1.is_empty())
}

/// 🔌️ Maps a synapse endpoint onto the `IoPortSpec.id` the board keys its handles with.
/// An exact id is kept. Legacy `out` / `in` and an empty id select the first port on that side.
/// A missing widget or an unknown port stays unresolved so the canvas draws no wire.
fn resolve_synapse_port(widget_id: &str, port_id: &str, side: PortSide, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> Option<String> {
    let widget = widgets.iter().find(|widget| widget_id_for(widget) == widget_id)?;
    let (inputs, outputs, _, _) = widget_io_ports(widget, synapses, kind_infos);
    let ports: Vec<String> = match side {
        PortSide::Output => outputs.iter().map(|port| port.id.clone()).collect(),
        PortSide::Input if inputs.is_empty() && matches!(widget, Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. }) => vec![String::new()],
        PortSide::Input => inputs.iter().map(|port| port.id.clone()).collect(),
    };
    if ports.iter().any(|id| id == port_id) {
        return Some(port_id.to_string());
    }
    let legacy = matches!((side, port_id), (PortSide::Output, "" | "out") | (PortSide::Input, "" | "in"));
    if legacy { ports.first().cloned() } else { None }
}

fn first_output_port(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> String {
    widgets.iter().find(|w| widget_id_for(w) == widget_id).and_then(|w| widget_io_ports(w, synapses, kind_infos).1.first().map(|port| port.id.clone())).unwrap_or_default()
}

fn first_input_port(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> String {
    widgets
        .iter()
        .find(|w| widget_id_for(w) == widget_id)
        .map(|w| match w {
            Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } => String::new(),
            _ => widget_io_ports(w, synapses, kind_infos).0.first().map(|port| port.id.clone()).unwrap_or_default(),
        })
        .unwrap_or_default()
}

fn widget_has_input(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> bool {
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

fn widget_node_size(widget: &Widget, synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> (f64, f64) {
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

#[path = "🚪️io/🦀️.rs"]
pub mod io;
