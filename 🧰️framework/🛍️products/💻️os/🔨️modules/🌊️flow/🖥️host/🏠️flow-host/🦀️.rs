//! 🏠️ The retained `FlowHost` value: wheel plan, baseline publication custody and every field of the host.

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct FlowWheelPlan {
    pub(super) revision: u64,
    pub expected: [f64; 3],
    pub(super) next: [f64; 3],
}

impl FlowWheelPlan {
    pub fn camera(&self) -> [f64; 3] {
        self.next
    }
}

pub struct FlowPublicationShared<T:RetireOwned+Sync>(pub Arc<T>);
impl<T:RetireOwned+Sync> RetireOwned for FlowPublicationShared<T> {
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(semio_framework_value::retirement::shared::SharedControlledRetirement::lease(self.0))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<T>())}
    fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct FlowBaselinePublication {
    pub(super) revision:u64,
    pub display:Option<FlowDisplayCursor>,
    pub snapshot:Option<TreeSnapshot>,
    pub channels:Option<EvalChannels>,
    pub(super) shared_snapshot:Option<FlowPublicationShared<TreeSnapshot>>,
    pub(super) shared_channels:Option<FlowPublicationShared<EvalChannels>>,
    pub(super) generation:u64,
    pub(super) converged:bool,
    pub(super) pending:Vec<neural::PendingExtensionEval>,
    pub(super) remaining:Vec<String>,
}

pub struct FlowBaselineLeases {
    pub snapshot:Option<Arc<TreeSnapshot>>,
    pub channels:Option<Arc<EvalChannels>>,
    pub current:Option<Arc<EvalChannels>>,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct FlowPublicationDisplaced {
    pub(super) publication:Option<FlowBaselinePublication>,
    pub(super) text:Option<String>,
    pub display:Option<FlowDisplayCursor>,
    pub exports:Option<HistoryFoldIndex<String,Dictionary>>,
    pub(super) pending:Vec<neural::PendingExtensionEval>,
    pub(super) remaining:Vec<String>,
    pub snapshot:Option<TreeSnapshot>,
    pub(super) leases:FlowBaselineLeases,
}

impl RetireOwned for FlowBaselineLeases {
    fn retirement(self)->Box<dyn RetirementCursor>{
        use semio_framework_value::retirement::{RetireOwned,sequence,shared::SharedControlledRetirement};
        let snapshot:Box<dyn RetirementCursor>=match self.snapshot{Some(source)=>Box::new(SharedControlledRetirement::lease(source)),None=>().retirement()};
        let channels:Box<dyn RetirementCursor>=match self.channels{Some(source)=>Box::new(SharedControlledRetirement::lease(source)),None=>().retirement()};
        let current:Box<dyn RetirementCursor>=match self.current{Some(source)=>Box::new(SharedControlledRetirement::lease(source)),None=>().retirement()};
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
    pub geometry_port: Option<Box<dyn semio_framework_os_flow_geometry::GeometryPort>>,
    pub operator_registry: Option<neural::SharedRegistry>,
    pub host_snapshot: FlowHostSnapshot,
    pub dag: DagHost,
    pub current_channels: Option<Arc<EvalChannels>>,
    pub(super) export_payloads: HistoryFoldIndex<String, Dictionary>,
    pub last_eval_json: String,
    pub host_catalogue_json: String,
    /// 🧠️ The operator catalogue this host indexes, SHARED with every other live host: it is a pure
    /// projection of the extension registry, ~108 kB of it, and an evaluation tick that rebuilt its
    /// own copy paid 11-26 ms per tick for a map nobody had changed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub kind_infos: Arc<HistoryFoldIndex<String, OperatorInfo>>,
    pub neural_cache: Arc<NeuralCache>,
    pub previous_snapshot: Option<Arc<TreeSnapshot>>,
    pub previous_channels: Option<Arc<EvalChannels>>,
    pub(super) pending_baseline_publication:Option<FlowBaselinePublication>,
    pub(super) baseline_retirement:Option<ControlledRetirement<FlowPublicationDisplaced>>,
    pub(super) baseline_publication_progress:RetainedCloneProgress,
    pub(super) pending_evaluation:Option<FlowEvaluationWork>,
    pub(super) evaluation_revision:u64,
    pub(super) evaluation_progress:RetainedCloneProgress,
    /// 🔢 Which replacement of the process-wide flow extension registry
    /// `previous_snapshot`/`previous_channels` were computed against. An unchanged TREE is not an
    /// unchanged EVALUATION: the operator table the tree is dispatched through is process-wide
    /// state that a contribution install replaces underneath a host, so the incremental fast path's
    /// premise ("nothing that could change the answer has changed") is false the moment the
    /// generation moves — a node that faulted `unknown kind` against the empty registry would
    /// otherwise keep its fault forever, because its tree never changed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub(super) baseline_registry_generation: u64,
    pub(super) next_widget_serial: u64,
    pub(super) next_synapse_serial: u64,
    pub viewport_w: u32,
    pub viewport_h: u32,
    pub viewport_dpr: f64,
    pub(super) pan_anchor: Option<(f64, f64, f64, f64)>,
    pub ghost_node: Option<DagNodeSpec>,
    /// ↩️ Undo/redo, backed by the standard `semio_framework_os_kernel::os_store::ArtifactStore<FlowHostSnapshot, FlowMutation>`
    /// mechanism (see the `impl FlowHost`'s `🔖️History` region) instead of a hand-rolled snapshot stack.
    pub history_store: Option<FlowStore>,
    pub(super) history_store_factory:Option<semio_framework_os_kernel::os_store::OriginalStoreGenesisFactory<FlowHostSnapshot,FlowMutation>>,
    pub(super) pending_history_baseline: Option<FlowHostSnapshot>,
    /// 🚩️ Armed by `begin_change`/`begin_gesture` for a discrete mutation not yet flushed into
    /// `history_store` — lets `can_undo` reflect it immediately, mirroring how the old snapshot stack's
    /// `begin_change` pushed synchronously instead of lazily.
    pub(super) pending_change: bool,
    /// 🧾️ The concrete flow leaves the edit in progress emitted at its gesture sites, in application order; recorded as ONE
    /// store transaction by `record_history_edit`, retired cold when discarded.
    pub pending_leaves: Vec<FlowMutation>,
    /// 🧯️ The refusal of the last history row, kept until [`FlowHost::take_history_fault`] hands it over.
    pub(super) history_fault: Option<FlowCoreError>,
    /// 🧪️ The leaves of every recorded edit, in order, so tests read what each gesture emitted.
    #[cfg(test)]
    pub recorded: Vec<Vec<FlowMutation>>,
    /// 🔗️ How many dag journal rows predated the open gesture, so only the gesture's own rows become leaves.
    pub(super) journal_mark: usize,
    /// ✏️ The note widget an inline note edit is changing, recorded as one `ChangeWidget` at `note_commit_edit`.
    pub edited_note: Option<String>,
    /// 🖐️ `true` while a coalescing gesture (drag, inline note edit) is in progress — guards
    /// `begin_change` from checkpointing mid-gesture; see `begin_gesture`/`commit_gesture_history`.
    pub(super) gesture_active: bool,
    /// 🌊️ The contributed-operator requests the last budgeted step parked — ONE topological wave,
    /// never one node: every member's inputs were ready in the same walk, so they are independent by
    /// construction and all cross to their plugins on the same hop.
    pub pending_extension_evals: Vec<neural::PendingExtensionEval>,
    pub(super) pending_remaining: Vec<String>,
    pub(super) interaction_revision: u64,
    pub(super) interaction_projection: Option<dag::DagInteractionProjection>,
    /// 🧹️ Cold owner for neural values this host DISPLACES while it is live — the previous tick's
    /// `outputs` map and `previous_channels`, and the parameter bag a `set_neuron_params` merge
    /// replaces. Those roots are SHARED with [`NeuralCache`] and with the current tick's own
    /// evaluation, so they may not be retired at the moment they are displaced; they wait here and
    /// are drained at the START of the next evaluation (`drain_displaced`) or, if none follows, when
    /// the host itself closes ([`FlowHostRetirement::new`] takes this frontier over). The frontier
    /// therefore holds at most one tick's displacement and never outlives the host
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️unit-suite-3d-2026-09-09.md` §5).
    pub(super) displaced: neural::ValueRetirement,
}

impl Default for FlowHost {
    fn default() -> Self {
        Self::from_host_snapshot(FlowHostSnapshot::default())
    }
}
