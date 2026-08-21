//! 🧵️ The universal resumable job protocol for the Semio interactive job runtime: [`InteractiveJob`]
//! is a SYNCHRONOUS, explicitly-resumable `step(&mut StepContext) -> StepOutcome` every interactive
//! operation implements instead of running to completion in one call — the governing rule of design
//! ticket `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR` (packet P2a): "no interactive operation is a
//! function call that runs until the operation is finished; every interactive operation is a
//! persistent state machine whose individual step is bounded, cancellable, observable and
//! preview-producing." [`semio_framework_trace::INTERACTIVE_STEP_CEILING_US`] (8 ms) is the hard
//! ceiling for one `step()` call; 0.5–2 ms is the normal slice.
//!
//! 🚫️async, deliberately: [`InteractiveJob::step`] is NOT `async fn`. Phase 0's census found 88% of
//! this repo's ~53,000 `async fn` never suspend, and marking a CPU loop `async` does not make it
//! cooperative — it still runs to completion in one `poll`. A bounded, resumable step is achieved by
//! RETURNING, not by yielding inside an executor. `async` stays reserved for genuine suspension
//! ([`semio_framework_async::HostAsyncRuntime`], the future-polling layer this crate never touches).
//!
//! 🧬️ **Design inputs**: this module generalizes three existing patterns surveyed in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️p2-design-inputs.md` —
//! `semio_framework_machine`'s persist/restore/step round-trip (count-bounded, no yield, no preview/
//! fault channel — this module adds all three), the actor layer's `Budget`/`TurnStatus`/`Usage`
//! vocabulary (direct fit for [`StepBudget`]/[`StepOutcome`]), and Puzzle 3D's `FillBuilder` precompute
//! session (the proven `applied_count`/two-lane/seeded-RNG template [`Checkpoint::applied_progress`]
//! and [`TortureJob`] generalize). See `📓️p2a-job-protocol.md` in this ticket's Phase 2 folder for the
//! full API writeup, the decisions this file makes, and every deviation from that design doc.
//!
//! 🔗️ **Trace, not a second instrumentation layer**: [`drive_step`] is the ONE place that turns a
//! returned [`StepOutcome`] into a `semio_framework_trace::record_*` call, and wraps every `step()`
//! call in a `semio_framework_trace::Watchdog` — jobs themselves only call [`StepContext::set_stage`]
//! for intra-step stage labels. No parallel preview/checkpoint channel exists; correlation is the
//! trace ring's `(operation, generation)` pair, exactly as the design doc's Decision 4/7 prescribe.
//!
//! ⛓️ **Sync-over-async seam**: [`semio_framework_async::CancelToken`]'s ops are `async fn` even
//! though none of them ever actually suspend (pure atomic loads/stores — the same "88% never suspend"
//! shape this crate's own module doc warns about, in a crate this packet must not edit). Since
//! [`InteractiveJob::step`] is synchronous, [`poll_ready_now`] polls such a future exactly once with a
//! no-op waker and panics on `Pending` — never `semio_framework_async::block_on`, which is explicitly
//! gated to entry points and forbidden on interactive-reachable code by that crate's own doc.

use std::future::Future;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::OnceLock;
use std::task::{Context, Poll, Waker};
use std::time::Instant;

use semio_framework_async::{CancelToken, ChannelPolicy, Lane, WorkerPool};
use semio_framework_trace::{record_cancelled, record_checkpoint, record_committed, record_failed, record_operation_started, record_preview_published, record_stage_changed, InteractiveStage, TraceEvent, Watchdog};

pub use semio_framework_trace::{allocate_operation_id, Generation, OperationId};

//#region 🔁️SyncPoll
/// 🔁️ Polls `fut` exactly once with a no-op waker and returns its output, panicking on `Pending` —
/// see the module doc's "sync-over-async seam" section for why this is safe here (every
/// [`CancelToken`] op is a pure atomic read/write with no real suspension point) and why it is NOT
/// [`semio_framework_async::block_on`] (no parking, no loop, and callable from `step()` itself, which
/// `block_on` explicitly forbids). Private: every public crossing of this seam goes through a named
/// method ([`StepContext::is_cancelled`], [`JobScope::root`], …) so a future upstream change that
/// actually introduces suspension fails loudly here instead of silently spinning.
fn poll_ready_now<F: Future>(fut: F) -> F::Output {
    let mut fut = std::pin::pin!(fut);
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => {
            unreachable!("semio_framework_job::poll_ready_now: a semio_framework_async primitive returned Pending — that crate's CancelToken/CancelState ops are documented pure-atomic (never truly suspend); this invariant broke upstream")
        }
    }
}
//#endregion 🔁️SyncPoll

//#region 🕰️Clock
/// 🕰️ Default millisecond wall clock for callers that don't already own one (tests, the batch
/// adapter's default). Mirrors `semio_framework_trace::now_us`'s per-process monotonic-since-first-
/// call shape, at millisecond rather than microsecond resolution to match [`StepBudget::deadline_ms`]/
/// the actor layer's `Budget::wall_ms`. A host with its own clock (a UI frame clock, a replay clock)
/// supplies its own `fn() -> u64` to [`drive_step`]/[`run_to_completion`] instead of this default.
pub fn default_now_ms() -> u64 {
    static START: OnceLock<Instant> = OnceLock::new();
    START.get_or_init(Instant::now).elapsed().as_millis() as u64
}
//#endregion 🕰️Clock

//#region 🪪️Identity
/// 🧬️ Opaque authoritative-document-revision identity an [`Operation`] is based on — bumped by the
/// model-actor on every committed mutation. A [`CommitCandidate`] is only [`CommitValidation::Accepted`]
/// while both this AND the operation's [`Generation`] still match the live document; see
/// [`validate_commit`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RevisionId(pub u64);

/// 🪪️ Everything identifying one interactive operation across its whole step → preview → checkpoint →
/// commit lifecycle: the trace-correlation [`OperationId`], the authoritative [`RevisionId`] it was
/// based on, its retry/replay [`Generation`], a monotonic preview-sequence cursor (see
/// [`Operation::next_preview_sequence`]) and the deterministic seed every job derives its RNG state
/// from (design doc Decision 5 — seeded at job creation, never re-seeded per step).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Operation {
    pub operation: OperationId,
    pub base_revision: RevisionId,
    pub generation: Generation,
    pub preview_sequence: u64,
    pub seed: u64,
}

impl Operation {
    /// 🌱️ A fresh [`Operation`] with its preview-sequence cursor at zero.
    pub fn new(operation: OperationId, base_revision: RevisionId, generation: Generation, seed: u64) -> Operation {
        Operation { operation, base_revision, generation, preview_sequence: 0, seed }
    }

    /// 🔢️ The next preview sequence number, advancing the cursor — one call per
    /// [`StepOutcome::PreviewReady`] a job for this operation emits.
    pub fn next_preview_sequence(&mut self) -> u64 {
        let sequence = self.preview_sequence;
        self.preview_sequence += 1;
        sequence
    }
}

/// ✅️ Result of [`validate_commit`]: whether a [`CommitCandidate`]'s base revision/generation still
/// match the live document, or the live values it was found stale against — a stale candidate must be
/// explicitly rebased or discarded by the caller, NEVER silently applied (design ticket's governing
/// commit-validation rule).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitValidation {
    Accepted,
    Stale { live_revision: RevisionId, live_generation: Generation },
}

/// ✅️ Checks `op`'s base revision and generation against the document's current `live_revision`/
/// `live_generation` — the ONLY gate a [`CommitCandidate`] passes through before it may be applied.
pub fn validate_commit(op: &Operation, live_revision: RevisionId, live_generation: Generation) -> CommitValidation {
    if op.base_revision == live_revision && op.generation == live_generation {
        CommitValidation::Accepted
    } else {
        CommitValidation::Stale { live_revision, live_generation }
    }
}
//#endregion 🪪️Identity

//#region ⛽️Budget
/// ⛽️ Two-bound step budget: a fuel counter (job-defined instruction-equivalent units, decremented via
/// [`StepContext::consume_fuel`]) AND an absolute wall-clock `deadline_ms` — design doc Decision 3.
/// `deadline_ms` is ABSOLUTE (`now_ms() + slice`), not a remaining duration, so a job never has to
/// re-derive wall-clock math from a countdown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepBudget {
    pub fuel: u64,
    pub deadline_ms: u64,
}

impl StepBudget {
    pub fn new(fuel: u64, deadline_ms: u64) -> StepBudget {
        StepBudget { fuel, deadline_ms }
    }
}

/// 🎯️ Lane budget defaults verbatim from `🎭️actor::Budget`'s `Interactive`/`UserVisible`/
/// `Background`/`Maintenance` constants (design doc §2/Decision 3) — a fuel/wall_ms pair per lane, so
/// a caller building a [`BatchDriveConfig`] or a per-step [`StepBudget`] doesn't have to re-derive
/// these from the actor crate (which this crate must not depend on).
pub const INTERACTIVE_LANE_WALL_MS: u64 = 4;
pub const INTERACTIVE_LANE_FUEL: u64 = 2_000_000;
pub const USER_VISIBLE_LANE_WALL_MS: u64 = 16;
pub const USER_VISIBLE_LANE_FUEL: u64 = 6_000_000;
pub const BACKGROUND_LANE_WALL_MS: u64 = 50;
pub const BACKGROUND_LANE_FUEL: u64 = 20_000_000;
pub const MAINTENANCE_LANE_WALL_MS: u64 = 200;
pub const MAINTENANCE_LANE_FUEL: u64 = 80_000_000;
//#endregion ⛽️Budget

//#region 🧭️StepContext
/// 🧭️ Everything one [`InteractiveJob::step`] call needs: identity ([`OperationId`]/[`Generation`]),
/// the two-bound budget, cancellation, the clock, and the running preview-sequence cursor. Fields are
/// private with accessor methods (a deliberate narrowing from the design doc's Decision 1 sketch,
/// which exposed `pub fuel: &mut u64`/`pub cancel: CancelToken` directly) so [`StepContext::is_cancelled`]
/// can own the [`poll_ready_now`] seam in exactly one place instead of every job reimplementing it.
pub struct StepContext<'a> {
    operation: OperationId,
    generation: Generation,
    fuel_remaining: u64,
    deadline_ms: u64,
    now_ms: fn() -> u64,
    cancel: CancelToken,
    stage: &'static str,
    preview_sequence: &'a mut u64,
}

impl<'a> StepContext<'a> {
    pub fn new(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_ms: fn() -> u64, preview_sequence: &'a mut u64) -> StepContext<'a> {
        StepContext { operation, generation, fuel_remaining: budget.fuel, deadline_ms: budget.deadline_ms, now_ms, cancel, stage: "initial", preview_sequence }
    }

    pub fn operation(&self) -> OperationId {
        self.operation
    }

    pub fn generation(&self) -> Generation {
        self.generation
    }

    /// 🏷️ The label passed to the most recent [`StepContext::set_stage`] call (`"initial"` before the
    /// first one).
    pub fn stage(&self) -> &'static str {
        self.stage
    }

    pub fn now_ms(&self) -> u64 {
        (self.now_ms)()
    }

    pub fn deadline_ms(&self) -> u64 {
        self.deadline_ms
    }

    pub fn deadline_exceeded(&self) -> bool {
        self.now_ms() >= self.deadline_ms
    }

    pub fn fuel_remaining(&self) -> u64 {
        self.fuel_remaining
    }

    /// ⛽️ Decrements the remaining fuel by `units`, saturating at zero — a job calls this after doing
    /// `units` worth of its own work, never before.
    pub fn consume_fuel(&mut self, units: u64) {
        self.fuel_remaining = self.fuel_remaining.saturating_sub(units);
    }

    pub fn fuel_exhausted(&self) -> bool {
        self.fuel_remaining == 0
    }

    /// 🚦️ Whether the job must return NOW (before the hard 8 ms ceiling) — either bound crossed.
    pub fn should_yield(&self) -> bool {
        self.fuel_exhausted() || self.deadline_exceeded()
    }

    /// 🛑️ Whether this step's [`CancelToken`] (or an ancestor's) is cancelled — checked via a single
    /// non-blocking [`poll_ready_now`], see the module doc. A job MUST check this on entry and after
    /// every bounded unit of work (design doc Decision 6): return [`StepOutcome::Cancelled`] without
    /// doing further work once true.
    pub fn is_cancelled(&self) -> bool {
        poll_ready_now(self.cancel.is_cancelled())
    }

    /// 👶️ A clone of this step's [`CancelToken`] — `Arc`-cheap — for a job that wants to derive a
    /// child scope (see [`JobScope::child_of`]) or hand the token to work it submits elsewhere.
    pub fn cancel_token(&self) -> CancelToken {
        self.cancel.clone()
    }

    /// 🏷️ Records a `semio_framework_trace::StageChanged` event and updates [`StepContext::stage`] —
    /// the job's own instrumentation call for switching between internal lanes/phases (Puzzle 3D's
    /// brush → fill switch is the template). Terminal per-call events (preview/checkpoint/commit/
    /// cancel/fail) are recorded once by [`drive_step`] from the returned [`StepOutcome`] instead —
    /// see the module doc's "trace, not a second instrumentation layer" section.
    pub fn set_stage(&mut self, label: &'static str) -> TraceEvent {
        self.stage = label;
        record_stage_changed(self.operation, self.generation, label)
    }

    /// 🔢️ The next preview-sequence number for this operation, advancing a cursor that survives
    /// across every [`StepContext`] built for the same [`run_to_completion`]/[`drive_step`] run — one
    /// call per [`StepOutcome::PreviewReady`]/[`ProgressEvent::PreviewPatch`] a job emits.
    pub fn next_preview_sequence(&mut self) -> u64 {
        let sequence = *self.preview_sequence;
        *self.preview_sequence += 1;
        sequence
    }
}
//#endregion 🧭️StepContext

//#region 🚦️StepOutcome
/// 📸️ A pause point where work is resumable but not yet committed — `state` is opaque, pack-encoded
/// (or, for a dependency-free job like [`TortureJob`], hand-rolled little-endian) bytes the job alone
/// interprets; `applied_progress` is the Puzzle 3D `FillBuilder.applied_count` pattern generalized: how
/// much of `state` is COMMITTED versus merely planned, so a caller can show "these N are done" without
/// decoding `state` itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checkpoint {
    pub state: Vec<u8>,
    pub applied_progress: u64,
}

/// 🏁️ Terminal success payload: the job's final persisted `state` plus its `output` — both opaque
/// bytes, so the runtime stays completely job-agnostic (design doc Decision 2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitCandidate {
    pub state: Vec<u8>,
    pub output: Vec<u8>,
}

/// 💥️ Opaque, job-specific error payload — never interpreted by the runtime, same reasoning as
/// [`CommitCandidate`]'s fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JobFault {
    pub detail: Vec<u8>,
}

/// 🚦️ What one [`InteractiveJob::step`] call reports. [`StepOutcome::Yield`]/[`StepOutcome::PreviewReady`]/
/// [`StepOutcome::CheckpointReady`] all mean "call `step` again"; [`StepOutcome::is_terminal`] marks
/// the other three.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StepOutcome {
    Yield,
    PreviewReady(Vec<u8>),
    CheckpointReady(Checkpoint),
    Complete(CommitCandidate),
    Cancelled,
    Fault(JobFault),
}

impl StepOutcome {
    pub fn is_terminal(&self) -> bool {
        matches!(self, StepOutcome::Complete(_) | StepOutcome::Cancelled | StepOutcome::Fault(_))
    }
}
//#endregion 🚦️StepOutcome

//#region 🧩️InteractiveJob
/// 🧩️ The protocol every interactive operation implements instead of a run-to-completion function
/// call — see the module doc's governing rule. `step` is bounded (checks [`StepContext::should_yield`]
/// and returns before the hard ceiling), cancellable ([`StepContext::is_cancelled`]) and explicitly
/// resumable (a fresh [`StepContext`] each call, job-owned state carries everything between calls).
pub trait InteractiveJob: Send {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome;
}
//#endregion 🧩️InteractiveJob

//#region 🐕️Drive
/// ▶️ Runs exactly one [`InteractiveJob::step`] call under a [`Watchdog`] (so an 8 ms-plus step is
/// ALWAYS caught — never eyeballed, see this ticket's exit gate), pre-checks cancellation so an
/// already-cancelled operation never even enters the job, and is the ONE place a returned
/// [`StepOutcome`] becomes a `semio_framework_trace::record_*` call (module doc). `site` is the
/// `&'static str` label `Watchdog`/the trace ring key on; `stage` is which [`InteractiveStage`]
/// contract family this call belongs to (mirrors the caller's `semio_framework_async::Lane`, kept a
/// separate parameter rather than converted from `Lane` since this crate must not depend on the actor
/// crate's lane-to-stage mapping). `preview_sequence` is threaded across an entire run — see
/// [`StepContext::next_preview_sequence`].
#[allow(clippy::too_many_arguments)]
pub fn drive_step<J: InteractiveJob + ?Sized>(
    job: &mut J,
    site: &'static str,
    operation: OperationId,
    generation: Generation,
    stage: InteractiveStage,
    budget: StepBudget,
    cancel: CancelToken,
    now_ms: fn() -> u64,
    preview_sequence: &mut u64,
) -> StepOutcome {
    if poll_ready_now(cancel.is_cancelled()) {
        record_cancelled(operation, generation);
        return StepOutcome::Cancelled;
    }
    let outcome = {
        let _watchdog = Watchdog::start(site, operation, generation, stage);
        let mut cx = StepContext::new(operation, generation, budget, cancel, now_ms, preview_sequence);
        job.step(&mut cx)
    };
    match &outcome {
        StepOutcome::Yield => {}
        StepOutcome::PreviewReady(_) => {
            record_preview_published(operation, generation);
        }
        StepOutcome::CheckpointReady(_) => {
            record_checkpoint(operation, generation);
        }
        StepOutcome::Complete(_) => {
            record_committed(operation, generation);
        }
        StepOutcome::Cancelled => {
            record_cancelled(operation, generation);
        }
        StepOutcome::Fault(_) => {
            record_failed(operation, generation);
        }
    }
    outcome
}
//#endregion 🐕️Drive

//#region 👶️JobScope
/// 🌱️ A [`CancelToken::root`] via [`poll_ready_now`] — the one place [`JobScope::root`]/callers that
/// need a fresh root token (batch entry points, tests) cross the sync-over-async seam for token
/// creation, mirroring [`StepContext::is_cancelled`]'s single-owner pattern.
pub fn root_cancel_token() -> CancelToken {
    poll_ready_now(CancelToken::root())
}

/// 👶️ Structured child-job ownership built directly on [`CancelToken`]'s parent-chain fold (design
/// doc Decision 6: "no registry needed") rather than a new scope registry: [`JobScope::cancel_token`]
/// is a [`CancelToken::child`] of whatever token this scope was built under, so cancelling an ancestor
/// transitively cancels every job holding this scope's token with no bookkeeping here. What THIS type
/// adds on top of a bare token is the "cannot complete while any child is live" rule (design ticket
/// packet P2a item 3) via [`JobScope::spawn_child`]'s live-count guard — see this module's doc-comment
/// deviation note for why this stops short of a live `semio_framework_async::ScopeHandle`.
pub struct JobScope {
    cancel: CancelToken,
    live_children: AtomicU32,
}

impl JobScope {
    /// 🌱️ A root scope with no parent — the top of one job tree.
    pub fn root() -> JobScope {
        JobScope { cancel: root_cancel_token(), live_children: AtomicU32::new(0) }
    }

    /// 👶️ A scope whose cancellation is folded with `parent`'s (see [`CancelToken::child`]) — a
    /// parent job derives one of these per child job it spawns.
    pub fn child_of(parent: &CancelToken) -> JobScope {
        JobScope { cancel: poll_ready_now(parent.child()), live_children: AtomicU32::new(0) }
    }

    pub fn cancel_token(&self) -> CancelToken {
        self.cancel.clone()
    }

    pub fn is_cancelled(&self) -> bool {
        poll_ready_now(self.cancel.is_cancelled())
    }

    /// 👶️ Registers one live child, returning a guard that releases it on drop — call once per child
    /// job this scope's owner spawns, and hold the guard for exactly that child's lifetime.
    pub fn spawn_child(&self) -> ChildJobGuard<'_> {
        self.live_children.fetch_add(1, Ordering::SeqCst);
        ChildJobGuard { scope: self }
    }

    pub fn live_child_count(&self) -> u32 {
        self.live_children.load(Ordering::SeqCst)
    }

    pub fn has_live_children(&self) -> bool {
        self.live_child_count() > 0
    }

    /// 🚨️ Debug-only tripwire (compiles to nothing in release, same shape as
    /// `semio_framework_trace::assert_ui_thread`): a job calls this immediately before returning
    /// [`StepOutcome::Complete`] to enforce "a parent may not complete while any child is live".
    pub fn assert_completable(&self) {
        debug_assert_eq!(self.live_child_count(), 0, "JobScope: a parent job must not return StepOutcome::Complete while child jobs are still live");
    }
}

/// 👶️ RAII guard from [`JobScope::spawn_child`] — releases its live-child slot on drop, whether that
/// drop is an ordinary scope exit or unwinding past a panic.
pub struct ChildJobGuard<'a> {
    scope: &'a JobScope,
}

impl Drop for ChildJobGuard<'_> {
    fn drop(&mut self) {
        let previous = self.scope.live_children.fetch_sub(1, Ordering::SeqCst);
        debug_assert!(previous >= 1, "JobScope: ChildJobGuard dropped with no live child to release");
    }
}
//#endregion 👶️JobScope

//#region 📡️Progress
/// 🔖️ Opaque id for one addressable entity a [`ProgressEvent`] touches (a mesh, a brush placement, a
/// document node) — a bare `u64` so this crate never depends on any domain's entity-id type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(pub u64);

/// 🩺️ What kind of non-terminal report a [`ProgressEvent::Diagnostic`]/[`ProgressEvent::Failed`]
/// carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DiagnosticKind {
    Info,
    Warning,
    Stalled,
    Error,
}

/// 📡️ The ten-event progress vocabulary (design ticket packet P2a item 4), proven by Puzzle 3D's
/// precompute session (design doc §6) — `Started`/`StageChanged`/`CandidateTested`/`PreviewPatch`/
/// `Diagnostic`/`Checkpoint`/`CommitCandidate`/`Completed`/`Cancelled`/`Failed`. This is a caller-side
/// UI/log projection, distinct from the trace ring [`drive_step`] writes to: a host assembles these
/// from [`StepOutcome`]s plus its own domain data (affected entities, quality/tolerance) to hand to a
/// UI over a channel governed by [`channel_policy_for`]/[`default_channel_kind_for`] — the trace ring
/// alone has no entity/quality/tolerance vocabulary, by design (it stays domain-neutral).
#[derive(Clone, Debug, PartialEq)]
pub enum ProgressEvent {
    Started {
        operation: OperationId,
        generation: Generation,
        base_revision: RevisionId,
        at_ms: u64,
    },
    StageChanged {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        stage: &'static str,
        at_ms: u64,
    },
    CandidateTested {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        entity: EntityId,
        accepted: bool,
        quality: f32,
        at_ms: u64,
    },
    PreviewPatch {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        base_revision: RevisionId,
        stage: &'static str,
        completed_units: u64,
        total_units: Option<u64>,
        quality: f32,
        tolerance: f32,
        affected: Vec<EntityId>,
        patch: Vec<u8>,
        at_ms: u64,
    },
    Diagnostic {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        kind: DiagnosticKind,
        detail: Vec<u8>,
        at_ms: u64,
    },
    Checkpoint {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        base_revision: RevisionId,
        applied_progress: u64,
        at_ms: u64,
    },
    CommitCandidate {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        base_revision: RevisionId,
        at_ms: u64,
    },
    Completed {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        at_ms: u64,
    },
    Cancelled {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        at_ms: u64,
    },
    Failed {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        kind: DiagnosticKind,
        detail: Vec<u8>,
        at_ms: u64,
    },
}

impl ProgressEvent {
    pub fn operation(&self) -> OperationId {
        match self {
            ProgressEvent::Started { operation, .. }
            | ProgressEvent::StageChanged { operation, .. }
            | ProgressEvent::CandidateTested { operation, .. }
            | ProgressEvent::PreviewPatch { operation, .. }
            | ProgressEvent::Diagnostic { operation, .. }
            | ProgressEvent::Checkpoint { operation, .. }
            | ProgressEvent::CommitCandidate { operation, .. }
            | ProgressEvent::Completed { operation, .. }
            | ProgressEvent::Cancelled { operation, .. }
            | ProgressEvent::Failed { operation, .. } => *operation,
        }
    }

    pub fn generation(&self) -> Generation {
        match self {
            ProgressEvent::Started { generation, .. }
            | ProgressEvent::StageChanged { generation, .. }
            | ProgressEvent::CandidateTested { generation, .. }
            | ProgressEvent::PreviewPatch { generation, .. }
            | ProgressEvent::Diagnostic { generation, .. }
            | ProgressEvent::Checkpoint { generation, .. }
            | ProgressEvent::CommitCandidate { generation, .. }
            | ProgressEvent::Completed { generation, .. }
            | ProgressEvent::Cancelled { generation, .. }
            | ProgressEvent::Failed { generation, .. } => *generation,
        }
    }
}

/// 🚰️ The six channel-policy categories the design ticket's progress-stream vocabulary names —
/// distinct from [`ProgressEvent`]'s ten variants because two categories (`PointerHover`/`Telemetry`)
/// are UI/sampling channels outside the job progress vocabulary itself, and one vocabulary variant
/// ([`ProgressEvent::PreviewPatch`]) splits across two categories by payload size (see
/// [`default_channel_kind_for`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressChannelKind {
    /// 🖱️ Pointer/hover UI events — latest-wins, one slot.
    PointerHover,
    /// 🎨️ Preview geometry — coalesced by `(operation, entity, stage)`.
    PreviewGeometry,
    /// 🔒️ Commits and checkpoints — lossless, bounded (never dropped, backpressure instead).
    CommitAndCheckpoint,
    /// 🩺️ Diagnostics — a bounded ring: `Coalesced` by `(operation, diagnostic kind)` is the closest
    /// fit in [`ChannelPolicy`]'s four variants to "ring" (bounded, drops OLDEST rather than reject/
    /// stall) since this crate adds no fifth variant to that enum — see `📓️p2a-job-protocol.md`'s
    /// deviation note.
    DiagnosticRing,
    /// 📉️ Telemetry — lossy, latest sample only.
    Telemetry,
    /// 🪨️ Large preview geometry — byte-credit controlled.
    LargeGeometry,
}

/// 🚰️ The recommended [`ChannelPolicy`] for one [`ProgressChannelKind`] — design ticket packet P2a
/// item 4's channel-policy matrix, made concrete. A host wiring an actual channel may widen these
/// bounds for its own deployment; these are the floor every implementation should start from.
pub fn channel_policy_for(kind: ProgressChannelKind) -> ChannelPolicy {
    match kind {
        ProgressChannelKind::PointerHover => ChannelPolicy::LatestWins { max_bytes: 4 * 1024 },
        ProgressChannelKind::PreviewGeometry => ChannelPolicy::Coalesced { key: "operation:entity:stage".to_string(), max_items: 64, max_bytes: 4 * 1024 * 1024 },
        ProgressChannelKind::CommitAndCheckpoint => ChannelPolicy::LosslessBounded { max_items: 256, max_bytes: 16 * 1024 * 1024 },
        ProgressChannelKind::DiagnosticRing => ChannelPolicy::Coalesced { key: "operation:diagnostic_kind".to_string(), max_items: 128, max_bytes: 512 * 1024 },
        ProgressChannelKind::Telemetry => ChannelPolicy::LatestWins { max_bytes: 1024 },
        ProgressChannelKind::LargeGeometry => ChannelPolicy::ByteCredit { max_items: 32, max_bytes: 32 * 1024 * 1024 },
    }
}

/// 📏️ A [`ProgressEvent::PreviewPatch`] at or above this many patch bytes routes to
/// [`ProgressChannelKind::LargeGeometry`] instead of [`ProgressChannelKind::PreviewGeometry`].
pub const LARGE_PREVIEW_PATCH_BYTES: usize = 256 * 1024;

/// 🗺️ The recommended [`ProgressChannelKind`] for one [`ProgressEvent`] — the default routing a host
/// applies before [`channel_policy_for`].
pub fn default_channel_kind_for(event: &ProgressEvent) -> ProgressChannelKind {
    match event {
        ProgressEvent::Started { .. } => ProgressChannelKind::CommitAndCheckpoint,
        ProgressEvent::StageChanged { .. } => ProgressChannelKind::DiagnosticRing,
        ProgressEvent::CandidateTested { .. } => ProgressChannelKind::DiagnosticRing,
        ProgressEvent::PreviewPatch { patch, .. } if patch.len() >= LARGE_PREVIEW_PATCH_BYTES => ProgressChannelKind::LargeGeometry,
        ProgressEvent::PreviewPatch { .. } => ProgressChannelKind::PreviewGeometry,
        ProgressEvent::Diagnostic { .. } => ProgressChannelKind::DiagnosticRing,
        ProgressEvent::Checkpoint { .. } => ProgressChannelKind::CommitAndCheckpoint,
        ProgressEvent::CommitCandidate { .. } => ProgressChannelKind::CommitAndCheckpoint,
        ProgressEvent::Completed { .. } => ProgressChannelKind::CommitAndCheckpoint,
        ProgressEvent::Cancelled { .. } => ProgressChannelKind::CommitAndCheckpoint,
        ProgressEvent::Failed { .. } => ProgressChannelKind::CommitAndCheckpoint,
    }
}
//#endregion 📡️Progress

//#region 🏭️Batch
/// 🏭️ Fixed per-step configuration for [`run_to_completion`]/[`run_on_worker`] — a headless/CLI
/// caller picks one lane's worth of budget once, up front, rather than re-deriving a [`StepBudget`]
/// every iteration.
#[derive(Clone, Copy, Debug)]
pub struct BatchDriveConfig {
    pub site: &'static str,
    pub stage: InteractiveStage,
    pub fuel_per_step: u64,
    pub step_budget_ms: u64,
}

/// 🏭️ Everything [`run_to_completion`]/[`run_on_worker`] need beyond the job itself — bundled into one
/// struct rather than passed as five-plus loose parameters (clippy's `too_many_arguments`), and so a
/// caller building one [`BatchJobParams`] can hand the identical value to both a direct call and a
/// worker-submitted one.
#[derive(Clone)]
pub struct BatchJobParams {
    pub operation: OperationId,
    pub generation: Generation,
    pub cancel: CancelToken,
    pub config: BatchDriveConfig,
    pub now_ms: fn() -> u64,
}

/// ▶️ Drives `job` via repeated [`drive_step`] calls — each one still individually bounded by
/// `params.config`'s [`StepBudget`] — until a terminal [`StepOutcome`] comes back. Design ticket
/// packet P2a item 6: this is what lets a CLI/headless path reuse the EXACT SAME [`InteractiveJob`]
/// impl the interactive path drives one step at a time, instead of a second "just run it all" code
/// path that could silently diverge. Records `Started` once up front (never per step — a step-level
/// `Started` would misrepresent operation lifecycle in the trace ring).
pub fn run_to_completion<J: InteractiveJob>(job: &mut J, params: &BatchJobParams) -> StepOutcome {
    record_operation_started(params.operation, params.generation);
    let mut preview_sequence: u64 = 0;
    loop {
        let budget = StepBudget::new(params.config.fuel_per_step, (params.now_ms)().saturating_add(params.config.step_budget_ms));
        let outcome = drive_step(job, params.config.site, params.operation, params.generation, params.config.stage, budget, params.cancel.clone(), params.now_ms, &mut preview_sequence);
        if outcome.is_terminal() {
            return outcome;
        }
    }
}

/// ▶️ [`run_to_completion`], submitted onto a `semio_framework_async::WorkerPool` lane — the same
/// substrate every other subsystem's CPU-bound work schedules onto (Phase 1 packet P1a). The returned
/// `Receiver` yields exactly one [`StepOutcome`] once the job reaches a terminal state; a caller that
/// doesn't want to block reads it via `try_recv`.
pub fn run_on_worker<J: InteractiveJob + 'static>(pool: &WorkerPool, lane: Lane, mut job: J, params: BatchJobParams) -> Receiver<StepOutcome> {
    let (sender, receiver) = std::sync::mpsc::channel();
    pool.submit(
        lane,
        Box::new(move || {
            let outcome = run_to_completion(&mut job, &params);
            let _ = sender.send(outcome);
        }),
    );
    receiver
}
//#endregion 🏭️Batch

//#region 🔥️TortureJob
/// 🎲️ A tiny, dependency-free xorshift64 step — deterministic given `x`, no allocation, no external
/// RNG crate (this crate stays zero-third-party-dependency, mirroring `semio_framework_trace`'s own
/// leaf-crate mandate). `| 1` on first seeding (see [`TortureJob::new`]) keeps the state off the
/// all-zero fixed point xorshift can never escape.
fn xorshift64(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

/// 🎲️ splitmix64 seed expansion — avalanches a caller-supplied `seed` into a well-mixed 64-bit state
/// before [`xorshift64`] ever sees it. Without this, [`TortureJob::new`]'s old plain `seed | 1` let
/// adjacent seeds (e.g. `42`/`43`) collapse onto the identical state (`|1` only ever touches bit 0),
/// which made two DIFFERENT seeds silently replay identical output — exactly the determinism bug this
/// conformance job exists to catch, so it must not carry one itself.
fn splitmix64(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn read_u64_le(bytes: &[u8], cursor: &mut usize) -> u64 {
    let value = u64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().expect("TortureJob::from_checkpoint: truncated u64 field"));
    *cursor += 8;
    value
}

/// 🔥️ The Phase 2 conformance job (design ticket packet P2a item 7 / exit gate): long-running,
/// continuously preview-producing, checkpointable, cancellable, and deterministic given its seed —
/// every "unit" mixes a xorshift64 draw into an accumulator, cancellation and the fuel/deadline bound
/// are checked every unit, and every [`TortureJob::preview_every_units`]/[`TortureJob::checkpoint_every_units`]
/// units it returns [`StepOutcome::PreviewReady`]/[`StepOutcome::CheckpointReady`] instead of looping
/// further — so a caller sees continuous, real progress, not just a final answer. State is hand-rolled
/// little-endian bytes (design doc Decision 2's "opaque, job-encoded `Vec<u8>`" — this job has no
/// `RecordSpec` to hand `pack`'s schema-typed `encode_record_body` and stays zero-dependency, see
/// `📓️p2a-job-protocol.md`'s deviation note).
pub struct TortureJob {
    total_units: u64,
    completed_units: u64,
    rng_state: u64,
    accumulator: u64,
    checkpoint_every_units: u64,
    preview_every_units: u64,
    units_since_checkpoint: u64,
    units_since_preview: u64,
    scope: JobScope,
}

/// 🩺️ How many units [`TortureJob::step`] processes between cheap `should_yield` polls — small enough
/// that overshoot past the 8 ms ceiling within one check interval is negligible (each unit is a
/// handful of integer ops), large enough that the `now_ms`/fuel check itself isn't the hot-loop
/// bottleneck.
const TORTURE_YIELD_CHECK_INTERVAL: u64 = 64;

impl TortureJob {
    pub fn new(seed: u64, total_units: u64, checkpoint_every_units: u64, preview_every_units: u64, parent_cancel: &CancelToken) -> TortureJob {
        TortureJob { total_units, completed_units: 0, rng_state: splitmix64(seed) | 1, accumulator: 0, checkpoint_every_units, preview_every_units, units_since_checkpoint: 0, units_since_preview: 0, scope: JobScope::child_of(parent_cancel) }
    }

    pub fn completed_units(&self) -> u64 {
        self.completed_units
    }

    pub fn total_units(&self) -> u64 {
        self.total_units
    }

    fn checkpoint(&self) -> Checkpoint {
        let mut state = Vec::with_capacity(48);
        state.extend_from_slice(&self.total_units.to_le_bytes());
        state.extend_from_slice(&self.completed_units.to_le_bytes());
        state.extend_from_slice(&self.rng_state.to_le_bytes());
        state.extend_from_slice(&self.accumulator.to_le_bytes());
        state.extend_from_slice(&self.checkpoint_every_units.to_le_bytes());
        state.extend_from_slice(&self.preview_every_units.to_le_bytes());
        Checkpoint { state, applied_progress: self.completed_units }
    }

    /// 🔁️ Rebuilds a [`TortureJob`] from a [`Checkpoint::state`] produced by [`TortureJob::checkpoint`]
    /// — the resume half of the checkpoint → restore → resume conformance test. `parent_cancel` is
    /// supplied fresh (a restored job gets a NEW scope, same as any resumed operation reattaching to
    /// whatever scope owns it now).
    pub fn from_checkpoint(bytes: &[u8], parent_cancel: &CancelToken) -> TortureJob {
        let mut cursor = 0usize;
        let total_units = read_u64_le(bytes, &mut cursor);
        let completed_units = read_u64_le(bytes, &mut cursor);
        let rng_state = read_u64_le(bytes, &mut cursor);
        let accumulator = read_u64_le(bytes, &mut cursor);
        let checkpoint_every_units = read_u64_le(bytes, &mut cursor);
        let preview_every_units = read_u64_le(bytes, &mut cursor);
        TortureJob { total_units, completed_units, rng_state, accumulator, checkpoint_every_units, preview_every_units, units_since_checkpoint: 0, units_since_preview: 0, scope: JobScope::child_of(parent_cancel) }
    }

    fn encode_preview(&self, sequence: u64) -> Vec<u8> {
        let mut out = Vec::with_capacity(24);
        out.extend_from_slice(&sequence.to_le_bytes());
        out.extend_from_slice(&self.completed_units.to_le_bytes());
        out.extend_from_slice(&self.accumulator.to_le_bytes());
        out
    }

    fn commit(&self) -> CommitCandidate {
        let mut output = Vec::with_capacity(16);
        output.extend_from_slice(&self.completed_units.to_le_bytes());
        output.extend_from_slice(&self.accumulator.to_le_bytes());
        CommitCandidate { state: self.checkpoint().state, output }
    }
}

impl InteractiveJob for TortureJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.completed_units == 0 {
            cx.set_stage("torture:grinding");
        }
        let mut since_check = 0u64;
        while self.completed_units < self.total_units {
            if cx.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            self.rng_state = xorshift64(self.rng_state);
            let mix = self.rng_state.rotate_left((self.completed_units % 61) as u32);
            self.accumulator = self.accumulator.wrapping_add(mix);
            self.completed_units += 1;
            self.units_since_checkpoint += 1;
            self.units_since_preview += 1;
            cx.consume_fuel(1);
            since_check += 1;
            if since_check >= TORTURE_YIELD_CHECK_INTERVAL {
                since_check = 0;
                if cx.should_yield() {
                    return StepOutcome::Yield;
                }
            }
            if self.units_since_preview >= self.preview_every_units {
                self.units_since_preview = 0;
                let sequence = cx.next_preview_sequence();
                return StepOutcome::PreviewReady(self.encode_preview(sequence));
            }
            if self.units_since_checkpoint >= self.checkpoint_every_units {
                self.units_since_checkpoint = 0;
                return StepOutcome::CheckpointReady(self.checkpoint());
            }
        }
        self.scope.assert_completable();
        StepOutcome::Complete(self.commit())
    }
}
//#endregion 🔥️TortureJob

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_async::{ProcessKind, WorkerPoolConfig};
    use std::time::Instant as StdInstant;

    fn test_now_ms() -> u64 {
        default_now_ms()
    }

    //#region 🪪️Identity
    #[test]
    fn commit_validation_accepts_matching_revision_and_generation() {
        let op = Operation::new(allocate_operation_id(), RevisionId(7), Generation(3), 42);
        assert_eq!(validate_commit(&op, RevisionId(7), Generation(3)), CommitValidation::Accepted);
    }

    #[test]
    fn commit_validation_reports_stale_on_mismatch() {
        let op = Operation::new(allocate_operation_id(), RevisionId(7), Generation(3), 42);
        assert_eq!(validate_commit(&op, RevisionId(8), Generation(3)), CommitValidation::Stale { live_revision: RevisionId(8), live_generation: Generation(3) });
        assert_eq!(validate_commit(&op, RevisionId(7), Generation(4)), CommitValidation::Stale { live_revision: RevisionId(7), live_generation: Generation(4) });
    }

    #[test]
    fn operation_preview_sequence_advances_monotonically() {
        let mut op = Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 0);
        assert_eq!(op.next_preview_sequence(), 0);
        assert_eq!(op.next_preview_sequence(), 1);
        assert_eq!(op.next_preview_sequence(), 2);
    }
    //#endregion 🪪️Identity

    //#region 👶️JobScope
    #[test]
    fn job_scope_cascades_cancellation_from_parent() {
        let parent = root_cancel_token();
        let scope = JobScope::child_of(&parent);
        assert!(!scope.is_cancelled());
        poll_ready_now(parent.cancel());
        assert!(scope.is_cancelled(), "a child scope must observe its parent's cancellation");
    }

    #[test]
    fn job_scope_tracks_live_children_and_releases_on_drop() {
        let scope = JobScope::root();
        assert!(!scope.has_live_children());
        let guard_a = scope.spawn_child();
        let guard_b = scope.spawn_child();
        assert_eq!(scope.live_child_count(), 2);
        drop(guard_a);
        assert_eq!(scope.live_child_count(), 1);
        drop(guard_b);
        assert!(!scope.has_live_children());
    }
    //#endregion 👶️JobScope

    //#region 🚰️Progress
    #[test]
    fn channel_policy_matrix_bounds_every_kind_in_items_and_bytes() {
        for kind in [ProgressChannelKind::PointerHover, ProgressChannelKind::PreviewGeometry, ProgressChannelKind::CommitAndCheckpoint, ProgressChannelKind::DiagnosticRing, ProgressChannelKind::Telemetry, ProgressChannelKind::LargeGeometry] {
            let policy = channel_policy_for(kind);
            let max_bytes = match &policy {
                ChannelPolicy::LatestWins { max_bytes } => *max_bytes,
                ChannelPolicy::Coalesced { max_bytes, .. } => *max_bytes,
                ChannelPolicy::LosslessBounded { max_bytes, .. } => *max_bytes,
                ChannelPolicy::ByteCredit { max_bytes, .. } => *max_bytes,
            };
            assert!(max_bytes > 0, "{kind:?} must bound bytes");
        }
    }

    fn preview_patch_event(patch_bytes: usize) -> ProgressEvent {
        ProgressEvent::PreviewPatch {
            operation: allocate_operation_id(),
            generation: Generation(0),
            sequence: 0,
            base_revision: RevisionId(0),
            stage: "test",
            completed_units: 1,
            total_units: Some(10),
            quality: 1.0,
            tolerance: 0.1,
            affected: vec![EntityId(1)],
            patch: vec![0u8; patch_bytes],
            at_ms: 0,
        }
    }

    #[test]
    fn large_preview_patch_routes_to_large_geometry_kind() {
        assert_eq!(default_channel_kind_for(&preview_patch_event(16)), ProgressChannelKind::PreviewGeometry);
        assert_eq!(default_channel_kind_for(&preview_patch_event(LARGE_PREVIEW_PATCH_BYTES)), ProgressChannelKind::LargeGeometry);
    }
    //#endregion 🚰️Progress

    //#region 🔥️TortureConformance
    fn small_torture(seed: u64) -> TortureJob {
        TortureJob::new(seed, 20_000, 500, 137, &root_cancel_token())
    }

    /// ⏱️ Exit gate #1: no single `step()` call ever reaches the 8 ms hard ceiling — asserted against
    /// `semio_framework_trace::Watchdog`'s own violation ring, never by eyeballing elapsed time.
    #[test]
    fn torture_job_never_trips_the_watchdog_ceiling() {
        let operation = allocate_operation_id();
        let generation = Generation(1);
        let cancel = root_cancel_token();
        let mut job = small_torture(0xC0FFEE);
        let mut preview_sequence = 0u64;
        record_operation_started(operation, generation);
        loop {
            let budget = StepBudget::new(200, test_now_ms().saturating_add(INTERACTIVE_LANE_WALL_MS));
            let outcome = drive_step(&mut job, "test.torture.ceiling", operation, generation, InteractiveStage::InteractiveStep, budget, cancel.clone(), test_now_ms, &mut preview_sequence);
            if outcome.is_terminal() {
                assert!(matches!(outcome, StepOutcome::Complete(_)), "expected the torture job to finish uninterrupted");
                break;
            }
        }
        let violations: Vec<_> = Watchdog::violations().into_iter().filter(|violation| violation.operation == operation).collect();
        assert!(violations.is_empty(), "torture job tripped the 8ms watchdog ceiling: {violations:?}");
    }

    /// 📡️ Exit gate #2: the job previews continuously, not just at the end.
    #[test]
    fn torture_job_previews_continuously() {
        let operation = allocate_operation_id();
        let generation = Generation(2);
        let cancel = root_cancel_token();
        let mut job = small_torture(1234);
        let mut preview_sequence = 0u64;
        let mut preview_count = 0u32;
        loop {
            let budget = StepBudget::new(200, test_now_ms().saturating_add(INTERACTIVE_LANE_WALL_MS));
            let outcome = drive_step(&mut job, "test.torture.preview", operation, generation, InteractiveStage::InteractiveStep, budget, cancel.clone(), test_now_ms, &mut preview_sequence);
            if let StepOutcome::PreviewReady(_) = &outcome {
                preview_count += 1;
            }
            if outcome.is_terminal() {
                break;
            }
        }
        assert!(preview_count >= 5, "expected several previews across a 20_000-unit run, got {preview_count}");
    }

    /// 🛑️ Exit gate #3: cancellation is observed within 8 ms at p99.
    #[test]
    fn torture_job_observes_cancellation_within_8ms_at_p99() {
        const TRIALS: usize = 40;
        let mut latencies_us: Vec<u64> = Vec::with_capacity(TRIALS);
        for trial in 0..TRIALS {
            let operation = allocate_operation_id();
            let generation = Generation(trial as u64);
            let cancel = root_cancel_token();
            let mut job = TortureJob::new(0xA5A5_0000 + trial as u64, 2_000_000, 5_000, 5_000, &cancel);
            let mut preview_sequence = 0u64;
            // ▶️ Warm the job up a little before cancelling, so cancellation lands mid-flight.
            for _ in 0..3 {
                let budget = StepBudget::new(400, test_now_ms().saturating_add(INTERACTIVE_LANE_WALL_MS));
                drive_step(&mut job, "test.torture.cancel-warmup", operation, generation, InteractiveStage::InteractiveStep, budget, cancel.clone(), test_now_ms, &mut preview_sequence);
            }
            let cancel_start = StdInstant::now();
            poll_ready_now(cancel.cancel());
            loop {
                let budget = StepBudget::new(400, test_now_ms().saturating_add(INTERACTIVE_LANE_WALL_MS));
                let outcome = drive_step(&mut job, "test.torture.cancel", operation, generation, InteractiveStage::InteractiveStep, budget, cancel.clone(), test_now_ms, &mut preview_sequence);
                if matches!(outcome, StepOutcome::Cancelled) {
                    break;
                }
                assert!(!outcome.is_terminal(), "expected Cancelled, got a different terminal outcome: {outcome:?}");
            }
            latencies_us.push(cancel_start.elapsed().as_micros() as u64);
        }
        latencies_us.sort_unstable();
        let p99_index = ((latencies_us.len() as f64) * 0.99).floor() as usize;
        let p99_us = latencies_us[p99_index.min(latencies_us.len() - 1)];
        assert!(p99_us < 8_000, "cancellation p99 latency {p99_us}us exceeded the 8ms exit-gate ceiling");
    }

    /// 🔁️ Exit gate #4: deterministic replay — byte-identical results across worker counts 1..N.
    #[test]
    fn torture_job_replays_deterministically_across_worker_counts() {
        let seed = 0x5EED_1234;
        let total_units = 50_000;
        let mut outputs: Vec<Vec<u8>> = Vec::new();
        for worker_count in [1usize, 2, 4] {
            let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, worker_count));
            let cancel = root_cancel_token();
            let job = TortureJob::new(seed, total_units, 1_000, 1_000, &cancel);
            let operation = allocate_operation_id();
            let config = BatchDriveConfig { site: "test.torture.determinism", stage: InteractiveStage::BackgroundStep, fuel_per_step: 10_000, step_budget_ms: BACKGROUND_LANE_WALL_MS };
            let params = BatchJobParams { operation, generation: Generation(1), cancel, config, now_ms: default_now_ms };
            let receiver = run_on_worker(&pool, Lane::Background, job, params);
            let outcome = receiver.recv().expect("torture job on worker never sent a result");
            pool.shutdown();
            match outcome {
                StepOutcome::Complete(candidate) => outputs.push(candidate.output),
                other => panic!("expected Complete for worker_count={worker_count}, got {other:?}"),
            }
        }
        assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]), "torture job output diverged across worker counts: {outputs:?}");
    }

    /// 💾️ Exit gate #5: checkpoint → restore → resume yields the same final result as an
    /// uninterrupted run.
    #[test]
    fn torture_job_checkpoint_restore_resume_matches_uninterrupted_run() {
        let seed = 0x900D_5EED;
        let total_units = 30_000;

        let uninterrupted_output = {
            let cancel = root_cancel_token();
            let mut job = TortureJob::new(seed, total_units, 700, 900, &cancel);
            let operation = allocate_operation_id();
            let config = BatchDriveConfig { site: "test.torture.uninterrupted", stage: InteractiveStage::InteractiveStep, fuel_per_step: 5_000, step_budget_ms: INTERACTIVE_LANE_WALL_MS };
            let params = BatchJobParams { operation, generation: Generation(1), cancel, config, now_ms: test_now_ms };
            match run_to_completion(&mut job, &params) {
                StepOutcome::Complete(candidate) => candidate.output,
                other => panic!("expected Complete, got {other:?}"),
            }
        };

        let resumed_output = {
            let cancel = root_cancel_token();
            let mut job = TortureJob::new(seed, total_units, 700, 900, &cancel);
            let operation = allocate_operation_id();
            let generation = Generation(1);
            let mut preview_sequence = 0u64;
            let checkpoint_state = loop {
                let budget = StepBudget::new(5_000, test_now_ms().saturating_add(INTERACTIVE_LANE_WALL_MS));
                let outcome = drive_step(&mut job, "test.torture.checkpoint-phase", operation, generation, InteractiveStage::InteractiveStep, budget, cancel.clone(), test_now_ms, &mut preview_sequence);
                if let StepOutcome::CheckpointReady(checkpoint) = outcome {
                    break checkpoint.state;
                }
                assert!(!outcome.is_terminal(), "expected a checkpoint before completion, got terminal outcome {outcome:?}");
            };
            let mut resumed_job = TortureJob::from_checkpoint(&checkpoint_state, &cancel);
            loop {
                let budget = StepBudget::new(5_000, test_now_ms().saturating_add(INTERACTIVE_LANE_WALL_MS));
                let outcome = drive_step(&mut resumed_job, "test.torture.resume-phase", operation, generation, InteractiveStage::InteractiveStep, budget, cancel.clone(), test_now_ms, &mut preview_sequence);
                if let StepOutcome::Complete(candidate) = outcome {
                    break candidate.output;
                }
                assert!(!outcome.is_terminal(), "expected the resumed job to complete, got terminal outcome {outcome:?}");
            }
        };

        assert_eq!(uninterrupted_output, resumed_output, "checkpoint -> restore -> resume must match an uninterrupted run byte-for-byte");
    }

    #[test]
    fn torture_job_is_deterministic_given_identical_seed_and_inputs() {
        let run = |seed: u64| -> Vec<u8> {
            let cancel = root_cancel_token();
            let mut job = TortureJob::new(seed, 10_000, 400, 600, &cancel);
            let operation = allocate_operation_id();
            let config = BatchDriveConfig { site: "test.torture.golden", stage: InteractiveStage::InteractiveStep, fuel_per_step: 5_000, step_budget_ms: INTERACTIVE_LANE_WALL_MS };
            let params = BatchJobParams { operation, generation: Generation(1), cancel, config, now_ms: test_now_ms };
            match run_to_completion(&mut job, &params) {
                StepOutcome::Complete(candidate) => candidate.output,
                other => panic!("expected Complete, got {other:?}"),
            }
        };
        assert_eq!(run(42), run(42), "identical seed and inputs must replay byte-identical");
        assert_ne!(run(42), run(43), "different seeds must not collide for this conformance job");
    }
    //#endregion 🔥️TortureConformance

    //#region 🏭️Batch
    #[test]
    fn run_on_worker_reuses_the_same_job_impl_as_the_interactive_path() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
        let cancel = root_cancel_token();
        let job = small_torture(777);
        let operation = allocate_operation_id();
        let config = BatchDriveConfig { site: "test.batch.reuse", stage: InteractiveStage::BackgroundStep, fuel_per_step: 5_000, step_budget_ms: BACKGROUND_LANE_WALL_MS };
        let params = BatchJobParams { operation, generation: Generation(1), cancel, config, now_ms: default_now_ms };
        let receiver = run_on_worker(&pool, Lane::Background, job, params);
        let outcome = receiver.recv().expect("worker never produced a result");
        pool.shutdown();
        assert!(matches!(outcome, StepOutcome::Complete(_)));
    }
    //#endregion 🏭️Batch

    //#region 🔁️SyncPoll
    #[test]
    fn poll_ready_now_resolves_a_root_cancel_token_synchronously() {
        let token = root_cancel_token();
        assert!(!poll_ready_now(token.is_cancelled()));
    }
    //#endregion 🔁️SyncPoll

    //#region 🕰️Clock
    #[test]
    fn default_now_ms_is_monotonically_non_decreasing() {
        let first = default_now_ms();
        let second = default_now_ms();
        assert!(second >= first);
    }
    //#endregion 🕰️Clock
}
//#endregion 🧪️Tests
