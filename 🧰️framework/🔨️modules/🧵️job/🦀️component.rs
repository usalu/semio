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
use std::mem::{ManuallyDrop, MaybeUninit};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};
use std::task::{Context, Poll, Waker};
use std::time::Instant;

use semio_framework_async::ChannelPolicy;
use semio_framework_trace::{TraceEvent, Watchdog, record_cancelled, record_checkpoint, record_committed, record_failed, record_operation_started, record_preview_published, record_stage_changed};

pub use semio_framework_async::CancelToken;
pub use semio_framework_async::{Lane, ProcessKind, WorkerPool, WorkerPoolConfig};
pub use semio_framework_trace::{Generation, InteractiveStage, OperationId, allocate_operation_id};

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

/// 🎯️ Per-step wall budgets. Actor lane grants may span many steps; they are never reused as one
/// step's deadline. These values leave watchdog margin below the hard eight-millisecond ceiling.
pub const INTERACTIVE_LANE_WALL_MS: u64 = 1;
pub const INTERACTIVE_LANE_FUEL: u64 = 2_000_000;
pub const USER_VISIBLE_LANE_WALL_MS: u64 = 2;
pub const USER_VISIBLE_LANE_FUEL: u64 = 6_000_000;
pub const BACKGROUND_LANE_WALL_MS: u64 = 4;
pub const BACKGROUND_LANE_FUEL: u64 = 20_000_000;
pub const MAINTENANCE_LANE_WALL_MS: u64 = 4;
pub const MAINTENANCE_LANE_FUEL: u64 = 80_000_000;
//#endregion ⛽️Budget

//#region 📄️RetainedPayload
pub const JOB_PAYLOAD_PAGE_BYTES: usize = 16 * 1024;
pub const JOB_PAYLOAD_OPERATION_PAGES: usize = 64;
pub const JOB_PAYLOAD_OPERATION_BYTES: usize = JOB_PAYLOAD_PAGE_BYTES * JOB_PAYLOAD_OPERATION_PAGES;
pub const JOB_PAYLOAD_PROCESS_BYTES: usize = 64 * 1024 * 1024;

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
    storage: Box<[MaybeUninit<u8>; JOB_PAYLOAD_PAGE_BYTES]>,
}

impl std::fmt::Debug for JobPayloadPageSource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("JobPayloadPageSource").field("backing_identity", &self.backing_identity()).finish()
    }
}

impl JobPayloadPageSource {
    pub fn new() -> Self {
        Self { storage: Box::new([MaybeUninit::uninit(); JOB_PAYLOAD_PAGE_BYTES]) }
    }

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
    source: Option<JobPayloadPageSource>,
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

    fn reserve(&self, stream: JobPayloadStream) -> Result<(), JobPayloadAdmissionFault> {
        let stream_index = stream as usize;
        let pages = self.pages.fetch_update(Ordering::AcqRel, Ordering::Acquire, |pages| pages.checked_add(1).filter(|pages| *pages <= JOB_PAYLOAD_OPERATION_PAGES)).map_err(|_| JobPayloadAdmissionFault::OperationItems)?;
        if self.bytes.fetch_update(Ordering::AcqRel, Ordering::Acquire, |bytes| bytes.checked_add(JOB_PAYLOAD_PAGE_BYTES).filter(|bytes| *bytes <= JOB_PAYLOAD_OPERATION_BYTES)).is_err() {
            self.pages.store(pages, Ordering::Release);
            return Err(JobPayloadAdmissionFault::OperationBytes);
        }
        if self.stream_pages[stream_index].fetch_update(Ordering::AcqRel, Ordering::Acquire, |pages| pages.checked_add(1).filter(|pages| *pages <= JOB_PAYLOAD_OPERATION_PAGES)).is_err() {
            self.bytes.fetch_sub(JOB_PAYLOAD_PAGE_BYTES, Ordering::AcqRel);
            self.pages.fetch_sub(1, Ordering::AcqRel);
            return Err(JobPayloadAdmissionFault::StreamItems);
        }
        if self.stream_bytes[stream_index].fetch_update(Ordering::AcqRel, Ordering::Acquire, |bytes| bytes.checked_add(JOB_PAYLOAD_PAGE_BYTES).filter(|bytes| *bytes <= JOB_PAYLOAD_OPERATION_BYTES)).is_err() {
            self.stream_pages[stream_index].fetch_sub(1, Ordering::AcqRel);
            self.bytes.fetch_sub(JOB_PAYLOAD_PAGE_BYTES, Ordering::AcqRel);
            self.pages.fetch_sub(1, Ordering::AcqRel);
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        if JOB_PAYLOAD_PROCESS_OWNED_BYTES.fetch_update(Ordering::AcqRel, Ordering::Acquire, |bytes| bytes.checked_add(JOB_PAYLOAD_PAGE_BYTES).filter(|bytes| *bytes <= JOB_PAYLOAD_PROCESS_BYTES)).is_err() {
            self.stream_bytes[stream_index].fetch_sub(JOB_PAYLOAD_PAGE_BYTES, Ordering::AcqRel);
            self.stream_pages[stream_index].fetch_sub(1, Ordering::AcqRel);
            self.bytes.fetch_sub(JOB_PAYLOAD_PAGE_BYTES, Ordering::AcqRel);
            self.pages.fetch_sub(1, Ordering::AcqRel);
            return Err(JobPayloadAdmissionFault::ProcessBytes);
        }
        Ok(())
    }

    fn release(&self, stream: JobPayloadStream) {
        let stream_index = stream as usize;
        JOB_PAYLOAD_PROCESS_OWNED_BYTES.fetch_sub(JOB_PAYLOAD_PAGE_BYTES, Ordering::AcqRel);
        self.stream_bytes[stream_index].fetch_sub(JOB_PAYLOAD_PAGE_BYTES, Ordering::AcqRel);
        self.stream_pages[stream_index].fetch_sub(1, Ordering::AcqRel);
        self.bytes.fetch_sub(JOB_PAYLOAD_PAGE_BYTES, Ordering::AcqRel);
        self.pages.fetch_sub(1, Ordering::AcqRel);
    }

    fn terminal_is_empty(&self) -> bool {
        self.pages.load(Ordering::Acquire) == 0 && self.bytes.load(Ordering::Acquire) == 0 && self.stream_pages.iter().all(|count| count.load(Ordering::Acquire) == 0) && self.stream_bytes.iter().all(|count| count.load(Ordering::Acquire) == 0)
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
    ledger: Option<Arc<JobPayloadOperationLedger>>,
}

impl RetainedJobPayload {
    pub fn empty(stream: JobPayloadStream) -> Self {
        Self { stream, pages: ManuallyDrop::new(std::array::from_fn(|_| None)), page_count: 0, length: 0, ledger: None }
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

    pub fn reader(&self) -> RetainedJobPayloadReader<'_> {
        RetainedJobPayloadReader { payload: self, page: 0 }
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> JobPayloadCloseStep {
        if self.page_count == 0 {
            self.ledger = None;
            return JobPayloadCloseStep::Complete;
        }
        let index = self.pages.iter().position(Option::is_some).expect("retained payload page count matches occupied pages");
        let page_bytes = self.pages[index].as_ref().expect("retained payload close page").length;
        if maximum_items == 0 || maximum_bytes < page_bytes {
            return JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let page = self.pages[index].take().expect("retained payload close owns exact page");
        self.page_count -= 1;
        self.length -= page.length;
        if let Some(ledger) = self.ledger.as_ref() {
            ledger.release(self.stream);
        }
        let released_bytes = page.length;
        drop(page);
        if self.page_count == 0 {
            self.ledger = None;
            JobPayloadCloseStep::Complete
        } else {
            JobPayloadCloseStep::Pending { released_items: 1, released_bytes }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.page_count == 0 && self.length == 0 && self.pages.iter().all(Option::is_none)
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
        if self.page_count == 0 {
            return;
        }
        debug_assert!(false, "RetainedJobPayload requires one-page close to terminal-empty; ordinary Drop intentionally preserves page backing");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobPayloadCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

pub struct RetainedJobPayloadWriter {
    payload: Option<RetainedJobPayload>,
    rejected: Option<JobPayloadPageSource>,
    sealed: bool,
}

impl std::fmt::Debug for RetainedJobPayloadWriter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("RetainedJobPayloadWriter").field("payload", &self.payload).field("rejected", &self.rejected).field("sealed", &self.sealed).finish()
    }
}

impl RetainedJobPayloadWriter {
    pub fn new(stream: JobPayloadStream) -> Self {
        Self { payload: Some(RetainedJobPayload::empty(stream)), rejected: None, sealed: false }
    }

    pub fn take_rejected_source(&mut self) -> Option<JobPayloadPageSource> {
        self.rejected.take()
    }

    pub fn finish(mut self) -> Result<RetainedJobPayload, Self> {
        if self.rejected.is_some() {
            return Err(self);
        }
        self.sealed = true;
        Ok(self.payload.take().expect("retained payload writer owns payload until finish"))
    }

    pub fn begin_close(&mut self) {
        self.sealed = true;
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> JobPayloadCloseStep {
        self.sealed = true;
        if self.rejected.is_some() {
            if maximum_items == 0 {
                return JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.rejected = None;
            return JobPayloadCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let Some(payload) = self.payload.as_mut() else { return JobPayloadCloseStep::Complete };
        if !payload.terminal_is_empty() {
            return payload.close_step(maximum_items, maximum_bytes);
        }
        if maximum_items == 0 {
            return JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.payload = None;
        JobPayloadCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.sealed && self.rejected.is_none() && self.payload.is_none()
    }

    pub fn write_slice_page(&mut self, cx: &mut StepContext<'_>, bytes: &[u8], cursor: &mut usize) -> Result<bool, JobPayloadAdmissionFault> {
        if *cursor > bytes.len() {
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        if *cursor == bytes.len() {
            return Ok(true);
        }
        let source = self.rejected.take().unwrap_or_default();
        let mut page = match cx.admit_payload_page(self, source) {
            Ok(page) => page,
            Err(rejected) => {
                let fault = rejected.fault;
                self.rejected = Some(rejected.into_source());
                return Err(fault);
            }
        };
        let end = cursor.saturating_add(JOB_PAYLOAD_PAGE_BYTES).min(bytes.len());
        page.write(&bytes[*cursor..end])?;
        page.commit();
        *cursor = end;
        Ok(*cursor == bytes.len())
    }

    fn begin_page<'a>(&'a mut self, ledger: Arc<JobPayloadOperationLedger>, source: JobPayloadPageSource) -> Result<JobPayloadPageGrant<'a>, JobPayloadRejectedPage> {
        if self.sealed {
            return Err(JobPayloadRejectedPage { fault: JobPayloadAdmissionFault::WriterSealed, source: Some(source) });
        }
        if self.rejected.is_some() {
            return Err(JobPayloadRejectedPage { fault: JobPayloadAdmissionFault::RejectedSourcePending, source: Some(source) });
        }
        let payload = self.payload.as_mut().expect("retained payload writer owns payload before finish");
        if payload.page_count >= JOB_PAYLOAD_OPERATION_PAGES {
            return Err(JobPayloadRejectedPage { fault: JobPayloadAdmissionFault::WriterFull, source: Some(source) });
        }
        if let Err(fault) = ledger.reserve(payload.stream) {
            return Err(JobPayloadRejectedPage { fault, source: Some(source) });
        }
        Ok(JobPayloadPageGrant { writer: self, ledger: Some(ledger), source: Some(source), length: 0, committed: false })
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
        JOB_PAYLOAD_PAGE_BYTES - self.length
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

    pub fn commit(mut self) {
        let payload = self.writer.payload.as_mut().expect("retained payload writer owns payload while page is granted");
        let index = payload.pages.iter().position(Option::is_none).expect("preflighted payload page slot remains vacant");
        let source = self.source.take().expect("committed job payload grant owns page source");
        payload.pages[index] = Some(JobPayloadPage { source, length: self.length });
        payload.page_count += 1;
        payload.length += self.length;
        payload.ledger = self.ledger.take();
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
            ledger.release(stream);
        }
        self.writer.rejected = self.source.take();
    }
}
//#endregion 📄️RetainedPayload

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
    payload_ledger: Arc<JobPayloadOperationLedger>,
    payload_page_granted: bool,
}

impl<'a> StepContext<'a> {
    pub fn new(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_ms: fn() -> u64, preview_sequence: &'a mut u64) -> StepContext<'a> {
        StepContext::with_payload_ledger(operation, generation, budget, cancel, now_ms, preview_sequence, Arc::new(JobPayloadOperationLedger::new(operation, generation)))
    }

    fn with_payload_ledger(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_ms: fn() -> u64, preview_sequence: &'a mut u64, payload_ledger: Arc<JobPayloadOperationLedger>) -> StepContext<'a> {
        StepContext { operation, generation, fuel_remaining: budget.fuel, deadline_ms: budget.deadline_ms, now_ms, cancel, stage: "initial", preview_sequence, payload_ledger, payload_page_granted: false }
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
    pub fn next_preview_sequence(&mut self) -> Result<u64, JobSequenceExhausted> {
        let sequence = *self.preview_sequence;
        *self.preview_sequence = (*self.preview_sequence).checked_add(1).ok_or(JobSequenceExhausted::Preview)?;
        Ok(sequence)
    }

    pub fn admit_payload_page<'b>(&mut self, writer: &'b mut RetainedJobPayloadWriter, source: JobPayloadPageSource) -> Result<JobPayloadPageGrant<'b>, JobPayloadRejectedPage> {
        if self.payload_page_granted {
            return Err(JobPayloadRejectedPage { fault: JobPayloadAdmissionFault::OpportunityExhausted, source: Some(source) });
        }
        let grant = writer.begin_page(Arc::clone(&self.payload_ledger), source)?;
        self.payload_page_granted = true;
        Ok(grant)
    }

    pub fn payload_from_bytes(&mut self, stream: JobPayloadStream, bytes: &[u8]) -> Result<RetainedJobPayload, JobPayloadRejectedPage> {
        let source = JobPayloadPageSource::new();
        if bytes.len() > JOB_PAYLOAD_PAGE_BYTES {
            return Err(JobPayloadRejectedPage { fault: JobPayloadAdmissionFault::StreamBytes, source: Some(source) });
        }
        let mut writer = RetainedJobPayloadWriter::new(stream);
        {
            let mut page = self.admit_payload_page(&mut writer, source)?;
            page.write(bytes).expect("single-page payload was length-checked before write");
            page.commit();
        }
        Ok(writer.finish().unwrap_or_else(|_| unreachable!("committed one-page writer has no rejected source")))
    }
}
//#endregion 🧭️StepContext

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

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> JobPayloadCloseStep {
        match self {
            StepOutcome::Yield | StepOutcome::Cancelled => JobPayloadCloseStep::Complete,
            StepOutcome::PreviewReady(payload) => payload.close_step(maximum_items, maximum_bytes),
            StepOutcome::CheckpointReady(checkpoint) => checkpoint.state.close_step(maximum_items, maximum_bytes),
            StepOutcome::Complete(candidate) if !candidate.state.terminal_is_empty() => candidate.state.close_step(maximum_items, maximum_bytes),
            StepOutcome::Complete(candidate) => candidate.output.close_step(maximum_items, maximum_bytes),
            StepOutcome::Fault(fault) => fault.detail.close_step(maximum_items, maximum_bytes),
        }
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
    Pending { released_items: usize, released_bytes: usize },
    Blocked,
    Complete,
}

/// 🧩️ The protocol every interactive operation implements instead of a run-to-completion function
/// call — see the module doc's governing rule. `step` is bounded (checks [`StepContext::should_yield`]
/// and returns before the hard ceiling), cancellable ([`StepContext::is_cancelled`]) and explicitly
/// resumable (a fresh [`StepContext`] each call, job-owned state carries everything between calls).
pub trait InteractiveJob: Send {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome;
    fn begin_close(&mut self);
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep;
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
    drive_step_with_payload_ledger(job, site, operation, generation, stage, budget, cancel, now_ms, preview_sequence, Arc::new(JobPayloadOperationLedger::new(operation, generation)))
}

#[allow(clippy::too_many_arguments)]
fn drive_step_with_payload_ledger<J: InteractiveJob + ?Sized>(
    job: &mut J,
    site: &'static str,
    operation: OperationId,
    generation: Generation,
    stage: InteractiveStage,
    budget: StepBudget,
    cancel: CancelToken,
    now_ms: fn() -> u64,
    preview_sequence: &mut u64,
    payload_ledger: Arc<JobPayloadOperationLedger>,
) -> StepOutcome {
    if poll_ready_now(cancel.is_cancelled()) {
        record_cancelled(operation, generation);
        return StepOutcome::Cancelled;
    }
    let outcome = {
        let _watchdog = Watchdog::start(site, operation, generation, stage);
        let mut cx = StepContext::with_payload_ledger(operation, generation, budget, cancel, now_ms, preview_sequence, payload_ledger);
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

/// 🐕️ Returns the latest hard-ceiling violation for one operation generation.
pub fn watchdog_step_overrun_us(operation: OperationId, generation: Generation) -> Option<u64> {
    Watchdog::violations().into_iter().rev().find(|violation| violation.operation == operation && violation.generation == generation).map(|violation| violation.elapsed_us)
}

pub const JOB_CHILD_SLOTS: usize = 64;

const CHILD_VACANT: u8 = 0;
const CHILD_LIVE: u8 = 1;
const CHILD_CLOSE_INTENT: u8 = 2;
const CHILD_EXHAUSTED: u8 = 3;

struct JobChildSlot {
    generation: AtomicU64,
    state: AtomicU8,
}

impl JobChildSlot {
    fn vacant() -> Self {
        Self { generation: AtomicU64::new(0), state: AtomicU8::new(CHILD_VACANT) }
    }
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

    pub fn spawn_child(&self) -> Result<ChildJobGuard<'_>, JobChildAdmissionFault> {
        if self.closing.load(Ordering::Acquire) || self.is_cancelled() {
            return Err(JobChildAdmissionFault::Closing);
        }
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
            slot.generation.store(generation, Ordering::Release);
            self.live_children.fetch_add(1, Ordering::AcqRel);
            let token = JobChildToken { parent_operation: self.parent_operation, parent_generation: self.parent_generation, slot: index as u16, generation };
            return Ok(ChildJobGuard { scope: self, token: Some(token) });
        }
        let exhausted = self.slots.iter().all(|slot| slot.state.load(Ordering::Acquire) == CHILD_EXHAUSTED);
        Err(if exhausted { JobChildAdmissionFault::Exhausted } else { JobChildAdmissionFault::Capacity })
    }

    pub fn live_child_count(&self) -> u32 {
        self.live_children.load(Ordering::SeqCst)
    }

    pub fn has_live_children(&self) -> bool {
        self.live_child_count() > 0
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

    pub fn pump_child_close(&self) -> bool {
        for slot in &self.slots {
            if slot.state.compare_exchange(CHILD_CLOSE_INTENT, CHILD_VACANT, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                self.live_children.fetch_sub(1, Ordering::AcqRel);
                self.raise_wake();
                return true;
            }
        }
        false
    }

    pub fn take_wake(&self) -> bool {
        self.wake_pending.swap(false, Ordering::AcqRel)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.live_child_count() == 0 && self.slots.iter().all(|slot| matches!(slot.state.load(Ordering::Acquire), CHILD_VACANT | CHILD_EXHAUSTED))
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
        if !matches!(state, CHILD_LIVE | CHILD_CLOSE_INTENT) {
            return Err(JobChildCompletionFault::Duplicate);
        }
        slot.state.compare_exchange(state, CHILD_VACANT, Ordering::AcqRel, Ordering::Acquire).map_err(|_| JobChildCompletionFault::Duplicate)?;
        self.live_children.fetch_sub(1, Ordering::AcqRel);
        self.raise_wake();
        Ok(())
    }

    fn raise_wake(&self) {
        self.wake_pending.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).ok();
    }
}

pub struct ChildJobGuard<'a> {
    scope: &'a JobScope,
    token: Option<JobChildToken>,
}

impl ChildJobGuard<'_> {
    pub fn token(&self) -> JobChildToken {
        self.token.expect("live child guard owns token")
    }

    pub fn complete(mut self) -> Result<(), JobChildCompletionFault> {
        let token = self.token.take().expect("live child guard owns token");
        self.scope.complete_child(token)
    }
}

impl Drop for ChildJobGuard<'_> {
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
    pub site: &'static str,
    pub stage: InteractiveStage,
    pub fuel_per_step: u64,
    pub step_budget_ms: u64,
}

#[derive(Clone)]
pub struct BatchJobParams {
    pub operation: OperationId,
    pub generation: Generation,
    pub cancel: CancelToken,
    pub config: BatchDriveConfig,
    pub now_ms: fn() -> u64,
}

struct WorkerJobAuthority<J> {
    job: Option<J>,
    params: Option<BatchJobParams>,
    preview_sequence: u64,
    step_sequence: u64,
    payload_ledger: Arc<JobPayloadOperationLedger>,
    outcome: Option<StepOutcome>,
    close_stage: u8,
}

impl<J> WorkerJobAuthority<J> {
    fn new(job: J, params: BatchJobParams) -> Self {
        let payload_ledger = Arc::new(JobPayloadOperationLedger::new(params.operation, params.generation));
        Self { job: Some(job), params: Some(params), preview_sequence: 0, step_sequence: 0, payload_ledger, outcome: None, close_stage: 0 }
    }
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
    TerminalEmpty,
}

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
    Pending { released_items: usize, released_bytes: usize },
    Blocked,
    Complete,
}

pub struct BatchJobSession<J> {
    authority: Option<WorkerJobAuthority<J>>,
    terminal: bool,
    checked_out: bool,
    close_requested: bool,
}

impl<J: InteractiveJob> BatchJobSession<J> {
    pub fn new(job: J, params: BatchJobParams) -> Self {
        record_operation_started(params.operation, params.generation);
        Self { authority: Some(WorkerJobAuthority::new(job, params)), terminal: false, checked_out: false, close_requested: false }
    }

    pub fn step(&mut self) -> Result<WorkerJobPoll, WorkerJobContention> {
        if self.close_requested {
            return Ok(WorkerJobPoll::Closing);
        }
        if self.checked_out {
            return Err(WorkerJobContention::CheckedOut(self.generation()));
        }
        let authority = self.authority.as_mut().expect("live batch session owns exact authority");
        if authority.outcome.is_some() {
            return Ok(if self.terminal { WorkerJobPoll::Terminal } else { WorkerJobPoll::Outcome });
        }
        if authority.step_sequence == u64::MAX {
            authority.outcome = Some(StepOutcome::Fault(JobFault { detail: retained_static_payload(&authority.payload_ledger, JobPayloadStream::Fault, b"batch-job.step-sequence-exhausted") }));
            self.terminal = true;
            return Ok(WorkerJobPoll::Terminal);
        }
        let params = authority.params.as_ref().expect("live batch session owns parameters").clone();
        let config = params.config;
        let budget = StepBudget::new(config.fuel_per_step, (params.now_ms)().checked_add(config.step_budget_ms).unwrap_or(u64::MAX));
        let outcome = drive_step_with_payload_ledger(
            authority.job.as_mut().expect("live batch session owns job"),
            config.site,
            params.operation,
            params.generation,
            config.stage,
            budget,
            params.cancel.clone(),
            params.now_ms,
            &mut authority.preview_sequence,
            Arc::clone(&authority.payload_ledger),
        );
        authority.step_sequence = authority.step_sequence.checked_add(1).expect("batch step sequence was preflighted");
        self.terminal = outcome.is_terminal();
        authority.outcome = Some(outcome);
        Ok(if self.terminal { WorkerJobPoll::Terminal } else { WorkerJobPoll::Outcome })
    }

    pub fn outcome(&self) -> Option<&StepOutcome> {
        self.authority.as_ref().and_then(|authority| authority.outcome.as_ref())
    }

    pub fn take_outcome(&mut self) -> Option<StepOutcome> {
        self.authority.as_mut()?.outcome.take()
    }

    pub fn resume(&mut self) -> Result<(), WorkerJobContention> {
        if self.terminal {
            return Err(WorkerJobContention::Terminal(self.ticket()));
        }
        if self.authority.as_ref().is_some_and(|authority| authority.outcome.is_some()) {
            return Err(WorkerJobContention::Outcome(self.ticket()));
        }
        Ok(())
    }

    pub fn begin_close(&mut self) {
        self.close_requested = true;
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> WorkerJobCloseStep {
        self.close_requested = true;
        let Some(authority) = self.authority.as_mut() else { return WorkerJobCloseStep::Complete };
        if let Some(outcome) = authority.outcome.as_mut() {
            if !outcome.terminal_is_empty() {
                return match outcome.close_step(maximum_items, maximum_bytes) {
                    JobPayloadCloseStep::Pending { released_items, released_bytes } => WorkerJobCloseStep::Pending { released_items, released_bytes },
                    JobPayloadCloseStep::Complete => WorkerJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            if maximum_items == 0 {
                return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            authority.outcome = None;
            return WorkerJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if authority.close_stage == 0 {
            if maximum_items == 0 {
                return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            authority.job.as_mut().expect("closing batch authority owns job").begin_close();
            authority.close_stage = 1;
            return WorkerJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if authority.close_stage == 1 {
            match authority.job.as_mut().expect("closing batch authority owns job").close_step(maximum_items, maximum_bytes) {
                InteractiveJobCloseStep::Pending { released_items, released_bytes } => return WorkerJobCloseStep::Pending { released_items, released_bytes },
                InteractiveJobCloseStep::Blocked => return WorkerJobCloseStep::Blocked,
                InteractiveJobCloseStep::Complete if !authority.job.as_ref().expect("closing batch authority owns job").terminal_is_empty() => return WorkerJobCloseStep::Blocked,
                InteractiveJobCloseStep::Complete => authority.close_stage = 2,
            }
        }
        if authority.close_stage == 2 {
            if maximum_items == 0 {
                return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            drop(authority.job.take());
            authority.close_stage = 3;
            return WorkerJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if authority.close_stage == 3 {
            if maximum_items == 0 {
                return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            drop(authority.params.take());
            authority.close_stage = 4;
            return WorkerJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if !authority.payload_ledger.terminal_is_empty() {
            return WorkerJobCloseStep::Blocked;
        }
        self.authority = None;
        WorkerJobCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.authority.is_none()
    }

    fn generation(&self) -> Generation {
        self.authority.as_ref().and_then(|authority| authority.params.as_ref()).map_or(Generation(0), |params| params.generation)
    }

    fn ticket(&self) -> WorkerJobTicket {
        WorkerJobTicket { generation: self.generation(), step_sequence: self.authority.as_ref().map_or(0, |authority| authority.step_sequence) }
    }
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
    pump: unsafe fn(*mut WorkerJobRetirementHeader, usize, usize) -> bool,
    destroy: unsafe fn(*mut WorkerJobRetirementHeader),
}

const WORKER_JOB_RETIREMENT_RESERVED: *mut WorkerJobRetirementHeader = std::ptr::without_provenance_mut(1);
static WORKER_JOB_RETIREMENT_SLOTS: [AtomicPtr<WorkerJobRetirementHeader>; WORKER_JOB_SESSION_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; WORKER_JOB_SESSION_SLOTS];
static WORKER_JOB_RETIREMENT_WAKE: AtomicBool = AtomicBool::new(false);

fn reserve_worker_job_retirement_slot() -> Option<usize> {
    WORKER_JOB_RETIREMENT_SLOTS.iter().enumerate().find_map(|(index, slot)| {
        slot.compare_exchange(std::ptr::null_mut(), WORKER_JOB_RETIREMENT_RESERVED, Ordering::AcqRel, Ordering::Acquire).ok().map(|_| index)
    })
}

pub fn take_worker_job_retirement_wake() -> bool {
    WORKER_JOB_RETIREMENT_WAKE.swap(false, Ordering::AcqRel)
}

pub fn pump_worker_job_retirements(maximum_sessions: usize, maximum_items: usize, maximum_bytes: usize) -> usize {
    if maximum_sessions == 0 {
        return 0;
    }
    let mut advanced = 0;
    for slot in &WORKER_JOB_RETIREMENT_SLOTS {
        if advanced == maximum_sessions {
            break;
        }
        let pointer = slot.load(Ordering::Acquire);
        if pointer.is_null() || pointer == WORKER_JOB_RETIREMENT_RESERVED {
            continue;
        }
        if slot.compare_exchange(pointer, WORKER_JOB_RETIREMENT_RESERVED, Ordering::AcqRel, Ordering::Acquire).is_err() {
            continue;
        }
        let complete = unsafe { ((*pointer).pump)(pointer, maximum_items, maximum_bytes) };
        if complete {
            slot.store(std::ptr::null_mut(), Ordering::Release);
            unsafe { ((*pointer).destroy)(pointer) };
        } else {
            slot.store(pointer, Ordering::Release);
        }
        advanced += 1;
    }
    if WORKER_JOB_RETIREMENT_SLOTS.iter().any(|slot| {
        let pointer = slot.load(Ordering::Acquire);
        !pointer.is_null() && pointer != WORKER_JOB_RETIREMENT_RESERVED
    }) {
        WORKER_JOB_RETIREMENT_WAKE.store(true, Ordering::Release);
    }
    advanced
}

struct WorkerJobSessionInner<J> {
    generation: Generation,
    phase: AtomicU8,
    authority: ManuallyDrop<std::cell::UnsafeCell<Option<WorkerJobAuthority<J>>>>,
    rejection_kind: AtomicU8,
    close_requested: AtomicBool,
    terminal_intent: AtomicU8,
    wake_pending: AtomicBool,
    wake_sequence: AtomicU64,
    wake_exhausted: AtomicBool,
    wake_guard: AtomicBool,
    waker: ManuallyDrop<std::cell::UnsafeCell<Option<Waker>>>,
}

unsafe impl<J: Send> Send for WorkerJobSessionInner<J> {}
unsafe impl<J: Send> Sync for WorkerJobSessionInner<J> {}

impl<J> WorkerJobSessionInner<J> {
    fn phase(&self) -> u8 {
        self.phase.load(Ordering::Acquire)
    }

    unsafe fn take_authority(&self) -> WorkerJobAuthority<J> {
        unsafe { (&mut *self.authority.get()).take().expect("session phase owns exact authority") }
    }

    unsafe fn put_authority(&self, authority: WorkerJobAuthority<J>, phase: u8) {
        unsafe {
            let storage = &mut *self.authority.get();
            assert!(storage.is_none(), "session transition cannot overwrite an authority");
            *storage = Some(authority);
        }
        self.phase.store(phase, Ordering::Release);
        self.raise_wake();
    }

    fn raise_wake(&self) {
        if self.wake_pending.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        if self.wake_sequence.fetch_update(Ordering::AcqRel, Ordering::Acquire, |sequence| sequence.checked_add(1)).is_err() {
            self.wake_exhausted.store(true, Ordering::Release);
        }
        if self.wake_guard.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            let waker = unsafe { (&mut *self.waker.get()).take() };
            self.wake_guard.store(false, Ordering::Release);
            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }

    fn register_waker(&self, waker: &Waker) -> Result<(), WorkerJobContention> {
        if self.wake_guard.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobContention::CheckedOut(self.generation));
        }
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
            }
        }
    }
}

pub struct WorkerJobSession<J: InteractiveJob + 'static> {
    inner: Arc<WorkerJobSessionInner<J>>,
    retirement: std::cell::UnsafeCell<Option<Box<WorkerJobRetirementNode<J>>>>,
    retirement_state: AtomicU8,
}

unsafe impl<J: InteractiveJob + 'static> Send for WorkerJobSession<J> {}
unsafe impl<J: InteractiveJob + 'static> Sync for WorkerJobSession<J> {}

#[repr(C)]
struct WorkerJobRetirementNode<J> {
    header: WorkerJobRetirementHeader,
    inner: Option<Arc<WorkerJobSessionInner<J>>>,
}

unsafe fn pump_worker_job_retirement_node<J: InteractiveJob + 'static>(pointer: *mut WorkerJobRetirementHeader, maximum_items: usize, maximum_bytes: usize) -> bool {
    let node = unsafe { &mut *pointer.cast::<WorkerJobRetirementNode<J>>() };
    let inner = node.inner.as_ref().expect("mounted worker retirement owns the exact session authority");
    if matches!(worker_job_begin_close(inner), WorkerJobCloseStep::Blocked) {
        return false;
    }
    match worker_job_close_step(inner, maximum_items, maximum_bytes) {
        WorkerJobCloseStep::Complete => {
            node.inner.take();
            true
        }
        WorkerJobCloseStep::Pending { .. } | WorkerJobCloseStep::Blocked => false,
    }
}

unsafe fn destroy_worker_job_retirement_node<J>(pointer: *mut WorkerJobRetirementHeader) {
    drop(unsafe { Box::from_raw(pointer.cast::<WorkerJobRetirementNode<J>>()) });
}

pub struct WorkerJobSessionAdmissionRejected<J> {
    job: Option<J>,
    params: Option<BatchJobParams>,
}

impl<J> WorkerJobSessionAdmissionRejected<J> {
    pub fn job(&self) -> &J {
        self.job.as_ref().expect("rejected worker session admission owns exact job")
    }

    pub fn into_parts(mut self) -> (J, BatchJobParams) {
        (
            self.job.take().expect("rejected worker session admission owns exact job"),
            self.params.take().expect("rejected worker session admission owns exact parameters"),
        )
    }
}

struct WorkerJobSubmission<J> {
    inner: Arc<WorkerJobSessionInner<J>>,
    authority: Option<WorkerJobAuthority<J>>,
    ran: bool,
}

impl<J: InteractiveJob + 'static> WorkerJobSubmission<J> {
    fn run(mut self) {
        self.ran = true;
        let mut authority = self.authority.take().expect("submitted worker closure owns exact job authority");
        let params = authority.params.as_ref().expect("submitted job authority owns parameters").clone();
        let terminal = if authority.step_sequence == u64::MAX {
            authority.outcome = Some(StepOutcome::Fault(JobFault { detail: retained_static_payload(&authority.payload_ledger, JobPayloadStream::Fault, b"worker-job.step-sequence-exhausted") }));
            true
        } else {
            let config = params.config;
            let budget = StepBudget::new(config.fuel_per_step, (params.now_ms)().checked_add(config.step_budget_ms).unwrap_or(u64::MAX));
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                drive_step_with_payload_ledger(
                    authority.job.as_mut().expect("submitted authority owns job"),
                    config.site,
                    params.operation,
                    params.generation,
                    config.stage,
                    budget,
                    params.cancel.clone(),
                    params.now_ms,
                    &mut authority.preview_sequence,
                    Arc::clone(&authority.payload_ledger),
                )
            }));
            authority.step_sequence = authority.step_sequence.checked_add(1).unwrap_or(u64::MAX);
            authority.outcome = Some(match result {
                Ok(outcome) => outcome,
                Err(_) => StepOutcome::Fault(JobFault { detail: retained_static_payload(&authority.payload_ledger, JobPayloadStream::Fault, b"worker-job.step-panicked") }),
            });
            authority.outcome.as_ref().is_some_and(StepOutcome::is_terminal)
        };
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

fn retained_static_payload(ledger: &Arc<JobPayloadOperationLedger>, stream: JobPayloadStream, bytes: &'static [u8]) -> RetainedJobPayload {
    let source = JobPayloadPageSource::new();
    let mut writer = RetainedJobPayloadWriter::new(stream);
    let Ok(mut page) = writer.begin_page(Arc::clone(ledger), source) else { return RetainedJobPayload::empty(stream) };
    if page.write(bytes).is_err() {
        return RetainedJobPayload::empty(stream);
    }
    page.commit();
    writer.finish().unwrap_or_else(|_| RetainedJobPayload::empty(stream))
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
            return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if phase == SESSION_EMPTY {
            return WorkerJobCloseStep::Complete;
        }
        if inner.phase.compare_exchange(phase, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            let authority = unsafe { inner.take_authority() };
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
    }
}

fn worker_job_close_step<J: InteractiveJob>(inner: &WorkerJobSessionInner<J>, maximum_items: usize, maximum_bytes: usize) -> WorkerJobCloseStep {
    if inner.phase.compare_exchange(SESSION_CLOSE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return if inner.phase() == SESSION_EMPTY { WorkerJobCloseStep::Complete } else { WorkerJobCloseStep::Blocked };
    }
    let mut authority = unsafe { inner.take_authority() };
    if let Some(outcome) = authority.outcome.as_mut() {
        if !outcome.terminal_is_empty() {
            let step = outcome.close_step(maximum_items, maximum_bytes);
            let result = match step {
                JobPayloadCloseStep::Pending { released_items, released_bytes } => WorkerJobCloseStep::Pending { released_items, released_bytes },
                JobPayloadCloseStep::Complete => WorkerJobCloseStep::Pending { released_items: usize::from(maximum_items > 0), released_bytes: 0 },
            };
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return result;
        }
        if maximum_items == 0 {
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        authority.outcome = None;
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return WorkerJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
    }
    if authority.close_stage == 0 {
        if maximum_items == 0 {
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        authority.job.as_mut().expect("closing worker authority owns job").begin_close();
        authority.close_stage = 1;
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return WorkerJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
    }
    if authority.close_stage == 1 {
        let step = authority.job.as_mut().expect("closing worker authority owns job").close_step(maximum_items, maximum_bytes);
        match step {
            InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                unsafe { inner.put_authority(authority, SESSION_CLOSE) };
                return WorkerJobCloseStep::Pending { released_items, released_bytes };
            }
            InteractiveJobCloseStep::Blocked => {
                unsafe { inner.put_authority(authority, SESSION_CLOSE) };
                return WorkerJobCloseStep::Blocked;
            }
            InteractiveJobCloseStep::Complete => {
                if !authority.job.as_ref().expect("closing worker authority owns job").terminal_is_empty() {
                    unsafe { inner.put_authority(authority, SESSION_CLOSE) };
                    return WorkerJobCloseStep::Blocked;
                }
                authority.close_stage = 2;
            }
        }
    }
    if authority.close_stage == 2 {
        if maximum_items == 0 {
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        drop(authority.job.take());
        authority.close_stage = 3;
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return WorkerJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
    }
    if authority.close_stage == 3 {
        if maximum_items == 0 {
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        drop(authority.params.take());
        authority.close_stage = 4;
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return WorkerJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
    }
    if !authority.payload_ledger.terminal_is_empty() {
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return WorkerJobCloseStep::Blocked;
    }
    inner.phase.store(SESSION_EMPTY, Ordering::Release);
    inner.raise_wake();
    WorkerJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
}

impl<J: InteractiveJob + 'static> WorkerJobSession<J> {
    pub fn try_new(job: J, params: BatchJobParams) -> Result<Self, WorkerJobSessionAdmissionRejected<J>> {
        let Some(slot) = reserve_worker_job_retirement_slot() else {
            return Err(WorkerJobSessionAdmissionRejected { job: Some(job), params: Some(params) });
        };
        record_operation_started(params.operation, params.generation);
        let generation = params.generation;
        let inner = Arc::new(WorkerJobSessionInner {
                generation,
                phase: AtomicU8::new(SESSION_IDLE),
                authority: ManuallyDrop::new(std::cell::UnsafeCell::new(Some(WorkerJobAuthority::new(job, params)))),
                rejection_kind: AtomicU8::new(u8::MAX),
                close_requested: AtomicBool::new(false),
                terminal_intent: AtomicU8::new(0),
                wake_pending: AtomicBool::new(false),
                wake_sequence: AtomicU64::new(0),
                wake_exhausted: AtomicBool::new(false),
                wake_guard: AtomicBool::new(false),
                waker: ManuallyDrop::new(std::cell::UnsafeCell::new(None)),
            });
        let retirement = Box::new(WorkerJobRetirementNode {
            header: WorkerJobRetirementHeader { slot, pump: pump_worker_job_retirement_node::<J>, destroy: destroy_worker_job_retirement_node::<J> },
            inner: None,
        });
        Ok(Self { inner, retirement: std::cell::UnsafeCell::new(Some(retirement)), retirement_state: AtomicU8::new(0) })
    }

    pub fn poll(&self) -> WorkerJobPoll {
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

    pub fn register_wake(&self, waker: &Waker) -> Result<(), WorkerJobContention> {
        self.inner.register_waker(waker)
    }

    pub fn take_wake(&self) -> bool {
        self.inner.wake_pending.swap(false, Ordering::AcqRel)
    }

    pub fn try_submit_step(&self, pool: &WorkerPool, lane: Lane) -> Result<WorkerJobTicket, WorkerJobSubmitFault> {
        if self.inner.phase.compare_exchange(SESSION_IDLE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobSubmitFault::Contention(self.contention()));
        }
        let authority = unsafe { self.inner.take_authority() };
        if authority.step_sequence == u64::MAX {
            unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
            return Err(WorkerJobSubmitFault::SequenceExhausted);
        }
        let ticket = WorkerJobTicket { generation: self.inner.generation, step_sequence: authority.step_sequence };
        self.inner.rejection_kind.store(u8::MAX, Ordering::Release);
        self.inner.phase.store(SESSION_SUBMITTED, Ordering::Release);
        let submission = WorkerJobSubmission { inner: Arc::clone(&self.inner), authority: Some(authority), ran: false };
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
        Ok(WorkerJobOutcome { inner: Arc::clone(&self.inner), authority: Some(authority), restore_phase: SESSION_OUTCOME })
    }

    pub fn take_terminal(&self) -> Result<WorkerJobOutcome<J>, WorkerJobTakeFault> {
        if self.inner.phase.compare_exchange(SESSION_TERMINAL, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(if self.inner.phase() == SESSION_SUBMITTED { WorkerJobTakeFault::Pending } else { WorkerJobTakeFault::WrongPhase });
        }
        let authority = unsafe { self.inner.take_authority() };
        Ok(WorkerJobOutcome { inner: Arc::clone(&self.inner), authority: Some(authority), restore_phase: SESSION_TERMINAL })
    }

    pub fn take_rejected(&self) -> Result<WorkerJobRejected<J>, WorkerJobTakeFault> {
        if self.inner.phase.compare_exchange(SESSION_REJECTED, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobTakeFault::WrongPhase);
        }
        let authority = unsafe { self.inner.take_authority() };
        let kind = worker_rejection_kind(self.inner.rejection_kind.load(Ordering::Acquire));
        Ok(WorkerJobRejected { inner: Arc::clone(&self.inner), authority: Some(authority), kind })
    }

    pub fn begin_close(&self) -> WorkerJobCloseStep {
        worker_job_begin_close(&self.inner)
    }

    pub fn close_step(&self, maximum_items: usize, maximum_bytes: usize) -> WorkerJobCloseStep {
        match worker_job_close_step(&self.inner, maximum_items, maximum_bytes) {
            WorkerJobCloseStep::Complete if self.release_retirement_slot(maximum_items) => WorkerJobCloseStep::Complete,
            WorkerJobCloseStep::Complete => WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 },
            step => step,
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.inner.phase() == SESSION_EMPTY && unsafe { (&*self.inner.authority.get()).is_none() } && self.retirement_state.load(Ordering::Acquire) == 3
    }

    fn contention(&self) -> WorkerJobContention {
        let generation = self.inner.generation;
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

    fn release_retirement_slot(&self, maximum_items: usize) -> bool {
        if self.retirement_state.load(Ordering::Acquire) == 3 {
            return true;
        }
        if maximum_items == 0 || self.retirement_state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        let retirement = unsafe { (&mut *self.retirement.get()).take().expect("live worker session owns pre-admitted retirement node") };
        WORKER_JOB_RETIREMENT_SLOTS[retirement.header.slot].store(std::ptr::null_mut(), Ordering::Release);
        drop(retirement);
        self.retirement_state.store(3, Ordering::Release);
        true
    }
}

impl<J: InteractiveJob + 'static> Drop for WorkerJobSession<J> {
    fn drop(&mut self) {
        self.inner.close_requested.store(true, Ordering::Release);
        self.inner.terminal_intent.store(1, Ordering::Release);
        self.inner.raise_wake();
        if self.retirement_state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let mut retirement = unsafe { (&mut *self.retirement.get()).take().expect("live worker session owns pre-admitted retirement node") };
        if self.inner.phase() == SESSION_EMPTY {
            WORKER_JOB_RETIREMENT_SLOTS[retirement.header.slot].store(std::ptr::null_mut(), Ordering::Release);
            self.retirement_state.store(3, Ordering::Release);
            return;
        }
        retirement.inner = Some(Arc::clone(&self.inner));
        let slot = retirement.header.slot;
        let pointer = Box::into_raw(retirement).cast::<WorkerJobRetirementHeader>();
        WORKER_JOB_RETIREMENT_SLOTS[slot].store(pointer, Ordering::Release);
        self.retirement_state.store(2, Ordering::Release);
        WORKER_JOB_RETIREMENT_WAKE.store(true, Ordering::Release);
    }
}

pub struct WorkerJobOutcome<J> {
    inner: Arc<WorkerJobSessionInner<J>>,
    authority: Option<WorkerJobAuthority<J>>,
    restore_phase: u8,
}

impl<J> WorkerJobOutcome<J> {
    pub fn job(&self) -> &J {
        self.authority.as_ref().and_then(|authority| authority.job.as_ref()).expect("checked-out worker outcome owns exact job")
    }

    pub fn job_mut(&mut self) -> &mut J {
        self.authority.as_mut().and_then(|authority| authority.job.as_mut()).expect("checked-out worker outcome owns exact job")
    }

    pub fn outcome(&self) -> &StepOutcome {
        self.authority.as_ref().and_then(|authority| authority.outcome.as_ref()).expect("checked-out worker outcome owns exact outcome")
    }

    pub fn take_outcome(&mut self) -> StepOutcome {
        self.authority.as_mut().and_then(|authority| authority.outcome.take()).expect("checked-out worker outcome owns exact outcome")
    }

    pub fn resume(mut self) -> Result<(), Self> {
        let authority = self.authority.as_ref().expect("checked-out worker outcome owns authority");
        if authority.outcome.as_ref().is_some_and(|outcome| !outcome.terminal_is_empty() || outcome.is_terminal()) {
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
    inner: Arc<WorkerJobSessionInner<J>>,
    authority: Option<WorkerJobAuthority<J>>,
    kind: semio_framework_async::WorkerSubmitErrorKind,
}

impl<J> WorkerJobRejected<J> {
    pub fn kind(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.kind
    }

    pub fn job(&self) -> &J {
        self.authority.as_ref().and_then(|authority| authority.job.as_ref()).expect("checked-out rejected worker owner remains exact")
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
    terminal_state: Option<RetainedJobPayload>,
    scope: JobScope,
    closing: bool,
}

/// 🩺️ How many units [`TortureJob::step`] processes between cheap `should_yield` polls — small enough
/// that overshoot past the 8 ms ceiling within one check interval is negligible (each unit is a
/// handful of integer ops), large enough that the `now_ms`/fuel check itself isn't the hot-loop
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
            terminal_state: None,
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
        TortureJob { total_units, completed_units, rng_state, accumulator, checkpoint_every_units, preview_every_units, units_since_checkpoint: 0, units_since_preview: 0, terminal_state: None, scope: JobScope::child_of(parent_cancel), closing: false }
    }

    fn encode_preview(&self, sequence: u64) -> [u8; 24] {
        let mut out = [0u8; 24];
        out[..8].copy_from_slice(&sequence.to_le_bytes());
        out[8..16].copy_from_slice(&self.completed_units.to_le_bytes());
        out[16..].copy_from_slice(&self.accumulator.to_le_bytes());
        out
    }

    fn output_bytes(&self) -> [u8; 16] {
        let mut output = [0u8; 16];
        output[..8].copy_from_slice(&self.completed_units.to_le_bytes());
        output[8..].copy_from_slice(&self.accumulator.to_le_bytes());
        output
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
                let Ok(sequence) = cx.next_preview_sequence() else {
                    let detail = cx.payload_from_bytes(JobPayloadStream::Fault, b"torture.preview-sequence-exhausted").unwrap_or_else(|_| RetainedJobPayload::empty(JobPayloadStream::Fault));
                    return StepOutcome::Fault(JobFault { detail });
                };
                let preview = self.encode_preview(sequence);
                let payload = cx.payload_from_bytes(JobPayloadStream::Preview, &preview).unwrap_or_else(|_| RetainedJobPayload::empty(JobPayloadStream::Preview));
                return StepOutcome::PreviewReady(payload);
            }
            if self.units_since_checkpoint >= self.checkpoint_every_units {
                self.units_since_checkpoint = 0;
                let state = self.checkpoint_bytes();
                let payload = cx.payload_from_bytes(JobPayloadStream::CheckpointState, &state).unwrap_or_else(|_| RetainedJobPayload::empty(JobPayloadStream::CheckpointState));
                return StepOutcome::CheckpointReady(Checkpoint { state: payload, applied_progress: self.completed_units });
            }
        }
        if self.scope.assert_completable().is_err() {
            let detail = cx.payload_from_bytes(JobPayloadStream::Fault, b"torture.live-structured-child").unwrap_or_else(|_| RetainedJobPayload::empty(JobPayloadStream::Fault));
            return StepOutcome::Fault(JobFault { detail });
        }
        if self.terminal_state.is_none() {
            let state = self.checkpoint_bytes();
            self.terminal_state = Some(cx.payload_from_bytes(JobPayloadStream::CommitState, &state).unwrap_or_else(|_| RetainedJobPayload::empty(JobPayloadStream::CommitState)));
            return StepOutcome::Yield;
        }
        let output = self.output_bytes();
        let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, &output).unwrap_or_else(|_| RetainedJobPayload::empty(JobPayloadStream::CommitOutput));
        StepOutcome::Complete(CommitCandidate { state: self.terminal_state.take().expect("terminal state page was retained across one opportunity"), output })
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.scope.begin_close();
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if self.scope.pump_child_close() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(state) = self.terminal_state.as_mut() {
            if !state.terminal_is_empty() {
                return match state.close_step(maximum_items, maximum_bytes) {
                    JobPayloadCloseStep::Pending { released_items, released_bytes } => InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    JobPayloadCloseStep::Complete => InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.terminal_state = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.scope.terminal_is_empty() { InteractiveJobCloseStep::Complete } else { InteractiveJobCloseStep::Blocked }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.terminal_state.is_none() && self.scope.terminal_is_empty()
    }
}
//#endregion 🔥️TortureJob

//#region 🧪️Tests
#[cfg(any())]
mod tests {
    use super::*;
    use semio_framework_async::{ProcessKind, WorkerPoolConfig};
    use std::sync::atomic::AtomicBool;
    use std::time::Duration;
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
                ChannelPolicy::Ring { max_bytes, .. } => *max_bytes,
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

    struct GatedJob {
        release: Arc<AtomicBool>,
        completed: Arc<AtomicBool>,
        first_step: Option<Sender<()>>,
    }

    impl InteractiveJob for GatedJob {
        fn step(&mut self, _cx: &mut StepContext<'_>) -> StepOutcome {
            if let Some(sender) = self.first_step.take() {
                let _ = sender.send(());
            }
            if !self.release.load(Ordering::SeqCst) {
                return StepOutcome::Yield;
            }
            self.completed.store(true, Ordering::SeqCst);
            StepOutcome::Complete(CommitCandidate { state: Vec::new(), output: Vec::new() })
        }
    }

    #[test]
    fn run_on_worker_releases_a_single_worker_between_job_steps() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let release = Arc::new(AtomicBool::new(false));
        let completed = Arc::new(AtomicBool::new(false));
        let (first_tx, first_rx) = std::sync::mpsc::channel();
        let job = GatedJob { release: Arc::clone(&release), completed: Arc::clone(&completed), first_step: Some(first_tx) };
        let params = BatchJobParams {
            operation: allocate_operation_id(),
            generation: Generation(1),
            cancel: root_cancel_token(),
            config: BatchDriveConfig { site: "test.batch.finite-turn", stage: InteractiveStage::BackgroundStep, fuel_per_step: 1, step_budget_ms: BACKGROUND_LANE_WALL_MS },
            now_ms: default_now_ms,
        };
        let terminal = run_on_worker(&pool, Lane::Background, job, params);
        first_rx.recv_timeout(Duration::from_secs(1)).expect("first job step did not run");
        let (competitor_tx, competitor_rx) = std::sync::mpsc::channel();
        pool.submit(
            Lane::Interactive,
            Box::new(move || {
                let completed_before_competitor = completed.load(Ordering::SeqCst);
                release.store(true, Ordering::SeqCst);
                competitor_tx.send(completed_before_competitor).expect("competitor receiver alive");
            }),
        );
        assert!(!competitor_rx.recv_timeout(Duration::from_secs(1)).expect("competing closure never ran"));
        assert!(matches!(terminal.recv_timeout(Duration::from_secs(1)), Ok(StepOutcome::Complete(_))));
        pool.shutdown();
    }

    struct CountedSessionJob {
        steps: Arc<AtomicU32>,
    }

    impl InteractiveJob for CountedSessionJob {
        fn step(&mut self, _cx: &mut StepContext<'_>) -> StepOutcome {
            let step = self.steps.fetch_add(1, Ordering::SeqCst) + 1;
            if step < 3 { StepOutcome::Yield } else { StepOutcome::Complete(CommitCandidate { state: Vec::new(), output: vec![step as u8] }) }
        }
    }

    struct GatedSessionJob {
        release: Arc<AtomicBool>,
        steps: Arc<AtomicU32>,
    }

    impl InteractiveJob for GatedSessionJob {
        fn step(&mut self, _context: &mut StepContext<'_>) -> StepOutcome {
            self.steps.fetch_add(1, Ordering::SeqCst);
            while !self.release.load(Ordering::SeqCst) {
                std::thread::yield_now();
            }
            StepOutcome::Yield
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn submitted_first_step_can_be_polled_pending_without_panicking_or_double_submitting() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let release = Arc::new(AtomicBool::new(false));
        let steps = Arc::new(AtomicU32::new(0));
        let params = BatchJobParams {
            operation: allocate_operation_id(),
            generation: Generation(1),
            cancel: root_cancel_token(),
            config: BatchDriveConfig { site: "test.worker-session.pending-first-poll", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: INTERACTIVE_LANE_WALL_MS },
            now_ms: default_now_ms,
        };
        let session = WorkerJobSession::new(GatedSessionJob { release: Arc::clone(&release), steps: Arc::clone(&steps) }, params);
        let mut pending = session.submit_step(&pool, Lane::Interactive);
        assert_eq!(pending.try_recv(), Err(semio_framework_async::oneshot::TryRecvError::Empty));
        assert!(steps.load(Ordering::SeqCst) <= 1, "one submitted receiver must never enter the job twice");
        release.store(true, Ordering::SeqCst);
        assert_eq!(pending.await.expect("gated worker result"), StepOutcome::Yield);
        assert_eq!(steps.load(Ordering::SeqCst), 1);
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn worker_job_session_admits_exactly_one_step_per_caller_turn() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let steps = Arc::new(AtomicU32::new(0));
        let params = BatchJobParams {
            operation: allocate_operation_id(),
            generation: Generation(1),
            cancel: root_cancel_token(),
            config: BatchDriveConfig { site: "test.worker-session.single-turn", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_ms: USER_VISIBLE_LANE_WALL_MS },
            now_ms: default_now_ms,
        };
        let session = WorkerJobSession::new(CountedSessionJob { steps: Arc::clone(&steps) }, params);
        assert_eq!(session.step(&pool, Lane::UserVisible).await.expect("first worker turn"), StepOutcome::Yield);
        assert_eq!(steps.load(Ordering::SeqCst), 1, "session must not self-requeue after the caller receives one outcome");
        assert_eq!(session.step(&pool, Lane::UserVisible).await.expect("second worker turn"), StepOutcome::Yield);
        assert_eq!(steps.load(Ordering::SeqCst), 2);
        assert!(matches!(session.step(&pool, Lane::UserVisible).await.expect("terminal worker turn"), StepOutcome::Complete(candidate) if candidate.output == vec![3]));
        assert_eq!(session.step(&pool, Lane::UserVisible).await.expect("post-terminal worker turn"), StepOutcome::Yield, "a terminal outcome must be delivered exactly once");
        assert_eq!(steps.load(Ordering::SeqCst), 3, "the job must not be entered after its terminal outcome");
        pool.shutdown();
    }

    #[test]
    fn rejected_worker_step_admission_retains_the_exact_persistent_session() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        pool.shutdown();
        let steps = Arc::new(AtomicU32::new(0));
        let params = BatchJobParams {
            operation: allocate_operation_id(),
            generation: Generation(1),
            cancel: root_cancel_token(),
            config: BatchDriveConfig { site: "test.worker-session.rejected-admission", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: INTERACTIVE_LANE_WALL_MS },
            now_ms: default_now_ms,
        };
        let session = WorkerJobSession::new(CountedSessionJob { steps: Arc::clone(&steps) }, params);
        assert!(matches!(session.try_submit_step(&pool, Lane::Interactive), Err(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
        assert_eq!(steps.load(Ordering::SeqCst), 0);
        assert!(session.try_into_job().is_ok(), "rejected admission must release only its shallow scheduling closure");
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

#[cfg(test)]
mod retained_ownership_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

    fn params(operation: OperationId, generation: Generation, cancel: CancelToken) -> BatchJobParams {
        BatchJobParams { operation, generation, cancel, config: BatchDriveConfig { site: "test.retained-job", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 1 }, now_ms: default_now_ms }
    }

    fn wait_for(session: &WorkerJobSession<HostileJob>, expected: WorkerJobPoll) {
        for _ in 0..4_096 {
            if session.poll() == expected {
                return;
            }
            std::thread::yield_now();
        }
        panic!("worker session did not reach {expected:?}");
    }

    #[test]
    fn retained_payload_max_plus_one_zero_grant_nested_and_exact_close_are_owned() {
        let operation = OperationId(90_001);
        let generation = Generation(7);
        let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
        let process_before = JOB_PAYLOAD_PROCESS_OWNED_BYTES.load(Ordering::Acquire);
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState);
        for index in 0..JOB_PAYLOAD_OPERATION_PAGES {
            let mut preview_sequence = index as u64;
            let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_ms, &mut preview_sequence, Arc::clone(&ledger));
            let source = JobPayloadPageSource::new();
            let mut page = context.admit_payload_page(&mut writer, source).expect("each fixed payload page is admitted before write");
            page.write(&[index as u8]).expect("one byte fits admitted page");
            page.commit();
        }
        let mut sequence = 0;
        let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_ms, &mut sequence, Arc::clone(&ledger));
        let plus_one = JobPayloadPageSource::new();
        let plus_one_pointer = plus_one.backing_identity();
        let rejected = match context.admit_payload_page(&mut writer, plus_one) {
            Ok(_) => panic!("page maximum plus one must not receive an output grant"),
            Err(rejected) => rejected,
        };
        assert_eq!(rejected.source().backing_identity(), plus_one_pointer);
        let returned = rejected.into_source();
        assert_eq!(returned.backing_identity(), plus_one_pointer);
        drop(returned);
        let mut payload = writer.finish().expect("full payload has no rejected source retained");
        assert_eq!(payload.page_count(), JOB_PAYLOAD_OPERATION_PAGES);
        assert_eq!(payload.close_step(0, 0), JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 });
        for _ in 0..JOB_PAYLOAD_OPERATION_PAGES {
            let _ = payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
        assert!(payload.terminal_is_empty());
        assert!(ledger.terminal_is_empty());
        assert_eq!(JOB_PAYLOAD_PROCESS_OWNED_BYTES.load(Ordering::Acquire), process_before);
    }

    #[test]
    fn retained_state_and_output_have_separate_credits_and_close_one_page_per_grant() {
        let operation = OperationId(90_002);
        let generation = Generation(8);
        let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
        let mut state_writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitState);
        let mut output_writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput);
        let mut sequence = 0;
        let mut state_context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_ms, &mut sequence, Arc::clone(&ledger));
        let mut state_page = state_context.admit_payload_page(&mut state_writer, JobPayloadPageSource::new()).expect("state page");
        state_page.write(b"state").expect("state bytes");
        state_page.commit();
        let mut output_context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_ms, &mut sequence, Arc::clone(&ledger));
        let mut output_page = output_context.admit_payload_page(&mut output_writer, JobPayloadPageSource::new()).expect("separate output page");
        output_page.write(b"output").expect("output bytes");
        output_page.commit();
        let mut terminal = StepOutcome::Complete(CommitCandidate { state: state_writer.finish().expect("state"), output: output_writer.finish().expect("output") });
        assert!(matches!(terminal.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Complete));
        assert!(!terminal.terminal_is_empty());
        assert!(matches!(terminal.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Complete));
        assert!(terminal.terminal_is_empty());
    }

    #[test]
    fn child_registry_max_plus_one_stale_duplicate_exhaustion_and_parent_completion_are_exact() {
        let scope = JobScope::for_operation(&root_cancel_token(), OperationId(90_003), Generation(9));
        let mut guards: [Option<ChildJobGuard<'_>>; JOB_CHILD_SLOTS] = std::array::from_fn(|_| Some(scope.spawn_child().expect("fixed child slot")));
        assert!(matches!(scope.spawn_child(), Err(JobChildAdmissionFault::Capacity)));
        assert_eq!(scope.assert_completable(), Err(JobChildCompletionFault::LiveChildren));
        let token = guards[0].as_ref().expect("first child").token();
        guards[0].take().expect("first child").complete().expect("first exact completion");
        assert_eq!(scope.complete_child(token), Err(JobChildCompletionFault::Duplicate));
        let stale = JobChildToken { generation: token.generation + 1, ..token };
        assert_eq!(scope.complete_child(stale), Err(JobChildCompletionFault::Stale));
        drop(guards);
        assert!(scope.assert_completable().is_ok());
        for slot in &scope.slots {
            slot.generation.store(u64::MAX, Ordering::Release);
            slot.state.store(CHILD_VACANT, Ordering::Release);
        }
        assert!(matches!(scope.spawn_child(), Err(JobChildAdmissionFault::Exhausted)));
        scope.begin_close();
        assert!(scope.terminal_is_empty());
    }

    struct HostileJob {
        backing: Box<u8>,
        steps: Arc<AtomicUsize>,
        panic: bool,
        closing: bool,
    }

    impl InteractiveJob for HostileJob {
        fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
            let step = self.steps.fetch_add(1, AtomicOrdering::AcqRel);
            if self.panic {
                panic!("hostile worker panic");
            }
            if step == 0 {
                return StepOutcome::Yield;
            }
            let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, &[*self.backing]).expect("hostile output page");
            StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output })
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
            InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing
        }
    }

    #[test]
    fn worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let operation = OperationId(90_004);
        let generation = Generation(10);
        let steps = Arc::new(AtomicUsize::new(0));
        let session = WorkerJobSession::try_new(HostileJob { backing: Box::new(91), steps: Arc::clone(&steps), panic: false, closing: false }, params(operation, generation, root_cancel_token())).unwrap_or_else(|_| panic!("worker session slot"));
        let first = session.try_submit_step(&pool, Lane::Interactive).expect("first opportunity submitted");
        assert!(matches!(session.try_submit_step(&pool, Lane::Interactive), Err(WorkerJobSubmitFault::Contention(WorkerJobContention::Submitted(_)))));
        wait_for(&session, WorkerJobPoll::Outcome);
        let mut first_owner = session.take_outcome(first).expect("first exact outcome");
        assert!(matches!(first_owner.take_outcome(), StepOutcome::Yield));
        first_owner.resume().unwrap_or_else(|_| panic!("yield owner resumes exact generation"));
        let second = session.try_submit_step(&pool, Lane::Interactive).expect("second opportunity submitted");
        wait_for(&session, WorkerJobPoll::Terminal);
        let terminal = session.take_terminal().expect("terminal owner is take-only");
        let terminal_pointer = terminal.job().backing.as_ref() as *const u8;
        drop(terminal);
        let terminal = session.take_terminal().expect("dropped checkout hands exact terminal back");
        assert_eq!(terminal.job().backing.as_ref() as *const u8, terminal_pointer);
        assert_eq!(second.generation, generation);
        terminal.begin_close();
        while !session.terminal_is_empty() {
            let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
        assert_eq!(steps.load(AtomicOrdering::Acquire), 2);
        pool.shutdown();
    }

    #[test]
    fn worker_pool_rejection_returns_exact_job_before_resume() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        pool.shutdown();
        let backing = Box::new(33u8);
        let backing_pointer = backing.as_ref() as *const u8;
        let session = WorkerJobSession::try_new(HostileJob { backing, steps: Arc::new(AtomicUsize::new(0)), panic: false, closing: false }, params(OperationId(90_005), Generation(11), root_cancel_token())).unwrap_or_else(|_| panic!("worker session slot"));
        assert_eq!(session.try_submit_step(&pool, Lane::Interactive), Err(WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
        let rejected = session.take_rejected().expect("pool rejection retained exact owner");
        assert_eq!(rejected.job().backing.as_ref() as *const u8, backing_pointer);
        rejected.resume();
        assert_eq!(session.poll(), WorkerJobPoll::Idle);
        session.begin_close();
        while !session.terminal_is_empty() {
            let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
    }

    #[test]
    fn worker_panic_and_quiet_wake_publish_one_durable_terminal_intent() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let session = WorkerJobSession::try_new(HostileJob { backing: Box::new(1), steps: Arc::new(AtomicUsize::new(0)), panic: true, closing: false }, params(OperationId(90_006), Generation(12), root_cancel_token())).unwrap_or_else(|_| panic!("worker session slot"));
        session.register_wake(Waker::noop()).expect("quiet wake registration");
        let _ = session.try_submit_step(&pool, Lane::Interactive).expect("panic opportunity submitted");
        wait_for(&session, WorkerJobPoll::Terminal);
        assert!(session.take_wake());
        assert!(!session.take_wake(), "redundant quiet poll raises no wake");
        let terminal = session.take_terminal().expect("panic becomes retained terminal");
        assert!(matches!(terminal.outcome(), StepOutcome::Fault(_)));
        terminal.begin_close();
        while !session.terminal_is_empty() {
            let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
        pool.shutdown();
    }

    #[test]
    fn batch_session_advances_exactly_one_external_opportunity() {
        let steps = Arc::new(AtomicUsize::new(0));
        let mut batch = BatchJobSession::new(HostileJob { backing: Box::new(7), steps: Arc::clone(&steps), panic: false, closing: false }, params(OperationId(90_007), Generation(13), root_cancel_token()));
        assert_eq!(batch.step(), Ok(WorkerJobPoll::Outcome));
        assert_eq!(steps.load(AtomicOrdering::Acquire), 1);
        assert!(matches!(batch.take_outcome(), Some(StepOutcome::Yield)));
        batch.resume().expect("caller explicitly resumes after first opportunity");
        assert_eq!(steps.load(AtomicOrdering::Acquire), 1, "batch adapter never drains itself to terminal");
        batch.begin_close();
        while !batch.terminal_is_empty() {
            let _ = batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
    }

    #[test]
    fn worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned() {
        let mut sessions = Vec::with_capacity(WORKER_JOB_SESSION_SLOTS);
        for index in 0..WORKER_JOB_SESSION_SLOTS {
            let job = HostileJob { backing: Box::new(index as u8), steps: Arc::new(AtomicUsize::new(0)), panic: false, closing: false };
            sessions.push(WorkerJobSession::try_new(job, params(OperationId(91_000 + index as u64), Generation(index as u64 + 1), root_cancel_token())).unwrap_or_else(|_| panic!("each fixed session slot admits once")));
        }
        let rejected_backing = Box::new(211u8);
        let rejected_pointer = rejected_backing.as_ref() as *const u8;
        let rejected = match WorkerJobSession::try_new(
            HostileJob { backing: rejected_backing, steps: Arc::new(AtomicUsize::new(0)), panic: false, closing: false },
            params(OperationId(92_000), Generation(500), root_cancel_token()),
        ) {
            Ok(_) => panic!("session maximum plus one must retain exact rejected job"),
            Err(rejected) => rejected,
        };
        assert_eq!(rejected.job().backing.as_ref() as *const u8, rejected_pointer);
        let (rejected_job, _) = rejected.into_parts();
        assert_eq!(rejected_job.backing.as_ref() as *const u8, rejected_pointer);
        drop(rejected_job);
        let dropped = sessions.pop().expect("last fixed session");
        drop(dropped);
        assert!(take_worker_job_retirement_wake());
        for _ in 0..8 {
            let _ = pump_worker_job_retirements(1, 1, JOB_PAYLOAD_PAGE_BYTES);
        }
        let replacement = WorkerJobSession::try_new(
            HostileJob { backing: Box::new(17), steps: Arc::new(AtomicUsize::new(0)), panic: false, closing: false },
            params(OperationId(92_001), Generation(501), root_cancel_token()),
        )
        .unwrap_or_else(|_| panic!("retirement pump returns exact fixed session slot"));
        sessions.push(replacement);
        for session in sessions {
            let _ = session.begin_close();
            while !session.terminal_is_empty() {
                let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
            }
        }
    }
}
