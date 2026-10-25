//! 🕰️ Clock, identity and budget: the synchronous poll, the cancel tokens, the millisecond and microsecond clocks, the revision and operation identities, the commit validation and the step budget with its lane constants. See [`super`].

use super::*;

//#region 🔁️SyncPoll
/// 🔁️ Polls `fut` exactly once with a no-op waker and returns its output, panicking on `Pending` —
/// see the module doc's "sync-over-async seam" section for why this is safe here (every
/// [`CancelToken`] op is a pure atomic read/write with no real suspension point) and why it is NOT
/// [`semio_framework_async::block_on`] (no parking, no loop, and callable from `step()` itself, which
/// `block_on` explicitly forbids). Private: every public crossing of this seam goes through a named
/// method ([`StepContext::is_cancelled`], [`JobScope::root`], …) so a future upstream change that
/// actually introduces suspension fails loudly here instead of silently spinning.
pub(super) fn poll_ready_now<F: Future>(fut: F) -> F::Output {
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
/// 🌱️ Creates the canonical root cancellation authority through the synchronous primitive seam.
pub fn root_cancel_token()->CancelToken{poll_ready_now(CancelToken::root())}

/// 👶️ Creates a child cancellation authority through the same synchronous primitive seam.
pub fn child_cancel_token(parent:&CancelToken)->CancelToken{poll_ready_now(parent.child())}

/// ⏰️ Coarse scheduler clock, never used to measure an interactive step deadline.
pub fn default_now_ms() -> Option<u64> {
    default_now_us().map(|now_us| now_us / 1_000)
}

/// 🔌️ Installs the embedding host's real monotonic microsecond clock exactly once.
pub fn install_microsecond_clock(clock: fn() -> Option<u64>) -> Result<(), fn() -> Option<u64>> {
    semio_framework_trace::install_clock(clock)
}

/// ⏱️ Reads a real microsecond clock; an unbound bare-Wasm host cannot admit work.
pub fn default_now_us() -> Option<u64> {
    semio_framework_trace::try_now_us()
}

/// 🧮️ Deterministic per-thread clock that advances one microsecond per read: correctness laws drive
/// jobs with it so a descheduled test thread never trips the wall-clock overrun quarantine, while
/// timing laws and production keep [`default_now_us`].
pub fn logical_now_us() -> Option<u64> {
    thread_local! { static LOGICAL_NOW_US: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }
    Some(LOGICAL_NOW_US.with(|now| { now.set(now.get() + 1); now.get() }))
}

/// 🌐️ Converts the browser's fractional monotonic milliseconds without losing its sub-ms precision.
pub fn microseconds_from_milliseconds(milliseconds: f64) -> Option<u64> {
    semio_framework_trace::microseconds_from_milliseconds(milliseconds)
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
    /// ⛓️ A fresh [`Operation`] with its preview-sequence cursor at zero.
    pub fn new(operation: OperationId, base_revision: RevisionId, generation: Generation, seed: u64) -> Operation {
        Operation { operation, base_revision, generation, preview_sequence: 0, seed }
    }

    /// 🔢️ The next preview sequence number, advancing the cursor — one call per
    /// [`StepOutcome::PreviewReady`] a job for this operation emits.
    pub fn next_preview_sequence(&mut self) -> Result<u64, JobSequenceExhausted> {
        let sequence = self.preview_sequence;
        self.preview_sequence = self.preview_sequence.checked_add(1).ok_or(JobSequenceExhausted::Preview)?;
        Ok(sequence)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobSequenceExhausted {
    Preview,
    Step,
    Session,
    Child,
    Wake,
}

/// ✔️ Result of [`validate_commit`]: whether a [`CommitCandidate`]'s base revision/generation still
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
    if op.base_revision == live_revision && op.generation == live_generation { CommitValidation::Accepted } else { CommitValidation::Stale { live_revision, live_generation } }
}
//#endregion 🪪️Identity

//#region ⛽️Budget
/// ⛽️ Two-bound step budget: a fuel counter (job-defined instruction-equivalent units, decremented via
/// [`StepContext::consume_fuel`]) AND an absolute wall-clock `deadline_us` — design doc Decision 3.
/// `deadline_us` is ABSOLUTE (`now_us() + slice`), not a remaining duration, so a job never has to
/// re-derive wall-clock math from a countdown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepBudget {
    pub fuel: u64,
    pub deadline_us: u64,
    pub retained: RetainedCloneGrant,
}

impl StepBudget {
    pub fn new(fuel: u64, deadline_us: u64, retained:RetainedCloneGrant) -> StepBudget {
        StepBudget { fuel, deadline_us, retained }
    }

    pub fn from_duration(fuel: u64, start_us: u64, duration_us: u64, retained:RetainedCloneGrant) -> Option<StepBudget> {
        start_us.checked_add(duration_us).map(|deadline_us| Self { fuel, deadline_us, retained })
    }
}

/// 🎯️ Per-step wall budgets. Actor lane grants may span many steps; they are never reused as one
/// step's deadline. These values leave watchdog margin below the hard eight-millisecond ceiling.
pub const INTERACTIVE_LANE_WALL_US: u64 = 1_000;

pub const INTERACTIVE_LANE_FUEL: u64 = 2_000_000;

pub const USER_VISIBLE_LANE_WALL_US: u64 = 2_000;

pub const USER_VISIBLE_LANE_FUEL: u64 = 6_000_000;

pub const BACKGROUND_LANE_WALL_US: u64 = 4_000;

pub const BACKGROUND_LANE_FUEL: u64 = 20_000_000;

pub const MAINTENANCE_LANE_WALL_US: u64 = 4_000;

pub const MAINTENANCE_LANE_FUEL: u64 = 80_000_000;
//#endregion ⛽️Budget
