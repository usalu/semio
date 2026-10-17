//! 🛰️ Operations and trace of the trace primitives: the operation, generation and trace identifiers, the trace stage and event and the bounded trace event ring with its latency queries. See [`super`].

use super::*;

//#region 🪪️Operation
/// 🔖️ Opaque per-interactive-operation id — a local, pack-free stand-in distinct from
/// `semio_framework_async::TraceId`/`OperationContext` (same seam discipline: this crate must not
/// depend on the async crate — see that crate's `CapabilityTokenId` doc for the precedent). Allocated
/// via [`allocate_operation_id`], or constructed directly by a caller/test that already owns an id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationId(pub u64);

/// 🔖️ Which retry/replay attempt of an [`OperationId`] a [`TraceEvent`] belongs to — bumped every time
/// the same logical operation restarts (a cancelled-then-retried interactive drag, a checkpoint replay).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Generation(pub u64);

/// 🔖️ Opaque cross-cutting trace correlation id, orthogonal to [`OperationId`] — meant to carry a whole
/// input→frame→operation→preview→commit chain that may span several operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TraceId(pub u64);

static NEXT_OPERATION_ID: AtomicU64 = AtomicU64::new(1);

/// 🌱️ A fresh, process-unique [`OperationId`] (never `0`, leaving `OperationId(0)` free for a caller's
/// own "no operation" sentinel).
pub fn allocate_operation_id() -> OperationId {
    OperationId(NEXT_OPERATION_ID.fetch_add(1, Ordering::SeqCst))
}

/// 🎰️ A fresh, process-unique [`OperationId`] congruent to `slot` modulo `slots`. Direct-mapped fixed
/// owner tables address one live owner per residue class of `id % slots`; minting the id first and then
/// testing that one residue class turns an unrelated live owner into a spurious capacity refusal, so the
/// admitting side picks the free slot and mints the id for it here instead. The shared counter still only
/// ever moves forward, so every id stays process-unique and never `0`.
pub fn allocate_operation_id_in_slot(slots: u64, slot: u64) -> OperationId {
    assert!(slots != 0 && slot < slots, "an operation slot must address one residue class of a fixed owner table");
    let advance = |next: u64| next + (slots + slot - next % slots) % slots;
    OperationId(advance(NEXT_OPERATION_ID.try_update(Ordering::SeqCst, Ordering::SeqCst, |next| Some(advance(next) + 1)).expect("operation id allocation never rejects its own update")))
}
//#endregion 🪪️Operation

//#region 🛰️Trace
/// 🛰️ What stage of an operation's lifecycle a [`TraceEvent`] marks. [`TraceStage::StageChanged`]
/// carries a `&'static str` label rather than growing a new variant per stage name —
/// [`CANCEL_REQUESTED_STAGE_LABEL`] is the one label this crate itself gives meaning to (for
/// [`cancellation_latency_us`]); every other label is caller-defined.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceStage {
    Started,
    StageChanged { label: &'static str },
    PreviewPublished,
    Checkpoint,
    Committed,
    Cancelled,
    Failed,
}

/// 🏷️ The [`TraceStage::StageChanged`] label [`record_cancel_requested`] uses, so
/// [`cancellation_latency_us`] can find "cancel requested" without a dedicated [`TraceStage`] variant.
pub const CANCEL_REQUESTED_STAGE_LABEL: &str = "cancel_requested";

/// 🛰️ One entry in the process-wide operation-trace ring: an [`OperationId`]/[`Generation`] pair, a
/// monotonic `sequence` (for total order even when two events share a microsecond), the [`TraceStage`],
/// and the [`now_us`] timestamp it was recorded at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraceEvent {
    pub operation: OperationId,
    pub generation: Generation,
    pub sequence: u64,
    pub stage: TraceStage,
    pub at_us: u64,
}

/// 🛰️ Ring capacity for the process-wide trace-event store.
const TRACE_RING_CAPACITY: usize = 4096;

static NEXT_SEQUENCE: AtomicU64 = AtomicU64::new(1);

pub(super) fn trace_ring() -> &'static StaticTryGate<BoundedRing<TraceEvent, TRACE_RING_CAPACITY>> {
    static RING: StaticTryGate<BoundedRing<TraceEvent, TRACE_RING_CAPACITY>> = StaticTryGate::new(BoundedRing::new());
    &RING
}

fn push_trace_event(operation: OperationId, generation: Generation, stage: TraceStage) -> Option<TraceEvent> {
    record_trace_event_at(operation, generation, stage, try_now_us())
}

/// 🛰️ Records one trace event stamped with a reading its caller already holds — a step driver's
/// entry or exit reading, a job's latest in-step reading — instead of reading the clock for it.
pub fn record_trace_event_at(operation: OperationId, generation: Generation, stage: TraceStage, at_us: Option<u64>) -> Option<TraceEvent> {
    let Some(at_us) = at_us else {
        OMITTED_EVENTS.fetch_add(1, Ordering::Relaxed);
        return None;
    };
    let event = TraceEvent { operation, generation, sequence: NEXT_SEQUENCE.fetch_add(1, Ordering::SeqCst), stage, at_us };
    if let Some(mut ring) = trace_ring().try_lock() {
        ring.push(event);
    } else {
        OMITTED_EVENTS.fetch_add(1, Ordering::Relaxed);
    }
    Some(event)
}

pub fn record_operation_started(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::Started)
}

pub fn record_stage_changed(operation: OperationId, generation: Generation, label: &'static str) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::StageChanged { label })
}

/// 🏷️ Convenience over [`record_stage_changed`] with [`CANCEL_REQUESTED_STAGE_LABEL`] — the start point
/// [`cancellation_latency_us`] measures from.
pub fn record_cancel_requested(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    record_stage_changed(operation, generation, CANCEL_REQUESTED_STAGE_LABEL)
}

pub fn record_preview_published(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::PreviewPublished)
}

pub fn record_checkpoint(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::Checkpoint)
}

pub fn record_committed(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::Committed)
}

/// 🏁️ The terminal "cancel observed" event — the end point [`cancellation_latency_us`] measures to.
pub fn record_cancelled(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::Cancelled)
}

pub fn record_failed(operation: OperationId, generation: Generation) -> Option<TraceEvent> {
    push_trace_event(operation, generation, TraceStage::Failed)
}

/// 📷️ Optional nonblocking event snapshot, oldest first, empty during another telemetry borrow.
pub fn trace_snapshot() -> Vec<TraceEvent> {
    trace_ring().try_lock().map_or_else(Vec::new, |ring| ring.snapshot())
}

/// 📸️ [`trace_snapshot`] filtered to one operation, in recorded order — the end-to-end
/// input/frame/operation/preview/commit trail later phases follow by id.
pub fn trace_snapshot_for(operation: OperationId) -> Vec<TraceEvent> {
    trace_snapshot().into_iter().filter(|event| event.operation == operation).collect()
}

fn stage_changed_label(stage: &TraceStage) -> Option<&'static str> {
    match stage {
        TraceStage::StageChanged { label } => Some(label),
        _ => None,
    }
}

/// ⏱️ Microseconds from `operation`'s [`TraceStage::Started`] event to its first
/// [`TraceStage::PreviewPublished`] event, or `None` if either hasn't happened yet.
pub fn preview_latency_us(operation: OperationId) -> Option<u64> {
    let events = trace_snapshot_for(operation);
    let started = events.iter().find(|event| event.stage == TraceStage::Started)?;
    let preview = events.iter().find(|event| event.stage == TraceStage::PreviewPublished)?;
    Some(preview.at_us.saturating_sub(started.at_us))
}

/// ⏱️ Microseconds from `operation`'s cancel-requested [`TraceStage::StageChanged`] (see
/// [`record_cancel_requested`]) to its terminal [`TraceStage::Cancelled`] event, or `None` if either
/// hasn't happened yet.
pub fn cancellation_latency_us(operation: OperationId) -> Option<u64> {
    let events = trace_snapshot_for(operation);
    let requested = events.iter().find(|event| stage_changed_label(&event.stage) == Some(CANCEL_REQUESTED_STAGE_LABEL))?;
    let observed = events.iter().find(|event| event.stage == TraceStage::Cancelled)?;
    Some(observed.at_us.saturating_sub(requested.at_us))
}
//#endregion 🛰️Trace
