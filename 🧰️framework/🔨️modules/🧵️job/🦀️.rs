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

#[cfg(test)]
macro_rules! admit_original_fixture_owner {
 ($owner:ident,$job:expr,$params:expr $(,)?)=>{{
  let mut original_job=ManuallyDrop::new(Some($job));let mut original_params=ManuallyDrop::new(Some($params));let original=original_params.as_ref().unwrap();let original_grant=original.config.retained;let identity=(original.operation,original.generation);let clock=original.now_us;let mut recipient=RetainedCloneProgress::default();let mut control=WorkerJobAdmissionContext::new(identity.0,identity.1,StepBudget::new(1,u64::MAX,original_grant),clock,&mut recipient).unwrap();
  let admitted=$owner::try_admit_owned(&mut original_job,&mut original_params,&mut control);drop(control);match admitted{Ok(Some((owner,progress)))=>{assert!(original_job.is_none()&&original_params.is_none());assert_eq!(recipient,progress);assert!(progress.fits(original_grant));Ok(owner)},Ok(None)=>Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture owner admission retained its unchanged sources")),Err(error)=>Err(error)}
 }};
}

use std::future::Future;
use std::mem::{ManuallyDrop, MaybeUninit,size_of_val};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::RetirementDemand;

#[path="♻️retirement/🔔️wake/🦀️.rs"]
mod worker_wake_retirement;
pub use worker_wake_retirement::{RetainedWorkerWake,OriginalWorkerWakeIssuer,admit_original_worker_wake,original_worker_wake_admission_demands};
#[cfg(not(target_arch="wasm32"))]
pub use worker_wake_retirement::admit_original_thread_worker_wake;
use semio_framework_value::{ValueError, ValueRefusalKind};
#[cfg(test)]
use std::time::Instant;

use semio_framework_async::ChannelPolicy;
use semio_framework_trace::{TraceEvent, Watchdog, record_operation_started};

pub use semio_framework_async::CancelToken;
pub use semio_framework_async::{Lane, ProcessKind, WorkerPool, WorkerPoolConfig};
pub use semio_framework_trace::{Generation, InteractiveStage, OperationId, allocate_operation_id, allocate_operation_id_in_slot, interactive_step_contract_violated, runtime_diagnostics_enabled, set_runtime_diagnostics, INTERACTIVE_STEP_CEILING_US, RUNTIME_DIAGNOSTICS_ENV, SUSTAINED_OVERRUN_QUARANTINE_STEPS};

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
/// 🕰️ Coarse scheduler clock, never used to measure an interactive step deadline.
pub fn default_now_ms() -> Option<u64> {
    default_now_us().map(|now_us| now_us / 1_000)
}

/// 🔌️ Installs the embedding host's real monotonic microsecond clock exactly once.
pub fn install_microsecond_clock(clock: fn() -> Option<u64>) -> Result<(), fn() -> Option<u64>> {
    semio_framework_trace::install_clock(clock)
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use semio_framework_async::install_browser_monotonic_clock;

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
    /// 🌱️ A fresh [`Operation`] with its preview-sequence cursor at zero.
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
    if op.base_revision == live_revision && op.generation == live_generation { CommitValidation::Accepted } else { CommitValidation::Stale { live_revision, live_generation } }
}
//#endregion 🪪️Identity

//#region 🗄️FixedOperationRegistry
/// 🗄️ Typed retained owner admitted to a fixed operation scheduler.
pub trait FixedOperationOwner {
    fn retained_bytes(&self) -> usize;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep;
    fn terminal_is_empty(&self) -> bool;
}

/// 🪪️ Exact scheduler identity. Reusing an operation id with another generation is never an ACK.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedOperationKey {
    pub operation: OperationId,
    pub generation: Generation,
}

impl FixedOperationKey {
    pub const fn new(operation: OperationId, generation: Generation) -> Self {
        Self { operation, generation }
    }
}

/// ↩️ Failed fixed-registry admission returns the exact owner unchanged.
#[derive(Debug)]
pub struct FixedOperationAdmissionRejected<T> {
    pub key: FixedOperationKey,
    pub owner: T,
}

struct FixedOperationEntry<T> {
    key: FixedOperationKey,
    admitted_bytes: usize,
    closing: bool,
    owner: T,
}

/// 🗄️ Fixed scheduler authority for retained operation owners. Slots and byte credit are admitted
/// once; no operation can resize the registry or detach an owner without its exact identity.
pub struct FixedOperationRegistry<T, const CAPACITY: usize> {
    slots: Box<[Option<FixedOperationEntry<T>>]>,
    maximum_bytes: usize,
    retained_bytes: usize,
    occupied: usize,
    close_cursor: usize,
    allocation_admitted: bool,
}

impl<T: FixedOperationOwner, const CAPACITY: usize> FixedOperationRegistry<T, CAPACITY> {
    pub const MAXIMUM_SLOTS: usize = 64;

    pub fn new(maximum_bytes: usize) -> Self {
        let mut slots = Vec::new();
        let allocation_admitted = CAPACITY > 0 && CAPACITY <= Self::MAXIMUM_SLOTS && slots.try_reserve_exact(CAPACITY).is_ok();
        if allocation_admitted {
            slots.resize_with(CAPACITY, || None);
        }
        Self { slots: slots.into_boxed_slice(), maximum_bytes, retained_bytes: 0, occupied: 0, close_cursor: 0, allocation_admitted }
    }

    fn index(&self, key: FixedOperationKey) -> usize {
        ((key.operation.0 ^ key.generation.0.rotate_left(17)) as usize) % CAPACITY.max(1)
    }

    pub fn can_admit(&self, key: FixedOperationKey, retained_bytes: usize) -> bool {
        let Some(next_retained_bytes) = self.retained_bytes.checked_add(retained_bytes) else { return false };
        if !self.allocation_admitted || self.occupied == CAPACITY || next_retained_bytes > self.maximum_bytes {
            return false;
        }
        self.slots[self.index(key)].is_none()
    }

    pub fn admit(&mut self, key: FixedOperationKey, owner: T) -> Result<(), FixedOperationAdmissionRejected<T>> {
        let retained_bytes = owner.retained_bytes();
        if !self.can_admit(key, retained_bytes) {
            return Err(FixedOperationAdmissionRejected { key, owner });
        }
        let index = self.index(key);
        self.slots[index] = Some(FixedOperationEntry { key, admitted_bytes: retained_bytes, closing: false, owner });
        self.retained_bytes = self.retained_bytes.checked_add(retained_bytes).expect("fixed operation byte admission was checked before exact owner insertion");
        self.occupied += 1;
        Ok(())
    }

    pub fn get(&self, key: FixedOperationKey) -> Option<&T> {
        self.slots.get(self.index(key))?.as_ref().filter(|entry| entry.key == key && !entry.closing).map(|entry| &entry.owner)
    }

    pub fn get_mut(&mut self, key: FixedOperationKey) -> Option<&mut T> {
        let index = self.index(key);
        self.slots.get_mut(index)?.as_mut().filter(|entry| entry.key == key && !entry.closing).map(|entry| &mut entry.owner)
    }

    pub fn get_operation(&self, operation: OperationId) -> Option<(FixedOperationKey, &T)> {
        self.slots.iter().flatten().find(|entry| entry.key.operation == operation && !entry.closing).map(|entry| (entry.key, &entry.owner))
    }

    pub fn get_operation_mut(&mut self, operation: OperationId) -> Option<(FixedOperationKey, &mut T)> {
        self.slots.iter_mut().flatten().find(|entry| entry.key.operation == operation && !entry.closing).map(|entry| (entry.key, &mut entry.owner))
    }

    pub fn take(&mut self, key: FixedOperationKey) -> Option<T> {
        let index = self.index(key);
        if self.slots.get(index)?.as_ref().is_none_or(|entry| entry.key != key || entry.closing) {
            return None;
        }
        let entry = self.slots[index].take().expect("exact fixed operation owner remains admitted");
        self.retained_bytes -= entry.admitted_bytes;
        self.occupied -= 1;
        Some(entry.owner)
    }

    pub fn cancel(&mut self, key: FixedOperationKey) -> bool {
        let index = self.index(key);
        let Some(entry) = self.slots.get_mut(index).and_then(Option::as_mut).filter(|entry| entry.key == key) else { return false };
        entry.owner.cancel();
        entry.owner.begin_close();
        entry.closing = true;
        true
    }

    pub fn cancel_stale_step(&mut self, operation: OperationId, live_generation: Generation) -> bool {
        if !self.allocation_admitted {
            return false;
        }
        let index = self.close_cursor;
        self.close_cursor = (self.close_cursor + 1) % CAPACITY;
        let Some(entry) = self.slots[index].as_mut() else { return false };
        if entry.key.operation != operation || entry.key.generation == live_generation {
            return false;
        }
        entry.owner.cancel();
        entry.owner.begin_close();
        entry.closing = true;
        true
    }

    pub fn begin_close_step(&mut self) -> bool {
        if !self.allocation_admitted {
            return false;
        }
        let index = self.close_cursor;
        self.close_cursor = (self.close_cursor + 1) % CAPACITY;
        let Some(entry) = self.slots[index].as_mut() else { return false };
        if !entry.closing {
            entry.owner.cancel();
            entry.owner.begin_close();
            entry.closing = true;
        }
        true
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
        if !self.allocation_admitted || maximum_items == 0 {
            return InteractiveJobCloseStep::Blocked;
        }
        let index = self.close_cursor;
        self.close_cursor = (self.close_cursor + 1) % CAPACITY;
        let Some(entry) = self.slots[index].as_mut() else {
            return if self.is_empty() { InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() } } else { InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } } };
        };
        if !entry.closing {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        let child_grant=RetainedCloneGrant{maximum_items:1,..grant};
        let step = entry.owner.close_step(child_grant).admit(child_grant,entry.owner.terminal_is_empty());
        if matches!(step,InteractiveJobCloseStep::Complete{..}) {
            let entry = self.slots[index].take().expect("terminal fixed operation owner remains admitted");
            self.retained_bytes -= entry.admitted_bytes;
            self.occupied -= 1;
            drop(entry);
        }
        if self.is_empty() {
            InteractiveJobCloseStep::Complete { progress: step.progress() }
        } else {
            match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.occupied == 0
    }

    pub fn retained_bytes(&self) -> usize {
        self.retained_bytes
    }
}

impl<T, const CAPACITY: usize> Drop for FixedOperationRegistry<T, CAPACITY> {
    fn drop(&mut self) {
        assert_eq!(self.occupied, 0, "fixed operation registry reached Drop before every exact owner was terminal-empty");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️fixed-operation-registry/🦀️.rs"]
mod fixed_operation_registry_tests;
//#endregion 🗄️FixedOperationRegistry

#[path="👷️worker/🚪️admission/🦀️.rs"]
mod original_worker_admission;
pub use original_worker_admission::{WorkerJobAdmissionControl,WorkerJobAdmissionContext,WorkerJobAdmission,WorkerJobInitializer,WorkerJobInitializationStep,WorkerJobSessionPreparation};
#[path="🔭️preview/🎟️drive/🦀️.rs"]
mod original_preview_driver;
pub use original_preview_driver::{JobPreviewDriver,JobPreviewLimits,JobPreviewStatus,JobPreviewStep,JobPreviewVerdict};

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

//#region 📄️RetainedPayload
pub const JOB_PAYLOAD_PAGE_BYTES: usize = 16 * 1024;

pub const JOB_PAYLOAD_OPERATION_PAGES: usize = 256;
pub const JOB_PAYLOAD_OPERATION_BYTES: usize = JOB_PAYLOAD_PAGE_BYTES * JOB_PAYLOAD_OPERATION_PAGES;
pub const JOB_PAYLOAD_PROCESS_BYTES: usize = 64 * 1024 * 1024;

/// 🏛️ The live sum of every operation ledger's retained payload pages, capped by
/// [`JOB_PAYLOAD_PROCESS_BYTES`]. It is a CEILING over the whole process — every concurrent
/// operation moves it — so it stays private and no owner samples its absolute value; an owner
/// reads its own share through `JobPayloadOperationLedger::process_share_bytes`.
static JOB_PAYLOAD_PROCESS_OWNED_BYTES: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum JobPayloadStream {
    CheckpointState = 0,
    Preview = 1,
    CommitState = 2,
    CommitOutput = 3,
    Fault = 4,
}

impl JobPayloadStream {
    const COUNT: usize = 5;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobPayloadAdmissionFault {
    OpportunityExhausted,
    OperationItems,
    OperationBytes,
    ProcessBytes,
    StreamItems,
    StreamBytes,
    WriterFull,
    WriterSealed,
    RejectedSourcePending,
}

pub struct JobPayloadPageSource {
    storage: Box<[MaybeUninit<u8>]>,
}

impl std::fmt::Debug for JobPayloadPageSource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("JobPayloadPageSource").field("backing_identity", &self.backing_identity()).finish()
    }
}

impl JobPayloadPageSource {
    pub fn new() -> Self {
        Self::with_extent(JOB_PAYLOAD_PAGE_BYTES)
    }

    fn with_extent(extent: usize) -> Self { assert!(extent > 0 && extent <= JOB_PAYLOAD_PAGE_BYTES); Self { storage: Box::<[u8]>::new_uninit_slice(extent) } }

    pub fn allocated_capacity_bytes(&self) -> usize { self.storage.len() }

    pub fn backing_identity(&self) -> *const MaybeUninit<u8> {
        self.storage.as_ptr()
    }
}

impl Default for JobPayloadPageSource {
    fn default() -> Self {
        Self::new()
    }
}

pub struct JobPayloadRejectedPage {
    pub fault: JobPayloadAdmissionFault,
    source: ManuallyDrop<Option<JobPayloadPageSource>>,
}

impl std::fmt::Debug for JobPayloadRejectedPage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("JobPayloadRejectedPage").field("fault", &self.fault).field("source", &self.source).finish()
    }
}

impl JobPayloadRejectedPage {
    pub fn source(&self) -> &JobPayloadPageSource {
        self.source.as_ref().expect("rejected job payload page already returned")
    }

    pub fn into_source(mut self) -> JobPayloadPageSource {
        self.source.take().expect("rejected job payload page already returned")
    }
}

impl Drop for JobPayloadRejectedPage {
    fn drop(&mut self) {
        if self.source.is_none() {
            unsafe { ManuallyDrop::drop(&mut self.source) };
        } else {
            debug_assert!(false, "rejected job payload page requires exact source handback");
        }
    }
}

struct JobPayloadOperationLedger {
    operation: OperationId,
    generation: Generation,
    pages: AtomicUsize,
    bytes: AtomicUsize,
    stream_pages: [AtomicUsize; JobPayloadStream::COUNT],
    stream_bytes: [AtomicUsize; JobPayloadStream::COUNT],
}

impl JobPayloadOperationLedger {
    fn new(operation: OperationId, generation: Generation) -> Self {
        Self { operation, generation, pages: AtomicUsize::new(0), bytes: AtomicUsize::new(0), stream_pages: std::array::from_fn(|_| AtomicUsize::new(0)), stream_bytes: std::array::from_fn(|_| AtomicUsize::new(0)) }
    }

    fn reserve(&self,stream:JobPayloadStream,extent:usize)->Result<(),JobPayloadAdmissionFault>{self.reserve_original(stream,extent).map(|_|()).map_err(|(fault,_)|fault)}
    fn reserve_original(&self,stream:JobPayloadStream,extent:usize)->Result<usize,(JobPayloadAdmissionFault,usize)>{
        let index=stream as usize;let owners=[(&self.pages,1,JOB_PAYLOAD_OPERATION_PAGES,JobPayloadAdmissionFault::OperationItems),(&self.bytes,extent,JOB_PAYLOAD_OPERATION_BYTES,JobPayloadAdmissionFault::OperationBytes),(&self.stream_pages[index],1,JOB_PAYLOAD_OPERATION_PAGES,JobPayloadAdmissionFault::StreamItems),(&self.stream_bytes[index],extent,JOB_PAYLOAD_OPERATION_BYTES,JobPayloadAdmissionFault::StreamBytes),(&JOB_PAYLOAD_PROCESS_OWNED_BYTES,extent,JOB_PAYLOAD_PROCESS_BYTES,JobPayloadAdmissionFault::ProcessBytes)];
        for (done,(owner,amount,maximum,fault))in owners.iter().enumerate(){if owner.try_update(Ordering::AcqRel,Ordering::Acquire,|value|value.checked_add(*amount).filter(|value|*value<=*maximum)).is_err(){for (original,amount,_,_)in owners[..done].iter().rev(){original.fetch_sub(*amount,Ordering::AcqRel);}return Err((*fault,0));}}Ok(0)
    }

    fn release(&self, stream: JobPayloadStream, extent: usize) {
        let stream_index = stream as usize;
        JOB_PAYLOAD_PROCESS_OWNED_BYTES.fetch_sub(extent, Ordering::AcqRel);
        self.stream_bytes[stream_index].fetch_sub(extent, Ordering::AcqRel);
        self.stream_pages[stream_index].fetch_sub(1, Ordering::AcqRel);
        self.bytes.fetch_sub(extent, Ordering::AcqRel);
        self.pages.fetch_sub(1, Ordering::AcqRel);
    }

    fn terminal_is_empty(&self) -> bool {
        self.pages.load(Ordering::Acquire) == 0 && self.bytes.load(Ordering::Acquire) == 0 && self.stream_pages.iter().all(|count| count.load(Ordering::Acquire) == 0) && self.stream_bytes.iter().all(|count| count.load(Ordering::Acquire) == 0)
    }

    /// 🧾️ This ledger's own share of [`JOB_PAYLOAD_PROCESS_BYTES`]. [`Self::reserve`] and
    /// [`Self::release`] are the ONLY mutators of `JOB_PAYLOAD_PROCESS_OWNED_BYTES`, and each moves
    /// the process counter and this ledger's `bytes` by the same page in the same call, so the
    /// process counter is exactly the sum of every live ledger's share and a ledger reading zero
    /// here has returned every page it ever took from the process budget. This — not the process
    /// counter's absolute value — is the observable an owner may assert: the absolute value is
    /// moved by every other operation alive in the process, so sampling it before and after a
    /// close ladder answers about the whole process rather than about this owner.
    fn process_share_bytes(&self) -> usize {
        self.bytes.load(Ordering::Acquire)
    }
}

struct JobPayloadPage {
    source: JobPayloadPageSource,
    length: usize,
}

impl JobPayloadPage {
    fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.source.storage.as_ptr().cast::<u8>(), self.length) }
    }
}

pub struct RetainedJobPayload {
    stream: JobPayloadStream,
    pages: ManuallyDrop<[Option<JobPayloadPage>; JOB_PAYLOAD_OPERATION_PAGES]>,
    page_count: usize,
    length: usize,
    ledger: ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>,
}

impl RetainedJobPayload {
    pub fn empty(stream: JobPayloadStream) -> Self {
        Self { stream, pages: ManuallyDrop::new(std::array::from_fn(|_| None)), page_count: 0, length: 0, ledger: ManuallyDrop::new(None) }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn page_count(&self) -> usize {
        self.page_count
    }

    pub fn page(&self, index: usize) -> Option<&[u8]> {
        self.pages.get(index).and_then(Option::as_ref).map(JobPayloadPage::bytes)
    }

    pub fn single_page(&self) -> Option<&[u8]> {
        (self.page_count == 1).then(|| self.page(0)).flatten()
    }

    pub fn reader(&self) -> RetainedJobPayloadReader<'_> {
        RetainedJobPayloadReader { payload: self, page: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.page_count == 0 && self.length == 0 && self.pages.iter().all(Option::is_none) && self.ledger.is_none()
    }
}

pub struct RetainedJobPayloadReader<'a> {
    payload: &'a RetainedJobPayload,
    page: usize,
}

impl<'a> RetainedJobPayloadReader<'a> {
    pub fn read_page(&mut self, maximum_items: usize, maximum_bytes: usize) -> Option<&'a [u8]> {
        if maximum_items == 0 {
            return None;
        }
        let page = self.payload.page(self.page)?;
        if page.len() > maximum_bytes {
            return None;
        }
        self.page += 1;
        Some(page)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.page == self.payload.page_count()
    }
}

impl std::fmt::Debug for RetainedJobPayload {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("RetainedJobPayload").field("stream", &self.stream).field("page_count", &self.page_count).field("length", &self.length).finish()
    }
}

impl PartialEq for RetainedJobPayload {
    fn eq(&self, other: &Self) -> bool {
        self.stream == other.stream && self.length == other.length && self.page_count == other.page_count && (0..self.page_count).all(|index| self.page(index) == other.page(index))
    }
}

impl Eq for RetainedJobPayload {}

impl Drop for RetainedJobPayload {
    fn drop(&mut self) {
        assert!(std::thread::panicking()||self.terminal_is_empty(),"retained job payload abandoned original pages or operation ledger");
        if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.pages);ManuallyDrop::drop(&mut self.ledger);}}
    }
}

pub struct RetainedJobPayloadWriter {
    payload: ManuallyDrop<Option<RetainedJobPayload>>,
    rejected: ManuallyDrop<Option<JobPayloadPageSource>>,
    staged: ManuallyDrop<Option<(Arc<JobPayloadOperationLedger>, JobPayloadPageSource, usize)>>,
    closing_ledger: ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>,
    sealed: bool,
}

impl std::fmt::Debug for RetainedJobPayloadWriter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("RetainedJobPayloadWriter").field("payload", &self.payload).field("rejected", &self.rejected).field("staged", &self.staged.as_ref().map(|(_, _, length)| length)).field("sealed", &self.sealed).finish()
    }
}

impl RetainedJobPayloadWriter {
    pub fn new(stream: JobPayloadStream) -> Self {
        Self { payload: ManuallyDrop::new(Some(RetainedJobPayload::empty(stream))), rejected: ManuallyDrop::new(None), staged: ManuallyDrop::new(None), closing_ledger: ManuallyDrop::new(None), sealed: false }
    }

    pub fn take_rejected_source(&mut self) -> Option<JobPayloadPageSource> {
        self.rejected.take()
    }

    pub fn page_count(&self) -> usize {
        self.payload.as_ref().map_or(0, RetainedJobPayload::page_count)
    }

    pub fn admit_page<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<JobPayloadPageGrant<'a>, JobPayloadAdmissionFault> {
        let source = self.rejected.take().unwrap_or_default();
        if cx.payload_page_granted {
            *self.rejected = Some(source);
            return Err(JobPayloadAdmissionFault::OpportunityExhausted);
        }
        let ledger = Arc::clone(&cx.payload_ledger);
        if let Err(fault) = self.reserve_page(&ledger,source.allocated_capacity_bytes()) {
            *self.rejected = Some(source);
            return Err(fault);
        }
        cx.payload_page_granted = true;
        Ok(self.begin_page(ledger, source))
    }

    #[expect(clippy::result_large_err, reason = "A refused finish returns the exact admitted writer and its staged pages without allocating.")]
    pub fn finish(mut self) -> Result<RetainedJobPayload, Self> {
        if self.rejected.is_some() || self.staged.is_some() || self.closing_ledger.is_some() {
            return Err(self);
        }
        self.sealed = true;
        Ok(self.payload.take().expect("retained payload writer owns payload until finish"))
    }

    pub fn begin_close(&mut self) {
        self.sealed = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.sealed && self.rejected.is_none() && self.staged.is_none() && self.closing_ledger.is_none() && self.payload.is_none()
    }

    pub fn begin_staged_page(&mut self, cx: &mut StepContext<'_>) -> Result<(), JobPayloadAdmissionFault> {
        if self.staged.is_some() {
            return Ok(());
        }
        self.admit_page(cx)?.stage();
        Ok(())
    }

    pub fn staged_page_remaining(&self) -> usize {
        self.staged.as_ref().map_or(0, |(_, source, length)| source.allocated_capacity_bytes() - length)
    }

    pub fn staged_page_len(&self) -> Option<usize> {
        self.staged.as_ref().map(|(_, _, length)| *length)
    }

    pub fn write_staged(&mut self, bytes: &[u8]) -> Result<(), JobPayloadAdmissionFault> {
        let (_, source, length) = self.staged.as_mut().ok_or(JobPayloadAdmissionFault::OpportunityExhausted)?;
        if bytes.len() > source.allocated_capacity_bytes() - *length {
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), source.storage.as_mut_ptr().cast::<u8>().add(*length), bytes.len()) };
        *length += bytes.len();
        Ok(())
    }

    pub fn commit_staged_page(&mut self) -> Result<(), JobPayloadAdmissionFault> {
        let index = self.payload.as_ref().ok_or(JobPayloadAdmissionFault::WriterSealed)?.pages.iter().position(Option::is_none).ok_or(JobPayloadAdmissionFault::WriterFull)?;
        let (ledger, source, length) = self.staged.take().ok_or(JobPayloadAdmissionFault::OpportunityExhausted)?;
        let payload = self.payload.as_mut().expect("staged page commit preflight retains writer payload");
        payload.pages[index] = Some(JobPayloadPage { source, length });
        payload.page_count += 1;
        payload.length += length;
        *payload.ledger = Some(ledger);
        Ok(())
    }

    pub fn write_slice_page(&mut self, cx: &mut StepContext<'_>, bytes: &[u8], cursor: &mut usize) -> Result<bool, JobPayloadAdmissionFault> {
        if *cursor > bytes.len() {
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        if *cursor == bytes.len() {
            return Ok(true);
        }
        if cx.should_yield() {
            return Ok(false);
        }
        let mut page = self.admit_page(cx)?;
        let end = cursor.saturating_add(page.remaining()).min(bytes.len());
        page.write(&bytes[*cursor..end])?;
        page.commit();
        *cursor = end;
        Ok(*cursor == bytes.len())
    }

    fn reserve_page(&self, ledger: &JobPayloadOperationLedger, extent: usize) -> Result<(), JobPayloadAdmissionFault> {
        if self.sealed {
            return Err(JobPayloadAdmissionFault::WriterSealed);
        }
        if self.rejected.is_some() {
            return Err(JobPayloadAdmissionFault::RejectedSourcePending);
        }
        let payload = self.payload.as_ref().ok_or(JobPayloadAdmissionFault::WriterSealed)?;
        if payload.page_count >= JOB_PAYLOAD_OPERATION_PAGES {
            return Err(JobPayloadAdmissionFault::WriterFull);
        }
        ledger.reserve(payload.stream,extent)
    }

    fn begin_page(&mut self, ledger: Arc<JobPayloadOperationLedger>, source: JobPayloadPageSource) -> JobPayloadPageGrant<'_> {
        JobPayloadPageGrant { writer: self, ledger: Some(ledger), source: Some(source), length: 0, committed: false }
    }
}

impl Drop for RetainedJobPayloadWriter {
    fn drop(&mut self) {
        if self.payload.is_none() && self.rejected.is_none() && self.staged.is_none() && self.closing_ledger.is_none() {
            unsafe {
                ManuallyDrop::drop(&mut self.payload);
                ManuallyDrop::drop(&mut self.rejected);
                ManuallyDrop::drop(&mut self.staged);
            }
        } else {
            debug_assert!(false, "retained job payload writer requires exact finish or incremental close");
        }
    }
}

pub struct JobPayloadPageGrant<'a> {
    writer: &'a mut RetainedJobPayloadWriter,
    ledger: Option<Arc<JobPayloadOperationLedger>>,
    source: Option<JobPayloadPageSource>,
    length: usize,
    committed: bool,
}

impl JobPayloadPageGrant<'_> {
    pub fn remaining(&self) -> usize {
        self.source.as_ref().expect("original page grant retains backing").allocated_capacity_bytes() - self.length
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<(), JobPayloadAdmissionFault> {
        if bytes.len() > self.remaining() {
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        let source = self.source.as_mut().expect("uncommitted job payload grant owns page source");
        for (target, byte) in source.storage[self.length..self.length + bytes.len()].iter_mut().zip(bytes.iter().copied()) {
            target.write(byte);
        }
        self.length += bytes.len();
        Ok(())
    }

    pub fn initialized_remaining_mut(&mut self) -> &mut [u8] {
        let source = self.source.as_mut().expect("uncommitted job payload grant owns page source");
        for byte in &mut source.storage[self.length..] {
            byte.write(0);
        }
        unsafe { std::slice::from_raw_parts_mut(source.storage.as_mut_ptr().cast::<u8>().add(self.length), source.allocated_capacity_bytes() - self.length) }
    }

    pub fn advance_written(&mut self, bytes: usize) -> Result<(), JobPayloadAdmissionFault> {
        if bytes > self.remaining() {
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        self.length += bytes;
        Ok(())
    }

    pub fn commit(mut self) {
        let payload = self.writer.payload.as_mut().expect("retained payload writer owns payload while page is granted");
        let index = payload.pages.iter().position(Option::is_none).expect("preflighted payload page slot remains vacant");
        let source = self.source.take().expect("committed job payload grant owns page source");
        payload.pages[index] = Some(JobPayloadPage { source, length: self.length });
        payload.page_count += 1;
        payload.length += self.length;
        *payload.ledger = self.ledger.take();
        self.committed = true;
    }

    pub fn stage(mut self) {
        let ledger = self.ledger.take().expect("admitted staged page owns ledger credit");
        let source = self.source.take().expect("admitted staged page owns backing");
        *self.writer.staged = Some((ledger, source, self.length));
        self.committed = true;
    }
}

impl Drop for JobPayloadPageGrant<'_> {
    fn drop(&mut self) {
        if self.committed {
            return;
        }
        if let Some(ledger) = self.ledger.take() {
            let stream = self.writer.payload.as_ref().expect("retained payload writer owns payload while grant is live").stream;
            ledger.release(stream,self.source.as_ref().expect("original page grant retains backing").allocated_capacity_bytes());
        }
        *self.writer.rejected = self.source.take();
    }
}
#[path="♻️retirement/📄️payload/🦀️.rs"]
mod payload_retirement;
pub use payload_retirement::{close_step_outcome_slot,step_outcome_slot_retirement_demands,JobOutcomeSlot,JobPayloadSlot};
pub use payload_retirement::WorkerJobSource;
#[path="📬️outcome/🤝️loan/🦀️.rs"]
mod outcome_loan;
pub use outcome_loan::{JobOutcomeAdmission,JobOutcomeBorrow,drive_step};
#[cfg(test)]
#[path="📬️outcome/⚠️fault-binding/🧪️tests/🦀️.rs"]
mod original_fault_binding_tests;
#[path="📬️outcome/🎟️descriptor/🦀️.rs"]
mod original_outcome_descriptor;
pub use original_outcome_descriptor::{JobOutcomeDescriptor,JobOutcomeKind,JobOutcomeView};
use original_outcome_descriptor::JobOutcomeDescriptorSlot;
#[path="📬️outcome/📖️source/🦀️.rs"]
mod original_payload_source;
#[path="📬️outcome/📦️builder/🦀️.rs"]
mod payload_builder;
pub use payload_builder::{JobPayloadAuthority,RetainedPayloadBuilder};
#[path="📬️outcome/📤️publication/🦀️.rs"]
mod original_publication;
pub use original_publication::{JobPublicationKind,RetainedJobPublication};
#[path="📬️outcome/📤️publication/⚠️fault/🦀️.rs"]
mod original_fault_publication;
pub use original_fault_publication::RetainedFaultPublication;

enum JobPayloadLedgerReference<'a>{Owned(Arc<JobPayloadOperationLedger>),Borrowed(&'a JobPayloadAuthority),OriginalWorker(&'a Arc<JobPayloadOperationLedger>)}
impl std::ops::Deref for JobPayloadLedgerReference<'_>{type Target=Arc<JobPayloadOperationLedger>;fn deref(&self)->&Self::Target{match self{Self::Owned(original)=>original,Self::OriginalWorker(original)=>original,Self::Borrowed(authority)=>authority.original().expect("original context outlived its payload authority")}}}
enum StepCancelReference<'a>{Owned(CancelToken),Borrowed(&'a CancelToken)}
impl std::ops::Deref for StepCancelReference<'_>{type Target=CancelToken;fn deref(&self)->&Self::Target{match self{Self::Owned(original)=>original,Self::Borrowed(original)=>original}}}
//#endregion 📄️RetainedPayload

//#region 🧭️StepContext
/// 🧭️ Everything one [`InteractiveJob::step`] call needs: identity ([`OperationId`]/[`Generation`]),
/// the two-bound budget, cancellation, the clock, and the running preview-sequence cursor. Fields are
/// private with accessor methods (a deliberate narrowing from the design doc's Decision 1 sketch,
/// which exposed `pub fuel: &mut u64`/`pub cancel: CancelToken` directly) so [`StepContext::is_cancelled`]
/// can own the [`poll_ready_now`] seam in exactly one place instead of every job reimplementing it.
pub struct StepContext<'a> {
    retained:RetainedCloneGrant,
    retained_progress:&'a mut RetainedCloneProgress,
    operation: OperationId,
    generation: Generation,
    fuel_remaining: u64,
    deadline_us: u64,
    now_us: fn() -> Option<u64>,
    clock: std::cell::Cell<ClockStride>,
    cancel: StepCancelReference<'a>,
    stage: &'static str,
    preview_sequence: &'a mut u64,
    payload_ledger: JobPayloadLedgerReference<'a>,
    payload_page_granted: bool,
    retained_work: retained_work::RetainedWorkBudget,
}

impl<'a> StepContext<'a> {
    fn with_original_worker_authority(operation:OperationId,generation:Generation,budget:StepBudget,cancel:&'a CancelToken,now_us:fn()->Option<u64>,clock:ClockStride,preview_sequence:&'a mut u64,retained_progress:&'a mut RetainedCloneProgress,payload_ledger:&'a Arc<JobPayloadOperationLedger>)->Result<Self,ValueError>{if payload_ledger.operation!=operation||payload_ledger.generation!=generation||!retained_progress.fits(budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"worker context requires its original admitted identity ledger and recipient"))}Ok(Self{retained:budget.retained,retained_progress,operation,generation,fuel_remaining:budget.fuel,deadline_us:budget.deadline_us,now_us,clock:std::cell::Cell::new(clock),cancel:StepCancelReference::Borrowed(cancel),stage:"initial",preview_sequence,payload_ledger:JobPayloadLedgerReference::OriginalWorker(payload_ledger),payload_page_granted:false})}
    pub fn new(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, preview_sequence: &'a mut u64,retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        StepContext::with_payload_ledger(operation, generation, budget, cancel, now_us, ClockStride::new(), preview_sequence, Arc::new(JobPayloadOperationLedger::new(operation, generation)),retained_progress)
    }

    #[allow(clippy::too_many_arguments)]
    fn with_payload_ledger(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, clock: ClockStride, preview_sequence: &'a mut u64, payload_ledger: Arc<JobPayloadOperationLedger>,retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        assert_eq!(payload_ledger.operation, operation, "job payload ledger operation must match its step context");
        assert_eq!(payload_ledger.generation, generation, "job payload ledger generation must match its step context");
        assert!(retained_progress.fits(budget.retained),"original job recipient exceeds its incoming retained grant");
        StepContext { retained:budget.retained, retained_progress, operation, generation, fuel_remaining: budget.fuel, deadline_us: budget.deadline_us, now_us, clock: std::cell::Cell::new(clock), cancel:StepCancelReference::Owned(cancel), stage: "initial", preview_sequence, payload_ledger:JobPayloadLedgerReference::Owned(payload_ledger), payload_page_granted: false }
    }

    /// 🤝️ Borrows the original paid ledger and cancellation while preserving the caller's exact recipient.
    pub fn with_payload_authority(operation:OperationId,generation:Generation,budget:StepBudget,cancel:&'a CancelToken,now_us:fn()->Option<u64>,preview_sequence:&'a mut u64,retained_progress:&'a mut RetainedCloneProgress,authority:&'a JobPayloadAuthority)->Result<Self,ValueError>{
        let Some(original)=authority.original()else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload authority has already closed"))};
        if original.operation!=operation||original.generation!=generation{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload authority identity does not match caller turn"))}
        if !retained_progress.fits(budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job recipient exceeds incoming retained grant"))}
        Ok(Self{retained:budget.retained,retained_progress,operation,generation,fuel_remaining:budget.fuel,deadline_us:budget.deadline_us,now_us,clock:std::cell::Cell::new(ClockStride::new()),cancel:StepCancelReference::Borrowed(cancel),stage:"initial",preview_sequence,payload_ledger:JobPayloadLedgerReference::Borrowed(authority),payload_page_granted:false})
    }

    /// 🎟️ Preserves the original caller depth and independently unspent physical credits.
    pub fn retained_grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.retained.maximum_items.saturating_sub(self.retained_progress.copied_items),maximum_copy_bytes:self.retained.maximum_copy_bytes.saturating_sub(self.retained_progress.copied_bytes),maximum_capacity_bytes:self.retained.maximum_capacity_bytes.saturating_sub(self.retained_progress.retained_capacity_bytes),maximum_release_bytes:self.retained.maximum_release_bytes.saturating_sub(self.retained_progress.released_bytes),..self.retained}}
    /// 🧾️ Records actual child effects in the external recipient before refusing exceeded authority.
    pub fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{
        let admitted=self.retained_progress.fits(self.retained)&&progress.fits(self.retained_grant());
        *self.retained_progress=self.retained_progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;
        if !admitted{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"job child receipt exceeds original remaining retained grant").with_retained_progress(progress));}Ok(())
    }
    /// 📊️ Exposes actual effects recorded by this same original step context, including failed turns.
    pub fn retained_progress(&self)->RetainedCloneProgress{*self.retained_progress}

    /// 🕰️ Borrows the caller's original clock source for a retained child authority.
    pub fn clock_source(&self)->fn()->Option<u64>{self.now_us}

    pub fn operation(&self) -> OperationId {
        self.operation
    }

    pub fn generation(&self) -> Generation {
        self.generation
    }

    /// 🤝️ Borrows this caller's original cancellation witness without issuing another alias.
    pub fn original_cancel_token(&self)->&CancelToken{&self.cancel}

    /// 🤝️ Issues one alias only after this original context admits its metadata item and depth.
    pub fn admit_original_cancel_alias(&mut self)->Result<Option<CancelToken>,ValueError>{
        let grant=self.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||self.is_cancelled()||self.should_yield(){return Ok(None)}
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.consume_retained(progress)?;
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant,progress,"original cancellation alias metadata")?;
        Ok(Some(self.original_cancel_token().clone()))
    }

    /// 🏷️ The label passed to the most recent [`StepContext::set_stage`] call (`"initial"` before the
    /// first one).
    pub fn stage(&self) -> &'static str {
        self.stage
    }

    /// ⏱️ The step's clock, read for real only every [`ClockStride`] calls — see there. A job may
    /// call this after every unit of work; inside a Wasm guest each real read is a host call.
    pub fn now_us(&self) -> Option<u64> {
        let mut clock = self.clock.get();
        let now = clock.read(self.now_us);
        self.clock.set(clock);
        now
    }

    /// 🕰️ The latest reading this step holds, without reading the clock.
    pub fn latest_us(&self) -> Option<u64> {
        self.clock.get().latest_us()
    }

    pub fn deadline_us(&self) -> u64 {
        self.deadline_us
    }

    pub fn deadline_exceeded(&self) -> bool {
        self.now_us().is_none_or(|now_us| now_us >= self.deadline_us)
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
    ///
    /// 🕰️ The event is stamped with the step's latest reading ([`StepContext::latest_us`]), at most
    /// one stride old, instead of a read of its own.
    pub fn set_stage(&mut self, label: &'static str) -> Option<TraceEvent> {
        self.stage = label;
        let at_us = self.latest_us().or_else(|| self.now_us());
        semio_framework_trace::record_trace_event_at(self.operation, self.generation, semio_framework_trace::TraceStage::StageChanged { label }, at_us)
    }

    /// 🔢️ The next preview-sequence number for this operation, advancing a cursor that survives
    /// across every [`StepContext`] built for the same retained session — one call per
    /// [`StepOutcome::PreviewReady`]/[`ProgressEvent::PreviewPatch`] a job emits.
    /// 🔢️ Reads the original caller cursor without admitting another preview.
    pub fn preview_sequence(&self) -> u64 { *self.preview_sequence }

    pub fn next_preview_sequence(&mut self) -> Result<u64, JobSequenceExhausted> {
        let sequence = *self.preview_sequence;
        *self.preview_sequence = (*self.preview_sequence).checked_add(1).ok_or(JobSequenceExhausted::Preview)?;
        Ok(sequence)
    }

    pub fn admit_payload_page<'b>(&mut self, writer: &'b mut RetainedJobPayloadWriter, source: JobPayloadPageSource) -> Result<JobPayloadPageGrant<'b>, JobPayloadRejectedPage> {
        if self.payload_page_granted {
            return Err(JobPayloadRejectedPage { fault: JobPayloadAdmissionFault::OpportunityExhausted, source: ManuallyDrop::new(Some(source)) });
        }
        let ledger = Arc::clone(&self.payload_ledger);
        if let Err(fault) = writer.reserve_page(&ledger,source.allocated_capacity_bytes()) {
            return Err(JobPayloadRejectedPage { fault, source: ManuallyDrop::new(Some(source)) });
        }
        self.payload_page_granted = true;
        Ok(writer.begin_page(ledger, source))
    }

    pub fn payload_from_bytes(&mut self, stream: JobPayloadStream, bytes: &[u8]) -> Result<RetainedJobPayload, JobPayloadRejectedPage> {
        let source = JobPayloadPageSource::new();
        if bytes.len() > JOB_PAYLOAD_PAGE_BYTES {
            return Err(JobPayloadRejectedPage { fault: JobPayloadAdmissionFault::StreamBytes, source: ManuallyDrop::new(Some(source)) });
        }
        let mut writer = RetainedJobPayloadWriter::new(stream);
        let rejected = match self.admit_payload_page(&mut writer, source) {
            Ok(mut page) => {
                page.write(bytes).expect("single-page payload was length-checked before write");
                page.commit();
                None
            }
            Err(rejected) => Some(rejected),
        };
        let payload = writer.finish().unwrap_or_else(|_| unreachable!("single-page admission leaves no source inside the writer"));
        match rejected {
            Some(rejected) => Err(rejected),
            None => Ok(payload),
        }
    }
}
//#endregion 🧭️StepContext

//#region 🪜️ClockStride
/// 🪜️ A step's clock, read for real only every `stride` calls. A job that checks its deadline after
/// every unit of work used to read the host clock just as often; inside a Wasm guest that read is a
/// component-model host call, and measured 2026-09-24 (ticket 26/09/23 slice G6) the `🀄️wfc` genesis
/// solve made 6.7 million of them in one `inference_run`. The stride is recalibrated at every real
/// read so reads land about [`ClockStride::TARGET_READ_INTERVAL_US`] apart, which is also the most a
/// deadline is overshot by. Every [`StepContext`] a driver builds carries one: the step begins at the
/// driver's own entry reading ([`ClockStride::begin`]), so a new step costs no read of its own, and the
/// calibrated stride carries from step to step. Law: `🧫️fixtures/🪜️clock-stride-law.json`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockStride {
    stride: u32,
    remaining: u32,
    last_read_us: Option<u64>,
}

impl ClockStride {
    pub const TARGET_READ_INTERVAL_US: u64 = 64;
    pub const MAXIMUM_STRIDE: u32 = 4_096;

    pub const fn new() -> Self {
        Self { stride: 1, remaining: 0, last_read_us: None }
    }

    /// 🏁️ Begins a step at `start_us`, the reading its driver made at entry: the step's first
    /// `stride - 1` reads answer it, and the calibrated stride is kept.
    pub fn begin(&mut self, start_us: Option<u64>) {
        self.last_read_us = start_us;
        self.remaining = if start_us.is_some() { self.stride - 1 } else { 0 };
    }

    /// ⏱️ The clock as of the last real read of `clock`, which is at most one stride of calls old.
    pub fn read(&mut self, clock: fn() -> Option<u64>) -> Option<u64> {
        if self.remaining > 0 {
            self.remaining -= 1;
            return self.last_read_us;
        }
        let now = clock();
        if let (Some(now), Some(last)) = (now, self.last_read_us) {
            let elapsed = now.saturating_sub(last).max(1);
            let calibrated = u64::from(self.stride) * Self::TARGET_READ_INTERVAL_US / elapsed;
            self.stride = calibrated.clamp(1, u64::from(Self::MAXIMUM_STRIDE).min(u64::from(self.stride) * 2)) as u32;
        }
        self.last_read_us = now;
        self.remaining = self.stride - 1;
        now
    }

    /// 🕰️ The last reading, without reading the clock.
    pub fn latest_us(&self) -> Option<u64> {
        self.last_read_us
    }

    /// 🪜️ How many calls the next real read is apart — the law's observable.
    pub fn stride(&self) -> u32 {
        self.stride
    }
}

impl Default for ClockStride {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🪜️ClockStride

//#region 🚦️StepOutcome
/// 📸️ A pause point where work is resumable but not yet committed — `state` is opaque, pack-encoded
/// (or, for a dependency-free job like [`TortureJob`], hand-rolled little-endian) bytes the job alone
/// interprets; `applied_progress` is the Puzzle 3D `FillBuilder.applied_count` pattern generalized: how
/// much of `state` is COMMITTED versus merely planned, so a caller can show "these N are done" without
/// decoding `state` itself.
#[derive(Debug, PartialEq, Eq)]
pub struct Checkpoint {
    pub state: RetainedJobPayload,
    pub applied_progress: u64,
}

/// 🏁️ Terminal success payload: the job's final persisted `state` plus its `output` — both opaque
/// bytes, so the runtime stays completely job-agnostic (design doc Decision 2).
#[derive(Debug, PartialEq, Eq)]
pub struct CommitCandidate {
    pub state: RetainedJobPayload,
    pub output: RetainedJobPayload,
}

/// 💥️ Opaque, job-specific error payload — never interpreted by the runtime, same reasoning as
/// [`CommitCandidate`]'s fields.
#[derive(Debug, PartialEq, Eq)]
pub struct JobFault {
    pub detail: RetainedJobPayload,
}

/// 🚦️ What one [`InteractiveJob::step`] call reports. [`StepOutcome::Yield`]/[`StepOutcome::PreviewReady`]/
/// [`StepOutcome::CheckpointReady`] all mean "call `step` again"; [`StepOutcome::is_terminal`] marks
/// the other three.
#[derive(Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "Step outcomes move preadmitted payload pages inline; boxing would allocate outside the step grant.")]
pub enum StepOutcome {
    Yield,
    PreviewReady(RetainedJobPayload),
    CheckpointReady(Checkpoint),
    Complete(CommitCandidate),
    Cancelled,
    Fault(JobFault),
}

impl StepOutcome {
    pub fn is_terminal(&self) -> bool {
        matches!(self, StepOutcome::Complete(_) | StepOutcome::Cancelled | StepOutcome::Fault(_))
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            StepOutcome::Yield | StepOutcome::Cancelled => true,
            StepOutcome::PreviewReady(payload) => payload.terminal_is_empty(),
            StepOutcome::CheckpointReady(checkpoint) => checkpoint.state.terminal_is_empty(),
            StepOutcome::Complete(candidate) => candidate.state.terminal_is_empty() && candidate.output.terminal_is_empty(),
            StepOutcome::Fault(fault) => fault.detail.terminal_is_empty(),
        }
    }
}
//#endregion 🚦️StepOutcome

//#region 🧩️InteractiveJob
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractiveJobCloseStep {
    Pending { progress: RetainedCloneProgress },
    Blocked,
    Complete { progress: RetainedCloneProgress },
    Refused { kind:ValueRefusalKind, progress:RetainedCloneProgress }
}

impl InteractiveJobCloseStep {
    /// 📊️ Keeps every admitted turn's physical and logical receipt, including terminal release.
    pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}|Self::Refused{progress,..}=>progress,Self::Blocked=>RetainedCloneProgress::default()}}
    /// 🛡️ Checks independent authority and the original owner's terminal witness before publication.
    pub fn admit(self,grant:RetainedCloneGrant,terminal_is_empty:bool)->Self{
        if !self.progress().fits(grant)||matches!(self,Self::Complete{..})&&!terminal_is_empty{Self::Refused{kind:ValueRefusalKind::InvariantViolated,progress:self.progress()}}else{self}
    }
}

/// 🧵️ Whether a job may be handed to another thread. Every threaded target elaborates this to
/// `Send`, so `J: InteractiveJob` still proves `J: Send` for [`WorkerJobSessionInner`]'s
/// `unsafe impl` and every pool submission. The browser build (`wasm32`, not `wasip2`) has no
/// second thread to hand a job to at all — its jobs own `Rc<JsValue>` browser handles by design
/// (`🎯️targets/🧊️wgpu/🧵️frame-job`'s frame worker is the single JS worker thread) — so requiring
/// `Send` there would only forbid the one ownership model that target can have.
#[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
pub trait JobThreadTransfer: Send {}
#[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
impl<T: Send + ?Sized> JobThreadTransfer for T {}

/// 🧵️ Browser twin of the marker above — see its docstring.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub trait JobThreadTransfer {}
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl<T: ?Sized> JobThreadTransfer for T {}

/// 🧩️ The protocol every interactive operation implements instead of a run-to-completion function
/// call — see the module doc's governing rule. `step` is bounded (checks [`StepContext::should_yield`]
/// and returns before the hard ceiling), cancellable ([`StepContext::is_cancelled`]) and explicitly
/// resumable (a fresh [`StepContext`] each call, job-owned state carries everything between calls).
pub trait InteractiveJob: JobThreadTransfer {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError>;
    fn borrow_outcome<'a>(&'a self, descriptor:&'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>,ValueError>;
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep;
    /// 🧮️ Requests the next original payload work without granting it.
    fn next_close_copy_byte_demand(&self) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// 📦️ Requests the actual natural frontier allocation for this bounded payload grant.
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes:usize) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// ♻️ Requests complete physical backing release independently of payload copying.
    fn next_close_release_byte_demand(&self) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// 🪜️ Requests the exact original frontier depth without enlarging the caller's limit.
    fn next_close_depth_demand(&self) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// 🔎️ Refuses undeclared retained demand instead of guessing a zero-credit owner.
    fn close_demand_without_owner(&self)->Result<usize,ValueError>{if self.terminal_is_empty(){Ok(0)}else{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"interactive job retained close demand is undeclared"))}}
    /// 🔔️ Registers the wake source that can advance a genuinely blocked close.
    fn register_close_wake(&self, _waker: &Waker) -> bool {
        false
    }
    fn terminal_is_empty(&self) -> bool;
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
//#endregion 🐕️Drive

//#region 👶️JobScope
/// 🌱️ A [`CancelToken::root`] via [`poll_ready_now`] — the one place [`JobScope::root`]/callers that
/// need a fresh root token (batch entry points, tests) cross the sync-over-async seam for token
/// creation, mirroring [`StepContext::is_cancelled`]'s single-owner pattern.
pub fn root_cancel_token() -> CancelToken {
    poll_ready_now(CancelToken::root())
}

pub const JOB_CHILD_SLOTS: usize = 64;

const CHILD_VACANT: u8 = 0;
const CHILD_LIVE: u8 = 1;
const CHILD_CLOSE_INTENT: u8 = 2;
const CHILD_EXHAUSTED: u8 = 3;
const CHILD_CHECKED_OUT: u8 = 4;

struct JobChildSlot {
    generation: AtomicU64,
    state: AtomicU8,
    node: AtomicPtr<JobChildNodeHeader>,
}

impl JobChildSlot {
    fn vacant() -> Self {
        Self { generation: AtomicU64::new(0), state: AtomicU8::new(CHILD_VACANT), node: AtomicPtr::new(std::ptr::null_mut()) }
    }
}

#[repr(C)]
struct JobChildNodeHeader {
    pump: unsafe fn(*mut JobChildNodeHeader, RetainedCloneGrant) -> InteractiveJobCloseStep,
    demands:unsafe fn(*mut JobChildNodeHeader,usize)->Result<RetirementDemand,ValueError>,
    release_bytes:usize,
    destroy: unsafe fn(*mut JobChildNodeHeader),
}

unsafe fn job_child_node_close_demands<J:InteractiveJob>(pointer:*mut JobChildNodeHeader,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
    let node=unsafe{&*pointer.cast::<JobChildNode<J>>()};
    if node.close_stage==1{
        let child=node.child.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"closing child node lacks original child"))?;
        return Ok(RetirementDemand{copy_bytes:child.next_close_copy_byte_demand()?,capacity_bytes:child.next_close_capacity_byte_demand(maximum_copy_bytes)?,release_bytes:child.next_close_release_byte_demand()?,depth:child.next_close_depth_demand()?})
    }
    Ok(RetirementDemand{copy_bytes:0,release_bytes:if node.close_stage==3{node.header.release_bytes}else{0},depth:1,..Default::default()})
}

#[repr(C)]
struct JobChildNode<J> {
    header: JobChildNodeHeader,
    child: Option<J>,
    close_stage: u8,
}

unsafe fn pump_job_child_node<J: InteractiveJob>(pointer: *mut JobChildNodeHeader, grant:RetainedCloneGrant) -> InteractiveJobCloseStep {
    let maximum_items=grant.maximum_items;
    let node = unsafe { &mut *pointer.cast::<JobChildNode<J>>() };
    if node.close_stage == 0 {
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        node.child.as_mut().expect("live child node owns exact child").begin_close();
        node.close_stage = 1;
        return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
    }
    if node.close_stage == 1 {
        let child = node.child.as_mut().expect("closing child node owns exact child");
        match child.close_step(grant).admit(grant,child.terminal_is_empty()) {
            InteractiveJobCloseStep::Pending { progress } => return InteractiveJobCloseStep::Pending { progress },
            InteractiveJobCloseStep::Blocked => return InteractiveJobCloseStep::Blocked,
            InteractiveJobCloseStep::Refused{kind,progress}=>return InteractiveJobCloseStep::Refused{kind,progress},
            InteractiveJobCloseStep::Complete { progress } if !child.terminal_is_empty() => return InteractiveJobCloseStep::Blocked,
            InteractiveJobCloseStep::Complete { progress } => {
                node.close_stage = 2;
                return InteractiveJobCloseStep::Pending { progress };
            }
        }
    }
    if node.close_stage == 2 {
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        drop(node.child.take());
        node.close_stage = 3;
        return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes:0, released_bytes: 0, ..RetainedCloneProgress::default() } };
    }
    InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
}

unsafe fn destroy_job_child_node<J>(pointer: *mut JobChildNodeHeader) {
    drop(unsafe { Box::from_raw(pointer.cast::<JobChildNode<J>>()) });
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobChildToken {
    pub parent_operation: OperationId,
    pub parent_generation: Generation,
    pub slot: u16,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobChildAdmissionFault {
    Capacity,
    Exhausted,
    Closing,
}

pub struct JobChildAdmissionRejected<J> {
    pub fault: JobChildAdmissionFault,
    child: ManuallyDrop<Option<J>>,
    closing: bool,
    close_stage: u8,
}

impl<J> JobChildAdmissionRejected<J> {
    pub fn child(&self) -> &J {
        self.child.as_ref().expect("rejected structured child owner remains exact")
    }

    pub fn into_child(mut self) -> J {
        self.child.take().expect("rejected structured child owner remains exact")
    }
}

impl<J: InteractiveJob> JobChildAdmissionRejected<J> {
    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(child) = self.child.as_mut() {
            child.begin_close();
        }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
        self.begin_close();
        if self.close_stage == 0 {
            let Some(child) = self.child.as_mut() else {
                self.close_stage = 2;
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
            };
            match child.close_step(grant).admit(grant,child.terminal_is_empty()) {
                InteractiveJobCloseStep::Pending { progress } => return InteractiveJobCloseStep::Pending { progress },
                InteractiveJobCloseStep::Blocked => return InteractiveJobCloseStep::Blocked,
                InteractiveJobCloseStep::Refused{kind,progress}=>return InteractiveJobCloseStep::Refused{kind,progress},
                InteractiveJobCloseStep::Complete { progress } if !child.terminal_is_empty() => return InteractiveJobCloseStep::Blocked,
                InteractiveJobCloseStep::Complete { progress } => {self.close_stage = 1;return InteractiveJobCloseStep::Pending{progress}},
            }
        }
        if self.close_stage == 1 {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
            }
            drop(self.child.take());
            self.close_stage = 2;
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.child.is_none()
    }
}

impl<J> Drop for JobChildAdmissionRejected<J> {
    fn drop(&mut self) {
        debug_assert!(self.child.is_none(), "rejected structured child requires exact handback");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobChildCompletionFault {
    LiveChildren,
    Stale,
    Duplicate,
}

pub struct JobScope {
    cancel: CancelToken,
    parent_operation: OperationId,
    parent_generation: Generation,
    slots: [JobChildSlot; JOB_CHILD_SLOTS],
    live_children: AtomicU32,
    closing: AtomicBool,
    wake_pending: AtomicBool,
}

impl JobScope {
    pub fn root() -> JobScope {
        JobScope::for_operation(&root_cancel_token(), OperationId(0), Generation(0))
    }

    pub fn child_of(parent: &CancelToken) -> JobScope {
        JobScope::for_operation(parent, OperationId(0), Generation(0))
    }

    pub fn for_operation(parent: &CancelToken, parent_operation: OperationId, parent_generation: Generation) -> JobScope {
        JobScope {
            cancel: poll_ready_now(parent.child()),
            parent_operation,
            parent_generation,
            slots: std::array::from_fn(|_| JobChildSlot::vacant()),
            live_children: AtomicU32::new(0),
            closing: AtomicBool::new(false),
            wake_pending: AtomicBool::new(false),
        }
    }

    pub fn cancel_token(&self) -> CancelToken {
        self.cancel.clone()
    }

    pub fn is_cancelled(&self) -> bool {
        poll_ready_now(self.cancel.is_cancelled())
    }

    pub fn spawn_child<J: InteractiveJob + 'static>(&self, child: J) -> Result<ChildJobGuard<'_, J>, JobChildAdmissionRejected<J>> {
        if self.closing.load(Ordering::Acquire) || self.is_cancelled() {
            return Err(JobChildAdmissionRejected { fault: JobChildAdmissionFault::Closing, child: ManuallyDrop::new(Some(child)), closing: false, close_stage: 0 });
        }
        let mut child = Some(child);
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.state.load(Ordering::Acquire) == CHILD_EXHAUSTED {
                continue;
            }
            if slot.state.compare_exchange(CHILD_VACANT, CHILD_LIVE, Ordering::AcqRel, Ordering::Acquire).is_err() {
                continue;
            }
            let previous = slot.generation.load(Ordering::Acquire);
            let Some(generation) = previous.checked_add(1) else {
                slot.state.store(CHILD_EXHAUSTED, Ordering::Release);
                continue;
            };
            let node = Box::new(JobChildNode { header: JobChildNodeHeader { pump: pump_job_child_node::<J>, demands:job_child_node_close_demands::<J>,release_bytes:std::mem::size_of::<JobChildNode<J>>(), destroy: destroy_job_child_node::<J> }, child: child.take(), close_stage: 0 });
            slot.generation.store(generation, Ordering::Release);
            slot.node.store(Box::into_raw(node).cast::<JobChildNodeHeader>(), Ordering::Release);
            self.live_children.fetch_add(1, Ordering::AcqRel);
            if self.closing.load(Ordering::Acquire) || self.is_cancelled() {
                let _ = slot.state.compare_exchange(CHILD_LIVE, CHILD_CLOSE_INTENT, Ordering::AcqRel, Ordering::Acquire);
                self.raise_wake();
            }
            let token = JobChildToken { parent_operation: self.parent_operation, parent_generation: self.parent_generation, slot: index as u16, generation };
            return Ok(ChildJobGuard { scope: self, token: Some(token), marker: std::marker::PhantomData });
        }
        let exhausted = self.slots.iter().all(|slot| slot.state.load(Ordering::Acquire) == CHILD_EXHAUSTED);
        Err(JobChildAdmissionRejected { fault: if exhausted { JobChildAdmissionFault::Exhausted } else { JobChildAdmissionFault::Capacity }, child: ManuallyDrop::new(child), closing: false, close_stage: 0 })
    }

    pub fn live_child_count(&self) -> u32 {
        self.live_children.load(Ordering::SeqCst)
    }

    pub fn has_live_children(&self) -> bool {
        self.live_child_count() > 0 || self.slots.iter().any(|slot| matches!(slot.state.load(Ordering::Acquire), CHILD_LIVE | CHILD_CLOSE_INTENT | CHILD_CHECKED_OUT))
    }

    pub fn assert_completable(&self) -> Result<(), JobChildCompletionFault> {
        if self.has_live_children() { Err(JobChildCompletionFault::LiveChildren) } else { Ok(()) }
    }

    pub fn begin_close(&self) {
        self.closing.store(true, Ordering::Release);
        self.cancel.cancel_now();
        for slot in &self.slots {
            let _ = slot.state.compare_exchange(CHILD_LIVE, CHILD_CLOSE_INTENT, Ordering::AcqRel, Ordering::Acquire);
        }
        self.raise_wake();
    }

    /// 🔐️ Reads all original child demands under the same exclusive node admission as its close.
    pub fn child_retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
        for slot in &self.slots{
            if slot.state.compare_exchange(CHILD_CLOSE_INTENT,CHILD_CHECKED_OUT,Ordering::AcqRel,Ordering::Acquire).is_err(){continue}
            let pointer=slot.node.load(Ordering::Acquire);
            let demand=if pointer.is_null(){Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closing scope child lacks original node"))}else{unsafe{((*pointer).demands)(pointer,maximum_copy_bytes)}};
            slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);return demand;
        }
        Ok(RetirementDemand::default())
    }

    pub fn pump_child_close(&self, grant:RetainedCloneGrant) -> InteractiveJobCloseStep {
        for slot in &self.slots {
            if slot.state.compare_exchange(CHILD_CLOSE_INTENT,CHILD_CHECKED_OUT,Ordering::AcqRel,Ordering::Acquire).is_err() {
                continue;
            }
            let pointer = slot.node.load(Ordering::Acquire);
            if pointer.is_null() {
                slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);
                return InteractiveJobCloseStep::Blocked;
            }
            let demand=match unsafe{((*pointer).demands)(pointer,grant.maximum_copy_bytes)}{Ok(demand)=>demand,Err(error)=>{slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);return InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()};}};
            if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);return InteractiveJobCloseStep::Pending{progress:Default::default()};}
            let step = unsafe { ((*pointer).pump)(pointer, grant) };
            if matches!(step,InteractiveJobCloseStep::Complete{..}) {
                let release_bytes=unsafe{(*pointer).release_bytes};
                if step.progress()!=RetainedCloneProgress::default()||grant.maximum_items==0||grant.maximum_release_bytes<release_bytes||grant.maximum_depth==0 {
                    slot.state.store(CHILD_CLOSE_INTENT, Ordering::Release);
                    return InteractiveJobCloseStep::Pending { progress:step.progress() };
                }
                let pointer = slot.node.swap(std::ptr::null_mut(), Ordering::AcqRel);
                unsafe { ((*pointer).destroy)(pointer) };
                self.live_children.fetch_sub(1, Ordering::AcqRel);
                slot.state.store(if slot.generation.load(Ordering::Acquire) == u64::MAX { CHILD_EXHAUSTED } else { CHILD_VACANT }, Ordering::Release);
                self.raise_wake();
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes:release_bytes, ..RetainedCloneProgress::default() } };
            }
            slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);
            return step;
        }
        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
    }

    pub fn take_wake(&self) -> bool {
        self.wake_pending.swap(false, Ordering::AcqRel)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.live_child_count() == 0 && self.slots.iter().all(|slot| matches!(slot.state.load(Ordering::Acquire), CHILD_VACANT | CHILD_EXHAUSTED) && slot.node.load(Ordering::Acquire).is_null())
    }

    fn complete_child(&self, token: JobChildToken) -> Result<(), JobChildCompletionFault> {
        if token.parent_operation != self.parent_operation || token.parent_generation != self.parent_generation {
            return Err(JobChildCompletionFault::Stale);
        }
        let Some(slot) = self.slots.get(token.slot as usize) else { return Err(JobChildCompletionFault::Stale) };
        if slot.generation.load(Ordering::Acquire) != token.generation {
            return Err(JobChildCompletionFault::Stale);
        }
        let state = slot.state.load(Ordering::Acquire);
        if state == CHILD_CLOSE_INTENT {
            return Err(JobChildCompletionFault::Duplicate);
        }
        if state != CHILD_LIVE {
            return Err(JobChildCompletionFault::Duplicate);
        }
        slot.state.compare_exchange(CHILD_LIVE, CHILD_CLOSE_INTENT, Ordering::AcqRel, Ordering::Acquire).map_err(|_| JobChildCompletionFault::Duplicate)?;
        self.raise_wake();
        Ok(())
    }

    fn raise_wake(&self) {
        self.wake_pending.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).ok();
    }
}

pub struct ChildJobGuard<'a, J: InteractiveJob + 'static> {
    scope: &'a JobScope,
    token: Option<JobChildToken>,
    marker: std::marker::PhantomData<J>,
}

impl<J: InteractiveJob + 'static> ChildJobGuard<'_, J> {
    pub fn token(&self) -> JobChildToken {
        self.token.expect("live child guard owns token")
    }

    pub fn complete(mut self) -> Result<(), JobChildCompletionFault> {
        let token = self.token.take().expect("live child guard owns token");
        self.scope.complete_child(token)
    }

    pub fn with_child_mut<R>(&mut self, use_child: impl FnOnce(&mut J) -> R) -> Result<R, JobChildCompletionFault> {
        let token = self.token.expect("live structured child guard owns token");
        if token.parent_operation != self.scope.parent_operation || token.parent_generation != self.scope.parent_generation {
            return Err(JobChildCompletionFault::Stale);
        }
        let slot = self.scope.slots.get(token.slot as usize).ok_or(JobChildCompletionFault::Stale)?;
        if slot.generation.load(Ordering::Acquire) != token.generation {
            return Err(JobChildCompletionFault::Stale);
        }
        slot.state.compare_exchange(CHILD_LIVE, CHILD_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).map_err(|_| JobChildCompletionFault::Duplicate)?;
        struct Handback<'a> {
            scope: &'a JobScope,
            slot: &'a JobChildSlot,
        }
        impl Drop for Handback<'_> {
            fn drop(&mut self) {
                self.slot.state.store(if self.scope.closing.load(Ordering::Acquire) { CHILD_CLOSE_INTENT } else { CHILD_LIVE }, Ordering::Release);
                self.scope.raise_wake();
            }
        }
        let handback = Handback { scope: self.scope, slot };
        let pointer = slot.node.load(Ordering::Acquire);
        if pointer.is_null() {
            return Err(JobChildCompletionFault::Stale);
        }
        let child = unsafe { (&mut *pointer.cast::<JobChildNode<J>>()).child.as_mut().expect("checked-out structured child node owns exact child") };
        let result = use_child(child);
        drop(handback);
        Ok(result)
    }
}

impl<J: InteractiveJob + 'static> Drop for ChildJobGuard<'_, J> {
    fn drop(&mut self) {
        if let Some(token) = self.token.take() {
            let _ = self.scope.complete_child(token);
        }
    }
}
//#endregion 👶️JobScope

//#region 📡️Progress
/// 🔖️ Opaque id for one addressable entity a [`ProgressEvent`] touches (a mesh, a brush placement, a
/// document node) — a bare `u64` so this crate never depends on any domain's entity-id type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(pub u64);

pub const JOB_PROGRESS_AFFECTED_ENTITIES: usize = 256;

#[derive(Debug, PartialEq, Eq)]
pub struct RetainedEntitySet {
    entries: [Option<EntityId>; JOB_PROGRESS_AFFECTED_ENTITIES],
    length: usize,
}

impl RetainedEntitySet {
    pub fn new() -> Self {
        Self { entries: [None; JOB_PROGRESS_AFFECTED_ENTITIES], length: 0 }
    }

    pub fn insert(&mut self, entity: EntityId) -> Result<(), EntityId> {
        if self.length == JOB_PROGRESS_AFFECTED_ENTITIES {
            return Err(entity);
        }
        self.entries[self.length] = Some(entity);
        self.length += 1;
        Ok(())
    }

    pub fn as_slice(&self) -> &[Option<EntityId>] {
        &self.entries[..self.length]
    }
}

impl Default for RetainedEntitySet {
    fn default() -> Self {
        Self::new()
    }
}

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
#[derive(Debug, PartialEq)]
#[expect(clippy::large_enum_variant, reason = "Progress events transfer retained payload and entity credits without an unbudgeted allocation per event.")]
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
        affected: RetainedEntitySet,
        patch: RetainedJobPayload,
        at_ms: u64,
    },
    Diagnostic {
        operation: OperationId,
        generation: Generation,
        sequence: u64,
        kind: DiagnosticKind,
        detail: RetainedJobPayload,
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
        detail: RetainedJobPayload,
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
    /// 🩺️ Diagnostics — a bounded overwrite-oldest ring.
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
        ProgressChannelKind::DiagnosticRing => ChannelPolicy::Ring { max_items: 128, max_bytes: 512 * 1024 },
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

//#region 🏭️RetainedSessions

#[derive(Clone, Copy, Debug)]
pub struct BatchDriveConfig {
    pub retained: RetainedCloneGrant,
    pub site: &'static str,
    pub stage: InteractiveStage,
    pub fuel_per_step: u64,
    pub step_budget_us: u64,
}

#[derive(Clone)]
pub struct BatchJobParams {
    pub operation: OperationId,
    pub generation: Generation,
    pub cancel: CancelToken,
    pub config: BatchDriveConfig,
    pub now_us: fn() -> Option<u64>,
}

struct WorkerJobAuthority<J> {
    job: WorkerJobSource<J>,
    params: Option<BatchJobParams>,
    cancel_retirement:Option<semio_framework_async::CancelTokenRetirement>,
    preview_sequence: u64,
    step_sequence: u64,
    issued_retained:RetainedCloneGrant,
    retained_receipt_pending:bool,
    retained_step_progress:RetainedCloneProgress,
    retained_progress:RetainedCloneProgress,
    payload_ledger: Option<Arc<JobPayloadOperationLedger>>,
    preadmitted_fault: JobPayloadSlot,
    admission_fault_source:Option<JobPayloadPageSource>,
    admission_fault_slots:usize,
    outcome: JobOutcomeDescriptorSlot,
    callback_verdict: Option<semio_framework_trace::CallbackVerdict>,
    overruns: semio_framework_trace::StepOverrunLedger,
    quarantined_outcome: JobOutcomeDescriptorSlot,
    worker_fault:bool,
    fault_pending:bool,
    step_fault:ManuallyDrop<Option<ValueError>>,
    step_fault_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    panic_fault:ManuallyDrop<Option<Box<dyn std::any::Any+Send>>>,
    clock: ClockStride,
    last_step_end_us: Option<u64>,
    close_stage: u8,
}

/// 🧺️ Unique heap owner for one worker authority. Its vector always contains exactly
/// one fully initialized authority, so submission, checkout, rejection, cancellation, and bounded
/// retirement transfer only this owner while the large retained payload carriers keep one address.
struct WorkerJobAuthorityOwner<J>(Vec<WorkerJobAuthority<J>>);

impl<J> std::ops::Deref for WorkerJobAuthorityOwner<J> {
    type Target = WorkerJobAuthority<J>;

    fn deref(&self) -> &Self::Target {
        &self.0[0]
    }
}

impl<J> std::ops::DerefMut for WorkerJobAuthorityOwner<J> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[0]
    }
}

impl<J> WorkerJobAuthorityOwner<J> {
    fn original_authority_admission_demand()->RetirementDemand{RetirementDemand{copy_bytes:b"job-session.terminal-fault".len(),capacity_bytes:size_of::<WorkerJobAuthority<J>>()+semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>()+JOB_PAYLOAD_PAGE_BYTES,depth:1,..Default::default()}}
    fn try_admit_owned(job:&mut Option<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let original=params.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original authority admission requires parameters"))?;
        if job.is_none()||control.admission_identity()!=(original.operation,original.generation){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original authority admission requires matching source identity"))}
        let mut demand=Self::original_authority_admission_demand();demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<J>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original authority source extent overflow"))?;let grant=control.admission_grant()?;
        if !control.admission_is_open()||original.cancel.is_cancelled_now()||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
        if JOB_PAYLOAD_PROCESS_OWNED_BYTES.try_update(Ordering::AcqRel,Ordering::Acquire,|value|value.checked_add(JOB_PAYLOAD_PAGE_BYTES).filter(|value|*value<=JOB_PAYLOAD_PROCESS_BYTES)).is_err(){return Ok(None)}
        let mut storage=Vec::new();if storage.try_reserve_exact(1).is_err(){JOB_PAYLOAD_PROCESS_OWNED_BYTES.fetch_sub(JOB_PAYLOAD_PAGE_BYTES,Ordering::AcqRel);return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original authority native storage reservation refused"))}
        let mut owner=Self::write_storage(storage,original.operation,original.generation);owner.job.original.write(job.take().expect("admitted original authority job"));owner.job.present=true;owner.params=params.take();owner.close_stage=0;let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()};control.receive_admission(progress)?;Ok(Some((owner,progress)))
    }
    /// 🪹️ Initializes the already-paid original authority directly in its stable heap slot.
    fn write_pending_storage(mut storage:Vec<WorkerJobAuthority<J>>,operation:OperationId,generation:Generation)->Self{
        const FAULT:&[u8]=b"job-session.terminal-fault";
        let payload_ledger=Arc::new(JobPayloadOperationLedger::new(operation,generation));
        payload_ledger.pages.store(1,Ordering::Relaxed);payload_ledger.bytes.store(JOB_PAYLOAD_PAGE_BYTES,Ordering::Relaxed);payload_ledger.stream_pages[JobPayloadStream::Fault as usize].store(1,Ordering::Relaxed);payload_ledger.stream_bytes[JobPayloadStream::Fault as usize].store(JOB_PAYLOAD_PAGE_BYTES,Ordering::Relaxed);
        let mut fault_source=JobPayloadPageSource::new();
        unsafe{std::ptr::copy_nonoverlapping(FAULT.as_ptr(),fault_source.storage.as_mut_ptr().cast::<u8>(),FAULT.len());}
        let overruns = semio_framework_trace::StepOverrunLedger::new();
        let target = storage.as_mut_ptr();
        unsafe {
            std::ptr::addr_of_mut!((*target).job.present).write(false);
            std::ptr::addr_of_mut!((*target).job.operation).write(operation);std::ptr::addr_of_mut!((*target).job.generation).write(generation);
            std::ptr::addr_of_mut!((*target).params).write(None);
            std::ptr::addr_of_mut!((*target).cancel_retirement).write(None);
            std::ptr::addr_of_mut!((*target).preview_sequence).write(0);
            std::ptr::addr_of_mut!((*target).step_sequence).write(0);
            std::ptr::addr_of_mut!((*target).issued_retained).write(RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0});
            std::ptr::addr_of_mut!((*target).retained_receipt_pending).write(false);
            std::ptr::addr_of_mut!((*target).retained_step_progress).write(RetainedCloneProgress::default());
            std::ptr::addr_of_mut!((*target).retained_progress).write(RetainedCloneProgress::default());
            std::ptr::addr_of_mut!((*target).payload_ledger).write(Some(Arc::clone(&payload_ledger)));
            std::ptr::addr_of_mut!((*target).preadmitted_fault.present).write(false);
            std::ptr::addr_of_mut!((*target).admission_fault_source).write(Some(fault_source));
            std::ptr::addr_of_mut!((*target).admission_fault_slots).write(0);
            JobOutcomeDescriptorSlot::initialize_empty(std::ptr::addr_of_mut!((*target).outcome));
            std::ptr::addr_of_mut!((*target).callback_verdict).write(None);
            std::ptr::addr_of_mut!((*target).overruns).write(overruns);
            JobOutcomeDescriptorSlot::initialize_empty(std::ptr::addr_of_mut!((*target).quarantined_outcome));
            std::ptr::addr_of_mut!((*target).worker_fault).write(false);
            std::ptr::addr_of_mut!((*target).fault_pending).write(false);
            std::ptr::addr_of_mut!((*target).step_fault).write(ManuallyDrop::new(None));
            std::ptr::addr_of_mut!((*target).step_fault_retirement).write(None);
            std::ptr::addr_of_mut!((*target).panic_fault).write(ManuallyDrop::new(None));
            std::ptr::addr_of_mut!((*target).clock).write(ClockStride::new());
            std::ptr::addr_of_mut!((*target).last_step_end_us).write(None);
            std::ptr::addr_of_mut!((*target).close_stage).write(0);
            storage.set_len(1);
        }
        drop(payload_ledger);
        Self(storage)
    }
    fn write_storage(storage:Vec<WorkerJobAuthority<J>>,operation:OperationId,generation:Generation)->Self{
        let mut owner=Self::write_pending_storage(storage,operation,generation);
        unsafe{let original=owner.preadmitted_fault.original.as_mut_ptr();let pages=std::ptr::addr_of_mut!((*original).pages).cast::<Option<JobPayloadPage>>();for index in 0..JOB_PAYLOAD_OPERATION_PAGES{pages.add(index).write(None);}}
        owner.admission_fault_slots=JOB_PAYLOAD_OPERATION_PAGES;owner.publish_original_admission_page();owner
    }
    fn pending_storage_initialization_demand()->RetirementDemand{RetirementDemand{copy_bytes:b"job-session.terminal-fault".len(),capacity_bytes:size_of::<WorkerJobAuthority<J>>()+semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>()+JOB_PAYLOAD_PAGE_BYTES,depth:1,..Default::default()}}
    fn admission_page_publication_copy_bytes()->usize{0}
    fn original_admission_initialization_demands(&self)->RetirementDemand{RetirementDemand{depth:usize::from(self.preadmitted_fault.is_empty()),..Default::default()}}
    fn advance_original_admission_initialization(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{
        if !self.preadmitted_fault.is_empty()||grant.maximum_items==0||grant.maximum_depth==0{return Default::default()}
        if self.admission_fault_slots<JOB_PAYLOAD_OPERATION_PAGES{let original=self.preadmitted_fault.original.as_mut_ptr();unsafe{let pages=std::ptr::addr_of_mut!((*original).pages).cast::<Option<JobPayloadPage>>();for index in self.admission_fault_slots..JOB_PAYLOAD_OPERATION_PAGES{pages.add(index).write(None);}}self.admission_fault_slots=JOB_PAYLOAD_OPERATION_PAGES;}else{self.publish_original_admission_page();}RetainedCloneProgress{copied_items:1,..Default::default()}
    }
    fn publish_original_admission_page(&mut self){
        assert_eq!(self.admission_fault_slots,JOB_PAYLOAD_OPERATION_PAGES);let source=self.admission_fault_source.take().expect("original prepaid page remains in its admission root");let ledger=Arc::clone(self.payload_ledger.as_ref().expect("original admission page retains its ledger"));let original=self.preadmitted_fault.original.as_mut_ptr();
        unsafe{std::ptr::addr_of_mut!((*original).stream).write(JobPayloadStream::Fault);std::ptr::addr_of_mut!((*original).pages).cast::<Option<JobPayloadPage>>().write(Some(JobPayloadPage{source,length:b"job-session.terminal-fault".len()}));std::ptr::addr_of_mut!((*original).page_count).write(1);std::ptr::addr_of_mut!((*original).length).write(b"job-session.terminal-fault".len());std::ptr::addr_of_mut!((*original).ledger).write(ManuallyDrop::new(Some(ledger)));}self.preadmitted_fault.present=true;
    }
    fn original_admission_page_demands(&self)->RetirementDemand{let Some(source)=self.admission_fault_source.as_ref()else{return Default::default()};RetirementDemand{copy_bytes:0,release_bytes:source.allocated_capacity_bytes(),depth:1,..Default::default()}}
    fn close_original_admission_page(&mut self,grant:RetainedCloneGrant)->WorkerJobCloseStep{
        let demand=self.original_admission_page_demands();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return WorkerJobCloseStep::Pending{progress:Default::default()}}
        let Some(source)=self.admission_fault_source.take()else{return WorkerJobCloseStep::Complete{progress:Default::default()}};let extent=source.allocated_capacity_bytes();self.payload_ledger.as_ref().expect("original admission page retains its ledger").release(JobPayloadStream::Fault,extent);drop(source);WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:extent,..Default::default()}}
    }

}

impl<J> WorkerJobAuthorityOwner<J> {
    fn original_worker_failure_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if let Some(original)=self.panic_fault.as_ref(){if let Some(text)=original.downcast_ref::<String>(){return Ok(Some(RetirementDemand{release_bytes:if text.capacity()>0{text.capacity()}else{size_of_val(original.as_ref())},depth:1,..Default::default()}))}if original.is::<&'static str>(){return Ok(Some(RetirementDemand{release_bytes:size_of_val(original.as_ref()),depth:1,..Default::default()}))}return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original arbitrary panic payload awaits its native retirement issuer"))}
        if let Some(original)=self.step_fault_retirement.as_ref(){return Ok(Some(if original.terminal_is_empty(){RetirementDemand{release_bytes:size_of_val(original.as_ref()),depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:original.next_copy_byte_demand()?,capacity_bytes:original.next_capacity_byte_demand(body)?,release_bytes:original.next_release_byte_demand()?,depth:original.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original worker error parent depth overflow"))?}}))}
        Ok(self.step_fault.as_ref().map(|_|RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<ValueError>(),depth:2,..Default::default()}))
    }
    fn close_original_worker_failure(&mut self,grant:RetainedCloneGrant)->Result<Option<WorkerJobCloseStep>,ValueError>{
        let Some(demand)=self.original_worker_failure_demands(grant.maximum_copy_bytes)?else{return Ok(None)};if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Some(WorkerJobCloseStep::Pending{progress:Default::default()}))}
        if let Some(original)=self.panic_fault.as_mut(){if let Some(text)=original.downcast_mut::<String>(){if text.capacity()>0{let released_bytes=text.capacity();drop(std::mem::take(text));return Ok(Some(WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}}))}}drop(self.panic_fault.take());return Ok(Some(WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}}))}
        if let Some(original)=self.step_fault_retirement.as_mut(){if original.terminal_is_empty(){drop(self.step_fault_retirement.take());return Ok(Some(WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}}))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=original.close_step(child)?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,original.terminal_is_empty(),"original worker failed callback")?;return Ok(Some(WorkerJobCloseStep::Pending{progress:step.progress()}))}
        let original=self.step_fault.take().expect("quoted original worker fault");let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};match semio_framework_value::retirement::admit_owned_retirement(original,child){Ok((owner,progress))=>{self.step_fault_retirement=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original worker error birth")?;Ok(Some(WorkerJobCloseStep::Pending{progress}))},Err((error,original))=>{*self.step_fault=Some(original);Err(error)}}
    }
    fn original_params_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.params.is_some(){return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()});}
        if let Some(cancel)=self.cancel_retirement.as_ref(){return if cancel.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}else{cancel.retirement_demands()};}
        Ok(Default::default())
    }
    /// 🛑️ Transfers original params, closes their cancellation ancestors, then removes the empty cursor separately.
    fn close_original_worker_params(&mut self,grant:RetainedCloneGrant)->WorkerJobCloseStep{
        let demand=match self.original_params_demands(){Ok(demand)=>demand,Err(error)=>return WorkerJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return WorkerJobCloseStep::Pending{progress:Default::default()};}
        if let Some(params)=self.params.take(){self.cancel_retirement=Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));return WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}};}
        if let Some(cancel)=self.cancel_retirement.as_mut(){
            if cancel.terminal_is_empty(){drop(self.cancel_retirement.take());return WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}};}
            return match cancel.close_step(grant){Ok(step)=>WorkerJobCloseStep::Pending{progress:step.progress()},Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>WorkerJobCloseStep::Blocked,Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>WorkerJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};
        }
        WorkerJobCloseStep::Complete{progress:Default::default()}
    }
    fn terminal_authority_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.payload_ledger.is_some(){return Ok(RetirementDemand{copy_bytes:0,release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>(),depth:1,..Default::default()});}
        let release_bytes=self.0.capacity().checked_mul(size_of::<WorkerJobAuthority<J>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"worker authority physical backing overflow"))?;
        Ok(RetirementDemand{copy_bytes:0,release_bytes,depth:1,..Default::default()})
    }
    /// 🧺️ Releases the original ledger before dropping its terminal authority in place and its vector backing.
    fn close_terminal_worker_authority(&mut self,grant:RetainedCloneGrant)->WorkerJobCloseStep{
        let authority=self;
        let demand=match authority.terminal_authority_demands(){Ok(demand)=>demand,Err(error)=>return WorkerJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return WorkerJobCloseStep::Pending{progress:Default::default()};}
        if let Some(ledger)=authority.payload_ledger.as_ref(){
            if !ledger.terminal_is_empty(){return WorkerJobCloseStep::Blocked;}
            let payload_ledger=authority.payload_ledger.take().unwrap();
            let (released_bytes,copied_bytes)=if let Some(ledger)=Arc::into_inner(payload_ledger){drop(ledger);(demand.release_bytes,demand.copy_bytes)}else{(0,0)};
            return WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes,released_bytes,..Default::default()}};
        }
        assert!(authority.job.is_empty()&&authority.params.is_none()&&authority.cancel_retirement.is_none()&&authority.admission_fault_source.is_none()&&authority.preadmitted_fault.is_empty()&&authority.outcome.is_empty()&&authority.quarantined_outcome.is_empty()&&authority.step_fault.is_none()&&authority.step_fault_retirement.is_none()&&authority.panic_fault.is_none());
        authority.0.clear();
        drop(std::mem::take(&mut authority.0));
        WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}}
    }
}

fn preadmitted_static_payload(ledger: &Arc<JobPayloadOperationLedger>, stream: JobPayloadStream, bytes: &'static [u8], mut source: JobPayloadPageSource) -> Result<RetainedJobPayload, JobPayloadPageSource> {
    if bytes.len() > source.allocated_capacity_bytes() || ledger.reserve(stream,source.allocated_capacity_bytes()).is_err() {
        return Err(source);
    }
    for (target, byte) in source.storage.iter_mut().zip(bytes.iter().copied()) {
        target.write(byte);
    }
    let mut pages = std::array::from_fn(|_| None);
    pages[0] = Some(JobPayloadPage { source, length: bytes.len() });
    Ok(RetainedJobPayload { stream, pages: ManuallyDrop::new(pages), page_count: 1, length: bytes.len(), ledger: ManuallyDrop::new(Some(Arc::clone(ledger))) })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerJobTicket {
    pub generation: Generation,
    pub step_sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobContention {
    Submitted(WorkerJobTicket),
    Outcome(WorkerJobTicket),
    Terminal(WorkerJobTicket),
    Rejected(Generation),
    CheckedOut(Generation),
    Closing(Generation),
    WakeExhausted(Generation),
    TerminalEmpty,
}

/// 🧭️ Keeps contention separate from a refused original ownership demand.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum WorkerJobDemandError {Contention(WorkerJobContention),Refused(ValueError)}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobSubmitFault {
    Contention(WorkerJobContention),
    Pool(semio_framework_async::WorkerSubmitErrorKind),
    SequenceExhausted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobPoll {
    Idle,
    Submitted,
    Outcome,
    Terminal,
    Rejected,
    CheckedOut,
    Closing,
    TerminalEmpty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobTakeFault {
    Pending,
    Stale,
    WrongPhase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobCloseStep {
    Pending { progress: RetainedCloneProgress },
    Blocked,
    Complete { progress: RetainedCloneProgress },
    Refused { kind:ValueRefusalKind, progress:RetainedCloneProgress }
}

impl WorkerJobCloseStep {
    /// 📊️ The actual currencies admitted by this close turn.
    pub fn progress(self) -> RetainedCloneProgress {
        match self {
            Self::Pending { progress } | Self::Complete { progress } | Self::Refused { progress, .. } => progress,
            Self::Blocked => RetainedCloneProgress::default(),
        }
    }
}

/// 🔭️ Which named phase a worker session's bounded close cursor is parked in, read without
/// spending a turn.
///
/// `close_step` reports HOW MUCH a turn released, never WHERE the cursor is, so a close that
/// stops advancing is indistinguishable from one that is merely slow: both answer
/// `Pending { 0, 0 }`. A host draining a session on a short grant, and every law that asserts a
/// session reaches terminal, needs the phase name to tell "the job is still releasing its own
/// owners" from "the cursor cannot leave the pre-admitted fault page" — the two that looked
/// identical while the mounted presence capture law spun 4096 times (ticket 26/09/18).
///
/// The phases are the exact ladder [`WorkerJobSession::close_step`] walks, in order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobClosePhase {
    OriginalFailure,
    /// 🚦️ Not closing: the session is idle, submitted, or holding an outcome nobody closed.
    Open,
    /// 🤝️ The authority is checked out, so the cursor has not been handed the session yet.
    CheckedOut,
    /// ☣️ Releasing the quarantined outcome's retained payload pages.
    QuarantinedOutcome,
    /// 📦️ Releasing the checked-out step outcome's retained payload pages.
    Outcome,
    /// 🔔️ The turn that hands `begin_close` to the job itself.
    BeginClose,
    /// 🧨️ Releasing the session's pre-admitted terminal-fault page.
    PreadmittedFault,
    /// 🧩️ The job is releasing its own owners through `InteractiveJob::close_step`.
    Job,
    /// 🗑️ Dropping the released job.
    JobRelease,
    /// 🎛️ Dropping the batch parameters.
    ParamsRelease,
    /// 📒️ Waiting on the payload ledger's outstanding stream credits.
    PayloadLedger,
    /// 🧺️ Releasing the original worker authority backing and its separately owned ledger.
    AuthorityRelease,
    /// 🎟️ Returning the pre-admitted retirement slot.
    RetirementSlot,
    SessionArc,
    /// 🕳️ Terminal-empty: every owner is released and the slot is returned.
    Empty,
}

pub struct BatchJobSession<J: InteractiveJob + 'static> {
    session: WorkerJobSession<J>,
    ticket: Option<WorkerJobTicket>,
    checked_out: Option<WorkerJobOutcome<J>>,
}

pub enum MountedWorkerJobPumpFault {
    Submit(WorkerJobSubmitFault),
    Take(WorkerJobTakeFault),
    MissingTicket,
    CheckedOut,
}

pub struct MountedWorkerJobSession<J: InteractiveJob + 'static> {
    session: WorkerJobSession<J>,
    ticket: Option<WorkerJobTicket>,
    checked_out: Option<WorkerJobOutcome<J>>,
}

impl<J: InteractiveJob + 'static> MountedWorkerJobSession<J> {
    pub fn try_admit_owned(job:&mut Option<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        WorkerJobSession::try_admit_owned(job,params,control).map(|admitted|admitted.map(|(session,progress)|(Self{session,ticket:None,checked_out:None},progress)))
    }

    pub fn generation(&self) -> Generation {
        self.session.generation()
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.checked_out.is_some() { WorkerJobPoll::CheckedOut } else { self.session.poll() }
    }

    /// 🧵️ `J: Send` for the same reason as [`WorkerJobSession::try_submit_step`] — this is the
    /// pool-submitting half of the mounted session, not the single-threaded drive path.
    pub fn pump_one(&mut self, pool: &WorkerPool, lane: Lane,retained:RetainedCloneGrant) -> Result<WorkerJobPoll, MountedWorkerJobPumpFault>
    where
        J: Send,
    {
        if self.checked_out.is_some() {
            return Err(MountedWorkerJobPumpFault::CheckedOut);
        }
        match self.session.poll() {
            WorkerJobPoll::Idle => {
                #[cfg(not(target_arch = "wasm32"))]
                if lane == Lane::Interactive {
                    let (ticket, poll) = self
                        .session
                        .try_step_on_caller(retained)
                        .map_err(|contention| MountedWorkerJobPumpFault::Submit(WorkerJobSubmitFault::Contention(contention)))?;
                    self.ticket = Some(ticket);
                    return match poll {
                        WorkerJobPoll::Outcome | WorkerJobPoll::Terminal => self.pump_one(pool, lane,retained),
                        other => Ok(other),
                    };
                }
                let ticket = self.session.try_submit_step(pool, lane,retained).map_err(MountedWorkerJobPumpFault::Submit)?;
                self.ticket = Some(ticket);
                Ok(WorkerJobPoll::Submitted)
            }
            WorkerJobPoll::Submitted => Ok(WorkerJobPoll::Submitted),
            WorkerJobPoll::Outcome => {
                let ticket = self.ticket.take().ok_or(MountedWorkerJobPumpFault::MissingTicket)?;
                let owner = self.session.take_outcome(ticket).map_err(MountedWorkerJobPumpFault::Take)?;
                self.checked_out = Some(owner);
                Ok(WorkerJobPoll::Outcome)
            }
            WorkerJobPoll::Terminal => {
                let owner = self.session.take_terminal().map_err(MountedWorkerJobPumpFault::Take)?;
                self.checked_out = Some(owner);
                Ok(WorkerJobPoll::Terminal)
            }
            WorkerJobPoll::Rejected => {
                let owner = self.session.take_rejected().map_err(MountedWorkerJobPumpFault::Take)?;
                owner.resume();
                Ok(WorkerJobPoll::Rejected)
            }
            poll => Ok(poll),
        }
    }

    /// 🧵️ Runs the session's next step on the calling thread and checks its outcome out. For a caller
    /// that already owns a bounded slice of its own — a guest `semio.infer` `step-job` crossing —
    /// where handing the step to a pool and waiting for it only adds a round trip per step.
    pub fn step_on_caller(&mut self,retained:RetainedCloneGrant) -> Result<WorkerJobPoll, MountedWorkerJobPumpFault> {
        if self.checked_out.is_some() {
            return Err(MountedWorkerJobPumpFault::CheckedOut);
        }
        let (ticket, poll) = self.session.try_step_on_caller(retained).map_err(|contention| MountedWorkerJobPumpFault::Submit(WorkerJobSubmitFault::Contention(contention)))?;
        if poll==WorkerJobPoll::Idle{return Ok(poll)}
        let owner = if poll == WorkerJobPoll::Terminal { self.session.take_terminal() } else { self.session.take_outcome(ticket) };
        self.checked_out = Some(owner.map_err(MountedWorkerJobPumpFault::Take)?);
        Ok(poll)
    }

    /// 🕰️ See [`WorkerJobSession::last_step_end_us`].
    pub fn last_step_end_us(&self) -> Option<u64> {
        self.session.last_step_end_us()
    }

    /// 🏃️ Executes the original interactive callback directly under this caller's full turn authority.
    pub fn pump_one_for_interactive_turn(&mut self,retained:RetainedCloneGrant)->Result<WorkerJobPoll,MountedWorkerJobPumpFault>{self.step_on_caller(retained)}

    /// 📥️ Returns the original effective grant and actual physical receipt once before semantic custody advances.
    /// 🔎️ Borrows the original issued authority and pending physical receipt without handing it back.
    pub fn checked_out_retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.checked_out.as_ref().and_then(WorkerJobOutcome::retained_step_receipt)}

    pub fn take_checked_out_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.checked_out.as_mut().and_then(WorkerJobOutcome::take_retained_step_receipt)}

    /// 🧾️ Borrows the actual same-owner worker receipt before transferring its outcome.
    pub fn checked_out_retained_step_progress(&self)->Option<RetainedCloneProgress>{self.checked_out.as_ref().map(WorkerJobOutcome::retained_step_progress)}

    pub fn checked_out_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{match self.checked_out.as_ref(){Some(original)=>original.outcome(),None=>Ok(None)}}

    /// 🎫️ Observes the same paid descriptor through its separate acknowledgement and removal phases.
    pub fn checked_out_outcome_descriptor(&self)->Option<&JobOutcomeDescriptor>{self.checked_out.as_ref().and_then(WorkerJobOutcome::outcome_descriptor)}

    /// 🛑️ Keeps the worker's original refusal inside its checked-out authority.
    pub fn checked_out_error(&self)->Option<&ValueError>{self.checked_out.as_ref().and_then(WorkerJobOutcome::retained_error)}

    pub fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::callback_verdict)
    }

    pub fn overrun_ledger(&self) -> Option<&semio_framework_trace::StepOverrunLedger> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::overrun_ledger)
    }

    pub fn acknowledge_checked_out_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.checked_out.as_mut().map_or(RetainedCloneStep::Complete(Default::default()),|original|original.acknowledge_outcome(grant))}

    pub fn checked_out_job_mut(&mut self) -> Option<&mut J> {
        self.checked_out.as_mut().and_then(|original|original.job_mut().ok())
    }
    pub fn checked_out_job(&self)->Option<&J>{self.checked_out.as_ref().map(WorkerJobOutcome::job)}

    /// 🧮️ Inspect the exact retained owner without advancing or enlarging its close grant.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,WorkerJobDemandError> {
        match self.checked_out.as_ref().and_then(|owner| owner.authority.as_ref()) {
            Some(authority)=>worker_job_authority_close_demands(authority,maximum_copy_bytes).map_err(WorkerJobDemandError::Refused),
            None=>self.session.retirement_demands(maximum_copy_bytes),
        }
    }

    pub fn resume(&mut self) -> Result<(), WorkerJobContention> {
        let owner = self.checked_out.take().ok_or_else(|| self.session.contention())?;
        owner.resume().map_err(|owner| {
            self.checked_out = Some(owner);
            self.session.contention()
        })
    }

    pub fn begin_close(&mut self) {
        if let Some(owner) = self.checked_out.take() {
            owner.begin_close();
        } else {
            let _ = self.session.begin_close();
        }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> WorkerJobCloseStep {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
        if let Some(mut owner) = self.checked_out.take() {
            if maximum_items == 0 {
                self.checked_out = Some(owner);
                return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
            }
            if owner.authority.as_ref().is_some_and(|authority|!authority.outcome.is_empty()){let progress=owner.acknowledge_outcome(grant).progress();self.checked_out=Some(owner);return WorkerJobCloseStep::Pending{progress};}
            owner.begin_close();
            return WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..RetainedCloneProgress::default()}};
        }
        self.session.close_step(grant)
    }

    /// 🪢️ Preserves checkout ownership while forwarding the caller's borrowed original cancellation root.
    pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{
        if self.checked_out.is_some(){return Ok(false);}
        self.session.has_original_cancel_alias_witness(witness)
    }

    /// 🪢️ Preserves checkout ownership while forwarding the caller's borrowed original cancellation root.
    pub fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{
        if self.checked_out.is_some(){return Ok(None);}
        self.session.return_original_cancel_alias_step(witness,grant)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.checked_out.is_none() && self.session.terminal_is_empty()
    }

    /// 🔭️ The named phase this mounted session's close cursor is parked in — see
    /// [`WorkerJobClosePhase`]. A retained checked-out outcome is the mounted half's own phase and
    /// outranks whatever the inner session reports.
    pub fn close_phase(&self) -> WorkerJobClosePhase {
        if self.checked_out.is_some() { WorkerJobClosePhase::CheckedOut } else { self.session.close_phase() }
    }
}

impl<J: InteractiveJob + 'static> BatchJobSession<J> {
    /// 🪪️ Borrows the original parameters under the same native custody gate used by close quotes.
    pub fn original_identity(&self)->Result<Option<(OperationId,Generation)>,WorkerJobContention>{
        if let Some(owner)=self.checked_out.as_ref(){return Ok(owner.authority.as_ref().and_then(|authority|authority.params.as_ref()).map(|params|(params.operation,params.generation)))}
        if self.session.inner.0.is_none(){return Ok(None)}
        let phase=self.session.inner.phase();
        if matches!(phase,SESSION_SUBMITTED|SESSION_TRANSITION|SESSION_CHECKED_OUT|SESSION_EMPTY)||self.session.inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self.session.contention())}
        let authority=unsafe{self.session.inner.take_authority()};
        let identity=authority.params.as_ref().map(|params|(params.operation,params.generation));
        unsafe{self.session.inner.put_authority_quiet(authority,phase)};Ok(identity)
    }

    pub fn try_admit_owned(job:&mut Option<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        WorkerJobSession::try_admit_owned(job,params,control).map(|admitted|admitted.map(|(session,progress)|(Self{session,ticket:None,checked_out:None},progress)))
    }

    pub fn step(&mut self,retained:RetainedCloneGrant) -> Result<WorkerJobPoll, WorkerJobContention> {
        if self.checked_out.is_some() {
            return Err(WorkerJobContention::CheckedOut(self.session.generation()));
        }
        let (ticket, poll) = self.session.try_step_on_caller(retained)?;
        if poll!=WorkerJobPoll::Idle{self.ticket = Some(ticket);}
        Ok(poll)
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.checked_out.is_some() { WorkerJobPoll::CheckedOut } else { self.session.poll() }
    }

    /// 🔎️ Borrows the original issued authority and pending physical receipt without handing it back.
    pub fn checked_out_retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.checked_out.as_ref().and_then(WorkerJobOutcome::retained_step_receipt)}

    pub fn take_checked_out_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.checked_out.as_mut().and_then(WorkerJobOutcome::take_retained_step_receipt)}

    pub fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.checked_out.as_mut().map_or(RetainedCloneStep::Progress(Default::default()),|original|original.acknowledge_outcome(grant))}

    pub fn checkout_outcome(&mut self) -> bool {
        if self.checked_out.is_some() {
            return true;
        }
        let owner = match self.session.poll() {
            WorkerJobPoll::Outcome => match self.ticket.take().and_then(|ticket| self.session.take_outcome(ticket).ok()) {
                Some(owner) => owner,
                None => return false,
            },
            WorkerJobPoll::Terminal => match self.session.take_terminal().ok() {
                Some(owner) => owner,
                None => return false,
            },
            _ => return false,
        };
        self.checked_out = Some(owner);
        true
    }

    pub fn checked_out_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{match self.checked_out.as_ref(){Some(original)=>original.outcome(),None=>Ok(None)}}

    pub fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::callback_verdict)
    }

    pub fn overrun_ledger(&self) -> Option<&semio_framework_trace::StepOverrunLedger> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::overrun_ledger)
    }

    pub fn checked_out_job_mut(&mut self) -> Option<&mut J> {
        WorkerJobOutcome::job_mut(self.checked_out.as_mut()?).ok()
    }

    /// 🧭️ Forward the exact owner demand while preserving checkout and worker ownership.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,WorkerJobDemandError> {
        match self.checked_out.as_ref().and_then(|owner| owner.authority.as_ref()) {
            Some(authority)=>worker_job_authority_close_demands(authority,maximum_copy_bytes).map_err(WorkerJobDemandError::Refused),
            None=>self.session.retirement_demands(maximum_copy_bytes),
        }
    }

    pub fn resume(&mut self) -> Result<(), WorkerJobContention> {
        let owner = self.checked_out.take().ok_or_else(|| self.session.contention())?;
        owner.resume().map_err(|owner| {
            self.checked_out = Some(owner);
            self.session.contention()
        })
    }

    pub fn begin_close(&mut self) {
        if let Some(owner) = self.checked_out.take() {
            owner.begin_close();
        } else {
            let _ = self.session.begin_close();
        }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> WorkerJobCloseStep {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
        if let Some(owner) = self.checked_out.take() {
            if maximum_items == 0 {
                self.checked_out = Some(owner);
                return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
            }
            owner.begin_close();
            return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        self.session.close_step(grant)
    }

    /// 🪢️ Preserves checkout custody before inspecting the original borrowed cancellation witness.
    pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{if self.checked_out.is_some(){return Ok(false)}self.session.has_original_cancel_alias_witness(witness)}
    /// 🪢️ Returns the original alias through its sole session after checkout custody has returned.
    pub fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{if self.checked_out.is_some(){return Ok(None)}self.session.return_original_cancel_alias_step(witness,grant)}

    pub fn terminal_is_empty(&self) -> bool {
        self.checked_out.is_none() && self.session.terminal_is_empty()
    }

    /// 🔭️ Observe the actual batch checkout or original worker retirement phase without effects.
    pub fn close_phase(&self)->WorkerJobClosePhase{if self.checked_out.is_some(){WorkerJobClosePhase::CheckedOut}else{self.session.close_phase()}}
}

const SESSION_TRANSITION: u8 = 0;
const SESSION_IDLE: u8 = 1;
const SESSION_SUBMITTED: u8 = 2;
const SESSION_OUTCOME: u8 = 3;
const SESSION_TERMINAL: u8 = 4;
const SESSION_REJECTED: u8 = 5;
const SESSION_CHECKED_OUT: u8 = 6;
const SESSION_CLOSE: u8 = 7;
const SESSION_EMPTY: u8 = 8;

pub const WORKER_JOB_SESSION_SLOTS: usize = 256;

#[repr(C)]
struct WorkerJobRetirementHeader {
    slot: usize,
    phase: unsafe fn(*mut WorkerJobRetirementHeader) -> WorkerJobClosePhase,
    pump: unsafe fn(*mut WorkerJobRetirementHeader, RetainedCloneGrant) -> WorkerJobCloseStep,
    destroy: unsafe fn(*mut WorkerJobRetirementHeader),
}

const WORKER_JOB_RETIREMENT_RESERVED: *mut WorkerJobRetirementHeader = std::ptr::without_provenance_mut(1);
static WORKER_JOB_RETIREMENT_SLOTS: [AtomicPtr<WorkerJobRetirementHeader>; WORKER_JOB_SESSION_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; WORKER_JOB_SESSION_SLOTS];
static WORKER_JOB_RETIREMENT_WAKE: AtomicBool = AtomicBool::new(false);

fn reserve_worker_job_retirement_slot() -> Option<usize> {
    WORKER_JOB_RETIREMENT_SLOTS.iter().enumerate().find_map(|(index, slot)| slot.compare_exchange(std::ptr::null_mut(), WORKER_JOB_RETIREMENT_RESERVED, Ordering::AcqRel, Ordering::Acquire).ok().map(|_| index))
}

/// 🧹️ Whether any dropped worker-job session still parks a node in the process-wide retirement array.
/// A host reads this to decide that it is NOT idle: the parked node holds one of the
/// [`WORKER_JOB_SESSION_SLOTS`] admissions every later session competes for, and only
/// [`pump_worker_job_retirements`] gives it back.
pub fn worker_job_retirements_are_parked() -> bool {
    WORKER_JOB_RETIREMENT_SLOTS.iter().any(|slot| {
        let pointer = slot.load(Ordering::Acquire);
        !pointer.is_null() && pointer != WORKER_JOB_RETIREMENT_RESERVED
    })
}

pub fn take_worker_job_retirement_wake() -> bool {
    WORKER_JOB_RETIREMENT_WAKE.swap(false, Ordering::AcqRel)
}

/// 🔭️ Observes one parked original owner under exclusive slot admission without funding it.
pub fn worker_job_retirement_phase(maximum_sessions:usize)->Option<WorkerJobClosePhase>{
    let mut inspected=0;
    for slot in &WORKER_JOB_RETIREMENT_SLOTS{
        if inspected==maximum_sessions{break}
        let pointer=slot.load(Ordering::Acquire);
        if pointer.is_null()||pointer==WORKER_JOB_RETIREMENT_RESERVED{continue}
        inspected+=1;
        if slot.compare_exchange(pointer,WORKER_JOB_RETIREMENT_RESERVED,Ordering::AcqRel,Ordering::Acquire).is_err(){continue}
        let phase=unsafe{((*pointer).phase)(pointer)};
        slot.store(pointer,Ordering::Release);
        return Some(phase);
    }
    worker_job_retirements_are_parked().then_some(WorkerJobClosePhase::CheckedOut)
}

pub fn pump_worker_job_retirements(maximum_sessions:usize,grant:RetainedCloneGrant)->WorkerJobCloseStep{
    let mut inspected=0;
    for slot in &WORKER_JOB_RETIREMENT_SLOTS{
        if inspected==maximum_sessions{break}
        let pointer=slot.load(Ordering::Acquire);
        if pointer.is_null()||pointer==WORKER_JOB_RETIREMENT_RESERVED{continue}
        inspected+=1;
        if slot.compare_exchange(pointer,WORKER_JOB_RETIREMENT_RESERVED,Ordering::AcqRel,Ordering::Acquire).is_err(){continue}
        let step=unsafe{((*pointer).pump)(pointer,grant)};
        if matches!(step,WorkerJobCloseStep::Complete{..}){slot.store(std::ptr::null_mut(),Ordering::Release);unsafe{((*pointer).destroy)(pointer)}}else{slot.store(pointer,Ordering::Release)}
        WORKER_JOB_RETIREMENT_WAKE.store(worker_job_retirements_are_parked(),Ordering::Release);
        return match step{WorkerJobCloseStep::Complete{progress} if worker_job_retirements_are_parked()=>WorkerJobCloseStep::Pending{progress},step=>step}
    }
    if worker_job_retirements_are_parked(){WORKER_JOB_RETIREMENT_WAKE.store(true,Ordering::Release);WorkerJobCloseStep::Blocked}else{WorkerJobCloseStep::Complete{progress:RetainedCloneProgress::default()}}
}

pub(crate) struct WorkerJobSessionInner<J> {
    generation: Generation,
    phase: AtomicU8,
    authority: ManuallyDrop<std::cell::UnsafeCell<Option<WorkerJobAuthorityOwner<J>>>>,
    rejection_kind: AtomicU8,
    close_requested: AtomicBool,
    terminal_intent: AtomicU8,
    wake_pending: AtomicBool,
    wake_sequence: AtomicU64,
    wake_exhausted: AtomicBool,
    wake_guard: AtomicBool,
    waker: ManuallyDrop<std::cell::UnsafeCell<Option<Waker>>>,
    retained_wake:ManuallyDrop<std::cell::UnsafeCell<Option<Box<dyn RetainedWorkerWake>>>>,
    last_step_end_us: AtomicU64,
}

unsafe impl<J: Send> Send for WorkerJobSessionInner<J> {}
unsafe impl<J: Send> Sync for WorkerJobSessionInner<J> {}

impl<J> WorkerJobSessionInner<J> {
    fn phase(&self) -> u8 {
        self.phase.load(Ordering::Acquire)
    }

    unsafe fn take_authority(&self) -> WorkerJobAuthorityOwner<J> {
        unsafe { (&mut *self.authority.get()).take().expect("session phase owns exact authority") }
    }

    unsafe fn put_authority(&self, authority: WorkerJobAuthorityOwner<J>, phase: u8) {
        unsafe {
            let storage = &mut *self.authority.get();
            assert!(storage.is_none(), "session transition cannot overwrite an authority");
            *storage = Some(authority);
        }
        self.phase.store(phase, Ordering::Release);
        self.raise_wake();
    }

    unsafe fn put_authority_quiet(&self, authority: WorkerJobAuthorityOwner<J>, phase: u8) {
        unsafe {
            let storage = &mut *self.authority.get();
            assert!(storage.is_none(), "session transition cannot overwrite an authority");
            *storage = Some(authority);
        }
        self.phase.store(phase, Ordering::Release);
    }

    fn raise_wake(&self) {
        if self.wake_pending.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        if self.wake_sequence.try_update(Ordering::AcqRel, Ordering::Acquire, |sequence| sequence.checked_add(1)).is_err() {
            self.wake_exhausted.store(true, Ordering::Release);
        }
        if self.wake_guard.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            if let Some(original)=unsafe{(&*self.retained_wake.get()).as_ref()}{original.wake_by_ref();}
            let waker = unsafe { (&mut *self.waker.get()).take() };
            self.wake_guard.store(false, Ordering::Release);
            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }

    fn register_waker(&self, waker: &Waker) -> Result<(), WorkerJobContention> {
        if self.wake_exhausted.load(Ordering::Acquire) {
            return Err(WorkerJobContention::WakeExhausted(self.generation));
        }
        if self.wake_guard.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobContention::CheckedOut(self.generation));
        }
        if unsafe{(&*self.retained_wake.get()).is_some()}{self.wake_guard.store(false,Ordering::Release);return Err(WorkerJobContention::CheckedOut(self.generation))}
        let wake_now = unsafe {
            *self.waker.get() = Some(waker.clone());
            if self.wake_pending.load(Ordering::Acquire) { (&mut *self.waker.get()).take() } else { None }
        };
        self.wake_guard.store(false, Ordering::Release);
        if let Some(waker) = wake_now {
            waker.wake();
        }
        Ok(())
    }
}

impl<J> Drop for WorkerJobSessionInner<J> {
    fn drop(&mut self) {
        if self.phase.load(Ordering::Acquire) == SESSION_EMPTY {
            unsafe {
                ManuallyDrop::drop(&mut self.authority);
                ManuallyDrop::drop(&mut self.waker);
                ManuallyDrop::drop(&mut self.retained_wake);
            }
        }
    }
}

struct WorkerJobSessionArc<J>(Option<Arc<WorkerJobSessionInner<J>>>);
impl<J> std::ops::Deref for WorkerJobSessionArc<J>{type Target=Arc<WorkerJobSessionInner<J>>;fn deref(&self)->&Self::Target{self.0.as_ref().expect("live session retains its original Arc")}}

pub struct WorkerJobSession<J: InteractiveJob + 'static> {
    inner: WorkerJobSessionArc<J>,
    generation: Generation,
    terminal_step_end_us: Option<u64>,
    retirement: std::cell::UnsafeCell<Option<Box<WorkerJobRetirementNode<J>>>>,
    retirement_state: AtomicU8,
}

unsafe impl<J: InteractiveJob + 'static> Send for WorkerJobSession<J> {}
unsafe impl<J: InteractiveJob + 'static> Sync for WorkerJobSession<J> {}

#[repr(C)]
struct WorkerJobRetirementNode<J> {
    header: WorkerJobRetirementHeader,
    inner: Option<session_return::SessionHandle<J>>,
}

unsafe fn worker_job_retirement_node_phase<J:InteractiveJob+'static>(pointer:*mut WorkerJobRetirementHeader)->WorkerJobClosePhase{
    let node=unsafe{&*pointer.cast::<WorkerJobRetirementNode<J>>()};
    let Some(inner)=node.inner.as_ref()else{return WorkerJobClosePhase::RetirementSlot};
    let phase=inner.phase();
    if phase==SESSION_EMPTY{return WorkerJobClosePhase::SessionArc}
    if matches!(phase,SESSION_SUBMITTED|SESSION_TRANSITION|SESSION_CHECKED_OUT)||inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return WorkerJobClosePhase::CheckedOut}
    let authority=unsafe{inner.take_authority()};
    let result=if phase!=SESSION_CLOSE{WorkerJobClosePhase::Open}else if !authority.quarantined_outcome.is_empty(){WorkerJobClosePhase::QuarantinedOutcome}else if !authority.outcome.is_empty(){WorkerJobClosePhase::Outcome}else if authority.step_fault.is_some()||authority.step_fault_retirement.is_some()||authority.panic_fault.is_some(){WorkerJobClosePhase::OriginalFailure}else if authority.close_stage==0{WorkerJobClosePhase::BeginClose}else if !authority.preadmitted_fault.is_empty(){WorkerJobClosePhase::PreadmittedFault}else{match authority.close_stage{1=>WorkerJobClosePhase::Job,2=>WorkerJobClosePhase::JobRelease,3=>WorkerJobClosePhase::ParamsRelease,_ if authority.payload_ledger.is_some()=>WorkerJobClosePhase::PayloadLedger,_=>WorkerJobClosePhase::AuthorityRelease}};
    unsafe{inner.put_authority(authority,phase)};
    result
}

unsafe fn pump_worker_job_retirement_node<J:InteractiveJob+'static>(pointer:*mut WorkerJobRetirementHeader,grant:RetainedCloneGrant)->WorkerJobCloseStep{
    let node=unsafe{&mut *pointer.cast::<WorkerJobRetirementNode<J>>()};
    let Some(inner)=node.inner.as_ref()else{let released_bytes=std::mem::size_of::<WorkerJobRetirementNode<J>>();return if grant.maximum_items>0&&grant.maximum_release_bytes>=released_bytes&&grant.maximum_depth>0{WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:0,released_bytes,..Default::default()}}}else{WorkerJobCloseStep::Pending{progress:Default::default()}}};
    if matches!(worker_job_begin_close(inner),WorkerJobCloseStep::Blocked){return WorkerJobCloseStep::Blocked}
    match worker_job_close_step(inner,grant){
        WorkerJobCloseStep::Complete{progress} if progress!=RetainedCloneProgress::default()=>WorkerJobCloseStep::Pending{progress},
        WorkerJobCloseStep::Complete{..}=>{
            if Arc::strong_count(inner)!=1||Arc::weak_count(inner)!=0||unsafe{(&*inner.waker.get()).is_some()}{return WorkerJobCloseStep::Blocked}
            if let Some(step)=worker_wake_retirement::close_session_retained_wake(inner,grant){return step}
            let copy_bytes=0;
            let arc_bytes=std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<WorkerJobSessionInner<J>>()).expect("worker original Arc layout").0.pad_to_align().size();
            let released_bytes=arc_bytes;
            if grant.maximum_items==0||grant.maximum_copy_bytes<copy_bytes||grant.maximum_release_bytes<released_bytes||grant.maximum_depth==0{return WorkerJobCloseStep::Pending{progress:RetainedCloneProgress::default()}}
            match Arc::try_unwrap(node.inner.take().unwrap()){
                Ok(inner)=>{drop(inner);WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,released_bytes,..RetainedCloneProgress::default()}}},
                Err(inner)=>{node.inner=Some(inner);WorkerJobCloseStep::Blocked}
            }
        },
        step=>step,
    }
}

unsafe fn destroy_worker_job_retirement_node<J>(pointer: *mut WorkerJobRetirementHeader) {
    drop(unsafe { Box::from_raw(pointer.cast::<WorkerJobRetirementNode<J>>()) });
}

pub struct WorkerJobSessionAdmissionRejected<J> {
    job: ManuallyDrop<Option<J>>,
    params: ManuallyDrop<Option<BatchJobParams>>,
    fault_source: ManuallyDrop<Option<JobPayloadPageSource>>,
    closing: bool,
    close_stage: u8,
}

impl<J> WorkerJobSessionAdmissionRejected<J> {
    pub fn job(&self) -> &J {
        self.job.as_ref().expect("rejected worker session admission owns exact job")
    }

    pub fn fault_backing_identity(&self) -> Option<*const MaybeUninit<u8>> {
        self.fault_source.as_ref().map(JobPayloadPageSource::backing_identity)
    }
}

impl<J: InteractiveJob> WorkerJobSessionAdmissionRejected<J> {
    /// 🪙️ Publish the physical demand of the next retained rejection owner.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,ValueError> {
        if self.close_stage == 0 {
            if let Some(job)=self.job.as_ref(){return Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(maximum_copy_bytes)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?})}
        }
        Ok(RetirementDemand{release_bytes:self.fault_source.as_ref().map_or(0,JobPayloadPageSource::allocated_capacity_bytes),depth:1,..RetirementDemand::default()})
    }

    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(job) = self.job.as_mut() {
            job.begin_close();
        }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
        self.begin_close();
        if self.close_stage == 0 {
            let Some(job) = self.job.as_mut() else {
                self.close_stage = 2;
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
            };
            match job.close_step(grant).admit(grant,job.terminal_is_empty()) {
                InteractiveJobCloseStep::Pending { progress } => return InteractiveJobCloseStep::Pending { progress },
                InteractiveJobCloseStep::Blocked => return InteractiveJobCloseStep::Blocked,
                InteractiveJobCloseStep::Refused{kind,progress}=>return InteractiveJobCloseStep::Refused{kind,progress},
                InteractiveJobCloseStep::Complete { progress } if !job.terminal_is_empty() => return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::InvariantViolated,progress},
                InteractiveJobCloseStep::Complete { progress } => {self.close_stage=1;return InteractiveJobCloseStep::Pending{progress}},
            }
        }
        if self.close_stage == 1 {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
            }
            drop(self.job.take());
            self.close_stage = 2;
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        if self.params.is_some() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
            }
            drop(self.params.take());
            self.close_stage = 3;
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        if self.fault_source.is_some() {
            let extent=self.fault_source.as_ref().unwrap().allocated_capacity_bytes();
            if maximum_items == 0 || maximum_bytes < extent {
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
            }
            drop(self.fault_source.take());
            self.close_stage = 4;
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..RetainedCloneProgress::default() } };
        }
        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.params.is_none() && self.fault_source.is_none()
    }
}

impl<J> Drop for WorkerJobSessionAdmissionRejected<J> {
    fn drop(&mut self) {
        debug_assert!(self.job.is_none() && self.params.is_none() && self.fault_source.is_none(), "rejected worker session admission requires exact incremental close");
    }
}

struct WorkerJobSubmission<J> {
    inner: session_return::SessionHandle<J>,
    authority: Option<WorkerJobAuthorityOwner<J>>,
    ran: bool,
}

/// ▶️ Runs one step of a retained worker session and folds its exact callback verdict into that
/// session's own `semio_framework_trace::StepOverrunLedger`. A step is quarantined with the
/// pre-admitted `job-session.terminal-fault` page only when the ledger attributes the breach to the
/// STEP — an unusable clock reading, or `SUSTAINED_OVERRUN_QUARANTINE_STEPS` consecutive over-ceiling
/// steps — never for one over-ceiling WALL reading, which a descheduled thread produces for work that
/// costs microseconds. A quarantined step's original outcome stays owned in `quarantined_outcome`.
/// 🪙️ Intersects every original Worker authority axis without converting temporal fuel or quoting demand.
fn original_worker_turn_grant(policy:RetainedCloneGrant,issued:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:policy.maximum_items.min(issued.maximum_items),maximum_copy_bytes:policy.maximum_copy_bytes.min(issued.maximum_copy_bytes),maximum_capacity_bytes:policy.maximum_capacity_bytes.min(issued.maximum_capacity_bytes),maximum_release_bytes:policy.maximum_release_bytes.min(issued.maximum_release_bytes),maximum_depth:policy.maximum_depth.min(issued.maximum_depth)}}

fn publish_original_worker_fault<J>(authority:&mut WorkerJobAuthority<J>)->bool{
 let params=authority.params.as_ref().expect("original fault retains its caller parameters");let before=authority.retained_step_progress;let result=JobOutcomeBorrow::admit_original_fault(params.operation,params.generation,authority.issued_retained,&mut authority.retained_step_progress,authority.preadmitted_fault.original().expect("original pre-admitted fault remains owned")).map(|outcome|outcome.map(JobOutcomeBorrow::into_descriptor));
 let progress=RetainedCloneProgress{copied_items:authority.retained_step_progress.copied_items-before.copied_items,copied_bytes:authority.retained_step_progress.copied_bytes-before.copied_bytes,retained_capacity_bytes:authority.retained_step_progress.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:authority.retained_step_progress.released_bytes-before.released_bytes};authority.retained_progress=authority.retained_progress.checked_add(progress).expect("original fault cumulative receipt overflow");
 let Ok(Some(original))=result else{return false};authority.outcome.retain(original).expect("exclusive original terminal fault descriptor");authority.worker_fault=true;authority.fault_pending=false;true
}

fn drive_worker_job_authority<J:InteractiveJob>(authority:&mut WorkerJobAuthority<J>)->bool{
 assert!(!authority.retained_receipt_pending,"original worker turn receipt must return before another callback");authority.retained_step_progress=Default::default();authority.retained_receipt_pending=true;if authority.fault_pending{return publish_original_worker_fault(authority)}
 let params=authority.params.as_ref().expect("submitted original job owns parameters");let config=params.config;let retained=authority.issued_retained;if retained.maximum_items==0||retained.maximum_depth==0{return false}let start_us=(params.now_us)();authority.last_step_end_us=start_us;let Some(budget)=start_us.and_then(|start_us|StepBudget::from_duration(config.fuel_per_step,start_us,config.step_budget_us,retained))else{authority.fault_pending=true;return publish_original_worker_fault(authority)};if authority.step_sequence==u64::MAX{authority.fault_pending=true}
 authority.clock.begin(start_us);let mut retained_step_progress=RetainedCloneProgress::default();let fault_pending=authority.fault_pending;
 let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
  let mut cx=StepContext::with_original_worker_authority(params.operation,params.generation,budget,&params.cancel,params.now_us,authority.clock,&mut authority.preview_sequence,&mut retained_step_progress,authority.payload_ledger.as_ref().expect("original worker payload ledger"))?;
  let result=if fault_pending{JobOutcomeBorrow::admit_fault(&mut cx,authority.preadmitted_fault.original().expect("original paid worker fault payload")).map(|outcome|outcome.map(JobOutcomeBorrow::into_descriptor))}else{drive_step(authority.job.original_mut().expect("submitted authority owns job"),&mut cx,config.site,config.stage,&mut authority.callback_verdict).map(|outcome|outcome.map(JobOutcomeBorrow::into_descriptor))};
  authority.clock=cx.clock.get();authority.last_step_end_us=cx.latest_us();result
 }));
 authority.retained_step_progress=retained_step_progress;authority.retained_progress=authority.retained_progress.checked_add(retained_step_progress).expect("original worker retained receipt overflow");authority.step_sequence=authority.step_sequence.saturating_add(1);
 let quarantine=authority.callback_verdict.as_ref().map_or(semio_framework_trace::StepQuarantine::Admitted,|verdict|authority.overruns.admit(verdict));
 match result{
  Ok(Ok(Some(outcome)))if !fault_pending&&quarantine.is_terminal()=>{authority.quarantined_outcome.retain(outcome).expect("exclusive original quarantine descriptor");authority.fault_pending=true;publish_original_worker_fault(authority)}
  Ok(Ok(None))if !fault_pending&&quarantine.is_terminal()=>{authority.fault_pending=true;publish_original_worker_fault(authority)}
  Ok(Ok(outcome))=>{authority.worker_fault=fault_pending;let terminal=outcome.as_ref().is_some_and(JobOutcomeDescriptor::is_terminal);if let Some(outcome)=outcome{authority.outcome.retain(outcome).expect("exclusive original semantic descriptor")}terminal}
  Ok(Err(error))=>{assert!(authority.step_fault.is_none(),"one original worker fault owner");*authority.step_fault=Some(error);authority.fault_pending=true;publish_original_worker_fault(authority)}
  Err(original)=>{assert!(authority.panic_fault.is_none(),"one original panic owner");*authority.panic_fault=Some(original);authority.fault_pending=true;publish_original_worker_fault(authority)}
 }
}

impl<J: InteractiveJob + 'static> WorkerJobSubmission<J> {
    fn run(mut self) {
        self.ran = true;
        let mut authority = self.authority.take().expect("submitted worker closure owns exact job authority");
        let terminal = drive_worker_job_authority(&mut authority);
        self.inner.last_step_end_us.store(authority.last_step_end_us.unwrap_or(u64::MAX), Ordering::Release);
        if self.inner.close_requested.load(Ordering::Acquire) {
            self.inner.terminal_intent.store(1, Ordering::Release);
        }
        unsafe { self.inner.put_authority(authority, if terminal { SESSION_TERMINAL } else { SESSION_OUTCOME }) };
    }
}

impl<J> Drop for WorkerJobSubmission<J> {
    fn drop(&mut self) {
        let Some(authority) = self.authority.take() else { return };
        let rejected = self.inner.rejection_kind.load(Ordering::Acquire) != u8::MAX;
        unsafe { self.inner.put_authority(authority, if rejected { SESSION_REJECTED } else { SESSION_CLOSE }) };
    }
}

fn worker_job_begin_close<J>(inner: &WorkerJobSessionInner<J>) -> WorkerJobCloseStep {
    inner.close_requested.store(true, Ordering::Release);
    loop {
        let phase = inner.phase();
        if phase == SESSION_SUBMITTED || phase == SESSION_TRANSITION || phase == SESSION_CHECKED_OUT {
            inner.terminal_intent.store(1, Ordering::Release);
            inner.raise_wake();
            return WorkerJobCloseStep::Blocked;
        }
        if phase == SESSION_CLOSE {
            return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        if phase == SESSION_EMPTY {
            return WorkerJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
        }
        if inner.phase.compare_exchange(phase, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            let authority = unsafe { inner.take_authority() };
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
    }
}

fn worker_job_authority_close_demands<J:InteractiveJob>(authority:&WorkerJobAuthorityOwner<J>,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
    if !authority.quarantined_outcome.is_empty(){return Ok(authority.quarantined_outcome.retirement_demands());}
    if !authority.outcome.is_empty(){return Ok(authority.outcome.retirement_demands());}
    if let Some(demand)=authority.original_worker_failure_demands(maximum_copy_bytes)?{return Ok(demand)}
    if authority.admission_fault_source.is_some(){return Ok(authority.original_admission_page_demands())}
    if authority.close_stage==0{return Ok(RetirementDemand{depth:1,..Default::default()});}
    if !authority.preadmitted_fault.is_empty(){return authority.preadmitted_fault.retirement_demands();}
    if authority.close_stage==1{let job=authority.job.original().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"closing worker lacks its original job"))?;return Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(maximum_copy_bytes)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?});}
    if authority.close_stage==2{return Ok(authority.job.terminal_removal_demands());}
    if authority.close_stage==3{return authority.original_params_demands();}
    authority.terminal_authority_demands()
}

fn worker_payload_step(step:Result<RetainedCloneStep,ValueError>)->WorkerJobCloseStep{match step{Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>WorkerJobCloseStep::Pending{progress},Err(error)=>WorkerJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}}}

fn worker_job_close_step<J: InteractiveJob>(inner: &WorkerJobSessionInner<J>, grant:RetainedCloneGrant) -> WorkerJobCloseStep {
    let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
    if inner.phase.compare_exchange(SESSION_CLOSE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return if inner.phase() == SESSION_EMPTY { WorkerJobCloseStep::Complete { progress: RetainedCloneProgress::default() } } else { WorkerJobCloseStep::Blocked };
    }
    let mut authority = unsafe { inner.take_authority() };
    if authority.retained_receipt_pending{unsafe{inner.put_authority(authority,SESSION_CLOSE)};return WorkerJobCloseStep::Blocked}
    let demand=match worker_job_authority_close_demands(&authority,grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>{unsafe{inner.put_authority(authority,SESSION_CLOSE)};return WorkerJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()};}};
    if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{unsafe{inner.put_authority(authority,SESSION_CLOSE)};return WorkerJobCloseStep::Pending{progress:Default::default()};}
    if !authority.quarantined_outcome.is_empty() {
        let result=worker_payload_step(Ok(authority.quarantined_outcome.close_step(grant)));
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return result;
    }
    if !authority.outcome.is_empty() {
        let result=worker_payload_step(Ok(authority.outcome.close_step(grant)));
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return result;
    }
    match authority.close_original_worker_failure(grant){Ok(Some(step))=>{unsafe{inner.put_authority(authority,SESSION_CLOSE)};return step},Ok(None)=>{},Err(error)=>{unsafe{inner.put_authority(authority,SESSION_CLOSE)};return WorkerJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}}}
    if authority.admission_fault_source.is_some(){let step=authority.close_original_admission_page(grant);unsafe{inner.put_authority(authority,SESSION_CLOSE)};return step}
    if authority.close_stage == 0 {
        if maximum_items == 0 {
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        authority.job.original_mut().expect("closing worker authority owns job").begin_close();
        authority.close_stage = 1;
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
    }
    if !authority.preadmitted_fault.is_empty() {
        let result=worker_payload_step(authority.preadmitted_fault.close_step(grant));
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return result;
    }
    if authority.close_stage == 1 {
        let step = authority.job.original_mut().expect("closing worker authority owns job").close_step(grant);
        match step.admit(grant,authority.job.original().unwrap().terminal_is_empty()) {
            InteractiveJobCloseStep::Pending { progress } => {
                unsafe { inner.put_authority(authority, SESSION_CLOSE) };
                return WorkerJobCloseStep::Pending { progress };
            }
            InteractiveJobCloseStep::Blocked => {
                unsafe { inner.put_authority(authority, SESSION_CLOSE) };
                return WorkerJobCloseStep::Blocked;
            }
            InteractiveJobCloseStep::Refused{kind,progress}=>{unsafe{inner.put_authority(authority,SESSION_CLOSE)};return WorkerJobCloseStep::Refused{kind,progress}},
            InteractiveJobCloseStep::Complete { progress } => {
                if !authority.job.original().expect("closing worker authority owns job").terminal_is_empty() {
                    unsafe { inner.put_authority(authority, SESSION_CLOSE) };
                    return WorkerJobCloseStep::Blocked;
                }
                authority.close_stage = 2;
                unsafe{inner.put_authority(authority,SESSION_CLOSE)};return WorkerJobCloseStep::Pending{progress};
            }
        }
    }
    if authority.close_stage == 2 {
        if maximum_items == 0 {
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        let progress=match authority.job.remove_terminal(grant){Ok(step)=>step.progress(),Err(error)=>{unsafe{inner.put_authority(authority,SESSION_CLOSE)};return WorkerJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}}};
        authority.close_stage = 3;
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return WorkerJobCloseStep::Pending { progress };
    }
    if authority.close_stage == 3 {
        let step=authority.close_original_worker_params(grant);
        if matches!(step,WorkerJobCloseStep::Complete{..}){authority.close_stage=4;}
        unsafe{inner.put_authority(authority,SESSION_CLOSE)};
        return match step{WorkerJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},step=>step};
    }
    let step=authority.close_terminal_worker_authority(grant);
    if matches!(step,WorkerJobCloseStep::Complete{..}){
        drop(authority);
        inner.phase.store(SESSION_EMPTY,Ordering::Release);
        inner.raise_wake();
    }else{unsafe{inner.put_authority(authority,SESSION_CLOSE)};}
    step
}

impl<J: InteractiveJob + 'static> WorkerJobSession<J> {
    /// 📏️ Quotes the actual fixed storage births and initialized source/control writes before custody changes.
    pub fn admission_demand()->RetirementDemand{
        let mut demand=WorkerJobAuthorityOwner::<J>::pending_storage_initialization_demand();
        demand.capacity_bytes+=semio_framework_value::shared_retirement_allocation_bytes::<WorkerJobSessionInner<J>>()+size_of::<WorkerJobRetirementNode<J>>();
        demand
    }
    /// 🚪️ Retains both original sources until their existing normal control admits every physical birth.
    /// 🎟️ Quotes complete atomic initialization and the original job/parameter moves before storage birth.
    pub fn owned_admission_demand()->Result<RetirementDemand,ValueError>{
        let mut demand=Self::admission_demand();let initialized=JOB_PAYLOAD_OPERATION_PAGES*size_of::<Option<JobPayloadPage>>()+size_of::<usize>()+WorkerJobAuthorityOwner::<J>::admission_page_publication_copy_bytes()+size_of::<J>()+2*size_of::<Option<J>>()+2*size_of::<Option<BatchJobParams>>()+2*size_of::<bool>()+size_of::<u8>();demand.copy_bytes=demand.copy_bytes.checked_add(initialized).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original atomic session initialization extent overflow"))?;Ok(demand)
    }

    pub fn try_admit_owned(job:&mut Option<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let original=params.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"session admission requires original parameters"))?;
        if job.is_none()||control.admission_identity()!=(original.operation,original.generation){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session admission requires the same original job and identity"))}
        let demand=Self::owned_admission_demand()?;let grant=control.admission_grant()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth||original.cancel.is_cancelled_now()||!control.admission_is_open(){return Ok(None)}
        let Some(session)=Self::birth_storage(original.operation,original.generation,control)?else{return Ok(None)};
        let inner=session.inner.0.as_ref().expect("admitted original session Arc");let authority=unsafe{(&mut *inner.authority.get()).as_mut().expect("admitted original authority")};
        while authority.preadmitted_fault.is_empty(){let progress=authority.advance_original_admission_initialization(grant);assert!(progress.copied_items>0,"admitted original atomic initialization must advance");}
        authority.job.original.write(job.take().expect("admitted original job"));authority.job.present=true;authority.params=params.take();authority.close_stage=0;
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:0};
        control.receive_admission(progress).expect("same exclusive caller admitted original session births");
        Ok(Some((session,progress)))
    }

    fn birth_storage(operation:OperationId,generation:Generation,control:&mut impl WorkerJobAdmissionControl)->Result<Option<Self>,ValueError>{
        if JOB_PAYLOAD_PROCESS_OWNED_BYTES.load(Ordering::Acquire)>JOB_PAYLOAD_PROCESS_BYTES-JOB_PAYLOAD_PAGE_BYTES{return Ok(None)}
        let Some(slot)=reserve_worker_job_retirement_slot()else{return Ok(None)};
        if JOB_PAYLOAD_PROCESS_OWNED_BYTES.try_update(Ordering::AcqRel,Ordering::Acquire,|value|value.checked_add(JOB_PAYLOAD_PAGE_BYTES).filter(|value|*value<=JOB_PAYLOAD_PROCESS_BYTES)).is_err(){WORKER_JOB_RETIREMENT_SLOTS[slot].store(std::ptr::null_mut(),Ordering::Release);control.receive_admission(RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()})?;return Ok(None)}
        let mut storage=Vec::<WorkerJobAuthority<J>>::new();
        if storage.try_reserve_exact(1).is_err(){JOB_PAYLOAD_PROCESS_OWNED_BYTES.fetch_sub(JOB_PAYLOAD_PAGE_BYTES,Ordering::AcqRel);WORKER_JOB_RETIREMENT_SLOTS[slot].store(std::ptr::null_mut(),Ordering::Release);control.receive_admission(RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()})?;return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original session storage reservation refused"))}
        let mut authority=WorkerJobAuthorityOwner::write_pending_storage(storage,operation,generation);
        authority.close_stage=3;
        let inner = Arc::new(WorkerJobSessionInner {
            generation,
            phase: AtomicU8::new(SESSION_IDLE),
            authority: ManuallyDrop::new(std::cell::UnsafeCell::new(Some(authority))),
            rejection_kind: AtomicU8::new(u8::MAX),
            close_requested: AtomicBool::new(false),
            terminal_intent: AtomicU8::new(0),
            wake_pending: AtomicBool::new(false),
            wake_sequence: AtomicU64::new(0),
            wake_exhausted: AtomicBool::new(false),
            wake_guard: AtomicBool::new(false),
            waker: ManuallyDrop::new(std::cell::UnsafeCell::new(None)),
            retained_wake:ManuallyDrop::new(std::cell::UnsafeCell::new(None)),
            last_step_end_us: AtomicU64::new(u64::MAX),
        });
        let retirement = Box::new(WorkerJobRetirementNode { header: WorkerJobRetirementHeader { slot, phase: worker_job_retirement_node_phase::<J>, pump: pump_worker_job_retirement_node::<J>, destroy: destroy_worker_job_retirement_node::<J> }, inner: None });
        let session=Self{inner:WorkerJobSessionArc(Some(inner)),generation,terminal_step_end_us:None,retirement:std::cell::UnsafeCell::new(Some(retirement)),retirement_state:AtomicU8::new(0)};
        Ok(Some(session))
    }

    /// 📥️ Returns an outstanding original physical receipt even when cancellation already requested close.
    pub fn take_retained_step_receipt(&self)->Result<Option<(RetainedCloneGrant,RetainedCloneProgress)>,WorkerJobContention>{if self.inner.0.is_none(){return Ok(None)}let phase=self.inner.phase();if matches!(phase,SESSION_SUBMITTED|SESSION_TRANSITION|SESSION_CHECKED_OUT)||self.inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self.contention())}let mut authority=unsafe{self.inner.take_authority()};let original=if authority.retained_receipt_pending{authority.retained_receipt_pending=false;Some((authority.issued_retained,authority.retained_step_progress))}else{None};unsafe{self.inner.put_authority_quiet(authority,phase)};Ok(original)}

    pub fn generation(&self) -> Generation {
        self.generation
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.inner.0.is_none(){return if self.retirement_state.load(Ordering::Acquire)==3{WorkerJobPoll::TerminalEmpty}else{WorkerJobPoll::Closing}}
        match self.inner.phase() {
            SESSION_IDLE => WorkerJobPoll::Idle,
            SESSION_SUBMITTED => WorkerJobPoll::Submitted,
            SESSION_OUTCOME => WorkerJobPoll::Outcome,
            SESSION_TERMINAL => WorkerJobPoll::Terminal,
            SESSION_REJECTED => WorkerJobPoll::Rejected,
            SESSION_CHECKED_OUT | SESSION_TRANSITION => WorkerJobPoll::CheckedOut,
            SESSION_CLOSE => WorkerJobPoll::Closing,
            SESSION_EMPTY => WorkerJobPoll::TerminalEmpty,
            _ => WorkerJobPoll::Closing,
        }
    }

    fn try_step_inline(&self,retained:RetainedCloneGrant) -> Result<(WorkerJobTicket, WorkerJobPoll), WorkerJobContention> {
        if self.inner.0.is_none(){return Err(WorkerJobContention::TerminalEmpty)}
        if self.inner.phase.compare_exchange(SESSION_IDLE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(self.contention());
        }
        let mut authority = unsafe { self.inner.take_authority() };
        let issued=original_worker_turn_grant(authority.params.as_ref().expect("original worker parameters").config.retained,retained);
        let ticket = WorkerJobTicket { generation: self.inner.generation, step_sequence: authority.step_sequence };
        if issued.maximum_items==0||issued.maximum_depth==0{unsafe{self.inner.put_authority_quiet(authority,SESSION_IDLE)};return Ok((ticket,WorkerJobPoll::Idle))}
        authority.issued_retained=issued;
        let terminal = drive_worker_job_authority(&mut authority);
        self.inner.last_step_end_us.store(authority.last_step_end_us.unwrap_or(u64::MAX), Ordering::Release);
        if self.inner.close_requested.load(Ordering::Acquire) {
            self.inner.terminal_intent.store(1, Ordering::Release);
        }
        unsafe { self.inner.put_authority(authority, if terminal { SESSION_TERMINAL } else { SESSION_OUTCOME }) };
        Ok((ticket, if terminal { WorkerJobPoll::Terminal } else { WorkerJobPoll::Outcome }))
    }

    pub fn try_step_on_caller(&self,retained:RetainedCloneGrant) -> Result<(WorkerJobTicket, WorkerJobPoll), WorkerJobContention> {
        self.try_step_inline(retained)
    }

    /// 🕰️ The clock reading the last step of this session ended at — its driver's exit reading, so a
    /// caller deciding whether to run another step needs no read of its own.
    pub fn last_step_end_us(&self) -> Option<u64> {
        if self.inner.0.is_none(){return self.terminal_step_end_us}
        Some(self.inner.last_step_end_us.load(Ordering::Acquire)).filter(|end_us| *end_us != u64::MAX)
    }

    /// 🧵️ Executes one exact owner turn from a retained scheduler already running inside its worker.
    pub fn try_step_on_worker(&self,retained:RetainedCloneGrant) -> Result<(WorkerJobTicket, WorkerJobPoll), WorkerJobContention> {
        self.try_step_inline(retained)
    }

    /// 👁️ Borrows the original typed registration state under its existing wake gate.
    pub fn has_retained_wake(&self)->Result<bool,WorkerJobContention>{
        if self.inner.0.is_none(){return Ok(false)}
        if self.inner.wake_guard.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_err(){return Err(WorkerJobContention::CheckedOut(self.generation))}
        let retained=unsafe{(&*self.inner.retained_wake.get()).is_some()};self.inner.wake_guard.store(false,Ordering::Release);Ok(retained)
    }

    /// 🔔️ Keeps the original admitted typed wake frame until its own paid close turns finish.
    pub fn register_retained_wake(&self,original:&mut Option<Box<dyn RetainedWorkerWake>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,WorkerJobContention>{
        if original.is_none(){return Ok(None)}
        if self.inner.0.is_none()||self.inner.phase()==SESSION_EMPTY{return Err(WorkerJobContention::TerminalEmpty)}
        let copy=size_of::<Option<Box<dyn RetainedWorkerWake>>>();
        if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(None)}
        if self.inner.wake_guard.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_err(){return Err(WorkerJobContention::CheckedOut(self.generation))}
        let slot=unsafe{&mut *self.inner.retained_wake.get()};
        if slot.is_some()||unsafe{(&*self.inner.waker.get()).is_some()}{self.inner.wake_guard.store(false,Ordering::Release);return Err(WorkerJobContention::CheckedOut(self.generation))}
        *slot=original.take();
        if self.inner.wake_pending.load(Ordering::Acquire){slot.as_ref().unwrap().wake_by_ref();}
        self.inner.wake_guard.store(false,Ordering::Release);
        Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
    }

    pub fn register_wake(&self, waker: &Waker) -> Result<(), WorkerJobContention> {
        if self.inner.0.is_none(){return Err(WorkerJobContention::TerminalEmpty)}
        self.inner.register_waker(waker)
    }

    /// 🔔️ Replaces a stale session wake with the exact job-owned close wake when available.
    pub fn register_close_wake(&self, waker: &Waker) -> Result<bool, WorkerJobContention> {
        if self.inner.0.is_none(){return Ok(false)}
        loop {
            let phase = self.inner.phase();
            if phase != SESSION_CLOSE {
                self.take_wake();
                self.register_wake(waker)?;
                return Ok(true);
            }
            if self.inner.phase.compare_exchange(SESSION_CLOSE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
                continue;
            }
            self.inner.wake_pending.store(false, Ordering::Release);
            let authority = unsafe { self.inner.take_authority() };
            let registered = authority.job.original().is_some_and(|job| job.register_close_wake(waker));
            unsafe { self.inner.put_authority_quiet(authority, SESSION_CLOSE) };
            return Ok(registered);
        }
    }

    pub fn take_wake(&self) -> bool {
        if self.inner.0.is_none(){return false}
        self.inner.wake_pending.swap(false, Ordering::AcqRel)
    }

    /// 🧵️ `J: Send` is stated here rather than on [`InteractiveJob`]: this is the one call that
    /// actually hands the job to another thread, so the requirement belongs to it. Targets with no
    /// second thread (the browser wasm build) never reach this and keep their `Rc`-owning jobs.
    pub fn try_submit_step(&self, pool: &WorkerPool, lane: Lane,retained:RetainedCloneGrant) -> Result<WorkerJobTicket, WorkerJobSubmitFault>
    where
        J: Send,
    {
        if self.inner.0.is_none(){return Err(WorkerJobSubmitFault::Contention(WorkerJobContention::TerminalEmpty))}
        if self.inner.phase.compare_exchange(SESSION_IDLE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobSubmitFault::Contention(self.contention()));
        }
        let mut authority = unsafe { self.inner.take_authority() };
        authority.issued_retained=original_worker_turn_grant(authority.params.as_ref().expect("original worker parameters").config.retained,retained);
        if authority.step_sequence == u64::MAX {
            unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
            return Err(WorkerJobSubmitFault::SequenceExhausted);
        }
        let ticket = WorkerJobTicket { generation: self.inner.generation, step_sequence: authority.step_sequence };
        self.inner.rejection_kind.store(u8::MAX, Ordering::Release);
        self.inner.phase.store(SESSION_SUBMITTED, Ordering::Release);
        let submission = WorkerJobSubmission { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), ran: false };
        let closure: semio_framework_async::Job = Box::new(move || submission.run());
        match pool.try_submit(lane, closure) {
            Ok(()) => Ok(ticket),
            Err(error) => {
                let kind = error.kind();
                self.inner.rejection_kind.store(worker_rejection_code(kind), Ordering::Release);
                drop(error.into_job());
                Err(WorkerJobSubmitFault::Pool(kind))
            }
        }
    }

    pub fn take_outcome(&self, ticket: WorkerJobTicket) -> Result<WorkerJobOutcome<J>, WorkerJobTakeFault> {
        if self.inner.0.is_none(){return Err(if ticket.generation!=self.generation{WorkerJobTakeFault::Stale}else{WorkerJobTakeFault::WrongPhase})}
        if ticket.generation != self.inner.generation {
            return Err(WorkerJobTakeFault::Stale);
        }
        if self.inner.phase.compare_exchange(SESSION_OUTCOME, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(if self.inner.phase() == SESSION_SUBMITTED { WorkerJobTakeFault::Pending } else { WorkerJobTakeFault::WrongPhase });
        }
        let authority = unsafe { self.inner.take_authority() };
        if authority.step_sequence != ticket.step_sequence.saturating_add(1) {
            unsafe { self.inner.put_authority(authority, SESSION_OUTCOME) };
            return Err(WorkerJobTakeFault::Stale);
        }
        Ok(WorkerJobOutcome { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), restore_phase: SESSION_OUTCOME })
    }

    pub fn take_terminal(&self) -> Result<WorkerJobOutcome<J>, WorkerJobTakeFault> {
        if self.inner.0.is_none(){return Err(WorkerJobTakeFault::WrongPhase)}
        if self.inner.phase.compare_exchange(SESSION_TERMINAL, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(if self.inner.phase() == SESSION_SUBMITTED { WorkerJobTakeFault::Pending } else { WorkerJobTakeFault::WrongPhase });
        }
        let authority = unsafe { self.inner.take_authority() };
        Ok(WorkerJobOutcome { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), restore_phase: SESSION_TERMINAL })
    }

    pub fn take_rejected(&self) -> Result<WorkerJobRejected<J>, WorkerJobTakeFault> {
        if self.inner.0.is_none(){return Err(WorkerJobTakeFault::WrongPhase)}
        if self.inner.phase.compare_exchange(SESSION_REJECTED, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobTakeFault::WrongPhase);
        }
        let authority = unsafe { self.inner.take_authority() };
        let kind = worker_rejection_kind(self.inner.rejection_kind.load(Ordering::Acquire));
        Ok(WorkerJobRejected { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), kind })
    }

    pub fn begin_close(&self) -> WorkerJobCloseStep {
        if self.inner.0.is_none(){return if self.terminal_is_empty(){WorkerJobCloseStep::Complete{progress:Default::default()}}else{WorkerJobCloseStep::Pending{progress:Default::default()}}}
        worker_job_begin_close(&self.inner)
    }

    /// 🪢️ Returns only the original cancellation alias witnessed by the caller's still-live matching root.
    pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{
        if self.inner.0.is_none()||self.inner.phase()!=SESSION_CLOSE{return Ok(false);}
        if self.inner.phase.compare_exchange(SESSION_CLOSE,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(WorkerJobDemandError::Contention(WorkerJobContention::CheckedOut(self.inner.generation)));}
        let authority=unsafe{self.inner.take_authority()};
        let original=authority.close_stage==3&&authority.params.is_none()&&authority.cancel_retirement.as_ref().is_some_and(|owner|owner.is_original_alias_witness(witness));
        unsafe{self.inner.put_authority_quiet(authority,SESSION_CLOSE)};
        Ok(original)
    }

    /// 🪢️ Returns only the original cancellation alias witnessed by the caller's still-live matching root.
    pub fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{
        if self.inner.0.is_none(){return Ok(None);}
        let phase=self.inner.phase();
        if phase!=SESSION_CLOSE{return Ok(None);}
        if self.inner.phase.compare_exchange(SESSION_CLOSE,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(WorkerJobDemandError::Contention(WorkerJobContention::CheckedOut(self.inner.generation)));}
        let mut authority=unsafe{self.inner.take_authority()};
        let result=if phase==SESSION_CLOSE&&authority.close_stage==3&&authority.params.is_none(){match authority.cancel_retirement.as_mut(){Some(original)if !original.terminal_is_empty()=>original.return_alias_step(witness,grant).map(Some).map_err(|error|match error{semio_framework_async::CancelTokenRetirementError::Refused(error)=>WorkerJobDemandError::Refused(error),semio_framework_async::CancelTokenRetirementError::Blocked(_)=>WorkerJobDemandError::Contention(WorkerJobContention::CheckedOut(self.inner.generation))}),_=>Ok(None)}}else{Ok(None)};
        unsafe{self.inner.put_authority_quiet(authority,phase)};
        result
    }
    /// 🔐️ Inspect under the same exclusive phase admission used by the close cursor.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,WorkerJobDemandError> {
        if self.inner.0.is_none(){return Ok(if self.retirement_state.load(Ordering::Acquire)==3{Default::default()}else{RetirementDemand{copy_bytes:0,release_bytes:size_of::<WorkerJobRetirementNode<J>>(),depth:1,..Default::default()}})}
        let phase = self.inner.phase();
        if phase == SESSION_EMPTY { return self.original_session_arc_demands(maximum_copy_bytes).map_err(WorkerJobDemandError::Refused); }
        if matches!(phase, SESSION_SUBMITTED | SESSION_TRANSITION | SESSION_CHECKED_OUT) || self.inner.phase.compare_exchange(phase, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobDemandError::Contention(WorkerJobContention::CheckedOut(self.inner.generation)));
        }
        let authority = unsafe { self.inner.take_authority() };
        let demand = worker_job_authority_close_demands(&authority,maximum_copy_bytes);
        unsafe { self.inner.put_authority_quiet(authority, phase) };
        demand.map_err(WorkerJobDemandError::Refused)
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> WorkerJobCloseStep {
        if self.inner.0.is_none(){return self.release_retirement_slot(grant)}
        match worker_job_close_step(&self.inner,grant){
            WorkerJobCloseStep::Complete{progress} if progress!=RetainedCloneProgress::default()=>WorkerJobCloseStep::Pending{progress},
            WorkerJobCloseStep::Complete{..}=>match self.close_original_session_arc(grant){WorkerJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},step=>step},
            step=>step,
        }
    }

    fn original_session_arc_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
        if self.inner.0.is_none(){return Ok(Default::default())}
        if let Some(demand)=worker_wake_retirement::session_retained_wake_demands(&self.inner,maximum_copy_bytes)?{return Ok(demand)}
        Ok(RetirementDemand{copy_bytes:0,release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<WorkerJobSessionInner<J>>(),depth:1,..Default::default()})
    }

    fn close_original_session_arc(&mut self,grant:RetainedCloneGrant)->WorkerJobCloseStep{
        let demand=match self.original_session_arc_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return WorkerJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return WorkerJobCloseStep::Pending{progress:Default::default()}}
        let original=self.inner.0.as_ref().expect("terminal session retains original Arc");
        if Arc::strong_count(original)!=1||Arc::weak_count(original)!=0||unsafe{(&*original.waker.get()).is_some()}{return WorkerJobCloseStep::Blocked}
        if let Some(step)=worker_wake_retirement::close_session_retained_wake(original,grant){return step}
        self.terminal_step_end_us=self.last_step_end_us();
        match Arc::try_unwrap(self.inner.0.take().unwrap()){
            Ok(original)=>{drop(original);WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}}},
            Err(original)=>{self.inner.0=Some(original);WorkerJobCloseStep::Blocked},
        }
    }

    pub fn terminal_is_empty(&self) -> bool {self.inner.0.is_none()&&self.retirement_state.load(Ordering::Acquire)==3}

    /// 🔭️ The named phase this session's close cursor is parked in — see [`WorkerJobClosePhase`].
    /// Reads the same fields the next `close_step` would walk, in the same order, without taking
    /// the authority or spending a turn.
    pub fn close_phase(&self) -> WorkerJobClosePhase {
        if self.inner.0.is_none(){return if self.retirement_state.load(Ordering::Acquire)==3{WorkerJobClosePhase::Empty}else{WorkerJobClosePhase::RetirementSlot}}
        match self.inner.phase() {
            SESSION_EMPTY => return WorkerJobClosePhase::SessionArc,
            SESSION_CHECKED_OUT | SESSION_TRANSITION => return WorkerJobClosePhase::CheckedOut,
            SESSION_CLOSE => {}
            _ => return WorkerJobClosePhase::Open,
        }
        let Some(authority) = (unsafe { (&*self.inner.authority.get()).as_ref() }) else { return WorkerJobClosePhase::CheckedOut };
        if !authority.quarantined_outcome.is_empty() {
            return WorkerJobClosePhase::QuarantinedOutcome;
        }
        if !authority.outcome.is_empty() {
            return WorkerJobClosePhase::Outcome;
        }
        if authority.step_fault.is_some()||authority.step_fault_retirement.is_some()||authority.panic_fault.is_some(){return WorkerJobClosePhase::OriginalFailure;}
        if authority.close_stage == 0 {
            return WorkerJobClosePhase::BeginClose;
        }
        if !authority.preadmitted_fault.is_empty() {
            return WorkerJobClosePhase::PreadmittedFault;
        }
        match authority.close_stage {
            1 => WorkerJobClosePhase::Job,
            2 => WorkerJobClosePhase::JobRelease,
            3 => WorkerJobClosePhase::ParamsRelease,
            _ if authority.payload_ledger.is_some() => WorkerJobClosePhase::PayloadLedger,
            _ => WorkerJobClosePhase::AuthorityRelease,
        }
    }

    fn contention(&self) -> WorkerJobContention {
        if self.inner.0.is_none(){return WorkerJobContention::TerminalEmpty}
        let generation = self.inner.generation;
        if self.inner.wake_exhausted.load(Ordering::Acquire) {
            return WorkerJobContention::WakeExhausted(generation);
        }
        let sequence = unsafe { (&*self.inner.authority.get()).as_ref().map_or(0, |authority| authority.step_sequence) };
        let ticket = WorkerJobTicket { generation, step_sequence: sequence };
        match self.inner.phase() {
            SESSION_SUBMITTED => WorkerJobContention::Submitted(ticket),
            SESSION_OUTCOME => WorkerJobContention::Outcome(ticket),
            SESSION_TERMINAL => WorkerJobContention::Terminal(ticket),
            SESSION_REJECTED => WorkerJobContention::Rejected(generation),
            SESSION_CHECKED_OUT | SESSION_TRANSITION => WorkerJobContention::CheckedOut(generation),
            SESSION_CLOSE => WorkerJobContention::Closing(generation),
            SESSION_EMPTY => WorkerJobContention::TerminalEmpty,
            _ => WorkerJobContention::CheckedOut(generation),
        }
    }

    fn release_retirement_slot(&self, grant:RetainedCloneGrant) -> WorkerJobCloseStep {
        if self.retirement_state.load(Ordering::Acquire) == 3 {
            return WorkerJobCloseStep::Complete{progress:RetainedCloneProgress::default()};
        }
        let released_bytes=std::mem::size_of::<WorkerJobRetirementNode<J>>();
        let copied_bytes=0;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_release_bytes<released_bytes||grant.maximum_depth==0||self.retirement_state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return WorkerJobCloseStep::Pending{progress:RetainedCloneProgress::default()};
        }
        let retirement = unsafe { (&mut *self.retirement.get()).take().expect("live worker session owns pre-admitted retirement node") };
        WORKER_JOB_RETIREMENT_SLOTS[retirement.header.slot].store(std::ptr::null_mut(), Ordering::Release);
        drop(retirement);
        self.retirement_state.store(3, Ordering::Release);
        WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:copied_bytes,released_bytes,..RetainedCloneProgress::default()}}
    }
}

impl<J: InteractiveJob + 'static> Drop for WorkerJobSession<J> {
    fn drop(&mut self) {
        if self.terminal_is_empty(){return}
        if let Some(original)=self.inner.0.as_ref(){original.close_requested.store(true,Ordering::Release);original.terminal_intent.store(1,Ordering::Release);original.raise_wake();}
        if self.retirement_state.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).is_err(){assert!(std::thread::panicking(),"original session Arc requires its admitted close turn");return}
        let mut retirement=unsafe{(&mut *self.retirement.get()).take().expect("live worker session owns original retirement node")};
        retirement.inner=self.inner.0.take();
        let slot=retirement.header.slot;
        let pointer=Box::into_raw(retirement).cast::<WorkerJobRetirementHeader>();
        WORKER_JOB_RETIREMENT_SLOTS[slot].store(pointer,Ordering::Release);
        self.retirement_state.store(2,Ordering::Release);
        WORKER_JOB_RETIREMENT_WAKE.store(true,Ordering::Release);
    }
}

pub struct WorkerJobOutcome<J> {
    inner: session_return::SessionHandle<J>,
    authority: Option<WorkerJobAuthorityOwner<J>>,
    restore_phase: u8,
}

impl<J> WorkerJobOutcome<J> {
    /// 📥️ The receiving caller consumes the same actual turn receipt exactly once before semantic access.
    /// 🧾️ The original checked-out authority retains both borrowed fields until its unique recipient accepts them.
    pub fn retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.authority.as_ref().and_then(|authority|authority.retained_receipt_pending.then_some((&authority.issued_retained,&authority.retained_step_progress)))}

    pub fn take_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{let authority=self.authority.as_mut()?;if !authority.retained_receipt_pending{return None}authority.retained_receipt_pending=false;Some((authority.issued_retained,authority.retained_step_progress))}

    /// 🧾️ Returns the exact latest actual worker turn receipt without changing its outcome owner.
    pub fn retained_step_progress(&self)->RetainedCloneProgress{self.authority.as_ref().expect("original worker outcome owns its actual turn receipt").retained_step_progress}
    /// 📒️ Returns the actual physical receipt accumulated by this original worker authority.
    pub fn retained_progress(&self)->RetainedCloneProgress{self.authority.as_ref().expect("original worker outcome owns its actual cumulative receipt").retained_progress}

    pub fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.authority.as_ref().and_then(|authority| authority.callback_verdict.as_ref())
    }

    /// 📒️ This session's own overrun ledger — how many of its steps breached the interactive ceiling,
    /// the longest consecutive run, and the worst reading, for a host that wants the counters without
    /// waiting for a quarantine.
    pub fn overrun_ledger(&self) -> Option<&semio_framework_trace::StepOverrunLedger> {
        self.authority.as_ref().map(|authority| &authority.overruns)
    }

    pub fn job(&self) -> &J {
        self.authority.as_ref().and_then(|authority| authority.job.original()).expect("checked-out worker outcome owns exact job")
    }

    pub fn job_mut(&mut self)->Result<&mut J,ValueError>{let authority=self.authority.as_mut().expect("checked out original worker authority");if authority.retained_receipt_pending||!authority.outcome.is_empty()||!authority.quarantined_outcome.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original semantic result requires acknowledgement before mutable job access"))}Ok(authority.job.original_mut().expect("original checked out job"))}
    pub fn retained_error(&self)->Option<&ValueError>{self.authority.as_ref().and_then(|authority|authority.step_fault.as_ref())}
    /// 🪪️ Borrows original semantic metadata while its payload remains in this worker authority.
    pub fn outcome_descriptor(&self)->Option<&JobOutcomeDescriptor>{self.authority.as_ref().and_then(|authority|if authority.retained_receipt_pending{None}else{authority.outcome.original()})}
    pub fn outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>where J:InteractiveJob{let authority=self.authority.as_ref().expect("checked out original worker authority");if authority.retained_receipt_pending{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker turn receipt awaits its receiving caller"))}let Some(original)=authority.outcome.original()else{return Ok(None)};let view=if authority.worker_fault{original.fault(authority.preadmitted_fault.original().expect("original paid terminal fault page"))?}else{authority.job.original().expect("original checked out job").borrow_outcome(original)?};Ok(Some(view))}
    pub fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{let authority=self.authority.as_mut().expect("checked out original worker authority");if authority.retained_receipt_pending{return RetainedCloneStep::Progress(Default::default())}authority.outcome.close_step(grant)}

    #[expect(clippy::result_large_err, reason = "Refused resumption returns the checked-out job authority and retained outcome without allocation or ownership loss.")]
    pub fn resume(mut self) -> Result<(), Self> {
        if self.restore_phase == SESSION_TERMINAL {
            return Err(self);
        }
        let authority = self.authority.as_ref().expect("checked-out worker outcome owns authority");
        if authority.retained_receipt_pending||!authority.outcome.is_empty() {
            return Err(self);
        }
        let authority = self.authority.take().expect("checked-out worker outcome owns authority");
        unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
        Ok(())
    }

    pub fn begin_close(mut self) {
        let authority = self.authority.take().expect("checked-out worker outcome owns authority");
        unsafe { self.inner.put_authority(authority, SESSION_CLOSE) };
    }
}

impl<J> Drop for WorkerJobOutcome<J> {
    fn drop(&mut self) {
        if let Some(authority) = self.authority.take() {
            unsafe { self.inner.put_authority(authority, self.restore_phase) };
        }
    }
}

pub struct WorkerJobRejected<J> {
    inner: session_return::SessionHandle<J>,
    authority: Option<WorkerJobAuthorityOwner<J>>,
    kind: semio_framework_async::WorkerSubmitErrorKind,
}

impl<J> WorkerJobRejected<J> {
    pub fn kind(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.kind
    }

    pub fn job(&self) -> &J {
        self.authority.as_ref().and_then(|authority| authority.job.original()).expect("checked-out rejected worker owner remains exact")
    }

    pub fn resume(mut self) {
        let authority = self.authority.take().expect("checked-out rejected worker owner remains exact");
        self.inner.rejection_kind.store(u8::MAX, Ordering::Release);
        unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
    }

    pub fn begin_close(mut self) {
        let authority = self.authority.take().expect("checked-out rejected worker owner remains exact");
        unsafe { self.inner.put_authority(authority, SESSION_CLOSE) };
    }
}

impl<J> Drop for WorkerJobRejected<J> {
    fn drop(&mut self) {
        if let Some(authority) = self.authority.take() {
            unsafe { self.inner.put_authority(authority, SESSION_REJECTED) };
        }
    }
}

fn worker_rejection_code(kind: semio_framework_async::WorkerSubmitErrorKind) -> u8 {
    match kind {
        semio_framework_async::WorkerSubmitErrorKind::Shutdown => 0,
        semio_framework_async::WorkerSubmitErrorKind::Contended => 1,
        semio_framework_async::WorkerSubmitErrorKind::Poisoned => 2,
        semio_framework_async::WorkerSubmitErrorKind::Saturated => 3,
    }
}

fn worker_rejection_kind(code: u8) -> semio_framework_async::WorkerSubmitErrorKind {
    match code {
        0 => semio_framework_async::WorkerSubmitErrorKind::Shutdown,
        1 => semio_framework_async::WorkerSubmitErrorKind::Contended,
        2 => semio_framework_async::WorkerSubmitErrorKind::Poisoned,
        _ => semio_framework_async::WorkerSubmitErrorKind::Saturated,
    }
}
//#endregion 🏭️RetainedSessions

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
    pending_kind:Option<JobOutcomeKind>,
    pending_bytes:[u8;64],
    pending_length:usize,
    pending_cursor:usize,
    outcome_delivered:bool,
    terminal_stage:u8,
    payload:RetainedPayloadBuilder,
    terminal_state:RetainedPayloadBuilder,
    terminal_output:RetainedPayloadBuilder,
    scope: JobScope,
    closing: bool,
}

/// 🩺️ How many units [`TortureJob::step`] processes between cheap `should_yield` polls — small enough
/// that overshoot past the 8 ms ceiling within one check interval is negligible (each unit is a
/// handful of integer ops), large enough that the `now_us`/fuel check itself isn't the hot-loop
/// bottleneck.
const TORTURE_YIELD_CHECK_INTERVAL: u64 = 64;

impl TortureJob {
    pub fn new(seed: u64, total_units: u64, checkpoint_every_units: u64, preview_every_units: u64, parent_cancel: &CancelToken) -> TortureJob {
        TortureJob {
            total_units,
            completed_units: 0,
            rng_state: splitmix64(seed) | 1,
            accumulator: 0,
            checkpoint_every_units,
            preview_every_units,
            units_since_checkpoint: 0,
            units_since_preview: 0,
            pending_kind:None,pending_bytes:[0;64],pending_length:0,pending_cursor:0,outcome_delivered:false,terminal_stage:0,
            payload:RetainedPayloadBuilder::new(JobPayloadStream::Preview),terminal_state:RetainedPayloadBuilder::new(JobPayloadStream::CommitState),terminal_output:RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput),
            scope: JobScope::child_of(parent_cancel),
            closing: false,
        }
    }

    pub fn completed_units(&self) -> u64 {
        self.completed_units
    }

    pub fn total_units(&self) -> u64 {
        self.total_units
    }

    fn checkpoint_bytes(&self) -> [u8; 48] {
        let mut state = [0u8; 48];
        for (index, value) in [self.total_units, self.completed_units, self.rng_state, self.accumulator, self.checkpoint_every_units, self.preview_every_units].into_iter().enumerate() {
            state[index * 8..index * 8 + 8].copy_from_slice(&value.to_le_bytes());
        }
        state
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
        TortureJob {
            total_units,
            completed_units,
            rng_state,
            accumulator,
            checkpoint_every_units,
            preview_every_units,
            units_since_checkpoint: 0,
            units_since_preview: 0,
            pending_kind:None,pending_bytes:[0;64],pending_length:0,pending_cursor:0,outcome_delivered:false,terminal_stage:0,
            payload:RetainedPayloadBuilder::new(JobPayloadStream::Preview),terminal_state:RetainedPayloadBuilder::new(JobPayloadStream::CommitState),terminal_output:RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput),
            scope: JobScope::child_of(parent_cancel),
            closing: false,
        }
    }

    fn encode_preview(&self, sequence: u64) -> [u8; 24] {
        let mut out = [0u8; 24];
        out[..8].copy_from_slice(&sequence.to_le_bytes());
        out[8..16].copy_from_slice(&self.completed_units.to_le_bytes());
        out[16..].copy_from_slice(&self.accumulator.to_le_bytes());
        out
    }

    fn prepare_original_scalar(&mut self,cx:&mut StepContext<'_>,kind:JobOutcomeKind,source:&[u8])->Result<bool,ValueError>{if source.len()>self.pending_bytes.len(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original scalar emission exceeds its inline extent"))}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_copy_bytes<source.len()||grant.maximum_depth==0{return Ok(false)}self.pending_bytes[..source.len()].copy_from_slice(source);self.pending_length=source.len();self.pending_cursor=0;self.pending_kind=Some(kind);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:source.len(),..Default::default()})?;Ok(true)}
    fn original_scalar_step(payload:&mut RetainedPayloadBuilder,bytes:&[u8],cursor:&mut usize,cx:&mut StepContext<'_>)->Result<bool,ValueError>{if !payload.is_initialized(){payload.advance_initialization(cx)?;return Ok(false)}if *cursor<bytes.len(){payload.append_original(cx,bytes,cursor)?;return Ok(false)}if payload.published().is_none(){payload.seal(cx)?;return Ok(false)}Ok(true)}
    fn original_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.scope.terminal_is_empty(){return self.scope.child_retirement_demands(body)}for original in[&self.payload,&self.terminal_state,&self.terminal_output]{if !original.terminal_is_empty(){let mut demand=original.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original Torture payload parent depth overflow"))?;return Ok(demand)}}Ok(RetirementDemand{depth:usize::from(self.pending_kind.is_some()||self.pending_length!=0||self.pending_cursor!=0||self.outcome_delivered),..Default::default()})}

    fn output_bytes(&self) -> [u8; 16] {
        let mut output = [0u8; 16];
        output[..8].copy_from_slice(&self.completed_units.to_le_bytes());
        output[8..].copy_from_slice(&self.accumulator.to_le_bytes());
        output
    }
}

impl InteractiveJob for TortureJob {
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original_retirement_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.original_retirement_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original_retirement_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.original_retirement_demands(0)?.depth)}
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError>{
        if cx.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(cx)}
        if self.outcome_delivered{if !self.payload.terminal_is_empty(){self.payload.close_step(cx)?;return Ok(None)}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}self.pending_kind=None;self.pending_length=0;self.pending_cursor=0;self.outcome_delivered=false;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(None)}
        if let Some(kind)=self.pending_kind{
            if matches!(kind,JobOutcomeKind::Complete){match self.terminal_stage{1=>{if Self::original_scalar_step(&mut self.terminal_state,&self.pending_bytes[..self.pending_length],&mut self.pending_cursor,cx)?{let grant=cx.retained_grant();if grant.maximum_items>0&&grant.maximum_depth>0{self.terminal_stage=2;self.pending_cursor=0;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;}}return Ok(None)},2=>{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_copy_bytes<16||grant.maximum_depth==0{return Ok(None)}let bytes=self.output_bytes();if self.prepare_original_scalar(cx,kind,&bytes)?{self.terminal_stage=3;}return Ok(None)},3=>{if Self::original_scalar_step(&mut self.terminal_output,&self.pending_bytes[..self.pending_length],&mut self.pending_cursor,cx)?{let grant=cx.retained_grant();if grant.maximum_items>0&&grant.maximum_depth>0{self.terminal_stage=4;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;}}return Ok(None)},_=>return JobOutcomeBorrow::admit_complete(cx,self.terminal_state.published(),self.terminal_output.published())}}
            let stream=if matches!(kind,JobOutcomeKind::PreviewReady){JobPayloadStream::Preview}else{JobPayloadStream::CheckpointState};let selected=self.payload.select_stream(stream,cx.retained_grant())?;cx.consume_retained(selected.progress())?;if matches!(selected,RetainedCloneStep::Progress(_))||selected.progress()!=Default::default(){return Ok(None)}
            if !Self::original_scalar_step(&mut self.payload,&self.pending_bytes[..self.pending_length],&mut self.pending_cursor,cx)?{return Ok(None)}let original=self.payload.published().expect("original sealed Torture payload");let result=match kind{JobOutcomeKind::PreviewReady=>JobOutcomeBorrow::admit_preview(cx,original),JobOutcomeKind::CheckpointReady{applied_progress}=>JobOutcomeBorrow::admit_checkpoint(cx,original,applied_progress),_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Torture semantic variant changed"))}?;if result.is_some(){self.outcome_delivered=true;}return Ok(result)
        }
        let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||cx.should_yield(){return Ok(None)}
        if self.completed_units==self.total_units{if grant.maximum_copy_bytes<48{return Ok(None)}self.scope.assert_completable().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original Torture scope has a live structured child"))?;let bytes=self.checkpoint_bytes();if self.prepare_original_scalar(cx,JobOutcomeKind::Complete,&bytes)?{self.terminal_stage=1;}return Ok(None)}
        if self.units_since_preview>=self.preview_every_units{if grant.maximum_copy_bytes<24{return Ok(None)}let sequence=cx.next_preview_sequence().map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Torture preview sequence exhausted"))?;let bytes=self.encode_preview(sequence);if self.prepare_original_scalar(cx,JobOutcomeKind::PreviewReady,&bytes)?{self.units_since_preview=0;}return Ok(None)}
        if self.units_since_checkpoint>=self.checkpoint_every_units{if grant.maximum_copy_bytes<48{return Ok(None)}let bytes=self.checkpoint_bytes();if self.prepare_original_scalar(cx,JobOutcomeKind::CheckpointReady{applied_progress:self.completed_units},&bytes)?{self.units_since_checkpoint=0;}return Ok(None)}
        self.rng_state=xorshift64(self.rng_state);let mix=self.rng_state.rotate_left((self.completed_units%61)as u32);self.accumulator=self.accumulator.wrapping_add(mix);self.completed_units+=1;self.units_since_checkpoint+=1;self.units_since_preview+=1;cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(None)
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),JobOutcomeKind::PreviewReady=>descriptor.preview(self.payload.published().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Torture preview custody absent"))?),JobOutcomeKind::CheckpointReady{..}=>descriptor.checkpoint(self.payload.published().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Torture checkpoint custody absent"))?),JobOutcomeKind::Complete=>descriptor.complete(self.terminal_state.published(),self.terminal_output.published()),JobOutcomeKind::Fault=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Torture worker fault requires worker custody"))}}

    fn begin_close(&mut self) {
        self.closing = true;
        self.scope.begin_close();
    }

    fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep{let demand=match self.original_retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};if self.terminal_is_empty(){return InteractiveJobCloseStep::Complete{progress:Default::default()}}if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return InteractiveJobCloseStep::Pending{progress:Default::default()}}if !self.scope.terminal_is_empty(){return match self.scope.pump_child_close(grant){InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step}}for original in[&mut self.payload,&mut self.terminal_state,&mut self.terminal_output]{if !original.terminal_is_empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};return match original.close_step_granted(child){Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()},Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}}}}self.pending_kind=None;self.pending_length=0;self.pending_cursor=0;self.outcome_delivered=false;InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}}}
    fn terminal_is_empty(&self)->bool{self.closing&&self.scope.terminal_is_empty()&&self.payload.terminal_is_empty()&&self.terminal_state.terminal_is_empty()&&self.terminal_output.terminal_is_empty()&&self.pending_kind.is_none()&&self.pending_length==0&&self.pending_cursor==0&&!self.outcome_delivered}
}
//#endregion 🔥️TortureJob

#[cfg(test)]
pub(crate) const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};

//#region 🧪️Tests
/// 🎟️ The [`WORKER_JOB_SESSION_SLOTS`] are process-wide, so one test binary's parallel tests share
/// them. A test that admits sessions holds a shared guard; the one test that owns every slot at
/// once holds the exclusive guard, so neither ever observes the other's admissions.
#[cfg(test)]
static WORKER_SESSION_SLOTS_TEST_AUTHORITY: std::sync::RwLock<()> = std::sync::RwLock::new(());

#[cfg(test)]
fn worker_session_slots_shared() -> std::sync::RwLockReadGuard<'static, ()> {
    WORKER_SESSION_SLOTS_TEST_AUTHORITY.read().unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
fn worker_session_slots_exclusive() -> std::sync::RwLockWriteGuard<'static, ()> {
    WORKER_SESSION_SLOTS_TEST_AUTHORITY.write().unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
#[path = "⏱️budget/🧪️tests/⏱️budget/🦀️.rs"]
mod microsecond_budget_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️retained-ownership/🦀️.rs"]
mod retained_ownership_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️clock-stride/🦀️.rs"]
mod clock_stride_tests;

#[cfg(test)]
#[path = "🧪️tests/📏️close-demand/🦀️.rs"]
mod close_demand_tests;

#[cfg(test)]
#[path="♻️retirement/👷️authority/🧪️tests/🦀️.rs"]
mod worker_authority_retirement_tests;

#[path = "🔎️reconcile/🧬️schema/🦀️.rs"]
pub mod reconcile;

#[cfg(test)]
#[global_allocator]
static JOB_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[cfg(test)]
#[path = "⏱️context/📦️owner/🧪️tests/🦀️.rs"]
mod retained_step_context_owner_tests;

#[path = "⏱️context/📦️owner/🦀️.rs"]
mod step_context_owner;
pub use step_context_owner::StepContextOwner;

#[cfg(test)]
#[path="👷️worker/🎟️turn/🧪️tests/🦀️.rs"]
mod original_worker_turn_tests;
