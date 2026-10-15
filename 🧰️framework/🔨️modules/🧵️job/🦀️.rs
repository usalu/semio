//! 🧵️ Resumable interactive job protocol and its original caller authority.
//!
//! StepContext retains the initiating clock, absolute deadline, cancellation, recipient and funded stock.
//! Results borrow their paid payload source through descriptors and close through exact physical receipts.
//! Core owns the private payload/context/retirement knot; worker, scope, registry and progress have distinct owners.


use std::future::Future;
use std::mem::{ManuallyDrop, MaybeUninit,size_of_val};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::RetirementDemand;

#[path="📥️snapshot/🧬️schema/🦀️.rs"]
mod original_snapshot_intake;
pub use original_snapshot_intake::{OriginalSnapshotIntake,OriginalSnapshotIntakeProducer};


use semio_framework_value::{ValueError, ValueRefusalKind};
#[path="⚠️failure/🦀️.rs"]
mod original_turn_failure;
pub use original_turn_failure::{OriginalTurnFailure,OriginalTurnPrimary,OriginalClosureFailure,OriginalDomainFailure,OriginalDomainPrimary,OriginalDomainProjection,OriginalDomainView,admit_original_domain_failure,original_domain_failure_demand,project_original_turn_failure,key_original_turn_failure,project_original_value_error,key_original_value_error,project_original_fault,key_original_fault};
#[path="⏱️context/🛬️decode/🦀️.rs"]
mod original_native_decode_source;
pub use original_native_decode_source::{OriginalNativeDecodePolicy,OriginalNativeDecodeIssued,OriginalNativeDecodeOutcome,OriginalNativeDecodeSource};
#[path="♻️retirement/🎟️context/🦀️.rs"]
mod original_close_context;
pub use original_close_context::close_original_retained_step;

#[cfg(test)]
use std::time::Instant;

use semio_framework_async::ChannelPolicy;
use semio_framework_trace::{TraceEvent, Watchdog};
#[cfg(test)] use semio_framework_trace::{record_operation_started};

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
/// 🌱️ Creates the canonical root cancellation authority through the synchronous primitive seam.
pub fn root_cancel_token()->CancelToken{poll_ready_now(CancelToken::root())}
/// 👶️ Creates a child cancellation authority through the same synchronous primitive seam.
pub fn child_cancel_token(parent:&CancelToken)->CancelToken{poll_ready_now(parent.child())}

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

    /// 📐️ Observes admitted page backing without borrowing reservation or release authority.
    pub fn allocated_capacity_bytes(&self)->usize{self.pages.iter().flatten().map(|page|page.source.allocated_capacity_bytes()).sum()}

    /// 🧱️ Reports one live native page extent separately from its initialized byte length.
    pub fn page_capacity_bytes(&self,index:usize)->Option<usize>{self.pages.get(index).and_then(Option::as_ref).map(|page|page.source.allocated_capacity_bytes())}

    /// 🔗️ Compares this live payload's original ledger without lending a reference or mutation.
    pub fn payload_authority_identity(&self)->Option<OriginalPayloadSourceIdentity>{self.ledger.as_ref().map(|ledger|OriginalPayloadSourceIdentity(Arc::as_ptr(ledger)as usize))}

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
    /// 🪪️ Observes only the committed payload ledger; staged and reservation custody remains private.
    pub fn payload_authority_identity(&self)->Option<OriginalPayloadSourceIdentity>{self.payload.as_ref().and_then(RetainedJobPayload::payload_authority_identity)}

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

#[path="🏃️work/🎟️grant/🦀️.rs"]
pub mod retained_work;

#[path="📬️outcome/🤝️loan/🦀️.rs"]
mod outcome_loan;
pub use outcome_loan::{JobOutcomeAdmission,JobOutcomeBorrow,drive_step};

#[path="📬️outcome/🎟️descriptor/🦀️.rs"]
mod original_outcome_descriptor;
pub use original_outcome_descriptor::{JobOutcomeDescriptor,JobOutcomeKind,JobOutcomeView};
pub use original_outcome_descriptor::JobOutcomeDescriptorSlot;
#[path="📬️outcome/📖️source/🦀️.rs"]
mod original_payload_source;
#[path="📬️outcome/📦️builder/🦀️.rs"]
mod payload_builder;
pub use payload_builder::{JobPayloadAuthority,RetainedPayloadBuilder,OriginalPayloadSourceIdentity,OriginalPayloadBuilderIdentity};
#[path="📬️outcome/👷️worker/🦀️.rs"]
mod original_worker_payload;
pub use original_worker_payload::{OriginalWorkerPayload,OriginalWorkerPageReservation};

enum JobPayloadLedgerReference<'a>{Owned(Arc<JobPayloadOperationLedger>),Borrowed(&'a JobPayloadAuthority),OriginalWorker(&'a Arc<JobPayloadOperationLedger>)}
impl std::ops::Deref for JobPayloadLedgerReference<'_>{type Target=Arc<JobPayloadOperationLedger>;fn deref(&self)->&Self::Target{match self{Self::Owned(original)=>original,Self::OriginalWorker(original)=>original,Self::Borrowed(authority)=>authority.original().expect("original context outlived its payload authority")}}}
enum StepCancelReference<'a>{Owned(CancelToken),Borrowed(&'a CancelToken),Observed(&'a CancelToken)}
impl std::ops::Deref for StepCancelReference<'_>{type Target=CancelToken;fn deref(&self)->&Self::Target{match self{Self::Owned(original)=>original,Self::Borrowed(original)|Self::Observed(original)=>original}}}
#[path="⏱️context/📸️capture/🦀️.rs"]
mod original_caller_capture;
pub use original_caller_capture::{OriginalCallerCapture,CapturedOriginalCaller,OriginalCallerIssuer,OriginalCallerTurn,OriginalCallerCloseReturn,OriginalCallerSourceParent,OriginalCallerSourceLoan,OriginalCallerChildSource,OriginalChildTurnReturn};
//#endregion 📄️RetainedPayload

#[path="⏱️context/📦️return/🪶️weak/🦀️.rs"]
mod original_native_weak_return;

#[path="⏱️context/⏸️park/🦀️.rs"]
mod original_owned_caller_turn;
pub use original_owned_caller_turn::{OriginalOwnedCallerTurn,OriginalOwnedCallerReturn,OriginalOwnedCallerOutcome};

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
    work_units_spent:u128,
    deadline_us: u64,
    now_us: fn() -> Option<u64>,
    clock: std::cell::Cell<ClockStride>,
    cancel: StepCancelReference<'a>,
    stage: &'static str,
    preview_sequence: &'a mut u64,
    payload_ledger: JobPayloadLedgerReference<'a>,
    source_parent: Option<&'a OriginalCallerSourceParent>,
    native_weak_parent:Option<original_native_weak_return::OriginalNativeWeakParent>,
    source_epoch: Option<u64>,
    original_retirement: bool,
    original_clock_binding: Option<u64>,
    original_total_fuel: u64,
    original_total_work_start: u128,
    original_absolute_deadline: u64,
    original_total_retained: RetainedCloneGrant,
    original_total_retained_start:RetainedCloneProgress,
    retained_policy:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    original_physical_recipient:Option<(&'a mut RetainedCloneProgress,RetainedCloneProgress)>,
    original_work_recipient:Option<&'a mut u128>,
    payload_page_granted: bool,
    retained_work: retained_work::RetainedWorkBudget,
}

fn retained_delta(after:RetainedCloneProgress,before:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items.saturating_sub(before.copied_items),copied_bytes:after.copied_bytes.saturating_sub(before.copied_bytes),retained_capacity_bytes:after.retained_capacity_bytes.saturating_sub(before.retained_capacity_bytes),released_bytes:after.released_bytes.saturating_sub(before.released_bytes)}}
fn retained_remaining(grant:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:grant.maximum_items.saturating_sub(spent.copied_items),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(spent.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(spent.released_bytes),..grant}}
fn retained_min(a:RetainedCloneGrant,b:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:a.maximum_items.min(b.maximum_items),maximum_copy_bytes:a.maximum_copy_bytes.min(b.maximum_copy_bytes),maximum_capacity_bytes:a.maximum_capacity_bytes.min(b.maximum_capacity_bytes),maximum_release_bytes:a.maximum_release_bytes.min(b.maximum_release_bytes),maximum_depth:a.maximum_depth.min(b.maximum_depth)}}
fn retained_is_subset(a:RetainedCloneGrant,b:RetainedCloneGrant)->bool{retained_min(a,b)==a}

impl Drop for StepContext<'_>{fn drop(&mut self){if let Some((recipient,start))=self.original_physical_recipient.as_mut(){let delta=retained_delta(*self.retained_progress,*start);**recipient=recipient.checked_add(delta).expect("original physical receipt exceeds usize");}if let Some(recipient)=self.original_work_recipient.as_mut(){**recipient=recipient.checked_add(self.work_units_spent).expect("original source work counter exceeds u128");}}}

#[derive(Clone,Copy)]
struct OriginalRetainedLoanIdentity{parent:Option<usize>,recipient:usize,cursor:usize,ledger:usize,operation:OperationId,generation:Generation,clock:fn()->Option<u64>,deadline:u64}
impl OriginalRetainedLoanIdentity{
 fn capture(cx:&StepContext<'_>)->Self{Self{parent:cx.source_parent.map(|parent|parent as*const _ as usize),recipient:cx.retained_progress as*const _ as usize,cursor:cx.preview_sequence as*const _ as usize,ledger:Arc::as_ptr(&cx.payload_ledger)as usize,operation:cx.operation,generation:cx.generation,clock:cx.now_us,deadline:cx.deadline_us}}
 fn matches(&self,cx:&StepContext<'_>)->bool{self.parent==cx.source_parent.map(|parent|parent as*const _ as usize)&&self.recipient==cx.retained_progress as*const _ as usize&&self.cursor==cx.preview_sequence as*const _ as usize&&self.ledger==Arc::as_ptr(&cx.payload_ledger)as usize&&self.operation==cx.operation&&self.generation==cx.generation&&std::ptr::fn_addr_eq(self.clock,cx.now_us)&&self.deadline==cx.deadline_us}
}
struct OriginalRetainedLoan<'a>{target:*mut StepContext<'a>,original:RetainedCloneGrant,identity:OriginalRetainedLoanIdentity}
struct OriginalRetirementLoan<'a>{target:*mut StepContext<'a>,original:bool,identity:OriginalRetainedLoanIdentity}
impl Drop for OriginalRetirementLoan<'_>{fn drop(&mut self){unsafe{if self.identity.matches(&*self.target){(*self.target).original_retirement=self.original}}}}
impl OriginalRetainedLoan<'_>{fn matches_original(&self)->bool{unsafe{self.identity.matches(&*self.target)}}}
impl Drop for OriginalRetainedLoan<'_>{fn drop(&mut self){if self.matches_original(){unsafe{(*self.target).retained=self.original}}}}
struct OriginalWorkLoan<'a>{target:*mut StepContext<'a>,fuel:u64,deadline:u64,issued:u64,spent:u128,cancel:usize,identity:OriginalRetainedLoanIdentity}
impl OriginalWorkLoan<'_>{fn matches_original(&self)->bool{let cx=unsafe{&*self.target};self.identity.matches(cx)&&self.cancel==cx.original_cancel_token()as*const CancelToken as usize&&cx.work_units_spent>=self.spent&&cx.fuel_remaining<=self.issued}}
impl Drop for OriginalWorkLoan<'_>{fn drop(&mut self){if self.matches_original(){let cx=unsafe{&mut*self.target};let spent=cx.work_units_spent-self.spent;cx.fuel_remaining=self.fuel.saturating_sub(u64::try_from(spent).unwrap_or(u64::MAX));cx.deadline_us=self.deadline;}}}

impl<'a> StepContext<'a> {
    fn with_original_worker_authority(operation:OperationId,generation:Generation,budget:StepBudget,cancel:&'a CancelToken,now_us:fn()->Option<u64>,clock:ClockStride,preview_sequence:&'a mut u64,retained_progress:&'a mut RetainedCloneProgress,payload_ledger:&'a Arc<JobPayloadOperationLedger>)->Result<Self,ValueError>{if payload_ledger.operation!=operation||payload_ledger.generation!=generation||!retained_progress.fits(budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"worker context requires its original admitted identity ledger and recipient"))}Ok(Self{retained:budget.retained,retained_progress,operation,generation,fuel_remaining:budget.fuel,original_total_fuel:budget.fuel,original_total_work_start:0,original_absolute_deadline:budget.deadline_us,original_total_retained:budget.retained,work_units_spent:0,deadline_us:budget.deadline_us,now_us,clock:std::cell::Cell::new(clock),cancel:StepCancelReference::Borrowed(cancel),stage:"initial",preview_sequence,payload_ledger:JobPayloadLedgerReference::OriginalWorker(payload_ledger),source_parent:None,native_weak_parent:None,source_epoch:None,original_retirement:false,original_clock_binding:None,original_work_recipient:None,original_total_retained_start:Default::default(),retained_policy:None,original_physical_recipient:None,payload_page_granted:false,retained_work:retained_work::RetainedWorkBudget::new(retained_work::NO_RETAINED_WORK)})}
    pub fn new(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, preview_sequence: &'a mut u64,retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        StepContext::with_payload_ledger(operation, generation, budget, cancel, now_us, ClockStride::new(), preview_sequence, Arc::new(JobPayloadOperationLedger::new(operation, generation)),retained_progress)
    }

    #[allow(clippy::too_many_arguments)]
    fn with_payload_ledger(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, clock: ClockStride, preview_sequence: &'a mut u64, payload_ledger: Arc<JobPayloadOperationLedger>,retained_progress:&'a mut RetainedCloneProgress) -> StepContext<'a> {
        assert_eq!(payload_ledger.operation, operation, "job payload ledger operation must match its step context");
        assert_eq!(payload_ledger.generation, generation, "job payload ledger generation must match its step context");
        assert!(retained_progress.fits(budget.retained),"original job recipient exceeds its incoming retained grant");
        StepContext { retained:budget.retained, retained_progress, operation, generation, fuel_remaining: budget.fuel,original_total_fuel:budget.fuel,original_total_work_start:0,original_absolute_deadline:budget.deadline_us,original_total_retained:budget.retained,
            work_units_spent:0, deadline_us: budget.deadline_us, now_us, clock: std::cell::Cell::new(clock), cancel:StepCancelReference::Owned(cancel), stage: "initial", preview_sequence, payload_ledger:JobPayloadLedgerReference::Owned(payload_ledger), source_parent:None,native_weak_parent:None,source_epoch:None,original_retirement:false,original_clock_binding:None,original_work_recipient:None,original_total_retained_start:Default::default(),retained_policy:None,original_physical_recipient:None, payload_page_granted: false, retained_work: retained_work::RetainedWorkBudget::new(retained_work::NO_RETAINED_WORK) }
    }

    /// 🤝️ Borrows the original paid ledger and cancellation while preserving the caller's exact recipient.
    pub fn with_payload_authority(operation:OperationId,generation:Generation,budget:StepBudget,cancel:&'a CancelToken,now_us:fn()->Option<u64>,preview_sequence:&'a mut u64,retained_progress:&'a mut RetainedCloneProgress,authority:&'a JobPayloadAuthority)->Result<Self,ValueError>{
        let Some(original)=authority.original()else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload authority has already closed"))};
        if original.operation!=operation||original.generation!=generation{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload authority identity does not match caller turn"))}
        if !retained_progress.fits(budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job recipient exceeds incoming retained grant"))}
        Ok(Self{retained:budget.retained,retained_progress,operation,generation,fuel_remaining:budget.fuel,original_total_fuel:budget.fuel,original_total_work_start:0,original_absolute_deadline:budget.deadline_us,original_total_retained:budget.retained,work_units_spent:0,deadline_us:budget.deadline_us,now_us,clock:std::cell::Cell::new(ClockStride::new()),cancel:StepCancelReference::Borrowed(cancel),stage:"initial",preview_sequence,payload_ledger:JobPayloadLedgerReference::Borrowed(authority),source_parent:None,native_weak_parent:None,source_epoch:None,original_retirement:false,original_clock_binding:None,original_work_recipient:None,original_total_retained_start:Default::default(),retained_policy:None,original_physical_recipient:None,payload_page_granted:false,retained_work:retained_work::RetainedWorkBudget::new(retained_work::NO_RETAINED_WORK)})
    }

    /// 🎟️ Preserves the original caller depth and independently unspent physical credits.
    pub fn retained_grant(&self)->RetainedCloneGrant{let remaining=retained_remaining(self.retained,*self.retained_progress);let total=retained_remaining(self.original_total_retained,retained_delta(*self.retained_progress,self.original_total_retained_start));let remaining=retained_min(remaining,total);match self.retained_policy{Some((policy,start))=>retained_min(remaining,retained_remaining(policy,retained_delta(*self.retained_progress,start))),None=>remaining}}
    /// 🪙️ Narrows the same original recipient and restores ceilings only while its initiating context remains intact.
    pub fn with_retained_loan<R:Copy>(&mut self,requested:RetainedCloneGrant,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let original=self.retained;let posted=self.retained_progress();
        if !posted.fits(original){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained loan recipient already exceeds its ceiling").into())}
        let remaining=self.retained_grant();let ceiling=|paid:usize,wanted:usize,available:usize|paid.checked_add(wanted.min(available)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original retained loan ceiling overflow"));
        let narrowed=RetainedCloneGrant{maximum_items:ceiling(posted.copied_items,requested.maximum_items,remaining.maximum_items)?,maximum_copy_bytes:ceiling(posted.copied_bytes,requested.maximum_copy_bytes,remaining.maximum_copy_bytes)?,maximum_capacity_bytes:ceiling(posted.retained_capacity_bytes,requested.maximum_capacity_bytes,remaining.maximum_capacity_bytes)?,maximum_release_bytes:ceiling(posted.released_bytes,requested.maximum_release_bytes,remaining.maximum_release_bytes)?,maximum_depth:requested.maximum_depth.min(remaining.maximum_depth)};
        let restore=OriginalRetainedLoan{target:std::ptr::addr_of_mut!(*self),original,identity:OriginalRetainedLoanIdentity::capture(self)};self.retained=narrowed;let result=run(self);
        if restore.matches_original(){return result}
        let refusal=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained loan callback replaced its initiating context");
        Err(match result{Ok(_)=>{let mut failure=OriginalTurnFailure::from(refusal());failure.context_refusal=Some(refusal());failure},Err(mut failure)=>{if failure.context_refusal.is_none(){failure.context_refusal=Some(refusal())}failure}})
    }
    /// ⏳️ Narrows actual CPU and absolute deadline on the same original context, preserving clock observations and charging actual child work even through unwind.
    pub fn with_original_work_loan<R:Copy>(&mut self,requested_fuel:u64,absolute_deadline_us:u64,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        self.with_retained_loan(self.retained_grant(),|cx|{
            let fuel=cx.fuel_remaining;let deadline=cx.deadline_us;let issued=requested_fuel.min(fuel);cx.fuel_remaining=issued;cx.deadline_us=absolute_deadline_us.min(deadline);
            let restore=OriginalWorkLoan{target:std::ptr::addr_of_mut!(*cx),fuel,deadline,issued,spent:cx.work_units_spent,cancel:cx.original_cancel_token()as*const CancelToken as usize,identity:OriginalRetainedLoanIdentity::capture(cx)};let result=run(cx);
            if restore.matches_original(){return result}
            let refusal=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original work loan callback replaced its initiating context or refilled its work");
            Err(match result{Ok(_)=>{let mut failure=OriginalTurnFailure::from(refusal());failure.context_refusal=Some(refusal());failure},Err(mut failure)=>{if failure.context_refusal.is_none(){failure.context_refusal=Some(refusal())}failure}})
        })
    }
    /// 🧾️ Records actual child effects in the external recipient before refusing exceeded authority.
    pub fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{
        let admitted=self.retained_progress.fits(self.retained)&&progress.fits(self.retained_grant());
        *self.retained_progress=self.retained_progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;
        if !admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"job child receipt exceeds original remaining retained grant").with_retained_progress(progress));}Ok(())
    }
    /// 📊️ Exposes actual effects recorded by this same original step context, including failed turns.
    pub fn retained_progress(&self)->RetainedCloneProgress{*self.retained_progress}
    /// 🎟️ Checks the original cumulative recipient against the issued turn grant.
    pub fn retained_recipient_is_admitted(&self)->bool{self.retained_progress().fits(self.retained)}

    /// 🪪️ Identifies the actual external recipient without creating a replacement or alias.
    pub fn original_recipient_identity(&self)->usize{self.retained_progress as*const RetainedCloneProgress as usize}
    /// 🪪️ Borrows only the equality identity of this turn's live original payload source.
    pub fn payload_authority_identity(&self)->OriginalPayloadSourceIdentity{OriginalPayloadSourceIdentity(Arc::as_ptr(&self.payload_ledger)as usize)}

    /// 🕰️ Borrows the caller's original clock source for a retained child authority.
    pub fn clock_source(&self)->fn()->Option<u64>{self.now_us}
    /// 🕰️ Returns the same monotonic observation state after one worker turn.
    pub fn clock_stride(&self)->ClockStride{self.clock.get()}

    /// 🎟️ Reads the original supplied source turn without deriving it from trace sequences.
    pub fn original_source_epoch(&self)->Option<u64>{self.source_epoch}

    /// 🕰️ Reads the opaque clock authority supplied by this actual retained source.
    pub fn original_clock_binding(&self)->Option<u64>{self.original_clock_binding}

    /// 💼️ Quotes the same source's total remaining CPU while its semantic work loan stays narrowed.
    pub fn original_total_fuel_remaining(&self)->u64{self.original_total_fuel.saturating_sub(u64::try_from(self.work_units_spent.saturating_sub(self.original_total_work_start)).unwrap_or(u64::MAX))}

    /// 🧭️ The supplied source deadline remains immutable while a semantic view may yield sooner.
    pub fn original_absolute_deadline_us(&self)->u64{self.original_absolute_deadline}

    /// 🪙️ Quotes total original remaining currencies without changing the active semantic loan.
    pub fn original_total_retained_grant(&self)->RetainedCloneGrant{retained_remaining(self.original_total_retained,retained_delta(*self.retained_progress,self.original_total_retained_start))}

    /// 🔏️ Binds one opaque source clock label without changing its actual callback or absolute deadline.
    pub fn bind_original_clock_binding(&mut self,binding:u64)->Result<(),ValueError>{self.original_source_parent()?.validate_borrowed_context(self).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original clock binding requires its genuine source context"))?;if binding==0||self.original_clock_binding.is_some_and(|original|original!=binding){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original source clock authority cannot be absent or rebound"))}self.original_clock_binding=Some(binding);Ok(())}

    /// 🪪️ The genuine parent issuer binds one input epoch before children borrow this context.
    pub fn bind_original_source_epoch(&mut self,epoch:u64)->Result<(),ValueError>{
        self.original_source_parent()?;
        if epoch==0||self.source_epoch.is_some_and(|original|original!=epoch){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original source epoch cannot be absent or rebound"))}
        self.source_epoch=Some(epoch);Ok(())
    }

    pub fn operation(&self) -> OperationId {
        self.operation
    }

    pub fn generation(&self) -> Generation {
        self.generation
    }

    /// 🤝️ Borrows this caller's original cancellation witness without issuing another alias.
    pub fn original_cancel_token(&self)->&CancelToken{&self.cancel}

    /// 🛑️ Borrows the admitted external cancellation for native controls without an alias or context replacement.
    pub fn borrowed_original_cancel_token(&self)->Result<&'a CancelToken,ValueError>{match &self.cancel{StepCancelReference::Borrowed(original)|StepCancelReference::Observed(original)=>Ok(*original),StepCancelReference::Owned(_)=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native job controls require an original borrowed cancellation witness"))}}

    /// 🏡️ Borrows the genuine supplier parent for its source lifetime without creating another strong loan.
    pub fn original_source_parent(&self)->Result<&'a OriginalCallerSourceParent,ValueError>{self.source_parent.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original caller has no parent source supplier"))}

    /// 📸️ Borrows the genuine external ledger and cancellation roots without creating authority.
    fn borrowed_original_caller_source(&self)->Result<original_caller_capture::OriginalCallerSource<'a>,ValueError>{let ledger=match &self.payload_ledger{JobPayloadLedgerReference::OriginalWorker(original)=>*original,JobPayloadLedgerReference::Borrowed(authority)=>authority.original().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original caller ledger has already closed"))?,JobPayloadLedgerReference::Owned(_)=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"caller capture requires its admitted borrowed original ledger"))};Ok(original_caller_capture::OriginalCallerSource{ledger,cancel:self.borrowed_original_cancel_token()?,operation:self.operation,generation:self.generation,clock:self.now_us,deadline:self.deadline_us})}

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

    /// 🧾️ Raw work remains visible even when a returned guest exceeds the caller's finite wallet.
    pub fn narrow_original_work_budget(&mut self,budget:StepBudget)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none()||!retained_is_subset(budget.retained,self.retained_grant())||budget.fuel>self.fuel_remaining||budget.deadline_us>self.deadline_us{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original work view cannot change source currencies or increase its work/deadline"))}self.retained_policy=Some((budget.retained,*self.retained_progress));self.fuel_remaining=budget.fuel;self.deadline_us=budget.deadline_us;Ok(())}
    pub fn bind_original_retained_total(&mut self,total:RetainedCloneGrant)->Result<(),ValueError>{self.original_source_parent()?;if !retained_is_subset(total,self.retained_grant()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original total exceeds its persistent source recipient"))}self.original_total_retained=total;self.original_total_retained_start=*self.retained_progress;Ok(())}
    pub fn bind_original_physical_recipient(&mut self,recipient:&'a mut RetainedCloneProgress)->Result<(),ValueError>{self.original_source_parent()?;if self.original_physical_recipient.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original physical delta already has its external recipient"))}self.original_physical_recipient=Some((recipient,*self.retained_progress));Ok(())}
    pub fn bind_original_work_recipient(&mut self,recipient:&'a mut u128)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none()||self.original_work_recipient.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original work return requires its one source-bound clock and external recipient"))}self.original_work_recipient=Some(recipient);Ok(())}
    pub fn original_work_units_spent(&self)->u128{self.work_units_spent}
    pub fn fuel_remaining(&self) -> u64 {
        self.fuel_remaining
    }

    /// ⛽️ Decrements the remaining fuel by `units`, saturating at zero — a job calls this after doing
    /// `units` worth of its own work, never before.
    pub fn consume_fuel(&mut self, units: u64) {
        self.work_units_spent+=u128::from(units);self.fuel_remaining = self.fuel_remaining.saturating_sub(units);
    }

    pub fn fuel_exhausted(&self) -> bool {
        self.fuel_remaining == 0
    }

    /// 🚦️ Whether the job must return NOW (before the hard 8 ms ceiling) — either bound crossed.
    pub fn should_yield(&self) -> bool {
        self.fuel_exhausted() || self.deadline_exceeded()
    }

    /// 🧹️ Physical close spends the same finite source even when its semantic deadline expired.
    pub fn retirement_should_yield(&self)->bool{self.fuel_exhausted()||(!self.original_retirement&&self.deadline_exceeded())}

    /// 🛂️ Only a genuine original source can lend close permission; semantic yield remains unchanged.
    pub fn with_original_retirement<T>(&mut self,run:impl FnOnce(&mut Self)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{self.original_source_parent()?.validate_borrowed_context(self)?;let restore=OriginalRetirementLoan{target:std::ptr::addr_of_mut!(*self),original:self.original_retirement,identity:OriginalRetainedLoanIdentity::capture(self)};self.original_retirement=true;let result=run(self);drop(restore);result}

    /// 🛑️ Whether this step's [`CancelToken`] (or an ancestor's) is cancelled — checked via a single
    /// non-blocking [`poll_ready_now`], see the module doc. A job MUST check this on entry and after
    /// every bounded unit of work (design doc Decision 6): return [`StepOutcome::Cancelled`] without
    /// doing further work once true.
    pub fn is_cancelled(&self) -> bool {
        matches!(self.cancel,StepCancelReference::Observed(_))||poll_ready_now(self.cancel.is_cancelled())
    }

    /// 🚩️ Observes the initiating supplier cancellation without spending an outstanding original loan.
    pub fn observe_original_cancellation(&mut self)->Result<(),ValueError>{self.original_source_parent()?;if self.original_clock_binding.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original cancellation observation requires its bound source clock"))}let original=self.borrowed_original_cancel_token()?;self.cancel=StepCancelReference::Observed(original);Ok(())}

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
        if start_us.is_none() || matches!((start_us,self.last_read_us),(Some(now),Some(last)) if now<last) {
            self.remaining=0;
            return;
        }
        self.last_read_us = start_us;
        self.remaining = if start_us.is_some() { self.stride - 1 } else { 0 };
    }

    /// ⏱️ The clock as of the last real read of `clock`, which is at most one stride of calls old.
    pub fn read(&mut self, clock: fn() -> Option<u64>) -> Option<u64> {
        if self.remaining > 0 {
            self.remaining -= 1;
            return self.last_read_us;
        }
        let now = clock()?;
        if let Some(last) = self.last_read_us {
            if now<last {self.remaining=0;return None;}
            let elapsed = (now-last).max(1);
            let calibrated = u64::from(self.stride) * Self::TARGET_READ_INTERVAL_US / elapsed;
            self.stride = calibrated.clamp(1, u64::from(Self::MAXIMUM_STRIDE).min(u64::from(self.stride) * 2)) as u32;
        }
        self.last_read_us = Some(now);
        self.remaining = self.stride - 1;
        Some(now)
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
#[derive(Clone, Copy, Debug, PartialEq, Eq,semio_framework_value::RetireOwned)]
pub enum InteractiveJobCloseStep {
    Pending { progress: RetainedCloneProgress },
    Blocked,
    Complete { progress: RetainedCloneProgress },
}

impl InteractiveJobCloseStep {
    /// 📊️ Keeps every admitted turn's physical and logical receipt, including terminal release.
    pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}=>progress,Self::Blocked=>RetainedCloneProgress::default()}}
    /// 🛡️ Checks independent authority and the original owner's terminal witness before publication.
    pub fn admit(self,grant:RetainedCloneGrant,terminal_is_empty:bool)->Result<Self,ValueError>{
        if !self.progress().fits(grant)||matches!(self,Self::Complete{..})&&!terminal_is_empty{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job close receipt or terminal witness refused").with_retained_progress(self.progress()))}else{Ok(self)}
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
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>;
    fn borrow_outcome<'a>(&'a self, descriptor:&'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>,ValueError>;
    fn begin_close(&mut self);
    /// 🎟️ Consumes physical and work effects once under the same original caller; successful progress reports that debit.
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
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









#[cfg(test)]
pub(crate) const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};

//#region 🧪️Tests


#[cfg(test)]
#[path = "🧪️tests/🔬️retained-ownership/🦀️.rs"]
mod retained_ownership_tests;







#[path = "🔎️reconcile/🧬️schema/🦀️.rs"]
pub mod reconcile;

#[cfg(test)]
#[global_allocator]
static JOB_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;



#[path = "⏱️context/📦️owner/🦀️.rs"]
mod step_context_owner;
pub use step_context_owner::StepContextOwner;
#[cfg(any(test,feature="testkit"))]
#[path="⏱️context/🧪️testing/🎟️caller/🦀️.rs"]
mod original_fixture_caller;
#[cfg(any(test,feature="testkit"))]
pub use original_fixture_caller::OriginalJobFixtureCaller;




