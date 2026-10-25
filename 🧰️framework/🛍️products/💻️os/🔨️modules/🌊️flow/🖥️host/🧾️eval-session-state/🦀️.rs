//! 🧾️ The flow evaluation session: tick budgets, ledgers, session state, retirement owners and the `FlowEvalSession` value.

use super::*;

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
pub struct FlowEvalWindowTickLatch {
    pub armed: bool,
    pub in_flight: u32,
    pub owed: bool,
    pub unfinished: bool,
}

semio_framework_value::artifact_retire_leaf!(FlowEvalWindowTickLatch);

/// 🔑️ The latch key for one preview window instance id. Hashed rather than retained: the latch map
/// must cost nothing to retire, and a window id is the caller's string, never the session's.
pub fn flow_eval_window_key(window_id: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    window_id.hash(&mut hasher);
    hasher.finish()
}

/// 🧵️ In-process evaluation session: neural cache, incremental baseline, eval output, and status — one per app instance, never serialized.
#[doc(hidden)]
pub struct FlowEvalSessionState {
    pub(super) retiring_geometry_port: Option<semio_framework_os_flow_geometry::GeometryPortRetirement>,
    pub geometry_port: Option<Box<dyn semio_framework_os_flow_geometry::GeometryPort>>,
    pub(super) operator_registry: Option<neural::SharedRegistry>,
    pub(super) neural_cache: Option<Arc<NeuralCache>>,
    pub(super) previous_snapshot: Option<Arc<TreeSnapshot>>,
    pub(super) previous_channels: Option<Arc<EvalChannels>>,
    pub eval_json: Option<Arc<String>>,
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
    pub painted_eval_json: String,
    /// 🖼️ The last evaluation that CONVERGED — the only honest source for a node the live walk has
    /// not reached. Advanced with the incremental baseline, under the same `remaining.is_empty()`
    /// condition and never apart from it.
    pub converged_eval_json: String,
    pub status_json: String,
    pub(super) tick_scheduled: bool,
    /// 🔒️ One arming latch per preview window this session publishes into, keyed by
    /// [`flow_eval_window_key`]. See [`FlowEvalWindowTickLatch`] — this is what makes "at most ONE
    /// pending `flowEvalTick` per (instance, window)" a fact the session owns rather than a
    /// convention every caller has to re-derive. Plain `Copy` rows keyed by hash, so retirement is
    /// a single `clear` (the `tessellate_progress_by_hash` precedent) and no window id is ever
    /// retained here.
    pub window_tick_latches: HistoryFoldIndex<u64, FlowEvalWindowTickLatch>,
    pub live_geometry_handles: HistoryFoldIndex<String, ()>,
    /// 🧊️ Canonical native Mesh16 ownership lands only through the original paid receiver.
    pub preview_mesh_by_handle: HistoryFoldIndex<String, MeshData>,
    /// ⏳ In-flight tessellate requests keyed by `nodeHash` forwarded through `InvokeExtension`. A
    /// request is removed the moment its answer is folded, even a partial one — the continuation
    /// re-admits it on the next tick.
    pub pending_tessellate_by_hash: HistoryFoldIndex<u64, String>,
    /// 🔗 The handle every admitted `nodeHash` belongs to, kept for as long as the tessellation is
    /// unfinished. This is what makes a MULTI-STEP tessellation survive `retain_preview_meshes`:
    /// its progress row and its half-received mesh body are keyed by hash, and pruning them by the
    /// (already emptied) pending table would restart the transfer from chunk zero forever.
    pub tessellate_handle_by_hash: HistoryFoldIndex<u64, String>,
    /// 📈 Progress of every tessellation this session has admitted, keyed by `nodeHash`. Plain
    /// `Copy` rows — no heap, so retirement is a single take.
    pub tessellate_progress_by_hash: HistoryFoldIndex<u64, PreviewTessellateProgress>,
    /// 🩺 Blocking validate-gate findings keyed by geometry handle, as a JSON array string.
    pub preview_diagnostics_by_handle: HistoryFoldIndex<String, String>,
    /// 🔢 Which replacement of the process-wide flow extension registry every result this session
    /// still holds was computed against. A contributed operator that no plugin had contributed yet
    /// evaluates to a fault, and the fault is CACHED — in the neural cache, in the incremental
    /// baseline and in the tessellation ledger — so a later contribution install has to be able to
    /// say "everything in here predates the registry you can now address"
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub flow_extension_generation: u64,
    /// 💥 The LAST `evaluate` answer this session was handed that faulted, so the surface publishes
    /// the fault it is actually living with instead of the addressing miss that preceded it. Cleared
    /// the moment a contribution install moves `flow_extension_generation`, and the moment any
    /// answer folds — a stale fault outranking a live evaluation is the same defect as an
    /// `extension-not-contributed` card surviving its own install
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub extension_evaluate_fault: Option<ExtensionEvaluateFault>,
    /// 🛑 Whether the user's explicit `cancelPreviewEval` gesture is the LAST thing that happened to
    /// this session's evaluation. It is not derivable from the tessellation ledger: a cancel raised
    /// while the chain was still in its `evaluate` round trips has no progress row to stamp, and a
    /// cancel of a never-admitted tessellation leaves an all-`Idle` ledger — both of which would
    /// publish `phase: "idle"` and silently swallow the gesture. Cleared by the next tick that
    /// actually begins ([`FlowEvalSession::begin_window_tick`]), so a later gesture resumes with a
    /// clean status rather than a frozen `cancelled` banner
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub preview_cancelled: bool,
    /// ⏱️ Per-node progress of the BUDGETED `evaluate` round trips — the half of the preview's work
    /// the tessellation ledger cannot see at all, because a long operator (a boolean) admits no
    /// tessellation until it has finished. Keyed by `nodeHash`, the same identity the extension
    /// resumes its retained job by (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️extension-evaluate-budget-2026-09-12.md`).
    pub eval_progress_by_hash: HistoryFoldIndex<u64, PreviewEvalProgress>,
    /// 📏️ Fingerprint of the mesh packs the preview last published — lets tessellation land without
    /// moving the evaluation text while still owing a republication
    /// (`📓️slider-mesh-supersession-fallback-2026-09-15.md`).
    pub(super) published_preview_mesh_digest: u64,
    pub baseline_geometry_pending:bool,
    pub(super) invocation_origin_growth:Option<Vec<FlowInvocationEntry>>,
    pub invocation_origins:Vec<FlowInvocationEntry>,
    pub(super) pending_tick_display:Option<SessionLivePublication>,
    pub(super) tick_display_progress:RetainedCloneProgress,
    pub(super) tick_evaluation_ready:u8,
    pub(super) pending_host:Option<FlowHost>,
    pub(super) pending_snapshot_copy:Option<Box<dyn semio_framework_job::OriginalSnapshotIntake<FlowHostSnapshot>>>,
    pub pending_tick_snapshot:Option<FlowHostSnapshot>,
    pub pending_tick_snapshot_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<FlowHostSnapshot>>,
    pub pending_tick_snapshot_origin:Option<(u64,u64)>,
    pub(super) retiring_host:Option<FlowHostRetirement>,
    pub(super) pending_host_cancelled:bool,
    pub(super) preview_cancellation_cursor:Option<usize>,
    pub(super) preview_cancellation_phase:u8,
    pub(super) preview_cancellation_progress:RetainedCloneProgress,
    pub preview_retention:Option<SessionPreviewRetention>,
    pub(super) preview_retention_progress:RetainedCloneProgress,
    pub(super) retirement: SessionRetirement,
    pub(super) closing: bool,
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
pub(super) static FLOW_EVAL_PUBLICATION_COUNTERS: [AtomicU64; 4] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];

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

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct SessionResetFields {
    invocation_growth:Option<Vec<FlowInvocationEntry>>,
    invocations:Vec<FlowInvocationEntry>,
    painted: String,
    pub(super) converged: String,
    status: String,
    pub handles: HistoryFoldIndex<String, ()>,
    meshes: HistoryFoldIndex<String, MeshData>,
    pub(super) diagnostics: HistoryFoldIndex<String, String>,
    pub(super) pending: HistoryFoldIndex<u64, String>,
    tessellate_handles: HistoryFoldIndex<u64, String>,
    tessellation: HistoryFoldIndex<u64, PreviewTessellateProgress>,
    pub(super) evaluation: HistoryFoldIndex<u64, PreviewEvalProgress>,
    pub(super) fault: Option<ExtensionEvaluateFault>,
    pub(super) latches: Option<HistoryFoldIndex<u64, FlowEvalWindowTickLatch>>,
    retention:Option<SessionPreviewRetention>,
}


pub(super) struct SessionResetSources {
    eval: Option<Arc<String>>,
    pub(super) baseline:FlowBaselineLeases,
    pub fields: SessionResetFields,
}

impl SessionResetSources {
    pub(super) fn take(state:&mut FlowEvalSessionState,reset_latches:bool)->Self {
        Self{eval:state.eval_json.take(),baseline:FlowBaselineLeases{snapshot:state.previous_snapshot.take(),channels:state.previous_channels.take(),current:None},fields:SessionResetFields{
            invocation_growth:state.invocation_origin_growth.take(),invocations:std::mem::take(&mut state.invocation_origins),
            painted:std::mem::take(&mut state.painted_eval_json),converged:std::mem::take(&mut state.converged_eval_json),status:std::mem::take(&mut state.status_json),
            handles:std::mem::take(&mut state.live_geometry_handles),meshes:std::mem::take(&mut state.preview_mesh_by_handle),diagnostics:std::mem::take(&mut state.preview_diagnostics_by_handle),
            pending:std::mem::take(&mut state.pending_tessellate_by_hash),tessellate_handles:std::mem::take(&mut state.tessellate_handle_by_hash),
            tessellation:std::mem::take(&mut state.tessellate_progress_by_hash),evaluation:std::mem::take(&mut state.eval_progress_by_hash),fault:state.extension_evaluate_fault.take(),latches:reset_latches.then(||std::mem::take(&mut state.window_tick_latches)),retention:state.preview_retention.take(),
        }}
    }
    pub(super) fn restore(self,state:&mut FlowEvalSessionState){
        let Self{eval,baseline:FlowBaselineLeases{snapshot,channels,current},fields:SessionResetFields{invocation_growth,invocations,painted,converged,status,handles,meshes,diagnostics,pending,tessellate_handles,tessellation,evaluation,fault,latches,retention}}=self;
        assert!(current.is_none(),"original session reset has no active Host channel lease");
        state.eval_json=eval;state.previous_snapshot=snapshot;state.previous_channels=channels;state.preview_retention=retention;
        state.invocation_origin_growth=invocation_growth;state.invocation_origins=invocations;
        state.painted_eval_json=painted;state.converged_eval_json=converged;state.status_json=status;
        state.live_geometry_handles=handles;state.preview_mesh_by_handle=meshes;state.preview_diagnostics_by_handle=diagnostics;
        state.pending_tessellate_by_hash=pending;state.tessellate_handle_by_hash=tessellate_handles;
        state.tessellate_progress_by_hash=tessellation;state.eval_progress_by_hash=evaluation;state.extension_evaluate_fault=fault;if let Some(latches)=latches{state.window_tick_latches=latches;}
    }
}

impl RetireOwned for SessionResetSources {
    fn retirement(self)->Box<dyn RetirementCursor>{
        use semio_framework_value::retirement::{RetireOwned,deferred,sequence,arc_custody::SharedControlledRetirement};
        let eval:Box<dyn RetirementCursor>=match self.eval{Some(value)=>Box::new(SharedControlledRetirement::lease(value)),None=>().retirement()};
        sequence(vec![eval,deferred(self.baseline),deferred(self.fields)])
    }
    fn retirement_birth_bytes(&self)->Option<usize>{
        use semio_framework_value::retirement::{deferred_birth_bytes_for,leaf_birth_bytes,sequence_birth_bytes,arc_custody::shared_retirement_birth_bytes};
        sequence_birth_bytes(&[if self.eval.is_some(){shared_retirement_birth_bytes::<String>()}else{leaf_birth_bytes::<()>()},deferred_birth_bytes_for(&self.baseline),deferred_birth_bytes_for(&self.fields)])
    }
    fn controlled_retirement_supported()->bool{true}
}

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub(super) struct SessionPreviewCancellationSources {
    invocation_growth:Option<Vec<FlowInvocationEntry>>,
    invocations:Vec<FlowInvocationEntry>,
    retention:Option<SessionPreviewRetention>,
    pub(super) pending:HistoryFoldIndex<u64,String>,
    pub handles:HistoryFoldIndex<u64,String>,
    pub(super) evaluation:HistoryFoldIndex<u64,PreviewEvalProgress>,
}
impl SessionPreviewCancellationSources {
    pub(super) fn take(state:&mut FlowEvalSessionState)->Self{Self{invocation_growth:state.invocation_origin_growth.take(),invocations:std::mem::take(&mut state.invocation_origins),retention:state.preview_retention.take(),pending:std::mem::take(&mut state.pending_tessellate_by_hash),handles:std::mem::take(&mut state.tessellate_handle_by_hash),evaluation:std::mem::take(&mut state.eval_progress_by_hash)}}
    pub(super) fn restore(self,state:&mut FlowEvalSessionState){state.invocation_origin_growth=self.invocation_growth;state.invocation_origins=self.invocations;state.preview_retention=self.retention;state.pending_tessellate_by_hash=self.pending;state.tessellate_handle_by_hash=self.handles;state.eval_progress_by_hash=self.evaluation;}
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) enum SessionRetentionRow {
    Mesh((String,MeshData)),Diagnostic((String,String)),Pending((u64,String)),Progress((u64,PreviewTessellateProgress)),Roster(Vec<String>),
}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub(super) struct SessionPreviewRetention {
    pub roster:Option<Vec<String>>,pub(super) phase:u8,pub(super) slot:usize,pub(super) comparison:usize,pub(super) matched:bool,
    pub(super) removed:Option<ControlledRetirement<SessionRetentionRow>>,
}

pub(super) enum SessionCollectionOwner {
    TickSnapshot(FlowHostSnapshot),
    InvocationOrigins(Vec<FlowInvocationEntry>),
    LivePublication(SessionLivePublication),
    Retention(SessionPreviewRetention),
    Cancellation(SessionPreviewCancellationSources),
    Reset(SessionResetSources),
    Baseline(FlowBaselineLeases),
    SourceLease(Arc<String>),
    Text(String),
    Handles(HistoryFoldIndex<String, ()>),
    Meshes(HistoryFoldIndex<String, MeshData>),
    Diagnostics(HistoryFoldIndex<String, String>),
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

impl RetireOwned for SessionCollectionOwner {
    fn retirement(self)->Box<dyn RetirementCursor>{
        use semio_framework_value::retirement::{RetireOwned,arc_custody::SharedControlledRetirement};
        match self{
            Self::TickSnapshot(value)=>value.retirement(),
            Self::InvocationOrigins(value)=>value.retirement(),
            Self::LivePublication(value)=>value.retirement(),
            Self::Retention(value)=>value.retirement(),
            Self::Cancellation(value)=>value.retirement(),
            Self::Reset(value)=>value.retirement(),
            Self::Baseline(value)=>value.retirement(),
            Self::SourceLease(value)=>Box::new(SharedControlledRetirement::lease(value)),
            Self::Snapshot(value)=>Box::new(SharedControlledRetirement::lease(value)),Self::Channels(value)=>Box::new(SharedControlledRetirement::lease(value)),
            Self::Cache(value)=>Box::new(SharedControlledRetirement::lease(value)),
            Self::Text(value)=>value.retirement(),Self::Handles(value)=>value.retirement(),Self::Meshes(value)=>value.retirement(),Self::Diagnostics(value)=>value.retirement(),Self::Pending(value)=>value.retirement(),
            Self::Latches(value)=>value.retirement(),Self::Tessellation(value)=>value.retirement(),Self::Evaluation(value)=>value.retirement(),
            Self::Registry(value)=>value.retirement(),Self::Fault(value)=>value.retirement(),
        }
    }
    fn retirement_birth_bytes(&self)->Option<usize>{
        use semio_framework_value::retirement::{RetireOwned,arc_custody::shared_retirement_birth_bytes};
        match self{
            Self::TickSnapshot(value)=>value.retirement_birth_bytes(),
            Self::InvocationOrigins(value)=>value.retirement_birth_bytes(),
            Self::LivePublication(value)=>value.retirement_birth_bytes(),
            Self::Retention(value)=>value.retirement_birth_bytes(),
            Self::Cancellation(value)=>value.retirement_birth_bytes(),
            Self::Reset(value)=>value.retirement_birth_bytes(),
            Self::Baseline(value)=>value.retirement_birth_bytes(),
            Self::SourceLease(_)=>Some(shared_retirement_birth_bytes::<String>()),Self::Cache(_)=>Some(shared_retirement_birth_bytes::<NeuralCache>()),
            Self::Snapshot(_)=>Some(shared_retirement_birth_bytes::<TreeSnapshot>()),Self::Channels(_)=>Some(shared_retirement_birth_bytes::<EvalChannels>()),
            Self::Text(value)=>value.retirement_birth_bytes(),Self::Handles(value)=>value.retirement_birth_bytes(),Self::Meshes(value)=>value.retirement_birth_bytes(),Self::Diagnostics(value)=>value.retirement_birth_bytes(),Self::Pending(value)=>value.retirement_birth_bytes(),
            Self::Latches(value)=>value.retirement_birth_bytes(),Self::Tessellation(value)=>value.retirement_birth_bytes(),Self::Evaluation(value)=>value.retirement_birth_bytes(),
            Self::Registry(value)=>value.retirement_birth_bytes(),Self::Fault(value)=>value.retirement_birth_bytes(),
        }
    }
    fn controlled_retirement_supported()->bool{true}
}

pub(super) fn retire_session_collection_cold(value:SessionCollectionOwner){
 let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|unreachable!("declared cold session source"));
 for _ in 0..FLOW_EVAL_SESSION_COLD_CLOSE_STEPS{if owner.terminal_is_empty(){return}let copy=4096.max(owner.next_copy_byte_demand().expect("cold session source copy"));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).expect("cold session source capacity"),maximum_release_bytes:owner.next_release_byte_demand().expect("cold session source release"),maximum_depth:owner.next_depth_demand().expect("cold session source depth")};owner.step(grant).expect("cold session source close");}panic!("cold original session source did not terminate");
}
/// 🧊️ Closes an original diagnostic reply returned unchanged by source admission refusal.
pub fn retire_invocation_reply_cold(source:String){retire_session_collection_cold(SessionCollectionOwner::Text(source));}

#[derive(Default)]
pub(super) struct SessionRetirement {
    pub progress:RetainedCloneProgress,
    pub(super) root: Option<ControlledRetirement<SessionCollectionOwner>>,
    pub(super) owners: RetirementQueue,
}

impl SessionRetirement {
    pub(super) fn terminal_is_empty(&self)->bool{self.root.is_none()&&self.owners.terminal_is_empty()}
    pub(super) fn admission_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.root.is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if !self.owners.has_reserved_slot(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"session retirement requires an admitted original queue slot"))}
        Ok(RetirementDemand{capacity_bytes:RetirementQueue::frame_birth_bytes::<SessionCollectionOwner>(),depth:self.owners.len()+1,..Default::default()})
    }
    pub(super) fn preflight(&self,grant:RetainedCloneGrant)->Result<(),ValueError>{
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"session mutation requires an admitted item"))}
        let demand=self.admission_demands()?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"session mutation requires original source depth"))}
        if grant.maximum_capacity_bytes<demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"session mutation requires its original retained frame capacity"))}
        Ok(())
    }
    pub fn admit(&mut self,value:SessionCollectionOwner,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,SessionCollectionOwner)>{
        if self.root.is_some(){return self.owners.admit_owned(value,grant)}
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"session handoff requires an admitted item"),value))}
        if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"session handoff requires admitted depth"),value))}
        match ControlledRetirement::new(value){Ok(owner)=>{self.root=Some(owner);Ok(RetainedCloneProgress{copied_items:1,..Default::default()})},Err(error)=>Err(error)}
    }
    pub(super) fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.root.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
        Ok(RetirementDemand{copy_bytes:self.owners.next_copy_byte_demand()?,capacity_bytes:self.owners.next_capacity_byte_demand(copy)?,release_bytes:self.owners.next_release_byte_demand()?,depth:self.owners.next_depth_demand()?})
    }
    pub(super) fn step_progress(&self)->RetainedCloneProgress{self.progress}
    pub(super) fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.progress=Default::default();
        let Some(owner)=self.root.as_mut()else{let result=self.owners.step(grant);self.progress=self.owners.step_progress();return result};
        let result=owner.step(grant);self.progress=owner.step_progress();let step=result?;
        let progress=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,owner.terminal_is_empty(),"Flow session original root")?.progress();
        if owner.terminal_is_empty(){self.root=None;}
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
}

/// 🔒️ Evaluation ownership stays guarded until every collection, domain, cache, and byte frontier is empty.
/// 🧳️ Intake failure retains the exact typed original source or identifies its same-session custody.
pub enum FlowSnapshotIntakeFailure<Original>{Original{failure:semio_framework_job::OriginalTurnFailure,source:Original},Retained{failure:semio_framework_job::OriginalTurnFailure}}

pub struct FlowEvalSession {
    pub(super) state: std::mem::ManuallyDrop<FlowEvalSessionState>,
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
pub(super) const FLOW_EVAL_SESSION_COLD_CLOSE_STEPS: usize = 1_000_000;

impl FlowSourceBaseline for FlowEvalSession {
    fn previous_snapshot(&self) -> Option<&Arc<TreeSnapshot>> { self.previous_snapshot.as_ref() }
    fn previous_channels(&self) -> Option<&Arc<EvalChannels>> { self.previous_channels.as_ref() }
    fn baseline_geometry_pending(&self) -> bool { self.baseline_geometry_pending }
    fn preview_cancelled(&self) -> bool { self.preview_cancelled }
    fn pending_host(&self) -> Option<&FlowHost> { self.pending_host.as_ref() }
    fn flow_extension_generation(&self) -> u64 { self.flow_extension_generation }
}
