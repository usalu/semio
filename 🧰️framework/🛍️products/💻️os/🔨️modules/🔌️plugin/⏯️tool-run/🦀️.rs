//! ⏯️ OS runtime of an interactive, transactional tool run with a visible process: the per-instance
//! [`ToolRunLedger`] (provisional document ops folded into an overlay the renderer reads as the document), the
//! [`ToolRunDriver`] turn stepping run and revalidate jobs, the one-edit finalize publication and the
//! framework ToolRun panel body.
//!
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md`
//! §2.7, §2.8, §3.3, §4.1 (layers 1 and 3). Domain-neutral types: `semio_framework_tool_run`.

use super::*;
use semio_framework_tool_run::{
    tool_run_format, tool_run_pointer_value, ToolRunAction, ToolRunCounter, ToolRunDefinition, ToolRunEffect, ToolRunEvent, ToolRunId, ToolRunIdentity, ToolRunLabel, ToolRunMachine, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunRejection, ToolRunSettingsReads, ToolRunSlot, ToolRunState,
    ToolRunStep, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTraceCursor, ToolRunTraceDelta, ToolRunTraceOp, ToolRunTraceStore, ToolRunVerdict, TOOL_RUN_ACTION_IDS, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID,
    TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_ARG_WINDOW_ID, TOOL_RUN_PROVISIONAL_OPS_MAX, TOOL_RUN_REASON_CONFLICT, TOOL_RUN_REASON_PROVISIONAL_CAP, TOOL_RUN_REASON_REBASING, TOOL_RUN_REASON_TRACE_TRUNCATED, TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS, TOOL_RUN_TICK_BYTES_MAX,
};
use semio_framework_ui_scene::{scene_lane_hash, Board2dScene, Board2dSceneLane, Canvas2dScene, Canvas2dSceneLane, SceneDoc, SceneLaneRef, World3dScene, World3dSceneLane};
use std::collections::BTreeMap;
use std::sync::Arc;

//#region 🔖️Limits
/// 🪧️ Framework-owned panel body served before any app body, next to `FRAMEWORK_HISTORY_BODY_KEY`.
pub const FRAMEWORK_TOOL_RUN_BODY_KEY: &str = "framework.body.toolRun";
/// ⏱️ Wall budget of one driver turn; every job step inside it keeps its own interactive-lane slice.
pub const TOOL_RUN_TURN_WALL_US: u64 = 4_000;
/// 🧮️ Folded ops between two wall-clock checks of a bounded refold.
pub const TOOL_RUN_REFOLD_CHECK_OPS: usize = 64;
/// 🧹️ Discarded provisional ops cold-retired per driver or close turn.
pub const TOOL_RUN_DISCARD_OPS_PER_TURN: usize = 4_096;
/// 📜️ Newest step-log rows the panel shows.
pub const TOOL_RUN_PANEL_STEP_ROWS: usize = 16;
/// 🧾️ Newest trace records the panel's keyboard trace list shows.
pub const TOOL_RUN_PANEL_TRACE_ROWS: usize = 32;
/// 📦️ Encoded trace delta bytes one renderer refresh receives.
pub const TOOL_RUN_TRACE_DELTA_BYTES: usize = 262_144;
/// 🔁️ Refreshes one window may re-request a trace backlog while its renderer's echoed cursor stands still;
/// a renderer that never echoes therefore costs a bounded number of refreshes, never a refresh loop.
pub const TOOL_RUN_TRACE_STALL_REFRESHES: u8 = 4;
const TOOL_RUN_JOB_SITE: &str = "tool-run.step";
const TOOL_RUN_PUBLICATION_GRANT: store::ArtifactStoreOneItemGrant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES };
//#endregion 🔖️Limits

//#region 🔖️Request
/// 🎯️ Which job of a run the app builds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolRunJobPurpose {
    Run,
    Revalidate,
}

/// 🧵️ A boxed run or revalidate job; it reports `ToolRunTick` pages via `StepOutcome::PreviewReady`. `Send`
/// explicitly: the browser target's `InteractiveJob` drops its thread-transfer bound, but the ledger lives
/// in a `PluginApp`, which stays `Send` on every target.
pub type ToolRunJob = Box<dyn semio_framework_job::InteractiveJob + Send>;

/// 🧳️ Everything an app needs to build one run or revalidate job (§3.7).
///
/// - `snapshot` is the run's base for `Run` and the committed head for `Revalidate`.
/// - `window_id` is the window instance the run was started from (`toolRunStart`'s `windowId`, else the
///   dispatching view's window), and `window_config` that window's config snapshot as of this build.
/// - `checkpoint` is the last `StepOutcome::CheckpointReady` state of this run (reconfigure `resume`).
/// - `provisional` is the provisional op list the job continues from or revalidates.
/// - `instance_owner` is the instance's retained operation owner, shared with its commands.
/// - `port` carries the host effects the job asks for and the wake of a job waiting on them.
/// - `trace_keys` allocates trace keys no record of this run uses, e.g. one per placement a revalidate job
///   re-tests; every job of the run shares it, so keys never collide across rebuilds and purposes.
/// - `entity_marks` holds `(provisional length at the end of the tick that appended it, entity)` per entity, in
///   append order: which provisional op prefix each placement's entity belongs to.
pub struct ToolRunJobRequest<'a, A: ArtifactApp> {
    pub tool_id: &'a str,
    pub definition: &'a ToolRunDefinition,
    pub purpose: ToolRunJobPurpose,
    pub identity: ToolRunIdentity,
    pub snapshot: Arc<A::Snapshot>,
    pub config: Arc<A::Config>,
    pub window_id: Option<&'a str>,
    pub window_config: Option<WindowConfigSnapshot>,
    pub checkpoint: Option<&'a [u8]>,
    pub provisional: &'a [A::Mutation],
    pub instance_owner: ArtifactInstanceOperationOwnerHandle,
    pub port: ToolRunJobPort,
    pub trace_keys: ToolRunTraceKeys,
    pub entity_marks: &'a [(u32, u64)],
}

/// 🔑️ The trace key allocator every job of one run shares: keys handed out lie above every key the run's
/// ticks have upserted and every key allocated before.
#[derive(Clone, Debug, Default)]
pub struct ToolRunTraceKeys {
    next: Arc<std::sync::atomic::AtomicU64>,
}

impl ToolRunTraceKeys {
    /// 🔢️ The first key neither upserted by this run nor allocated.
    pub fn next(&self) -> u64 {
        self.next.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// 🎟️ Reserves `count` consecutive unused keys.
    pub fn allocate(&self, count: u64) -> std::ops::Range<u64> {
        let start = self.next.try_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |next| Some(next.saturating_add(count))).unwrap_or_else(|next| next);
        start..start.saturating_add(count)
    }

    fn observe(&self, key: u64) {
        self.next.fetch_max(key.saturating_add(1), std::sync::atomic::Ordering::SeqCst);
    }
}

/// 🎯️ The in-place retarget hooks of the tool run job protocol (§3.3). A job built through
/// `ArtifactApp::build_retargetable_tool_run_job` stays resident across a base change and a settings change —
/// and while its run is `complete` — instead of being closed and rebuilt from its checkpoint.
pub trait ToolRunRetargetableJob<C>: semio_framework_job::InteractiveJob + Send {
    /// 🪢️ Every tick after this call carries `identity` (base changed: new generation and base revision).
    fn rebind(&mut self, identity: ToolRunIdentity);
    /// 🎚️ Retargets the job to `config` under `identity`; `false` when it cannot continue in place, and the
    /// driver then closes it and rebuilds it from its checkpoint.
    fn reconfigure(&mut self, identity: ToolRunIdentity, config: Arc<C>) -> bool;
}

/// 📮️ What a run job shares with its app instance across turns: the host effects it hands the driver (an
/// extension round trip whose answer returns as an app command cannot happen inside `step`) and the external
/// wake of a job waiting on such work. A waiting job is never stepped and keeps its instance idle, so waiting
/// costs no turn; whoever folds the awaited answer calls [`ToolRunJobPort::wake`].
#[derive(Clone, Default)]
pub struct ToolRunJobPort {
    inner: Arc<ToolRunJobPortState>,
}

#[derive(Default)]
struct ToolRunJobPortState {
    effects: std::sync::Mutex<std::collections::VecDeque<Effect>>,
    waiting: std::sync::atomic::AtomicBool,
}

impl ToolRunJobPort {
    /// 📨️ Queues one effect the driver hands the host on its next turn, in order.
    pub fn dispatch(&self, effect: Effect) {
        if let Ok(mut effects) = self.inner.effects.lock() {
            effects.push_back(effect);
        }
    }

    /// ⏸️ The job's next step makes no progress until [`ToolRunJobPort::wake`]; set it while holding whatever
    /// lock the waker folds its answer under, so no wake can slip between the decision and the flag.
    pub fn wait(&self) {
        self.inner.waiting.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// ⏰️ Makes a waiting job runnable again.
    pub fn wake(&self) {
        self.inner.waiting.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_waiting(&self) -> bool {
        self.inner.waiting.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn has_effects(&self) -> bool {
        self.inner.effects.lock().is_ok_and(|effects| !effects.is_empty())
    }

    fn pop_effect(&self) -> Option<Effect> {
        self.inner.effects.lock().ok()?.pop_front()
    }

    fn restore_effect(&self, effect: Effect) {
        if let Ok(mut effects) = self.inner.effects.lock() {
            effects.push_front(effect);
        }
    }
}

/// 🪟️ What a renderer learns about the run on this document instance (§4.1 layer 1): its identity and state, the
/// provisional entities, the §2.3 progress (stage, counters, step ring) and the latest plugin payload a tick carried.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolRunView {
    pub tool_id: String,
    pub identity: ToolRunIdentity,
    pub state: ToolRunState,
    pub provisional_entities: Arc<BTreeSet<u64>>,
    pub progress: semio_framework_tool_run::ToolRunProgress,
    pub payload: Option<Arc<[u8]>>,
}

impl ToolRunView {
    /// 🏗️ A view of a run with no provisional entities, no progress yet and no payload.
    pub fn new(tool_id: impl Into<String>, identity: ToolRunIdentity, state: ToolRunState) -> Self {
        let progress = semio_framework_tool_run::ToolRunProgress { identity, sequence: 0, state, stage: 0, completed: 0, total: None, counters: Vec::new(), units_per_second: 0.0, conflicts: 0, steps: ToolRunStepRing::new() };
        Self { tool_id: tool_id.into(), identity, state, provisional_entities: Arc::new(BTreeSet::new()), progress, payload: None }
    }
}

/// 🔖️ True for the seven framework-reserved tool run action ids (§2.5).
pub fn is_tool_run_action_id(action: &str) -> bool {
    TOOL_RUN_ACTION_IDS.contains(&action)
}
//#endregion 🔖️Request

//#region 🔖️Job
enum ToolRunJobHandle<C> {
    Plain(ToolRunJob),
    Retargetable(Box<dyn ToolRunRetargetableJob<C>>),
}

struct ToolRunJobSlot<C> {
    job: ToolRunJobHandle<C>,
    purpose: ToolRunJobPurpose,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    cancel: semio_framework_job::CancelToken,
    preview_sequence: u64,
    closing: bool,
}

impl<C> ToolRunJobSlot<C> {
    fn new(job: ToolRunJobHandle<C>, purpose: ToolRunJobPurpose, generation: u32) -> Self {
        Self { job, purpose, operation: semio_framework_job::allocate_operation_id(), generation: semio_framework_job::Generation(u64::from(generation)), cancel: semio_framework_job::CancelToken::root_now(), preview_sequence: 0, closing: false }
    }

    fn interactive(&mut self) -> &mut dyn semio_framework_job::InteractiveJob {
        match &mut self.job {
            ToolRunJobHandle::Plain(job) => job.as_mut(),
            ToolRunJobHandle::Retargetable(job) => job.as_mut(),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        match &self.job {
            ToolRunJobHandle::Plain(job) => job.terminal_is_empty(),
            ToolRunJobHandle::Retargetable(job) => job.terminal_is_empty(),
        }
    }

    fn retargetable(&mut self) -> Option<&mut dyn ToolRunRetargetableJob<C>> {
        match &mut self.job {
            ToolRunJobHandle::Retargetable(job) if !self.closing => Some(job.as_mut()),
            _ => None,
        }
    }

    fn begin_close(&mut self) {
        if !self.closing {
            self.cancel.cancel_now();
            self.interactive().begin_close();
            self.closing = true;
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
        self.begin_close();
        match self.interactive().close_step(maximum_items.max(1), maximum_bytes) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => Ok(Some(PluginCloseStep::Pending { released_items, released_bytes })),
            semio_framework_job::InteractiveJobCloseStep::Blocked => Ok(Some(PluginCloseStep::Blocked { reason: "tool run job close is blocked" })),
            semio_framework_job::InteractiveJobCloseStep::Complete if self.terminal_is_empty() => Ok(None),
            semio_framework_job::InteractiveJobCloseStep::Complete => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.job-close"), "tool run job reported Complete without its terminal-empty witness")),
        }
    }
}

fn close_job_payload(payload: &mut semio_framework_job::RetainedJobPayload) {
    while let semio_framework_job::JobPayloadCloseStep::Pending { released_items: 1, .. } = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {}
}

fn job_payload_bytes(payload: &semio_framework_job::RetainedJobPayload, limit: usize) -> Option<Vec<u8>> {
    if payload.len() > limit {
        return None;
    }
    let mut bytes = Vec::with_capacity(payload.len());
    for index in 0..payload.page_count() {
        bytes.extend_from_slice(payload.page(index)?);
    }
    Some(bytes)
}

/// 📼️ Lane text of one delta: `None` for an empty, non-clearing delta, else its base64url (unpadded) pack body.
fn tool_run_trace_lane_text(delta: &ToolRunTraceDelta) -> Option<String> {
    if !delta.clear && delta.pages.is_empty() {
        return None;
    }
    delta.encode().ok().map(base64_codec::base64_url_encode)
}

/// 🔎️ The first World3d, Canvas2d or Board2d scene surface of a rendered body, depth first.
fn tool_run_scene_surface(node: &mut BuiltNode) -> Option<&mut BuiltNode> {
    let is_scene = matches!(&node.component, Component::Surface(props) if [World3dScene::SCHEMA, Canvas2dScene::SCHEMA, Board2dScene::SCHEMA].contains(&props.doc_schema.as_str()));
    if is_scene {
        return Some(node);
    }
    node.children.iter_mut().find_map(tool_run_scene_surface)
}

/// 📎️ Re-attaches `lane` to a scene surface: the spine's lane refs gain the `toolRunTrace` entry and the
/// surface gains its paged carrier, exactly as `SceneDoc::split_lanes` plus `scene_surface` would emit it.
/// A surface whose producer already carries the lane is left untouched.
fn inject_tool_run_trace_lane_into(surface: &mut BuiltNode, lane: &str) -> UiAssemblyResult<()> {
    let Component::Surface(props) = &mut surface.component else { return Ok(()) };
    let (name, key) = match props.doc_schema.as_str() {
        World3dScene::SCHEMA => (World3dSceneLane::ToolRunTrace.name(), World3dSceneLane::ToolRunTrace.body_key()),
        Board2dScene::SCHEMA => (Board2dSceneLane::ToolRunTrace.name(), Board2dSceneLane::ToolRunTrace.body_key()),
        _ => (Canvas2dSceneLane::ToolRunTrace.name(), Canvas2dSceneLane::ToolRunTrace.body_key()),
    };
    if surface.children.iter().any(|child| child.key.as_str() == key) {
        return Ok(());
    }
    let reference = SceneLaneRef { lane: name.to_string(), bytes: lane.len() as u32, hash: scene_lane_hash(lane) };
    let encoded = match props.doc_schema.as_str() {
        World3dScene::SCHEMA => {
            let mut spine: World3dScene = semio_framework_ui_scene::decode(props).map_err(|error| ui_assembly_error_because("tool-run-trace.decode", error))?;
            spine.lanes.push(reference);
            semio_framework_ui_scene::encode(props.kind, &spine)
        }
        Board2dScene::SCHEMA => {
            let mut spine: Board2dScene = semio_framework_ui_scene::decode(props).map_err(|error| ui_assembly_error_because("tool-run-trace.decode", error))?;
            spine.lanes.push(reference);
            semio_framework_ui_scene::encode(props.kind, &spine)
        }
        _ => {
            let mut spine: Canvas2dScene = semio_framework_ui_scene::decode(props).map_err(|error| ui_assembly_error_because("tool-run-trace.decode", error))?;
            spine.lanes.push(reference);
            semio_framework_ui_scene::encode(props.kind, &spine)
        }
    };
    let mut encoded = encoded.map_err(|error| ui_assembly_error_because("tool-run-trace.encode", error))?;
    let carrier = paged_text_carrier(key, lane)?;
    if surface.children.len() == UI_BUILT_CHILDREN_MAX {
        return Err(ui_assembly_error("tool-run-trace.carrier"));
    }
    encoded.bindings = std::mem::take(&mut props.bindings);
    *props = encoded;
    surface.children.try_push(carrier).map_err(|_| ui_assembly_error("tool-run-trace.carrier"))
}

fn close_step_outcome(outcome: &mut semio_framework_job::StepOutcome) {
    while let semio_framework_job::JobPayloadCloseStep::Pending { released_items: 1, .. } = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {}
}
//#endregion 🔖️Job

//#region 🔖️Ledger
/// 🔁️ A bounded refold of the provisional ops from `base`. `boundary` is whether the job reached a point where
/// the refolded overlay is whole: a retract refold waits for the job's next checkpoint, completion or close,
/// so a retract followed by re-appends spread over several ticks never renders a half re-appended overlay —
/// the previous overlay stays rendered until then.
struct ToolRunRefold<P> {
    running: Option<P>,
    cursor: usize,
    boundary: bool,
}

/// 🚦️ Where one refold turn left the overlay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ToolRunRefoldTurn {
    Pending,
    AwaitingBoundary,
    Swapped,
}

/// 🎚️ The values a run's declared settings reads named when last observed: config pointers, then per declared
/// window kind the `(window id, values)` of every partition it reads.
#[derive(Clone, Debug, Default, PartialEq)]
struct ToolRunSettingsValues {
    config: Vec<Option<DslValue>>,
    window_config: Vec<(String, String, Vec<Option<DslValue>>)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ToolRunFinalizePhase {
    Pending,
    Revalidating,
    Publishing,
    Closing,
}

struct ToolRunFinalize<A: ArtifactApp> {
    phase: ToolRunFinalizePhase,
    publication: Option<store::ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>>,
    retracted: u32,
    published: bool,
}

struct ToolRunEntry<A: ArtifactApp> {
    tool_id: String,
    actor: String,
    definition: ToolRunDefinition,
    slot: ToolRunSlot,
    identity: ToolRunIdentity,
    base: Arc<A::Snapshot>,
    base_generation: u64,
    settings_generation: (u64, u64),
    settings_values: ToolRunSettingsValues,
    window: Option<(String, String)>,
    window_bodies: Vec<String>,
    payload: Option<Arc<[u8]>>,
    trace_keys: ToolRunTraceKeys,
    provisional: Vec<A::Mutation>,
    entity_marks: Vec<(u32, u64)>,
    entities: Arc<BTreeSet<u64>>,
    overlay: Arc<A::Snapshot>,
    refold: Option<ToolRunRefold<A::Snapshot>>,
    trace: ToolRunTraceStore,
    recent_trace: std::collections::VecDeque<u64>,
    stage: u16,
    completed: u64,
    total: Option<u64>,
    counters: Vec<ToolRunCounter>,
    steps: ToolRunStepRing,
    step_sequence: u64,
    sequence: u64,
    conflicts: u32,
    rate_origin: Option<(u64, u64)>,
    units_per_second: f32,
    job: Option<ToolRunJobSlot<A::Config>>,
    checkpoint: Option<Vec<u8>>,
    pending_step: bool,
    finalize: Option<ToolRunFinalize<A>>,
    announced: Option<(ToolRunState, u64, String)>,
}

impl<A: ArtifactApp> ToolRunEntry<A> {
    fn framework_step(&mut self, kind: ToolRunStepKind, reason: u16, args: &[ToolRunStepArg]) {
        self.step_sequence = self.step_sequence.saturating_add(1);
        self.steps.push(ToolRunStep { sequence: self.step_sequence, kind, stage: self.stage, reason, subject: None, repeat: 1, args: args.to_vec() });
    }

    fn begin_refold(&mut self, boundary: bool) {
        self.refold = Some(ToolRunRefold { running: None, cursor: 0, boundary });
    }

    /// 🏁️ The job reached a point where the refolded overlay is whole.
    fn settle_refold(&mut self) {
        if let Some(refold) = self.refold.as_mut() {
            refold.boundary = true;
        }
    }

    fn refold_is_work(&self) -> bool {
        self.refold.as_ref().is_some_and(|refold| refold.boundary || refold.cursor < self.provisional.len())
    }

    fn rebind(&mut self) {
        self.identity.generation = self.slot.generation;
        self.trace.rebind(self.identity);
    }

    fn rebuild_entities(&mut self) {
        let length = self.provisional.len() as u32;
        self.entity_marks.retain(|(end, _)| *end <= length);
        self.entities = Arc::new(self.entity_marks.iter().map(|(_, entity)| *entity).collect());
    }

    fn fold_one(snapshot: &A::Snapshot, op: &A::Mutation) -> Option<A::Snapshot> {
        let outcome = op.diff(snapshot);
        if !outcome.is_applicable(protocol::MergePolicy::default()) {
            return None;
        }
        outcome.diff().apply(snapshot).ok()
    }

    /// 🔁️ Folds provisional ops from `base` until the wall deadline; the overlay is replaced once every op is
    /// folded and the refold reached its boundary.
    fn refold_turn(&mut self, deadline_us: u64) -> ToolRunRefoldTurn {
        let Some(mut refold) = self.refold.take() else { return ToolRunRefoldTurn::Swapped };
        while refold.cursor < self.provisional.len() {
            let source = refold.running.as_ref().unwrap_or(&self.base);
            match Self::fold_one(source, &self.provisional[refold.cursor]) {
                Some(next) => refold.running = Some(next),
                None => self.conflicts = self.conflicts.saturating_add(1),
            }
            refold.cursor += 1;
            if refold.cursor % TOOL_RUN_REFOLD_CHECK_OPS == 0 && semio_framework_job::default_now_us().is_none_or(|now| now >= deadline_us) {
                self.refold = Some(refold);
                return ToolRunRefoldTurn::Pending;
            }
        }
        if !refold.boundary {
            self.refold = Some(refold);
            return ToolRunRefoldTurn::AwaitingBoundary;
        }
        self.overlay = refold.running.map_or_else(|| Arc::clone(&self.base), Arc::new);
        ToolRunRefoldTurn::Swapped
    }

    /// 🧬️ Applies one tick of the current identity: retract, append (O(k) fold), entities, steps, progress, trace.
    fn apply_tick(&mut self, mut tick: ToolRunTick, discarded: &mut Vec<A::Mutation>) -> Result<ToolRunTickReceipt, Fault> {
        let mut receipt = ToolRunTickReceipt::default();
        if tick.identity.id != self.identity.id || tick.identity.generation != self.identity.generation {
            receipt.stale = true;
            return Ok(receipt);
        }
        if let Some(payload) = tick.payload.take() {
            self.payload = Some(payload.into());
        }
        self.sequence = self.sequence.max(tick.sequence);
        if let Some(retract) = tick.retract_to.map(|length| length as usize).filter(|length| *length < self.provisional.len()) {
            receipt.retracted = (self.provisional.len() - retract) as u32;
            discarded.extend(self.provisional.drain(retract..));
            self.rebuild_entities();
            self.begin_refold(false);
        }
        let mut appended = Vec::with_capacity(tick.append_ops.len());
        for bytes in &tick.append_ops {
            if self.provisional.len() + appended.len() >= TOOL_RUN_PROVISIONAL_OPS_MAX as usize {
                receipt.capped = true;
                break;
            }
            appended.push(<A::Mutation as ::protocol::OpBinary>::decode_op(bytes).map_err(|error| error.into_fault())?);
        }
        if self.refold.is_none() && !appended.is_empty() {
            let mut running: Option<A::Snapshot> = None;
            for op in &appended {
                match Self::fold_one(running.as_ref().unwrap_or(&self.overlay), op) {
                    Some(next) => running = Some(next),
                    None => self.conflicts = self.conflicts.saturating_add(1),
                }
            }
            if let Some(running) = running {
                self.overlay = Arc::new(running);
            }
        }
        receipt.appended = appended.len() as u32;
        self.provisional.extend(appended);
        let end = self.provisional.len() as u32;
        if !tick.append_entities.is_empty() {
            let entities = Arc::make_mut(&mut self.entities);
            for entity in tick.append_entities {
                self.entity_marks.push((end, entity));
                entities.insert(entity);
            }
        }
        for mut step in tick.steps {
            self.step_sequence = self.step_sequence.saturating_add(1);
            step.sequence = self.step_sequence;
            self.steps.push(step);
        }
        if let Some(progress) = tick.progress {
            self.stage = progress.stage;
            self.total = progress.total;
            self.completed = progress.total.map_or(progress.completed, |total| progress.completed.min(total));
            self.counters = progress.counters;
        }
        for page in &tick.trace {
            let applied = self.trace.apply_page(page).map_err(|rejection| Fault::new(FaultOrigin::Framework, FaultCode::new(rejection.code()), "tool run trace page does not belong to the current run generation"))?;
            receipt.evicted = receipt.evicted.saturating_add(applied.evicted);
            receipt.capped |= applied.overflowed > 0;
            for op in &page.ops {
                match op {
                    ToolRunTraceOp::Upsert { key, .. } => {
                        self.trace_keys.observe(*key);
                        if self.recent_trace.len() == TOOL_RUN_PANEL_TRACE_ROWS {
                            self.recent_trace.pop_front();
                        }
                        self.recent_trace.push_back(*key);
                    }
                    ToolRunTraceOp::Clear => self.recent_trace.clear(),
                    ToolRunTraceOp::Retire { .. } => {}
                }
            }
        }
        if receipt.evicted > 0 {
            self.framework_step(ToolRunStepKind::Warning, TOOL_RUN_REASON_TRACE_TRUNCATED, &[ToolRunStepArg::Unsigned(u64::from(receipt.evicted))]);
        }
        if receipt.capped {
            self.framework_step(ToolRunStepKind::Warning, TOOL_RUN_REASON_PROVISIONAL_CAP, &[ToolRunStepArg::Unsigned(u64::from(TOOL_RUN_PROVISIONAL_OPS_MAX))]);
        }
        let now = semio_framework_job::default_now_us().unwrap_or(0);
        match self.rate_origin {
            None => self.rate_origin = Some((now, self.completed)),
            Some((origin_us, origin_completed)) if now > origin_us => self.units_per_second = self.completed.saturating_sub(origin_completed) as f32 * 1_000_000.0 / (now - origin_us) as f32,
            Some(_) => {}
        }
        Ok(receipt)
    }
}

/// 📬️ What one applied tick changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToolRunTickReceipt {
    pub stale: bool,
    pub appended: u32,
    pub retracted: u32,
    pub evicted: u32,
    pub capped: bool,
}

/// 🧭️ Driver watch state and turn budget (§3.3): settings publications the store generations cannot see.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolRunDriver {
    pub turn_wall_us: u64,
    window_config_publications: u64,
}

impl Default for ToolRunDriver {
    fn default() -> Self {
        Self { turn_wall_us: TOOL_RUN_TURN_WALL_US, window_config_publications: 0 }
    }
}

/// 🗄️ At most one tool run per document instance per local actor, plus the owners still retiring.
pub struct ToolRunLedger<A: ArtifactApp> {
    next_run: u64,
    entry: Option<Box<ToolRunEntry<A>>>,
    driver: ToolRunDriver,
    retired_jobs: Vec<ToolRunJobSlot<A::Config>>,
    retired_publications: Vec<store::ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>>,
    discarded: Vec<A::Mutation>,
    closing: bool,
    trace_windows: BTreeMap<String, ToolRunTraceWindow>,
    ui_dirty: bool,
    document_dirty: bool,
    port: ToolRunJobPort,
}

/// 🪟️ One scene window the trace lane was delivered to: its body key and what its renderer last echoed.
struct ToolRunTraceWindow {
    body_key: String,
    echoed: Option<ToolRunTraceCursor>,
    answered: bool,
    stalls: u8,
}

impl<A: ArtifactApp> Default for ToolRunLedger<A> {
    fn default() -> Self {
        Self { next_run: 1, entry: None, driver: ToolRunDriver::default(), retired_jobs: Vec::new(), retired_publications: Vec::new(), discarded: Vec::new(), closing: false, trace_windows: BTreeMap::new(), ui_dirty: false, document_dirty: false, port: ToolRunJobPort::default() }
    }
}

impl<A: ArtifactApp> ToolRunLedger<A> {
    pub fn state(&self) -> Option<ToolRunState> {
        self.entry.as_ref().map(|entry| entry.slot.state)
    }

    pub fn slot(&self) -> Option<ToolRunSlot> {
        self.entry.as_ref().map(|entry| entry.slot)
    }

    pub fn identity(&self) -> Option<ToolRunIdentity> {
        self.entry.as_ref().map(|entry| entry.identity)
    }

    pub fn tool_id(&self) -> Option<&str> {
        self.entry.as_ref().map(|entry| entry.tool_id.as_str())
    }

    /// 📮️ The port every job of this ledger shares with its app instance.
    pub fn port(&self) -> &ToolRunJobPort {
        &self.port
    }

    pub fn provisional(&self) -> &[A::Mutation] {
        self.entry.as_ref().map_or(&[], |entry| entry.provisional.as_slice())
    }

    pub fn provisional_entities(&self) -> Option<&BTreeSet<u64>> {
        self.entry.as_ref().map(|entry| entry.entities.as_ref())
    }

    pub fn steps(&self) -> Option<&ToolRunStepRing> {
        self.entry.as_ref().map(|entry| &entry.steps)
    }

    pub fn trace(&self) -> Option<&ToolRunTraceStore> {
        self.entry.as_ref().map(|entry| &entry.trace)
    }

    pub fn conflicts(&self) -> u32 {
        self.entry.as_ref().map_or(0, |entry| entry.conflicts)
    }

    pub fn is_refolding(&self) -> bool {
        self.entry.as_ref().is_some_and(|entry| entry.refold.is_some())
    }

    /// 📸️ The current run's last `StepOutcome::CheckpointReady` state, the one a rebuilt job continues from.
    pub fn checkpoint(&self) -> Option<&[u8]> {
        self.entry.as_ref()?.checkpoint.as_deref()
    }

    /// 🔑️ The trace key allocator every job of the current run shares.
    pub fn trace_keys(&self) -> Option<&ToolRunTraceKeys> {
        self.entry.as_ref().map(|entry| &entry.trace_keys)
    }

    /// 🏷️ `(provisional length at the end of the appending tick, entity)` per provisional entity, in append order.
    pub fn entity_marks(&self) -> &[(u32, u64)] {
        self.entry.as_ref().map_or(&[], |entry| entry.entity_marks.as_slice())
    }

    /// 🪟️ `(window instance id, window kind id)` the current run was started from.
    pub fn window(&self) -> Option<(&str, &str)> {
        self.entry.as_ref()?.window.as_ref().map(|(window_id, window_kind_id)| (window_id.as_str(), window_kind_id.as_str()))
    }

    /// 📊️ Progress snapshot derived from the ledger (§2.3); the step ring is the framework's own.
    pub fn progress(&self) -> Option<semio_framework_tool_run::ToolRunProgress> {
        self.entry.as_ref().map(|entry| semio_framework_tool_run::ToolRunProgress {
            identity: entry.identity,
            sequence: entry.sequence,
            state: entry.slot.state,
            stage: entry.stage,
            completed: entry.completed,
            total: entry.total,
            counters: entry.counters.clone(),
            units_per_second: entry.units_per_second,
            conflicts: entry.conflicts,
            steps: entry.steps.clone(),
        })
    }

    /// 🪞️ The document a renderer reads: the overlay while a run holds provisional state, else `committed`.
    pub fn overlay_or<'a>(&'a self, committed: &'a Arc<A::Snapshot>) -> &'a Arc<A::Snapshot> {
        match self.entry.as_ref() {
            Some(entry) if matches!(entry.slot.state, ToolRunState::Starting | ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete | ToolRunState::Finalizing) && !Arc::ptr_eq(&entry.overlay, &entry.base) => &entry.overlay,
            _ => committed,
        }
    }

    pub fn view(&self) -> Option<ToolRunView> {
        let progress = self.progress()?;
        self.entry.as_ref().map(|entry| ToolRunView { tool_id: entry.tool_id.clone(), identity: entry.identity, state: entry.slot.state, provisional_entities: Arc::clone(&entry.entities), progress, payload: entry.payload.clone() })
    }

    /// 🧊️ `freeze` rebase policy: local artifact emits fail with `toolRun.busy` while a run is non-terminal.
    pub fn freezes_local_emits(&self) -> bool {
        self.entry.as_ref().is_some_and(|entry| entry.definition.rebase == ToolRunRebasePolicy::Freeze && !entry.slot.state.is_terminal())
    }

    /// 🏃️ Whether a driver turn has work: a stepping job, a refold, a close, a finalize, cold retirement or port
    /// effects. A job waiting on its port is not work.
    pub fn has_pending_work(&self) -> bool {
        let stepping = |entry: &ToolRunEntry<A>| entry.job.is_none() || !self.port.is_waiting();
        !self.retired_jobs.is_empty()
            || !self.retired_publications.is_empty()
            || !self.discarded.is_empty()
            || self.port.has_effects()
            || self.entry.as_ref().is_some_and(|entry| {
                entry.refold_is_work()
                    || match entry.slot.state {
                        ToolRunState::Running => stepping(entry),
                        ToolRunState::Starting | ToolRunState::Finalizing | ToolRunState::Aborting => true,
                        ToolRunState::Paused => entry.pending_step && stepping(entry),
                        ToolRunState::Complete => false,
                        _ => entry.job.is_some(),
                    }
            })
    }

    /// 🧬️ Applies one decoded tick to the current run (the driver's per-tick O(k) overlay append);
    /// ticks of another run or generation are stale no-ops.
    pub fn apply_tick(&mut self, tick: ToolRunTick) -> Result<ToolRunTickReceipt, Fault> {
        let Some(entry) = self.entry.as_mut() else { return Ok(ToolRunTickReceipt { stale: true, ..ToolRunTickReceipt::default() }) };
        entry.apply_tick(tick, &mut self.discarded)
    }

    /// 🪟️ A window-config publication happened; the driver treats it as a settings change.
    pub fn note_window_config_published(&mut self) {
        self.driver.window_config_publications = self.driver.window_config_publications.wrapping_add(1);
    }

    /// 👥️ Ephemeral-shared presence summary (§3.4), `completed ≤ total` by construction.
    pub fn presence(&self) -> Option<protocol::PresenceToolRun> {
        let entry = self.entry.as_ref()?;
        Some(protocol::PresenceToolRun {
            tool_id: entry.tool_id.clone(),
            state: protocol::PresenceToolRunState::from_wire_name(entry.slot.state.as_str())?,
            stage: entry.stage,
            completed: entry.total.map_or(entry.completed, |total| entry.completed.min(total)),
            total: entry.total,
        })
    }

    /// 📼️ Base64url (unpadded) `ToolRunTraceDelta` after `cursor`, for the scene lane `toolRunTrace` (§3.2).
    ///
    /// - A cursor of another run or generation, or none, gets `clear` plus the resend of the live log.
    /// - A caught-up cursor gets `None`, so an idle run costs nothing.
    /// - Without a run (dismissed or retired), a renderer still holding pages (`page > 0`) gets one empty
    ///   `clear` delta; `page == 0` already means an empty layer.
    pub fn trace_delta(&self, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> Option<String> {
        let delta = match self.entry.as_ref() {
            Some(entry) => entry.trace.delta_after(cursor, byte_budget),
            None => {
                let cursor = cursor.filter(|cursor| cursor.page > 0)?;
                ToolRunTraceDelta { identity: ToolRunIdentity { id: ToolRunId { app_instance_id: 0, run: cursor.run }, generation: cursor.generation, base_revision: [0; 32] }, clear: true, next: 0, pages: Vec::new() }
            }
        };
        tool_run_trace_lane_text(&delta)
    }

    /// 🪟️ The lane text scene window `window_id` (body `body_key`) receives for its echoed `cursor`, and the
    /// bookkeeping that keeps a delivery backlog moving: pages left after the byte budget mark the ledger
    /// UI-dirty until the renderer's cursor stands still for `TOOL_RUN_TRACE_STALL_REFRESHES` refreshes.
    pub fn trace_lane(&mut self, window_id: &str, body_key: &str, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> Option<String> {
        let Some(entry) = self.entry.as_ref() else {
            self.trace_windows.remove(window_id);
            return self.trace_delta(cursor, byte_budget);
        };
        let delta = entry.trace.delta_after(cursor, byte_budget);
        let backlog = delta.next < entry.trace.next_page();
        let window = self.trace_windows.entry(window_id.to_string()).or_insert_with(|| ToolRunTraceWindow { body_key: body_key.to_string(), echoed: None, answered: false, stalls: 0 });
        if window.body_key != body_key {
            window.body_key = body_key.to_string();
        }
        window.stalls = if window.answered && window.echoed == cursor { window.stalls.saturating_add(1) } else { 0 };
        window.echoed = cursor;
        window.answered = true;
        if backlog && window.stalls < TOOL_RUN_TRACE_STALL_REFRESHES {
            self.ui_dirty = true;
        }
        tool_run_trace_lane_text(&delta)
    }

    /// 🧹️ Forgets scene windows that closed or stopped hosting a scene surface.
    pub fn retain_trace_windows(&mut self, live: impl Fn(&str) -> bool) {
        self.trace_windows.retain(|window_id, _| live(window_id));
    }

    /// 🎯️ The minimal UI scope a run change dirties: the ToolRun panel body, the body of every scene window its trace
    /// lane rides in and the body of every window kind the run declares it renders in (`ToolRunDefinition::windows`) —
    /// never unrelated windows or panels.
    pub fn dirty_scope(&self) -> UiDirtyScope {
        let mut window_bodies: Vec<String> = self.trace_windows.values().map(|window| window.body_key.clone()).chain(self.entry.iter().flat_map(|entry| entry.window_bodies.iter().cloned())).collect();
        window_bodies.sort_unstable();
        window_bodies.dedup();
        UiDirtyScope::Partial { window_bodies, panel_bodies: vec![FRAMEWORK_TOOL_RUN_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    /// 🚩️ Whether a run change still owes the host a dirty scope.
    pub fn is_ui_dirty(&self) -> bool {
        self.ui_dirty || self.document_dirty
    }

    fn apply_event(&mut self, event: ToolRunEvent) -> Result<ToolRunEffect, ToolRunRejection> {
        let transition = ToolRunMachine::apply(self.slot(), event)?;
        match (self.entry.as_mut(), transition.slot) {
            (Some(entry), Some(slot)) => {
                let generation_changed = entry.slot.generation != slot.generation;
                entry.slot = slot;
                if generation_changed {
                    entry.rebind();
                }
            }
            (Some(_), None) => {
                let entry = self.entry.take().expect("slot present");
                self.retire_entry(*entry);
            }
            (None, _) => {}
        }
        Ok(transition.effect)
    }

    fn retire_entry(&mut self, entry: ToolRunEntry<A>) {
        let ToolRunEntry { job, provisional, finalize, .. } = entry;
        self.retire_owners(job, provisional, finalize);
    }

    fn retire_owners(&mut self, job: Option<ToolRunJobSlot<A::Config>>, mut provisional: Vec<A::Mutation>, finalize: Option<ToolRunFinalize<A>>) {
        if let Some(mut job) = job {
            job.begin_close();
            self.retired_jobs.push(job);
        }
        self.discarded.append(&mut provisional);
        if let Some(publication) = finalize.and_then(|finalize| finalize.publication) {
            self.retired_publications.push(publication);
        }
    }

    fn close_current_job(&mut self) {
        let Some(entry) = self.entry.as_mut() else { return };
        entry.settle_refold();
        if let Some(mut job) = entry.job.take() {
            job.begin_close();
            self.retired_jobs.push(job);
        }
    }

    /// 🎯️ Hands the current job the entry's identity through `retarget`; a job that is not retargetable, or
    /// declines, is closed so the driver rebuilds it from its checkpoint.
    fn retarget_current_job(&mut self, retarget: impl FnOnce(&mut dyn ToolRunRetargetableJob<A::Config>, ToolRunIdentity) -> bool) {
        let Some(entry) = self.entry.as_mut() else { return };
        let identity = entry.identity;
        let Some(slot) = entry.job.as_mut() else { return };
        if slot.retargetable().is_some_and(|job| retarget(job, identity)) {
            slot.generation = semio_framework_job::Generation(u64::from(identity.generation));
        } else {
            self.close_current_job();
        }
    }

    fn discard_provisional(&mut self) {
        let Some(entry) = self.entry.as_mut() else { return };
        self.discarded.append(&mut entry.provisional);
        entry.entity_marks.clear();
        entry.entities = Arc::new(BTreeSet::new());
        entry.refold = None;
        entry.overlay = Arc::clone(&entry.base);
        entry.checkpoint = None;
        entry.payload = None;
    }

    /// 🧹️ Advances the owners that outlived their slot by one bounded unit; `None` when nothing is retiring.
    fn retire_step(&mut self, store: &mut ArtifactStore<A::Snapshot, A::Mutation>, maximum_items: usize, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
        if let Some(job) = self.retired_jobs.last_mut() {
            return match job.close_step(maximum_items, maximum_bytes)? {
                Some(step) => Ok(Some(step)),
                None => {
                    self.retired_jobs.pop();
                    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }))
                }
            };
        }
        if let Some(publication) = self.retired_publications.last_mut() {
            store.cancel_apply_batch(publication);
            publication.begin_close();
            return match publication.close_step(TOOL_RUN_PUBLICATION_GRANT).map_err(plugin_sdk_fault)? {
                store::SnapshotRetirementStep::Complete if publication.terminal_is_empty() => {
                    self.retired_publications.pop();
                    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }))
                }
                store::SnapshotRetirementStep::Complete => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.publication-close"), "tool run publication closed without its terminal-empty witness")),
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(Some(PluginCloseStep::Pending { released_items, released_bytes })),
                store::SnapshotRetirementStep::Blocked => Ok(Some(PluginCloseStep::Blocked { reason: "tool run publication close is blocked" })),
            };
        }
        if !self.discarded.is_empty() {
            let count = self.discarded.len().min(maximum_items.max(1)).min(TOOL_RUN_DISCARD_OPS_PER_TURN);
            for op in self.discarded.drain(self.discarded.len() - count..) {
                op.retire_cold();
            }
            return Ok(Some(PluginCloseStep::Pending { released_items: count, released_bytes: 0 }));
        }
        Ok(None)
    }

    /// 🚪️ Document close, instance retirement or reload: same as abort, ephemeral state vanishes (§2.2).
    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        let _ = self.apply_event(ToolRunEvent::Closed);
    }

    /// 🧹️ One bounded close unit of every retiring owner.
    pub fn close_step(&mut self, store: &mut ArtifactStore<A::Snapshot, A::Mutation>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        self.begin_close();
        Ok(self.retire_step(store, maximum_items, maximum_bytes)?.unwrap_or(PluginCloseStep::Complete))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.entry.is_none() && self.retired_jobs.is_empty() && self.retired_publications.is_empty() && self.discarded.is_empty()
    }
}
//#endregion 🔖️Ledger

//#region 🔖️Driver
/// 🎛️ Outcome of one tool run action dispatch; rejections are silent no-ops (§2.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolRunActionOutcome {
    Applied(ToolRunEffect),
    Rejected(ToolRunRejection),
}

/// 🔢️ One §2.5 action argument read as a `u64`, in every shape the wire can hand it over.
///
/// 🪪️ A JSON number that is an exact non-negative integer IS a `u64` here, and reading it as anything
/// else is not strictness, it is a silent refusal. `toolRunFinalize`'s `generation` crosses as
/// `Number(Float(1.0))` — the shell's own action arguments build it with `UiValue::Number(f64)`, and a
/// guest that mints it with `DslValue::uint` arrives the same way — so accepting only an exact `u64`
/// carrier or a numeric string made `generation` read `None`, and `(_, None)` is
/// `ToolRunRejection::Stale`. EVERY finalize of EVERY app was rejected: runs reached `Complete` and
/// stayed there, and because a start against a live run is `ToolRunRejection::Busy`, no tool run could
/// ever be started a second time. Measured on 6018 as a 3d preview that never re-evaluated after the
/// first evaluation of a session (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
pub(crate) fn tool_run_arg_u64(args: Option<&DslValue>, key: &str) -> Option<u64> {
    let value = args?.get(key)?;
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
        .or_else(|| value.as_f64().filter(|number| number.is_finite() && *number >= 0.0 && number.fract() == 0.0 && *number < 18_446_744_073_709_551_616.0).map(|number| number as u64))
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🏃️ Ledger work plus an unobserved store or settings generation change the watch still owes.
    pub(crate) fn tool_run_has_pending_work(&self) -> bool {
        self.tool_runs.has_pending_work()
            || self.tool_runs.is_ui_dirty()
            || self.tool_runs.entry.as_ref().is_some_and(|entry| {
                matches!(entry.slot.state, ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete | ToolRunState::Finalizing) && (entry.base_generation != self.store.generation() || entry.settings_generation != self.tool_run_settings_generation())
            })
    }

    /// 🎚️ The committed settings generations the driver watches (config store, window-config publications).
    fn tool_run_settings_generation(&self) -> (u64, u64) {
        (self.config_store.generation(), self.tool_runs.driver.window_config_publications)
    }

    /// ⏯️ Host-driven routing of the §2.5 actions: applied to the ledger now, never queued behind the run.
    pub(crate) async fn dispatch_tool_run_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
        let outcome = self.apply_tool_run_action(action, args, meta).await?;
        let output = match outcome {
            ToolRunActionOutcome::Applied(effect) => DslValue::Object(vec![("toolRun".into(), DslValue::String(effect.as_str().into()))]),
            ToolRunActionOutcome::Rejected(rejection) => DslValue::Object(vec![("rejected".into(), DslValue::String(rejection.code().into()))]),
        };
        let ui_scope = match outcome {
            ToolRunActionOutcome::Applied(ToolRunEffect::SpawnJob) => UiDirtyScope::Full,
            ToolRunActionOutcome::Applied(_) => self.tool_runs.dirty_scope(),
            ToolRunActionOutcome::Rejected(_) => UiDirtyScope::None,
        };
        let mut result = Self::empty_result(action, meta, Vec::new(), Vec::new(), ui_scope).await;
        result.output = output;
        Ok(result)
    }

    /// ⚖️ Applies one §2.5 action to the ledger and performs the transition's immediate effect. An action that
    /// names no identity at all (neither `runId` nor `generation`, as a keyboard chord dispatches it) targets
    /// the instance's live run at its generation of this dispatch; an identity it does name is checked as is.
    pub async fn apply_tool_run_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> Result<ToolRunActionOutcome, Fault> {
        let Some(action) = ToolRunAction::from_id(action) else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.unknown-action"), format!("'{action}' is not a tool run action")));
        };
        if action == ToolRunAction::Start {
            return self.start_tool_run(args, meta).await;
        }
        let names_identity = args.is_some_and(|args| args.get(TOOL_RUN_ARG_RUN_ID).is_some() || args.get(TOOL_RUN_ARG_GENERATION).is_some());
        let live = if names_identity { None } else { self.tool_runs.slot() };
        let run = live.map_or_else(|| tool_run_arg_u64(args, TOOL_RUN_ARG_RUN_ID).unwrap_or(0), |slot| slot.run);
        let generation = live.map(|slot| slot.generation).or_else(|| tool_run_arg_u64(args, TOOL_RUN_ARG_GENERATION).and_then(|value| u32::try_from(value).ok()));
        let event = match (action, generation) {
            (ToolRunAction::Dismiss, _) => ToolRunEvent::Dismiss { run },
            (_, None) => return Ok(ToolRunActionOutcome::Rejected(ToolRunRejection::Stale)),
            (ToolRunAction::Pause, Some(generation)) if self.tool_runs.state() == Some(ToolRunState::Paused) => ToolRunEvent::Resume { run, generation },
            (ToolRunAction::Pause, Some(generation)) => ToolRunEvent::Pause { run, generation },
            (ToolRunAction::Resume, Some(generation)) => ToolRunEvent::Resume { run, generation },
            (ToolRunAction::Step, Some(generation)) => ToolRunEvent::Step { run, generation },
            (ToolRunAction::Finalize, Some(generation)) => ToolRunEvent::Finalize { run, generation },
            (ToolRunAction::Abort, Some(generation)) => {
                let publishing = self.tool_runs.entry.as_ref().and_then(|entry| entry.finalize.as_ref()).is_some_and(|finalize| finalize.published || finalize.publication.as_ref().is_some_and(|publication| !matches!(publication.phase(), store::ArtifactStoreOneItemPublicationPhase::Preparing | store::ArtifactStoreOneItemPublicationPhase::PreparingCursor | store::ArtifactStoreOneItemPublicationPhase::PreflightingCommit)));
                ToolRunEvent::Abort { run, generation, publishing }
            }
            (ToolRunAction::Start, _) => unreachable!("start handled above"),
        };
        if let Some(entry) = self.tool_runs.entry.as_mut().filter(|entry| action == ToolRunAction::Finalize && entry.slot.run == run) {
            entry.actor = meta.actor.clone();
        }
        let effect = match self.tool_runs.apply_event(event) {
            Ok(effect) => effect,
            Err(rejection) => return Ok(ToolRunActionOutcome::Rejected(rejection)),
        };
        match (effect, self.tool_runs.entry.as_mut()) {
            (ToolRunEffect::DriveOneUnit, Some(entry)) => entry.pending_step = true,
            (ToolRunEffect::StopScheduling, Some(entry)) => entry.pending_step = false,
            (ToolRunEffect::BeginFinalize, Some(entry)) => entry.finalize = Some(ToolRunFinalize { phase: ToolRunFinalizePhase::Pending, publication: None, retracted: 0, published: false }),
            (ToolRunEffect::CloseJob, _) => self.tool_runs.close_current_job(),
            (ToolRunEffect::CancelBatch, _) => {
                self.tool_runs.close_current_job();
                if let Some(publication) = self.tool_runs.entry.as_mut().and_then(|entry| entry.finalize.as_mut()).and_then(|finalize| finalize.publication.as_mut()) {
                    self.store.cancel_apply_batch(publication);
                }
            }
            _ => {}
        }
        Ok(ToolRunActionOutcome::Applied(effect))
    }

    async fn start_tool_run(&mut self, args: Option<&DslValue>, meta: &ActionMeta) -> Result<ToolRunActionOutcome, Fault> {
        let view_state = meta.view_state.as_ref();
        let tool_id = args
            .and_then(|args| args.get(TOOL_RUN_ARG_TOOL_ID))
            .and_then(DslValue::as_str)
            .map(str::to_string)
            .or_else(|| view_state.and_then(|view| view.active_utility_id.clone().filter(|id| self.registry.tool_run(id).is_some()).or_else(|| view.active_tool_id.clone())))
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.tool-id"), "toolRunStart needs a toolId argument or an active tool"))?;
        let definition = self.registry.tool_run(&tool_id).map(|(_, definition)| definition.clone()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.unknown-tool"), format!("tool '{tool_id}' declares no ToolRunDefinition")))?;
        let run = self.tool_runs.next_run;
        let transition = match ToolRunMachine::apply(self.tool_runs.slot(), ToolRunEvent::Start { run }) {
            Ok(transition) => transition,
            Err(rejection) => return Ok(ToolRunActionOutcome::Rejected(rejection)),
        };
        let slot = transition.slot.expect("start yields a slot");
        self.tool_runs.next_run = run.wrapping_add(1);
        if let Some(previous) = self.tool_runs.entry.take() {
            self.tool_runs.retire_entry(*previous);
        }
        self.refresh_cache().await?;
        let (base, _, _) = self.command_cache_inputs();
        let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: self.live_runtime_instance_id.unwrap_or(meta.instance_id), run }, self.store.content_revision());
        let settings_generation = self.tool_run_settings_generation();
        let window_id = args.and_then(|args| args.get(TOOL_RUN_ARG_WINDOW_ID)).and_then(DslValue::as_str).map(str::to_string).or_else(|| view_state.and_then(|view| view.window_id.clone().or_else(|| view.focused_window_id.clone())));
        let window = window_id.and_then(|window_id| view_state?.window_instances.iter().find(|window| window.id == window_id).map(|window| (window.id.clone(), window.window_kind_id.clone())));
        let settings_values = self.tool_run_settings_values(&definition.settings, window.as_ref());
        let window_bodies = definition.windows.iter().filter_map(|window_kind_id| self.registry.window_body_key(window_kind_id).map(str::to_string)).collect();
        self.tool_runs.entry = Some(Box::new(ToolRunEntry {
            tool_id,
            actor: meta.actor.clone(),
            settings_values,
            window,
            window_bodies,
            payload: None,
            trace_keys: ToolRunTraceKeys::default(),
            definition,
            slot,
            identity,
            overlay: Arc::clone(&base),
            base,
            base_generation: self.store.generation(),
            settings_generation,
            provisional: Vec::new(),
            entity_marks: Vec::new(),
            entities: Arc::new(BTreeSet::new()),
            refold: None,
            trace: ToolRunTraceStore::new(identity),
            recent_trace: std::collections::VecDeque::with_capacity(TOOL_RUN_PANEL_TRACE_ROWS),
            stage: 0,
            completed: 0,
            total: None,
            counters: Vec::new(),
            steps: ToolRunStepRing::new(),
            step_sequence: 0,
            sequence: 0,
            conflicts: 0,
            rate_origin: None,
            units_per_second: 0.0,
            job: None,
            checkpoint: None,
            pending_step: false,
            finalize: None,
            announced: None,
        }));
        Ok(ToolRunActionOutcome::Applied(transition.effect))
    }

    /// 🧵️ Builds the run's job for `purpose`: the app's retargetable job when it builds one, else its plain job.
    fn build_tool_run_job_slot(&self, entry: &ToolRunEntry<A>, purpose: ToolRunJobPurpose) -> Result<Option<ToolRunJobSlot<A::Config>>, Fault> {
        let request = || ToolRunJobRequest::<A> {
            tool_id: &entry.tool_id,
            definition: &entry.definition,
            purpose,
            identity: entry.identity,
            snapshot: match purpose {
                ToolRunJobPurpose::Run => Arc::clone(&entry.base),
                ToolRunJobPurpose::Revalidate => self.store.snapshot_owner(),
            },
            config: self.config_store.snapshot_owner(),
            window_id: entry.window.as_ref().map(|(window_id, _)| window_id.as_str()),
            window_config: entry.window.as_ref().and_then(|(window_id, window_kind_id)| self.window_config_store.snapshot(window_kind_id, window_id)),
            checkpoint: entry.checkpoint.as_deref(),
            provisional: &entry.provisional,
            instance_owner: self.instance_operation_owner.clone(),
            port: self.tool_runs.port.clone(),
            trace_keys: entry.trace_keys.clone(),
            entity_marks: &entry.entity_marks,
        };
        let generation = entry.identity.generation;
        if let Some(job) = A::build_retargetable_tool_run_job(request())? {
            return Ok(Some(ToolRunJobSlot::new(ToolRunJobHandle::Retargetable(job), purpose, generation)));
        }
        Ok(A::build_tool_run_job(request())?.map(|job| ToolRunJobSlot::new(ToolRunJobHandle::Plain(job), purpose, generation)))
    }

    /// 🎚️ What the run's declared settings reads name right now. A declared window kind that is the starting
    /// window's reads only that window's config, the one its jobs receive; any other declared kind reads every
    /// partition of that kind.
    fn tool_run_settings_values(&self, reads: &ToolRunSettingsReads, window: Option<&(String, String)>) -> ToolRunSettingsValues {
        let config = if reads.config.is_empty() {
            Vec::new()
        } else {
            let document = protocol::ToValue::to_value(self.config_store.snapshot_owner().as_ref());
            reads.config.iter().map(|pointer| tool_run_pointer_value(&document, pointer).cloned()).collect()
        };
        let window_config = reads
            .window_config
            .iter()
            .flat_map(|(window_kind_id, pointers)| {
                let started = window.filter(|(_, kind)| kind == window_kind_id).map(|(window_id, _)| window_id.as_str());
                self.window_config_store.pointer_values(window_kind_id, pointers).into_iter().filter(move |(window_id, _)| started.is_none_or(|started| started == window_id)).map(move |(window_id, values)| (window_kind_id.clone(), window_id, values))
            })
            .collect();
        ToolRunSettingsValues { config, window_config }
    }

    fn apply_tool_run_driver_event(&mut self, event: ToolRunEvent) -> Option<ToolRunEffect> {
        self.tool_runs.apply_event(event).ok()
    }

    /// 📮️ Hands the host every effect jobs queued on the port, in order, while the typed effect outbox has room.
    fn drain_tool_run_port(&mut self) {
        while let Some(effect) = self.tool_runs.port.pop_effect() {
            if let Err(effect) = self.typed_effect_outbox.push(effect) {
                self.tool_runs.port.restore_effect(effect);
                return;
            }
        }
    }

    /// 🎯️ A tick-sized change: progress, steps, trace and provisional appends reach the panel and the scene
    /// windows only (`ToolRunLedger::dirty_scope`).
    fn mark_tool_run_ui_dirty(&mut self) {
        self.tool_runs.ui_dirty = true;
        self.flush_tool_run_ui_dirty();
    }

    /// 📄️ The document every window reads was swapped wholesale (overlay refolded, released or discarded), so
    /// every body is dirty once.
    fn mark_tool_run_document_dirty(&mut self) {
        self.tool_runs.document_dirty = true;
        self.flush_tool_run_ui_dirty();
    }

    /// 📮️ Hands the owed scope to the typed UI outbox once it has room; until then the flags stay set, so a
    /// queued unrelated scope never swallows a run change.
    pub(crate) fn flush_tool_run_ui_dirty(&mut self) {
        if !self.tool_runs.is_ui_dirty() || self.typed_ui_outbox.len() != 0 {
            return;
        }
        let scope = if self.tool_runs.document_dirty { UiDirtyScope::Full } else { self.tool_runs.dirty_scope() };
        if self.typed_ui_outbox.push(scope).is_ok() {
            self.tool_runs.ui_dirty = false;
            self.tool_runs.document_dirty = false;
        }
    }

    /// 🎬️ Scene lane injection (§3.2, §4.1 layer 2): the first World3d, Canvas2d or Board2d scene surface a window body
    /// renders carries the `toolRunTrace` lane answered for the cursor that window's renderer echoed.
    /// Panel bodies, override renders and bodies without a scene surface carry nothing.
    pub(crate) fn inject_tool_run_trace_lane(&mut self, tree: &mut ComponentTree, body_key: &str, view_state: &ViewModel) -> Result<(), Fault> {
        let Some(window_id) = view_state.window_id.as_deref() else { return Ok(()) };
        self.tool_runs.retain_trace_windows(|live| view_state.window_instances.iter().any(|window| window.id == live));
        let Some(surface) = tool_run_scene_surface(&mut tree.root) else {
            self.tool_runs.trace_windows.remove(window_id);
            return Ok(());
        };
        let cursor = view_state.tool_run_trace_cursor_by_window_id.get(window_id).copied();
        let Some(lane) = self.tool_runs.trace_lane(window_id, body_key, cursor, TOOL_RUN_TRACE_DELTA_BYTES) else { return Ok(()) };
        inject_tool_run_trace_lane_into(surface, &lane).map_err(|error| plugin_sdk_fault(error.to_string()))
    }

    /// 👀️ Store and settings watch (§3.3): emits `baseChanged` / `settingsChanged` with their policies.
    ///
    /// - A base change rebinds a retargetable job to the new identity in place; any other job is closed and
    ///   rebuilt under the new identity (`restart` also discards the provisional ops), so no tick of the old
    ///   identity is ever produced.
    /// - A config or window-config publication is only a settings change when a value behind one of the run's
    ///   declared `settings` pointers changed; `resume` then retargets a retargetable job in place and rebuilds
    ///   any other from its checkpoint.
    fn watch_tool_run_generations(&mut self) {
        let store_generation = self.store.generation();
        let settings_generation = self.tool_run_settings_generation();
        let Some(entry) = self.tool_runs.entry.as_ref() else { return };
        let (run, state, rebase, reconfigure) = (entry.slot.run, entry.slot.state, entry.definition.rebase, entry.definition.reconfigure);
        let base_changed = entry.base_generation != store_generation;
        let settings_moved = entry.settings_generation != settings_generation;
        if !matches!(state, ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete) {
            return;
        }
        if base_changed && self.apply_tool_run_driver_event(ToolRunEvent::BaseChanged { run }) == Some(ToolRunEffect::Refold) {
            let head = self.store.snapshot_owner();
            let restart = rebase == ToolRunRebasePolicy::Restart;
            if restart {
                self.tool_runs.close_current_job();
                self.tool_runs.discard_provisional();
            }
            let entry = self.tool_runs.entry.as_mut().expect("refold keeps the slot");
            entry.base = head;
            entry.base_generation = store_generation;
            entry.identity.base_revision = self.store.content_revision();
            entry.trace.rebind(entry.identity);
            if restart {
                entry.overlay = Arc::clone(&entry.base);
            } else {
                entry.begin_refold(true);
            }
            entry.framework_step(ToolRunStepKind::Warning, TOOL_RUN_REASON_REBASING, &[]);
            if !restart {
                self.tool_runs.retarget_current_job(|job, identity| {
                    job.rebind(identity);
                    true
                });
            }
            self.mark_tool_run_document_dirty();
        }
        if !settings_moved {
            return;
        }
        let entry = self.tool_runs.entry.as_ref().expect("the watch keeps the slot");
        let values = self.tool_run_settings_values(&entry.definition.settings, entry.window.as_ref());
        let entry = self.tool_runs.entry.as_mut().expect("the watch keeps the slot");
        entry.settings_generation = settings_generation;
        if values == entry.settings_values {
            return;
        }
        entry.settings_values = values;
        if self.apply_tool_run_driver_event(ToolRunEvent::SettingsChanged { run }) != Some(ToolRunEffect::Reconfigure) {
            return;
        }
        match reconfigure {
            ToolRunReconfigurePolicy::Restart => {
                self.tool_runs.close_current_job();
                self.tool_runs.discard_provisional();
                self.mark_tool_run_document_dirty();
            }
            ToolRunReconfigurePolicy::Resume => {
                let config = self.config_store.snapshot_owner();
                self.tool_runs.retarget_current_job(|job, identity| job.reconfigure(identity, config));
                self.mark_tool_run_ui_dirty();
            }
        }
    }

    /// ⏯️ One bounded driver turn (≤ `ToolRunDriver::turn_wall_us`): owed dirty scope, retirement, watch, refold,
    /// job steps, finalize.
    pub(crate) async fn drive_tool_run_turn(&mut self) -> Result<(), Fault> {
        self.flush_tool_run_ui_dirty();
        self.drain_tool_run_port();
        let started = semio_framework_job::default_now_us().unwrap_or(0);
        let deadline = started.saturating_add(self.tool_runs.driver.turn_wall_us);
        if let Some(step) = self.tool_runs.retire_step(&mut self.store, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)? {
            if !matches!(step, PluginCloseStep::Blocked { .. }) {
                return Ok(());
            }
        }
        let Some(state) = self.tool_runs.state() else { return Ok(()) };
        match state {
            ToolRunState::Aborting => {
                if self.tool_runs.retired_jobs.is_empty() && self.tool_runs.retired_publications.is_empty() {
                    let (run, generation) = self.tool_runs.slot().map(|slot| (slot.run, slot.generation)).expect("aborting slot");
                    if let Some(finalize) = self.tool_runs.entry.as_mut().and_then(|entry| entry.finalize.take()) {
                        self.tool_runs.retire_owners(None, Vec::new(), Some(finalize));
                        return Ok(());
                    }
                    if self.apply_tool_run_driver_event(ToolRunEvent::AbortComplete { run, generation }) == Some(ToolRunEffect::RetireProvisional) {
                        self.tool_runs.discard_provisional();
                        self.mark_tool_run_document_dirty();
                    }
                }
                Ok(())
            }
            ToolRunState::Starting => self.admit_tool_run_job().await,
            ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete => {
                self.watch_tool_run_generations();
                if self.refold_tool_run(deadline) {
                    return Ok(());
                }
                let entry = self.tool_runs.entry.as_ref().expect("live slot");
                let wants_job = match entry.slot.state {
                    ToolRunState::Running => true,
                    ToolRunState::Paused => entry.pending_step,
                    _ => false,
                };
                if wants_job && entry.job.is_none() {
                    return self.admit_tool_run_job().await;
                }
                if wants_job {
                    self.step_tool_run_job(deadline)?;
                }
                Ok(())
            }
            ToolRunState::Finalizing => self.finalize_tool_run_turn(deadline).await,
            ToolRunState::Finalized | ToolRunState::Aborted | ToolRunState::Faulted => Ok(()),
        }
    }

    /// 🔁️ `true` while a refold still owns this turn; a refold waiting only for its job's boundary lets the job step.
    fn refold_tool_run(&mut self, deadline: u64) -> bool {
        let Some(entry) = self.tool_runs.entry.as_mut().filter(|entry| entry.refold.is_some()) else { return false };
        if entry.job.is_none() {
            entry.settle_refold();
        }
        match entry.refold_turn(deadline) {
            ToolRunRefoldTurn::Pending => true,
            ToolRunRefoldTurn::AwaitingBoundary => false,
            ToolRunRefoldTurn::Swapped => {
                self.mark_tool_run_document_dirty();
                false
            }
        }
    }

    async fn admit_tool_run_job(&mut self) -> Result<(), Fault> {
        let Some(entry) = self.tool_runs.entry.as_ref() else { return Ok(()) };
        let (run, generation, state) = (entry.slot.run, entry.slot.generation, entry.slot.state);
        self.tool_runs.port.wake();
        match self.build_tool_run_job_slot(entry, ToolRunJobPurpose::Run) {
            Ok(Some(job)) => {
                self.tool_runs.entry.as_mut().expect("live slot").job = Some(job);
                if state == ToolRunState::Starting {
                    self.apply_tool_run_driver_event(ToolRunEvent::JobAdmitted { run, generation });
                }
            }
            Ok(None) | Err(_) => self.fault_tool_run(run, generation),
        }
        self.mark_tool_run_ui_dirty();
        Ok(())
    }

    fn fault_tool_run(&mut self, run: u64, generation: u32) {
        if self.apply_tool_run_driver_event(ToolRunEvent::JobFault { run, generation }) == Some(ToolRunEffect::DiscardProvisional) {
            self.tool_runs.close_current_job();
            self.tool_runs.discard_provisional();
            if let Some(entry) = self.tool_runs.entry.as_mut() {
                entry.framework_step(ToolRunStepKind::Danger, TOOL_RUN_REASON_CONFLICT, &[ToolRunStepArg::Unsigned(0)]);
            }
            self.mark_tool_run_document_dirty();
        }
    }

    /// 🦶️ Drives the current job until the turn deadline, a pause, a single step or a terminal outcome.
    fn step_tool_run_job(&mut self, deadline: u64) -> Result<(), Fault> {
        loop {
            let Some(entry) = self.tool_runs.entry.as_mut() else { return Ok(()) };
            let single = entry.slot.state == ToolRunState::Paused;
            let (run, generation, purpose) = (entry.slot.run, entry.slot.generation, entry.job.as_ref().map(|job| job.purpose));
            let Some(job) = entry.job.as_mut() else { return Ok(()) };
            let now = semio_framework_job::default_now_us().unwrap_or(0);
            let fuel = if single { 1 } else { semio_framework_job::INTERACTIVE_LANE_FUEL };
            let budget = semio_framework_job::StepBudget::from_duration(fuel, now, semio_framework_job::INTERACTIVE_LANE_WALL_US).unwrap_or(semio_framework_job::StepBudget::new(fuel, u64::MAX));
            let mut verdict = None;
            let interactive: &mut dyn semio_framework_job::InteractiveJob = match &mut job.job {
                ToolRunJobHandle::Plain(job) => job.as_mut(),
                ToolRunJobHandle::Retargetable(job) => job.as_mut(),
            };
            let mut outcome = semio_framework_job::drive_step(interactive, TOOL_RUN_JOB_SITE, job.operation, job.generation, semio_framework_job::InteractiveStage::InteractiveStep, budget, job.cancel.clone(), semio_framework_job::default_now_us, &mut job.preview_sequence, &mut verdict);
            if single && !matches!(outcome, semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::CheckpointReady(_)) {
                entry.pending_step = false;
            }
            let mut terminal = false;
            match &mut outcome {
                semio_framework_job::StepOutcome::Yield => {}
                semio_framework_job::StepOutcome::PreviewReady(payload) => {
                    let bytes = job_payload_bytes(payload, TOOL_RUN_TICK_BYTES_MAX);
                    close_job_payload(payload);
                    let tick = bytes.ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.tick-bytes"), "tool run tick exceeds its byte cap")).and_then(|bytes| ToolRunTick::decode(&bytes).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.tick-decode"), format!("{error:?}"))));
                    let receipt = match tick {
                        Ok(tick) => entry.apply_tick(tick, &mut self.tool_runs.discarded),
                        Err(fault) => Err(fault),
                    };
                    match receipt {
                        Ok(receipt) => {
                            if let (Some(finalize), ToolRunJobPurpose::Revalidate) = (entry.finalize.as_mut(), purpose.unwrap_or(ToolRunJobPurpose::Run)) {
                                finalize.retracted = finalize.retracted.saturating_add(receipt.retracted);
                            }
                            if receipt.capped && purpose == Some(ToolRunJobPurpose::Run) {
                                self.complete_tool_run_job(run, generation);
                                terminal = true;
                            }
                            self.mark_tool_run_ui_dirty();
                        }
                        Err(_) => {
                            self.fault_tool_run(run, generation);
                            terminal = true;
                        }
                    }
                }
                semio_framework_job::StepOutcome::CheckpointReady(checkpoint) => {
                    entry.checkpoint = job_payload_bytes(&checkpoint.state, semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES);
                    close_job_payload(&mut checkpoint.state);
                    entry.settle_refold();
                }
                semio_framework_job::StepOutcome::Complete(_) => {
                    close_step_outcome(&mut outcome);
                    entry.settle_refold();
                    match purpose {
                        Some(ToolRunJobPurpose::Revalidate) => self.complete_tool_run_revalidation(),
                        _ => self.complete_tool_run_job(run, generation),
                    }
                    terminal = true;
                }
                semio_framework_job::StepOutcome::Cancelled | semio_framework_job::StepOutcome::Fault(_) => {
                    close_step_outcome(&mut outcome);
                    self.fault_tool_run(run, generation);
                    terminal = true;
                }
            }
            self.drain_tool_run_port();
            if terminal || single || self.tool_runs.port.is_waiting() || semio_framework_job::default_now_us().is_none_or(|now| now >= deadline) {
                return Ok(());
            }
            if !self.tool_runs.entry.as_ref().is_some_and(|entry| entry.job.is_some() && matches!(entry.slot.state, ToolRunState::Running | ToolRunState::Finalizing)) {
                return Ok(());
            }
        }
    }

    /// 🏁️ A finished job's completion — and, for a READ-ONLY run, its finalization too.
    ///
    /// 🪪️ `Complete` means "the job is done and the run awaits the finalize that publishes its
    /// provisional edits". A run that declares `mutating: false` authors no provisional edit at all, so
    /// there is nothing for a finalize to publish and nothing for a user to review: leaving it parked in
    /// `Complete` only holds the tool's single run slot open, and a `toolRunStart` against a non-terminal
    /// run is `ToolRunRejection::Busy`. That is how generation3d's read-only `previewEval` ran exactly
    /// ONCE per session — every later start, whether a poll's or a gesture's own, was refused by the
    /// finished run still sitting in the slot, and the 3d preview stopped re-evaluating
    /// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`, `📋️tool-run-contract.md` §2.2). Asking a
    /// read-only run's owner to dispatch a finalize it has nothing to finalize is a round trip the host
    /// can always lose: it happens on the host's own refresh cadence, and a run action that publishes
    /// nothing moves no UI that would provoke the next pass.
    fn complete_tool_run_job(&mut self, run: u64, generation: u32) {
        let resident = self.tool_runs.entry.as_mut().is_some_and(|entry| entry.definition.mutating && entry.job.as_mut().is_some_and(|job| job.retargetable().is_some()));
        if resident {
            self.tool_runs.entry.as_mut().expect("a resident job has a slot").settle_refold();
        } else {
            self.tool_runs.close_current_job();
        }
        self.apply_tool_run_driver_event(ToolRunEvent::JobComplete { run, generation });
        if let Some(entry) = self.tool_runs.entry.as_mut() {
            entry.pending_step = false;
        }
        if self.tool_runs.entry.as_ref().is_some_and(|entry| !entry.definition.mutating && entry.slot.state == ToolRunState::Complete) {
            let (run, generation) = self.tool_runs.slot().map(|slot| (slot.run, slot.generation)).expect("a complete run has a slot");
            if let Some(ToolRunEffect::BeginFinalize) = self.apply_tool_run_driver_event(ToolRunEvent::Finalize { run, generation }) {
                if let Some(entry) = self.tool_runs.entry.as_mut() {
                    entry.finalize = Some(ToolRunFinalize { phase: ToolRunFinalizePhase::Pending, publication: None, retracted: 0, published: false });
                }
            }
        }
        self.mark_tool_run_document_dirty();
    }

    fn complete_tool_run_revalidation(&mut self) {
        self.tool_runs.close_current_job();
        let Some(entry) = self.tool_runs.entry.as_mut() else { return };
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        let retracted = entry.finalize.as_ref().map_or(0, |finalize| finalize.retracted);
        if retracted == 0 {
            if let Some(finalize) = entry.finalize.as_mut() {
                finalize.phase = ToolRunFinalizePhase::Publishing;
            }
            return;
        }
        entry.conflicts = entry.conflicts.saturating_add(retracted);
        entry.framework_step(ToolRunStepKind::Danger, TOOL_RUN_REASON_CONFLICT, &[ToolRunStepArg::Unsigned(u64::from(retracted))]);
        let finalize = entry.finalize.take();
        self.tool_runs.retire_owners(None, Vec::new(), finalize);
        self.apply_tool_run_driver_event(ToolRunEvent::RevalidationConflicts { run, generation });
        self.mark_tool_run_document_dirty();
    }

    /// 🏁️ Finalize (§2.7.4): freshness refold on the head, optional revalidate job, one outbound batched `Edit`.
    async fn finalize_tool_run_turn(&mut self, deadline: u64) -> Result<(), Fault> {
        let store_generation = self.store.generation();
        let entry = self.tool_runs.entry.as_mut().expect("finalizing slot");
        let phase = entry.finalize.as_ref().map_or(ToolRunFinalizePhase::Pending, |finalize| finalize.phase);
        if entry.base_generation != store_generation && matches!(phase, ToolRunFinalizePhase::Pending | ToolRunFinalizePhase::Revalidating) {
            entry.base = self.store.snapshot_owner();
            entry.base_generation = store_generation;
            entry.begin_refold(true);
            entry.framework_step(ToolRunStepKind::Warning, TOOL_RUN_REASON_REBASING, &[]);
            if let Some(finalize) = entry.finalize.as_mut() {
                finalize.phase = ToolRunFinalizePhase::Pending;
                finalize.retracted = 0;
            }
            self.tool_runs.close_current_job();
            return Ok(());
        }
        if self.refold_tool_run(deadline) {
            return Ok(());
        }
        match phase {
            ToolRunFinalizePhase::Pending => {
                if self.tool_runs.entry.as_ref().is_some_and(|entry| entry.job.as_ref().is_some_and(|job| job.purpose == ToolRunJobPurpose::Run)) {
                    self.tool_runs.close_current_job();
                    return Ok(());
                }
                let entry = self.tool_runs.entry.as_ref().expect("finalizing slot");
                let (run, generation) = (entry.slot.run, entry.slot.generation);
                let next = if entry.definition.revalidate_job.is_some() && !entry.provisional.is_empty() {
                    match self.build_tool_run_job_slot(entry, ToolRunJobPurpose::Revalidate) {
                        Ok(Some(job)) => {
                            self.tool_runs.entry.as_mut().expect("finalizing slot").job = Some(job);
                            ToolRunFinalizePhase::Revalidating
                        }
                        Ok(None) | Err(_) => return self.reject_tool_run_publication(run, generation),
                    }
                } else {
                    ToolRunFinalizePhase::Publishing
                };
                if let Some(finalize) = self.tool_runs.entry.as_mut().and_then(|entry| entry.finalize.as_mut()) {
                    finalize.phase = next;
                }
                Ok(())
            }
            ToolRunFinalizePhase::Revalidating => self.step_tool_run_job(deadline),
            ToolRunFinalizePhase::Publishing => self.publish_tool_run(deadline).await,
            ToolRunFinalizePhase::Closing => self.close_tool_run_publication(),
        }
    }

    fn reject_tool_run_publication(&mut self, run: u64, generation: u32) -> Result<(), Fault> {
        let Some(entry) = self.tool_runs.entry.as_mut() else { return Ok(()) };
        entry.framework_step(ToolRunStepKind::Danger, TOOL_RUN_REASON_CONFLICT, &[ToolRunStepArg::Unsigned(entry.provisional.len() as u64)]);
        let finalize = entry.finalize.take();
        self.tool_runs.close_current_job();
        self.tool_runs.retire_owners(None, Vec::new(), finalize);
        self.apply_tool_run_driver_event(ToolRunEvent::StoreRejected { run, generation });
        self.mark_tool_run_ui_dirty();
        Ok(())
    }

    async fn publish_tool_run(&mut self, deadline: u64) -> Result<(), Fault> {
        let entry = self.tool_runs.entry.as_mut().expect("finalizing slot");
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        if entry.provisional.is_empty() {
            entry.finalize.as_mut().expect("finalize owner").phase = ToolRunFinalizePhase::Closing;
            return Ok(());
        }
        if entry.finalize.as_ref().is_some_and(|finalize| finalize.publication.is_none()) {
            let description = self.registry.tool_run(&entry.tool_id).map(|(label, _)| label.resolve(Terminology::Native, Locale::En).to_string()).unwrap_or_else(|| entry.tool_id.clone());
            self.store.set_local_actor_id(Some(entry.actor.clone())).map_err(|error| error.into_fault())?;
            match self.store.begin_outbound_apply_batch(semio_framework_job::allocate_operation_id(), entry.base_generation, self.store.content_revision(), entry.actor.clone(), entry.provisional.clone(), Some(description), self.artifact_one_item_factory.as_ref()) {
                Ok(publication) => entry.finalize.as_mut().expect("finalize owner").publication = Some(publication),
                Err(_) => return self.reject_tool_run_publication(run, generation),
            }
        }
        loop {
            let entry = self.tool_runs.entry.as_mut().expect("finalizing slot");
            let publication = entry.finalize.as_mut().and_then(|finalize| finalize.publication.as_mut()).expect("admitted publication");
            match self.store.advance_apply_batch(publication, TOOL_RUN_PUBLICATION_GRANT) {
                Ok(store::ArtifactStoreOneItemAdvance::Published(_)) => {
                    let group_id = entry.identity.id.group_id();
                    let tool_id = entry.tool_id.clone();
                    let actor = entry.actor.clone();
                    self.store.stamp_tail_group_id(&group_id).await.map_err(|error| error.into_fault())?;
                    let publication = self.tool_runs.entry.as_mut().and_then(|entry| entry.finalize.as_mut()).and_then(|finalize| finalize.publication.as_mut()).expect("published publication");
                    self.store.flush_published_apply_batch(publication).await.map_err(|error| error.into_fault())?;
                    publication.acknowledge();
                    let description = self.store.envelope().vcs.edits.last().and_then(|edit| edit.description.clone());
                    let edit_id = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
                    self.record_command(&tool_id, ActionKind::Mutation, description, edit_id, None, None).await;
                    self.revalidate_interaction_state_after_document_change(&ActionMeta { actor, instance_id: self.live_runtime_instance_id.unwrap_or(1), view_state: None }).await?;
                    let entry = self.tool_runs.entry.as_mut().expect("finalizing slot");
                    let finalize = entry.finalize.as_mut().expect("finalize owner");
                    finalize.published = true;
                    finalize.phase = ToolRunFinalizePhase::Closing;
                    return Ok(());
                }
                Ok(store::ArtifactStoreOneItemAdvance::Blocked) => return Ok(()),
                Ok(_) => {}
                Err(_) => return self.reject_tool_run_publication(run, generation),
            }
            if semio_framework_job::default_now_us().is_none_or(|now| now >= deadline) {
                return Ok(());
            }
        }
    }

    fn close_tool_run_publication(&mut self) -> Result<(), Fault> {
        let entry = self.tool_runs.entry.as_mut().expect("finalizing slot");
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        let finalize = entry.finalize.as_mut().expect("finalize owner");
        if let Some(publication) = finalize.publication.as_mut() {
            match publication.close_step(TOOL_RUN_PUBLICATION_GRANT).map_err(plugin_sdk_fault)? {
                store::SnapshotRetirementStep::Complete if publication.terminal_is_empty() => finalize.publication = None,
                store::SnapshotRetirementStep::Complete => return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.publication-close"), "tool run publication closed without its terminal-empty witness")),
                _ => return Ok(()),
            }
            return Ok(());
        }
        entry.finalize = None;
        if self.apply_tool_run_driver_event(ToolRunEvent::PublicationComplete { run, generation }) == Some(ToolRunEffect::ReleaseProvisional) {
            let store_generation = self.store.generation();
            let head = self.store.snapshot_owner();
            self.tool_runs.discard_provisional();
            let entry = self.tool_runs.entry.as_mut().expect("finalized slot");
            entry.base = head;
            entry.overlay = Arc::clone(&entry.base);
            entry.base_generation = store_generation;
        }
        self.mark_tool_run_document_dirty();
        Ok(())
    }
}
//#endregion 🔖️Driver

//#region 🔖️Panel
fn tool_run_step_text(definition: &ToolRunDefinition, step: &ToolRunStep, locale: Locale) -> String {
    let template = match ToolRunLabel::for_reason(step.reason) {
        Some(label) => label.text(locale).to_string(),
        None => definition.reason(step.reason).map_or_else(|| step.reason.to_string(), |reason| reason.template.resolve(Terminology::Native, locale).to_string()),
    };
    let text = tool_run_format(&template, |name| name.parse::<usize>().ok().and_then(|index| step.args.get(index)).map(|arg| arg.to_plain_string()));
    if step.repeat > 1 {
        format!("{text} ×{}", step.repeat)
    } else {
        text
    }
}

fn tool_run_step_tone(kind: ToolRunStepKind) -> Tone {
    match kind {
        ToolRunStepKind::Info => Tone::Info,
        ToolRunStepKind::Success => Tone::Success,
        ToolRunStepKind::Warning => Tone::Warning,
        ToolRunStepKind::Danger => Tone::Danger,
    }
}

fn tool_run_verdict_tone(verdict: ToolRunVerdict) -> Tone {
    match verdict {
        ToolRunVerdict::Testing => Tone::Neutral,
        ToolRunVerdict::Success => Tone::Success,
        ToolRunVerdict::Warning => Tone::Warning,
        ToolRunVerdict::Danger => Tone::Danger,
    }
}

fn tool_run_state_label(state: ToolRunState) -> ToolRunLabel {
    state.label()
}

impl<A: ArtifactApp> ToolRunLedger<A> {
    /// 🪧️ The framework ToolRun panel (§2.6) as a `ComponentTree` root: group, polite status, progressbar,
    /// real buttons with `aria-keyshortcuts`, step log (live off) and a keyboard-navigable trace list.
    pub fn panel(&mut self, controller_id: &str, locale: Locale, tool_label: Option<String>) -> UiAssemblyResult<BuiltNode> {
        let error = ui_assembly_error;
        let Some(entry) = self.entry.as_mut() else {
            return column().try_id("framework.toolRun").map_err(|_| error("tool-run-panel.id"))?.try_build().map_err(|_| error("tool-run-panel.build"));
        };
        let state = entry.slot.state;
        let stage_count = entry.definition.stages.len();
        let stage_label = entry.definition.stage(entry.stage).map_or_else(String::new, |stage| stage.label.resolve(Terminology::Native, locale).to_string());
        let status = format!("{} · {stage_label} ({}/{stage_count})", tool_run_state_label(state).text(locale), usize::from(entry.stage) + 1);
        let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
        let announce = match entry.announced.as_ref() {
            Some((announced_state, at, _)) => *announced_state != state || now_ms.saturating_sub(*at) >= TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS,
            None => true,
        };
        if announce {
            entry.announced = Some((state, now_ms, status));
        }
        let status = entry.announced.as_ref().map(|(_, _, text)| text.clone()).unwrap_or_default();
        let live = if state == ToolRunState::Faulted || entry.conflicts > 0 { Liveness::Assertive } else { Liveness::Polite };
        let status_node = text(Label(UiText::clipped(&status))).live(live).try_id("framework.toolRun.status").map_err(|_| error("tool-run-panel.status-id"))?.try_build().map_err(|_| error("tool-run-panel.status"))?;
        let unit = entry.definition.unit.resolve(Terminology::Native, locale).to_string();
        let value_text = match entry.total {
            Some(total) => {
                let percent = if total == 0 { 100 } else { entry.completed.saturating_mul(100) / total };
                tool_run_format(ToolRunLabel::ProgressValueText.text(locale), |name| match name {
                    "stage" => Some(stage_label.clone()),
                    "i" => Some((usize::from(entry.stage) + 1).to_string()),
                    "n" => Some(stage_count.to_string()),
                    "completed" => Some(entry.completed.to_string()),
                    "total" => Some(total.to_string()),
                    "unit" => Some(unit.clone()),
                    "pct" => Some(percent.to_string()),
                    _ => None,
                })
            }
            None => format!("{stage_label} ({}/{stage_count}): {} {unit}", usize::from(entry.stage) + 1, entry.completed),
        };
        let mut bar = progress(entry.completed as f64, Label(UiText::clipped(&value_text)));
        if let Some(total) = entry.total {
            bar = bar.total(total as f64);
        }
        let bar = bar.try_id("framework.toolRun.progress").map_err(|_| error("tool-run-panel.progress-id"))?.try_build().map_err(|_| error("tool-run-panel.progress"))?;
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        let mut buttons = BuiltChildren::default();
        let toggle = if state == ToolRunState::Paused { ToolRunAction::Resume } else { ToolRunAction::Pause };
        let actions = if state.is_terminal() { vec![ToolRunAction::Start, ToolRunAction::Dismiss] } else { vec![toggle, ToolRunAction::Step, ToolRunAction::Abort, ToolRunAction::Finalize] };
        for action in actions {
            let mut arguments = UiMapBuilder::try_new().ok_or_else(|| error("tool-run-panel.args"))?;
            let pushed = match action {
                ToolRunAction::Start => arguments.push(TOOL_RUN_ARG_TOOL_ID.to_string(), UiValue::Text(UiText::clipped(&entry.tool_id))).is_ok(),
                _ => arguments.push(TOOL_RUN_ARG_RUN_ID.to_string(), UiValue::Text(UiText::clipped(&run.to_string()))).is_ok() && arguments.push(TOOL_RUN_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).is_ok(),
            };
            if !pushed {
                return Err(error("tool-run-panel.args"));
            }
            let action_id = ActionId::try_v1(controller_id, action.id()).ok_or_else(|| error("tool-run-panel.action-id"))?;
            let legal = action.is_legal_in(Some(state));
            let mut builder = button(Label(UiText::clipped(action.label().text(locale)))).disabled(!legal);
            builder = builder.try_id(format!("framework.toolRun.{}", action.id())).map_err(|_| error("tool-run-panel.button-id"))?;
            builder = builder.try_shortcut(action.chord()).map_err(|_| error("tool-run-panel.button-shortcut"))?;
            if action == ToolRunAction::Finalize && !legal {
                builder = builder.try_describe(ToolRunLabel::FinalizeDisabled.text(locale)).map_err(|_| error("tool-run-panel.finalize-description"))?;
            }
            let node = builder.try_on_with(Trigger::Activate, action_id, UiValue::Map(arguments.finish())).map_err(|_| error("tool-run-panel.button-binding"))?.try_build().map_err(|_| error("tool-run-panel.button"))?;
            buttons.try_push(node).map_err(|_| error("tool-run-panel.buttons"))?;
        }
        let toolbar = row().try_id("framework.toolRun.actions").map_err(|_| error("tool-run-panel.actions-id"))?.try_children(buttons).map_err(|_| error("tool-run-panel.actions"))?.try_build().map_err(|_| error("tool-run-panel.actions-build"))?;
        let mut step_rows = BuiltChildren::default();
        let newest_steps: Vec<&ToolRunStep> = entry.steps.iter().collect();
        for step in newest_steps.into_iter().rev().take(TOOL_RUN_PANEL_STEP_ROWS) {
            let row = text(Label(UiText::clipped(&tool_run_step_text(&entry.definition, step, locale)))).tone(tool_run_step_tone(step.kind)).try_id(format!("framework.toolRun.step.{}", step.sequence)).map_err(|_| error("tool-run-panel.step-id"))?.try_build().map_err(|_| error("tool-run-panel.step"))?;
            step_rows.try_push(row).map_err(|_| error("tool-run-panel.steps"))?;
        }
        let steps = column()
            .live(Liveness::Off)
            .try_label(if locale == Locale::De { "Schritte" } else { "Steps" })
            .map_err(|_| error("tool-run-panel.steps-label"))?
            .try_id("framework.toolRun.steps")
            .map_err(|_| error("tool-run-panel.steps-id"))?
            .try_children(step_rows)
            .map_err(|_| error("tool-run-panel.steps-children"))?
            .try_build()
            .map_err(|_| error("tool-run-panel.steps-build"))?;
        let mut trace_rows = BuiltChildren::default();
        for key in entry.recent_trace.iter().rev() {
            let Some(record) = entry.trace.record(*key) else { continue };
            let reason = ToolRunStep { sequence: 0, kind: ToolRunStepKind::Info, stage: entry.stage, reason: record.reason, subject: Some(*key), repeat: 1, args: Vec::new() };
            let item = ui::tree_item(Label(UiText::clipped(&tool_run_step_text(&entry.definition, &reason, locale)))).tone(tool_run_verdict_tone(record.verdict)).try_id(format!("framework.toolRun.trace.{key}")).map_err(|_| error("tool-run-panel.trace-id"))?.try_build().map_err(|_| error("tool-run-panel.trace-item"))?;
            trace_rows.try_push(item).map_err(|_| error("tool-run-panel.trace-rows"))?;
        }
        let trace = tree().try_label(if locale == Locale::De { "Versuche" } else { "Attempts" }).map_err(|_| error("tool-run-panel.trace-label"))?.try_id("framework.toolRun.trace").map_err(|_| error("tool-run-panel.trace-id"))?.try_children(trace_rows).map_err(|_| error("tool-run-panel.trace"))?.try_build().map_err(|_| error("tool-run-panel.trace-build"))?;
        let mut children = BuiltChildren::default();
        for child in [status_node, bar, toolbar, steps, trace] {
            children.try_push(child).map_err(|_| error("tool-run-panel.children"))?;
        }
        let label = tool_label.unwrap_or_else(|| entry.tool_id.clone());
        column().try_label(&label).map_err(|_| error("tool-run-panel.label"))?.try_id("framework.toolRun").map_err(|_| error("tool-run-panel.id"))?.try_children(children).map_err(|_| error("tool-run-panel.children"))?.try_build().map_err(|_| error("tool-run-panel.build"))
    }
}
//#endregion 🔖️Panel
