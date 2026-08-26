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

//#region 🗄️FixedOperationRegistry
/// 🗄️ Typed retained owner admitted to a fixed operation scheduler.
pub trait FixedOperationOwner {
    fn retained_bytes(&self) -> usize;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep;
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

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.allocation_admitted || maximum_items == 0 {
            return InteractiveJobCloseStep::Blocked;
        }
        let index = self.close_cursor;
        self.close_cursor = (self.close_cursor + 1) % CAPACITY;
        let Some(entry) = self.slots[index].as_mut() else {
            return if self.is_empty() { InteractiveJobCloseStep::Complete } else { InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 } };
        };
        if !entry.closing {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let step = entry.owner.close_step(1, maximum_bytes);
        if entry.owner.terminal_is_empty() {
            let entry = self.slots[index].take().expect("terminal fixed operation owner remains admitted");
            self.retained_bytes -= entry.admitted_bytes;
            self.occupied -= 1;
            drop(entry);
        }
        if self.is_empty() {
            InteractiveJobCloseStep::Complete
        } else {
            match step {
                InteractiveJobCloseStep::Complete => InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 },
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
mod fixed_operation_registry_tests {
    use super::*;

    #[derive(Debug)]
    struct Owner {
        identity: u64,
        fixture_label: Option<&'static str>,
        bytes: Vec<u8>,
        cancelled: bool,
        closing: bool,
    }

    impl Owner {
        fn new(identity: u64, bytes: usize) -> Self {
            Self { identity, fixture_label: None, bytes: vec![0; bytes], cancelled: false, closing: false }
        }

        fn fixture(identity: u64, label: &'static str, bytes: usize) -> Self {
            Self { identity, fixture_label: Some(label), bytes: vec![0; bytes], cancelled: false, closing: false }
        }

        fn close_all(&mut self) {
            self.begin_close();
            for _ in 0..16 {
                let _ = self.close_step(1, 1);
                if self.terminal_is_empty() {
                    return;
                }
            }
            panic!("fixture owner did not close within its declared fixed bound");
        }
    }

    impl FixedOperationOwner for Owner {
        fn retained_bytes(&self) -> usize {
            self.bytes.len()
        }

        fn cancel(&mut self) {
            self.cancelled = true;
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
            if !self.closing || maximum_items == 0 || maximum_bytes == 0 {
                return InteractiveJobCloseStep::Blocked;
            }
            if self.bytes.pop().is_some() {
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 1 };
            }
            InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.bytes.is_empty()
        }
    }

    impl Drop for Owner {
        fn drop(&mut self) {
            assert!(self.terminal_is_empty(), "fixture owner was dropped before terminal-empty");
        }
    }

    fn drain<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>) {
        for _ in 0..64 {
            let _ = registry.close_step(1, 1);
            if registry.is_empty() {
                return;
            }
        }
        panic!("fixed operation registry did not close within capacity × owner bound");
    }

    fn fixture_admit<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>, operation: OperationId, generation: Generation, bytes: usize, identity: u64, label: &'static str) {
        match registry.admit(FixedOperationKey::new(operation, generation), Owner::fixture(identity, label, bytes)) {
            Ok(()) => output.push(format!("admit:accepted:{label}")),
            Err(mut rejected) => {
                assert_eq!(rejected.owner.identity, identity);
                assert_eq!(rejected.owner.fixture_label, Some(label));
                output.push(format!("admit:rejected:{label}"));
                rejected.owner.close_all();
            }
        }
    }

    fn fixture_take<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>, operation: OperationId, generation: Generation) {
        match registry.take(FixedOperationKey::new(operation, generation)) {
            Some(mut owner) => {
                output.push(format!("take:{}", owner.fixture_label.expect("fixture owner label")));
                owner.close_all();
            }
            None => output.push("take:none".into()),
        }
    }

    fn fixture_cancel<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>, operation: OperationId, generation: Generation) {
        output.push(format!("cancel:{}", registry.cancel(FixedOperationKey::new(operation, generation))));
    }

    fn fixture_cancel_stale<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>, operation: OperationId, live_generation: Generation) {
        output.push(format!("stale:{}", registry.cancel_stale_step(operation, live_generation)));
    }

    fn fixture_close<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>, maximum_items: usize, maximum_bytes: usize) {
        let state = match registry.close_step(maximum_items, maximum_bytes) {
            InteractiveJobCloseStep::Blocked => "blocked",
            InteractiveJobCloseStep::Pending { .. } => "pending",
            InteractiveJobCloseStep::Complete => "complete",
        };
        output.push(format!("close:{state}"));
    }

    fn fixture_inspect<const CAPACITY: usize>(registry: &FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>) {
        let remaining = registry.slots.iter().filter_map(Option::as_ref).map(|entry| entry.owner.bytes.len()).sum::<usize>();
        output.push(format!("state:{}:{}:{remaining}", registry.occupied, registry.retained_bytes));
    }

    fn fixture_assert<const CAPACITY: usize>(id: &str, registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: Vec<String>, expected: &[&str]) {
        assert_eq!(output, expected, "language-neutral fixed operation case {id}");
        assert!(registry.is_empty(), "language-neutral fixed operation case {id} retained an owner");
    }

    include!("🧪️fixtures/fixed-operation-registry-cases.rs");

    #[test]
    fn maximum_plus_one_and_saturation_return_the_exact_owner() {
        let mut registry = FixedOperationRegistry::<Owner, 2>::new(4);
        let key = FixedOperationKey::new(OperationId(1), Generation(7));
        registry.admit(key, Owner::new(11, 4)).expect("exact maximum");
        let mut byte_rejected = registry.admit(FixedOperationKey::new(OperationId(2), Generation(7)), Owner::new(12, 1)).expect_err("maximum plus one");
        assert_eq!(byte_rejected.owner.identity, 12);
        assert_eq!(byte_rejected.owner.bytes.len(), 1);
        byte_rejected.owner.close_all();
        let mut collision_rejected = registry.admit(FixedOperationKey::new(OperationId(3), Generation(7)), Owner::new(13, 0)).expect_err("fixed-slot collision");
        assert_eq!(collision_rejected.owner.identity, 13);
        collision_rejected.owner.close_all();
        assert!(registry.cancel(key));
        drain(&mut registry);
        assert_eq!(registry.retained_bytes(), 0);

        let mut full = FixedOperationRegistry::<Owner, 2>::new(8);
        let first = FixedOperationKey::new(OperationId(0), Generation(0));
        let second = FixedOperationKey::new(OperationId(1), Generation(0));
        full.admit(first, Owner::new(14, 1)).expect("first distinct slot");
        full.admit(second, Owner::new(15, 1)).expect("exact fixed capacity");
        let mut capacity_rejected = full.admit(FixedOperationKey::new(OperationId(2), Generation(0)), Owner::new(16, 1)).expect_err("fixed capacity plus one");
        assert_eq!(capacity_rejected.owner.identity, 16);
        capacity_rejected.owner.close_all();
        assert!(full.cancel(first));
        assert!(full.cancel(second));
        drain(&mut full);
    }

    #[test]
    fn stale_generation_interrupted_close_and_aba_preserve_exact_authority() {
        let mut registry = FixedOperationRegistry::<Owner, 4>::new(8);
        let stale = FixedOperationKey::new(OperationId(9), Generation(1));
        registry.admit(stale, Owner::new(21, 2)).expect("stale owner");
        let (observed_key, observed) = registry.get_operation(OperationId(9)).expect("operation-owned lookup");
        assert_eq!(observed_key, stale);
        assert_eq!(observed.identity, 21);
        let (_, observed_mut) = registry.get_operation_mut(OperationId(9)).expect("mutable operation-owned lookup");
        observed_mut.identity = 23;
        assert_eq!(registry.get(stale).expect("same exact owner").identity, 23);
        assert!(registry.get_operation(OperationId(10)).is_none(), "another operation cannot observe the retained owner");
        assert!(registry.take(FixedOperationKey::new(OperationId(9), Generation(2))).is_none());
        for _ in 0..4 {
            let _ = registry.cancel_stale_step(OperationId(9), Generation(2));
        }
        let _ = registry.close_step(1, 1);
        assert!(!registry.is_empty(), "interrupted close must retain the exact owner");
        drain(&mut registry);
        let fresh = FixedOperationKey::new(OperationId(9), Generation(2));
        registry.admit(fresh, Owner::new(22, 0)).expect("fresh ABA generation");
        assert!(registry.take(stale).is_none());
        let mut owner = registry.take(fresh).expect("exact accepted owner handback");
        assert_eq!(owner.identity, 22);
        owner.close_all();
        assert!(matches!(registry.close_step(1, 1), InteractiveJobCloseStep::Complete));
        assert!(matches!(registry.close_step(1, 1), InteractiveJobCloseStep::Complete));
    }

    #[test]
    fn maximum_registry_backing_initializes_inside_one_interactive_ceiling_under_concurrent_load() {
        const WORKERS: usize = 4;
        const SAMPLES: usize = 31;
        let barrier = Arc::new(std::sync::Barrier::new(WORKERS));
        let mut workers = Vec::with_capacity(WORKERS);
        for _ in 0..WORKERS {
            let barrier = barrier.clone();
            workers.push(std::thread::spawn(move || {
                barrier.wait();
                let mut elapsed = [0_u128; SAMPLES];
                for sample in &mut elapsed {
                    let started = Instant::now();
                    let registry = FixedOperationRegistry::<Owner, 64>::new(4_096);
                    *sample = started.elapsed().as_micros();
                    assert!(registry.allocation_admitted);
                    drop(registry);
                }
                elapsed.sort_unstable();
                elapsed[SAMPLES / 2]
            }));
        }
        for worker in workers {
            let median = worker.join().expect("concurrent fixed registry initialization worker");
            assert!(median < u128::from(semio_framework_trace::INTERACTIVE_STEP_CEILING_US), "fixed registry median backing initialization exceeded the interactive ceiling under concurrent load: {median}us");
        }
    }
}
//#endregion 🗄️FixedOperationRegistry

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
pub const JOB_PAYLOAD_OPERATION_PAGES: usize = 256;
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

    fn reserve(&self, stream: JobPayloadStream) -> Result<(), JobPayloadAdmissionFault> {
        let stream_index = stream as usize;
        let pages = self.pages.try_update(Ordering::AcqRel, Ordering::Acquire, |pages| pages.checked_add(1).filter(|pages| *pages <= JOB_PAYLOAD_OPERATION_PAGES)).map_err(|_| JobPayloadAdmissionFault::OperationItems)?;
        if self.bytes.try_update(Ordering::AcqRel, Ordering::Acquire, |bytes| bytes.checked_add(JOB_PAYLOAD_PAGE_BYTES).filter(|bytes| *bytes <= JOB_PAYLOAD_OPERATION_BYTES)).is_err() {
            self.pages.store(pages, Ordering::Release);
            return Err(JobPayloadAdmissionFault::OperationBytes);
        }
        if self.stream_pages[stream_index].try_update(Ordering::AcqRel, Ordering::Acquire, |pages| pages.checked_add(1).filter(|pages| *pages <= JOB_PAYLOAD_OPERATION_PAGES)).is_err() {
            self.bytes.fetch_sub(JOB_PAYLOAD_PAGE_BYTES, Ordering::AcqRel);
            self.pages.fetch_sub(1, Ordering::AcqRel);
            return Err(JobPayloadAdmissionFault::StreamItems);
        }
        if self.stream_bytes[stream_index].try_update(Ordering::AcqRel, Ordering::Acquire, |bytes| bytes.checked_add(JOB_PAYLOAD_PAGE_BYTES).filter(|bytes| *bytes <= JOB_PAYLOAD_OPERATION_BYTES)).is_err() {
            self.stream_pages[stream_index].fetch_sub(1, Ordering::AcqRel);
            self.bytes.fetch_sub(JOB_PAYLOAD_PAGE_BYTES, Ordering::AcqRel);
            self.pages.fetch_sub(1, Ordering::AcqRel);
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        if JOB_PAYLOAD_PROCESS_OWNED_BYTES.try_update(Ordering::AcqRel, Ordering::Acquire, |bytes| bytes.checked_add(JOB_PAYLOAD_PAGE_BYTES).filter(|bytes| *bytes <= JOB_PAYLOAD_PROCESS_BYTES)).is_err() {
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

    pub fn single_page(&self) -> Option<&[u8]> {
        (self.page_count == 1).then(|| self.page(0)).flatten()
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
        }
        JobPayloadCloseStep::Pending { released_items: 1, released_bytes }
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
    payload: ManuallyDrop<Option<RetainedJobPayload>>,
    rejected: ManuallyDrop<Option<JobPayloadPageSource>>,
    staged: ManuallyDrop<Option<(Arc<JobPayloadOperationLedger>, JobPayloadPageSource, usize)>>,
    sealed: bool,
}

impl std::fmt::Debug for RetainedJobPayloadWriter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("RetainedJobPayloadWriter").field("payload", &self.payload).field("rejected", &self.rejected).field("staged", &self.staged.as_ref().map(|(_, _, length)| length)).field("sealed", &self.sealed).finish()
    }
}

impl RetainedJobPayloadWriter {
    pub fn new(stream: JobPayloadStream) -> Self {
        Self { payload: ManuallyDrop::new(Some(RetainedJobPayload::empty(stream))), rejected: ManuallyDrop::new(None), staged: ManuallyDrop::new(None), sealed: false }
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
        if let Err(fault) = self.reserve_page(&ledger) {
            *self.rejected = Some(source);
            return Err(fault);
        }
        cx.payload_page_granted = true;
        Ok(self.begin_page(ledger, source))
    }

    pub fn finish(mut self) -> Result<RetainedJobPayload, Self> {
        if self.rejected.is_some() || self.staged.is_some() {
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
        if let Some((ledger, _, length)) = self.staged.as_ref() {
            if maximum_items == 0 || maximum_bytes < *length {
                return JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let stream = self.payload.as_ref().expect("retained payload writer owns payload while staged page exists").stream;
            ledger.release(stream);
            let (_, source, released_bytes) = self.staged.take().expect("staged page remains owned until exact close");
            drop(source);
            return JobPayloadCloseStep::Pending { released_items: 1, released_bytes };
        }
        if self.rejected.is_some() {
            if maximum_items == 0 {
                return JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            *self.rejected = None;
            return JobPayloadCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let Some(payload) = self.payload.as_mut() else { return JobPayloadCloseStep::Complete };
        if !payload.terminal_is_empty() {
            return payload.close_step(maximum_items, maximum_bytes);
        }
        if maximum_items == 0 {
            return JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        *self.payload = None;
        JobPayloadCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.sealed && self.rejected.is_none() && self.staged.is_none() && self.payload.is_none()
    }

    pub fn begin_staged_page(&mut self, cx: &mut StepContext<'_>) -> Result<(), JobPayloadAdmissionFault> {
        if self.staged.is_some() {
            return Ok(());
        }
        self.admit_page(cx)?.stage();
        Ok(())
    }

    pub fn staged_page_remaining(&self) -> usize {
        self.staged.as_ref().map_or(0, |(_, _, length)| JOB_PAYLOAD_PAGE_BYTES - length)
    }

    pub fn staged_page_len(&self) -> Option<usize> {
        self.staged.as_ref().map(|(_, _, length)| *length)
    }

    pub fn write_staged(&mut self, bytes: &[u8]) -> Result<(), JobPayloadAdmissionFault> {
        let (_, source, length) = self.staged.as_mut().ok_or(JobPayloadAdmissionFault::OpportunityExhausted)?;
        if bytes.len() > JOB_PAYLOAD_PAGE_BYTES - *length {
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
        payload.ledger = Some(ledger);
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
        let end = cursor.saturating_add(JOB_PAYLOAD_PAGE_BYTES).min(bytes.len());
        page.write(&bytes[*cursor..end])?;
        page.commit();
        *cursor = end;
        Ok(*cursor == bytes.len())
    }

    fn reserve_page(&self, ledger: &JobPayloadOperationLedger) -> Result<(), JobPayloadAdmissionFault> {
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
        ledger.reserve(payload.stream)
    }

    fn begin_page(&mut self, ledger: Arc<JobPayloadOperationLedger>, source: JobPayloadPageSource) -> JobPayloadPageGrant<'_> {
        JobPayloadPageGrant { writer: self, ledger: Some(ledger), source: Some(source), length: 0, committed: false }
    }
}

impl Drop for RetainedJobPayloadWriter {
    fn drop(&mut self) {
        if self.payload.is_none() && self.rejected.is_none() && self.staged.is_none() {
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

    pub fn initialized_remaining_mut(&mut self) -> &mut [u8] {
        let source = self.source.as_mut().expect("uncommitted job payload grant owns page source");
        for byte in &mut source.storage[self.length..] {
            byte.write(0);
        }
        unsafe { std::slice::from_raw_parts_mut(source.storage.as_mut_ptr().cast::<u8>().add(self.length), JOB_PAYLOAD_PAGE_BYTES - self.length) }
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
        payload.ledger = self.ledger.take();
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
            ledger.release(stream);
        }
        *self.writer.rejected = self.source.take();
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
        assert_eq!(payload_ledger.operation, operation, "job payload ledger operation must match its step context");
        assert_eq!(payload_ledger.generation, generation, "job payload ledger generation must match its step context");
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
    /// across every [`StepContext`] built for the same retained session — one call per
    /// [`StepOutcome::PreviewReady`]/[`ProgressEvent::PreviewPatch`] a job emits.
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
        if let Err(fault) = writer.reserve_page(&ledger) {
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
    pump: unsafe fn(*mut JobChildNodeHeader, usize, usize) -> InteractiveJobCloseStep,
    destroy: unsafe fn(*mut JobChildNodeHeader),
}

#[repr(C)]
struct JobChildNode<J> {
    header: JobChildNodeHeader,
    child: Option<J>,
    close_stage: u8,
}

unsafe fn pump_job_child_node<J: InteractiveJob>(pointer: *mut JobChildNodeHeader, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
    let node = unsafe { &mut *pointer.cast::<JobChildNode<J>>() };
    if node.close_stage == 0 {
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        node.child.as_mut().expect("live child node owns exact child").begin_close();
        node.close_stage = 1;
        return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
    }
    if node.close_stage == 1 {
        let child = node.child.as_mut().expect("closing child node owns exact child");
        match child.close_step(maximum_items, maximum_bytes) {
            InteractiveJobCloseStep::Pending { released_items, released_bytes } => return InteractiveJobCloseStep::Pending { released_items, released_bytes },
            InteractiveJobCloseStep::Blocked => return InteractiveJobCloseStep::Blocked,
            InteractiveJobCloseStep::Complete if !child.terminal_is_empty() => return InteractiveJobCloseStep::Blocked,
            InteractiveJobCloseStep::Complete => {
                node.close_stage = 2;
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
        }
    }
    if node.close_stage == 2 {
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        drop(node.child.take());
        node.close_stage = 3;
        return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
    }
    InteractiveJobCloseStep::Complete
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

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.begin_close();
        if self.close_stage == 0 {
            let Some(child) = self.child.as_mut() else {
                self.close_stage = 2;
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            };
            match child.close_step(maximum_items, maximum_bytes) {
                InteractiveJobCloseStep::Pending { released_items, released_bytes } => return InteractiveJobCloseStep::Pending { released_items, released_bytes },
                InteractiveJobCloseStep::Blocked => return InteractiveJobCloseStep::Blocked,
                InteractiveJobCloseStep::Complete if !child.terminal_is_empty() => return InteractiveJobCloseStep::Blocked,
                InteractiveJobCloseStep::Complete => self.close_stage = 1,
            }
        }
        if self.close_stage == 1 {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            drop(self.child.take());
            self.close_stage = 2;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
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
            let node = Box::new(JobChildNode { header: JobChildNodeHeader { pump: pump_job_child_node::<J>, destroy: destroy_job_child_node::<J> }, child: child.take(), close_stage: 0 });
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

    pub fn pump_child_close(&self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        for slot in &self.slots {
            if slot.state.load(Ordering::Acquire) != CHILD_CLOSE_INTENT {
                continue;
            }
            let pointer = slot.node.load(Ordering::Acquire);
            if pointer.is_null() {
                return InteractiveJobCloseStep::Blocked;
            }
            let step = unsafe { ((*pointer).pump)(pointer, maximum_items, maximum_bytes) };
            if step == InteractiveJobCloseStep::Complete && slot.state.compare_exchange(CHILD_CLOSE_INTENT, CHILD_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                if maximum_items == 0 {
                    slot.state.store(CHILD_CLOSE_INTENT, Ordering::Release);
                    return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                let pointer = slot.node.swap(std::ptr::null_mut(), Ordering::AcqRel);
                unsafe { ((*pointer).destroy)(pointer) };
                self.live_children.fetch_sub(1, Ordering::AcqRel);
                slot.state.store(if slot.generation.load(Ordering::Acquire) == u64::MAX { CHILD_EXHAUSTED } else { CHILD_VACANT }, Ordering::Release);
                self.raise_wake();
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            return step;
        }
        InteractiveJobCloseStep::Complete
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
    preadmitted_fault: Option<RetainedJobPayload>,
    outcome: Option<StepOutcome>,
    close_stage: u8,
}

impl<J> WorkerJobAuthority<J> {
    fn try_new(job: J, params: BatchJobParams) -> Result<Self, (J, BatchJobParams, JobPayloadPageSource)> {
        let payload_ledger = Arc::new(JobPayloadOperationLedger::new(params.operation, params.generation));
        let fault_source = JobPayloadPageSource::new();
        let preadmitted_fault = match preadmitted_static_payload(&payload_ledger, JobPayloadStream::Fault, b"job-session.terminal-fault", fault_source) {
            Ok(payload) => payload,
            Err(fault_source) => return Err((job, params, fault_source)),
        };
        Ok(Self { job: Some(job), params: Some(params), preview_sequence: 0, step_sequence: 0, payload_ledger, preadmitted_fault: Some(preadmitted_fault), outcome: None, close_stage: 0 })
    }
}

fn preadmitted_static_payload(ledger: &Arc<JobPayloadOperationLedger>, stream: JobPayloadStream, bytes: &'static [u8], mut source: JobPayloadPageSource) -> Result<RetainedJobPayload, JobPayloadPageSource> {
    if bytes.len() > JOB_PAYLOAD_PAGE_BYTES || ledger.reserve(stream).is_err() {
        return Err(source);
    }
    for (target, byte) in source.storage.iter_mut().zip(bytes.iter().copied()) {
        target.write(byte);
    }
    let mut pages = std::array::from_fn(|_| None);
    pages[0] = Some(JobPayloadPage { source, length: bytes.len() });
    Ok(RetainedJobPayload { stream, pages: ManuallyDrop::new(pages), page_count: 1, length: bytes.len(), ledger: Some(Arc::clone(ledger)) })
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
    pub fn try_new(job: J, params: BatchJobParams) -> Result<Self, WorkerJobSessionAdmissionRejected<J>> {
        WorkerJobSession::try_new(job, params).map(|session| Self { session, ticket: None, checked_out: None })
    }

    pub fn generation(&self) -> Generation {
        self.session.generation()
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.checked_out.is_some() { WorkerJobPoll::CheckedOut } else { self.session.poll() }
    }

    pub fn pump_one(&mut self, pool: &WorkerPool, lane: Lane) -> Result<WorkerJobPoll, MountedWorkerJobPumpFault> {
        if self.checked_out.is_some() {
            return Err(MountedWorkerJobPumpFault::CheckedOut);
        }
        match self.session.poll() {
            WorkerJobPoll::Idle => {
                let ticket = self.session.try_submit_step(pool, lane).map_err(MountedWorkerJobPumpFault::Submit)?;
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

    pub fn checked_out_outcome(&self) -> Option<&StepOutcome> {
        self.checked_out.as_ref().map(WorkerJobOutcome::outcome)
    }

    pub fn take_checked_out_outcome(&mut self) -> Option<StepOutcome> {
        self.checked_out.as_mut().map(WorkerJobOutcome::take_outcome)
    }

    pub fn checked_out_job_mut(&mut self) -> Option<&mut J> {
        self.checked_out.as_mut().map(WorkerJobOutcome::job_mut)
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

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> WorkerJobCloseStep {
        if let Some(owner) = self.checked_out.take() {
            if maximum_items == 0 {
                self.checked_out = Some(owner);
                return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            owner.begin_close();
            return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.session.close_step(maximum_items, maximum_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.checked_out.is_none() && self.session.terminal_is_empty()
    }
}

impl<J: InteractiveJob + 'static> BatchJobSession<J> {
    pub fn try_new(job: J, params: BatchJobParams) -> Result<Self, WorkerJobSessionAdmissionRejected<J>> {
        WorkerJobSession::try_new(job, params).map(|session| Self { session, ticket: None, checked_out: None })
    }

    pub fn step(&mut self) -> Result<WorkerJobPoll, WorkerJobContention> {
        if self.checked_out.is_some() {
            return Err(WorkerJobContention::CheckedOut(self.session.generation()));
        }
        let (ticket, poll) = self.session.try_step_on_caller()?;
        self.ticket = Some(ticket);
        Ok(poll)
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.checked_out.is_some() { WorkerJobPoll::CheckedOut } else { self.session.poll() }
    }

    pub fn take_outcome(&mut self) -> Option<StepOutcome> {
        if !self.checkout_outcome() {
            return None;
        }
        self.checked_out.as_mut()?.authority.as_mut()?.outcome.take()
    }

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

    pub fn checked_out_outcome(&self) -> Option<&StepOutcome> {
        self.checked_out.as_ref()?.authority.as_ref()?.outcome.as_ref()
    }

    pub fn checked_out_job_mut(&mut self) -> Option<&mut J> {
        self.checked_out.as_mut()?.authority.as_mut()?.job.as_mut()
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

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> WorkerJobCloseStep {
        if let Some(owner) = self.checked_out.take() {
            if maximum_items == 0 {
                self.checked_out = Some(owner);
                return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            owner.begin_close();
            return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.session.close_step(maximum_items, maximum_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.checked_out.is_none() && self.session.terminal_is_empty()
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
    WORKER_JOB_RETIREMENT_SLOTS.iter().enumerate().find_map(|(index, slot)| slot.compare_exchange(std::ptr::null_mut(), WORKER_JOB_RETIREMENT_RESERVED, Ordering::AcqRel, Ordering::Acquire).ok().map(|_| index))
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
        if self.wake_sequence.try_update(Ordering::AcqRel, Ordering::Acquire, |sequence| sequence.checked_add(1)).is_err() {
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
        if self.wake_exhausted.load(Ordering::Acquire) {
            return Err(WorkerJobContention::WakeExhausted(self.generation));
        }
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
    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(job) = self.job.as_mut() {
            job.begin_close();
        }
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.begin_close();
        if self.close_stage == 0 {
            let Some(job) = self.job.as_mut() else {
                self.close_stage = 2;
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            };
            match job.close_step(maximum_items, maximum_bytes) {
                InteractiveJobCloseStep::Pending { released_items, released_bytes } => return InteractiveJobCloseStep::Pending { released_items, released_bytes },
                InteractiveJobCloseStep::Blocked => return InteractiveJobCloseStep::Blocked,
                InteractiveJobCloseStep::Complete if !job.terminal_is_empty() => return InteractiveJobCloseStep::Blocked,
                InteractiveJobCloseStep::Complete => self.close_stage = 1,
            }
        }
        if self.close_stage == 1 {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            drop(self.job.take());
            self.close_stage = 2;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.params.is_some() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            drop(self.params.take());
            self.close_stage = 3;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.fault_source.is_some() {
            if maximum_items == 0 || maximum_bytes < JOB_PAYLOAD_PAGE_BYTES {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            drop(self.fault_source.take());
            self.close_stage = 4;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES };
        }
        InteractiveJobCloseStep::Complete
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
    inner: Arc<WorkerJobSessionInner<J>>,
    authority: Option<WorkerJobAuthority<J>>,
    ran: bool,
}

fn drive_worker_job_authority<J: InteractiveJob>(authority: &mut WorkerJobAuthority<J>) -> bool {
    if authority.step_sequence == u64::MAX {
        authority.outcome = Some(StepOutcome::Fault(JobFault { detail: authority.preadmitted_fault.take().expect("worker session pre-admitted terminal fault page") }));
        return true;
    }
    let params = authority.params.as_ref().expect("submitted job authority owns parameters").clone();
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
        Err(_) => StepOutcome::Fault(JobFault { detail: authority.preadmitted_fault.take().expect("worker panic retains its pre-admitted terminal fault page") }),
    });
    authority.outcome.as_ref().is_some_and(StepOutcome::is_terminal)
}

impl<J: InteractiveJob + 'static> WorkerJobSubmission<J> {
    fn run(mut self) {
        self.ran = true;
        let mut authority = self.authority.take().expect("submitted worker closure owns exact job authority");
        let terminal = drive_worker_job_authority(&mut authority);
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
        return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
    }
    if let Some(fault) = authority.preadmitted_fault.as_mut() {
        if !fault.terminal_is_empty() {
            let step = match fault.close_step(maximum_items, maximum_bytes) {
                JobPayloadCloseStep::Pending { released_items, released_bytes } => WorkerJobCloseStep::Pending { released_items, released_bytes },
                JobPayloadCloseStep::Complete => WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 },
            };
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return step;
        }
        if maximum_items == 0 {
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        authority.preadmitted_fault = None;
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
    if maximum_items == 0 {
        unsafe { inner.put_authority(authority, SESSION_CLOSE) };
        return WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
    }
    drop(authority);
    inner.phase.store(SESSION_EMPTY, Ordering::Release);
    inner.raise_wake();
    WorkerJobCloseStep::Complete
}

impl<J: InteractiveJob + 'static> WorkerJobSession<J> {
    pub fn try_new(job: J, params: BatchJobParams) -> Result<Self, WorkerJobSessionAdmissionRejected<J>> {
        let Some(slot) = reserve_worker_job_retirement_slot() else {
            return Err(WorkerJobSessionAdmissionRejected { job: ManuallyDrop::new(Some(job)), params: ManuallyDrop::new(Some(params)), fault_source: ManuallyDrop::new(None), closing: false, close_stage: 0 });
        };
        let generation = params.generation;
        let operation = params.operation;
        let authority = match WorkerJobAuthority::try_new(job, params) {
            Ok(authority) => authority,
            Err((job, params, fault_source)) => {
                WORKER_JOB_RETIREMENT_SLOTS[slot].store(std::ptr::null_mut(), Ordering::Release);
                return Err(WorkerJobSessionAdmissionRejected { job: ManuallyDrop::new(Some(job)), params: ManuallyDrop::new(Some(params)), fault_source: ManuallyDrop::new(Some(fault_source)), closing: false, close_stage: 0 });
            }
        };
        record_operation_started(operation, generation);
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
        });
        let retirement = Box::new(WorkerJobRetirementNode { header: WorkerJobRetirementHeader { slot, pump: pump_worker_job_retirement_node::<J>, destroy: destroy_worker_job_retirement_node::<J> }, inner: None });
        Ok(Self { inner, retirement: std::cell::UnsafeCell::new(Some(retirement)), retirement_state: AtomicU8::new(0) })
    }

    pub fn generation(&self) -> Generation {
        self.inner.generation
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

    pub fn try_step_on_caller(&self) -> Result<(WorkerJobTicket, WorkerJobPoll), WorkerJobContention> {
        if self.inner.phase.compare_exchange(SESSION_IDLE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(self.contention());
        }
        let mut authority = unsafe { self.inner.take_authority() };
        let ticket = WorkerJobTicket { generation: self.inner.generation, step_sequence: authority.step_sequence };
        let terminal = drive_worker_job_authority(&mut authority);
        if self.inner.close_requested.load(Ordering::Acquire) {
            self.inner.terminal_intent.store(1, Ordering::Release);
        }
        unsafe { self.inner.put_authority(authority, if terminal { SESSION_TERMINAL } else { SESSION_OUTCOME }) };
        Ok((ticket, if terminal { WorkerJobPoll::Terminal } else { WorkerJobPoll::Outcome }))
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
        if self.restore_phase == SESSION_TERMINAL {
            return Err(self);
        }
        let authority = self.authority.as_ref().expect("checked-out worker outcome owns authority");
        if authority.outcome.as_ref().is_some_and(|outcome| !outcome.terminal_is_empty() || outcome.is_terminal()) {
            return Err(self);
        }
        let authority = self.authority.take().expect("checked-out worker outcome owns authority");
        unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
        Ok(())
    }

    pub fn begin_close(mut self) {
        let mut authority = self.authority.take().expect("checked-out worker outcome owns authority");
        if authority.outcome.as_ref().is_some_and(StepOutcome::terminal_is_empty) {
            authority.outcome = None;
        }
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
        TortureJob {
            total_units,
            completed_units,
            rng_state,
            accumulator,
            checkpoint_every_units,
            preview_every_units,
            units_since_checkpoint: 0,
            units_since_preview: 0,
            terminal_state: None,
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
        match self.scope.pump_child_close(maximum_items, maximum_bytes) {
            InteractiveJobCloseStep::Complete => {}
            step => return step,
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
    fn payload_ledger_identity_must_match_the_exact_step_context() {
        let ledger = Arc::new(JobPayloadOperationLedger::new(OperationId(90_000), Generation(6)));
        let operation_mismatch = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut sequence = 0;
            let _ = StepContext::with_payload_ledger(OperationId(90_001), Generation(6), StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_ms, &mut sequence, Arc::clone(&ledger));
        }));
        assert!(operation_mismatch.is_err());
        let generation_mismatch = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut sequence = 0;
            let _ = StepContext::with_payload_ledger(OperationId(90_000), Generation(7), StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_ms, &mut sequence, Arc::clone(&ledger));
        }));
        assert!(generation_mismatch.is_err());
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
        assert_eq!(terminal.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Pending { released_items: 1, released_bytes: 5 });
        assert!(!terminal.terminal_is_empty());
        assert_eq!(terminal.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Pending { released_items: 1, released_bytes: 6 });
        assert!(terminal.terminal_is_empty());
    }

    #[test]
    fn retained_writer_and_reader_advance_exactly_one_page_per_opportunity() {
        let operation = OperationId(90_008);
        let generation = Generation(14);
        let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
        let bytes = vec![19u8; JOB_PAYLOAD_PAGE_BYTES + 1];
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput);
        let mut cursor = 0;
        let mut sequence = 0;
        let mut zero = StepContext::with_payload_ledger(operation, generation, StepBudget::new(0, u64::MAX), root_cancel_token(), default_now_ms, &mut sequence, Arc::clone(&ledger));
        assert_eq!(writer.write_slice_page(&mut zero, &bytes, &mut cursor), Ok(false));
        assert_eq!(cursor, 0);
        let mut first = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_ms, &mut sequence, Arc::clone(&ledger));
        assert_eq!(writer.write_slice_page(&mut first, &bytes, &mut cursor), Ok(false));
        assert_eq!(cursor, JOB_PAYLOAD_PAGE_BYTES);
        let mut second = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_ms, &mut sequence, Arc::clone(&ledger));
        assert_eq!(writer.write_slice_page(&mut second, &bytes, &mut cursor), Ok(true));
        let mut payload = writer.finish().expect("two-page retained payload");
        let mut reader = payload.reader();
        assert_eq!(reader.read_page(0, JOB_PAYLOAD_PAGE_BYTES), None);
        assert_eq!(reader.read_page(1, JOB_PAYLOAD_PAGE_BYTES).map(|page| page.len()), Some(JOB_PAYLOAD_PAGE_BYTES));
        assert_eq!(reader.read_page(1, JOB_PAYLOAD_PAGE_BYTES).map(|page| page.len()), Some(1));
        assert!(reader.terminal_is_empty());
        assert_eq!(payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Pending { released_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES });
        assert_eq!(payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Pending { released_items: 1, released_bytes: 1 });
        assert_eq!(payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Complete);
        assert!(payload.terminal_is_empty());
    }

    struct StructuredChildJob {
        backing: Option<Box<u8>>,
        closing: bool,
    }

    impl InteractiveJob for StructuredChildJob {
        fn step(&mut self, _cx: &mut StepContext<'_>) -> StepOutcome {
            StepOutcome::Yield
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
            self.begin_close();
            if self.backing.is_some() {
                if maximum_items == 0 {
                    return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                self.backing = None;
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.backing.is_none()
        }
    }

    #[test]
    fn child_registry_max_plus_one_stale_duplicate_exhaustion_and_parent_completion_are_exact() {
        let scope = JobScope::for_operation(&root_cancel_token(), OperationId(90_003), Generation(9));
        let mut guards: [Option<ChildJobGuard<'_, StructuredChildJob>>; JOB_CHILD_SLOTS] =
            std::array::from_fn(|index| Some(scope.spawn_child(StructuredChildJob { backing: Some(Box::new(index as u8)), closing: false }).unwrap_or_else(|_| panic!("fixed child slot"))));
        let retained_child_pointer = guards[1].as_mut().expect("second structured child").with_child_mut(|child| child.backing.as_deref().expect("retained structured child backing") as *const u8).expect("generation-qualified child checkout");
        assert_eq!(unsafe { *retained_child_pointer }, 1);
        let plus_one_backing = Box::new(211u8);
        let plus_one_pointer = plus_one_backing.as_ref() as *const u8;
        let rejected = match scope.spawn_child(StructuredChildJob { backing: Some(plus_one_backing), closing: false }) {
            Ok(_) => panic!("child maximum plus one must be rejected"),
            Err(rejected) => rejected,
        };
        assert_eq!(rejected.fault, JobChildAdmissionFault::Capacity);
        assert_eq!(rejected.child().backing.as_deref().expect("rejected structured child backing") as *const u8, plus_one_pointer);
        let mut rejected_child = rejected.into_child();
        assert_eq!(rejected_child.backing.as_deref().expect("returned structured child backing") as *const u8, plus_one_pointer);
        rejected_child.begin_close();
        assert_eq!(rejected_child.close_step(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        while !rejected_child.terminal_is_empty() {
            let _ = rejected_child.close_step(1, 0);
        }
        assert_eq!(scope.assert_completable(), Err(JobChildCompletionFault::LiveChildren));
        let token = guards[0].as_ref().expect("first child").token();
        guards[0].take().expect("first child").complete().expect("first exact completion");
        assert_eq!(scope.complete_child(token), Err(JobChildCompletionFault::Duplicate));
        let stale = JobChildToken { generation: token.generation + 1, ..token };
        assert_eq!(scope.complete_child(stale), Err(JobChildCompletionFault::Stale));
        drop(guards);
        assert_eq!(scope.pump_child_close(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(scope.pump_child_close(1, JOB_PAYLOAD_PAGE_BYTES), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }, "child begin-close transfers control without claiming an owner release");
        while !scope.terminal_is_empty() {
            let _ = scope.pump_child_close(1, JOB_PAYLOAD_PAGE_BYTES);
        }
        assert!(scope.assert_completable().is_ok());
        for slot in &scope.slots {
            slot.generation.store(u64::MAX, Ordering::Release);
            slot.state.store(CHILD_VACANT, Ordering::Release);
        }
        let rejected_backing = Box::new(7);
        let rejected_backing_pointer = rejected_backing.as_ref() as *const u8;
        let mut rejected = match scope.spawn_child(StructuredChildJob { backing: Some(rejected_backing), closing: false }) {
            Ok(_) => panic!("exhausted child generation must reject"),
            Err(rejected) => rejected,
        };
        assert_eq!(rejected.fault, JobChildAdmissionFault::Exhausted);
        assert_eq!(rejected.child().backing.as_deref().expect("exhausted rejected backing") as *const u8, rejected_backing_pointer);
        rejected.begin_close();
        assert_eq!(rejected.close_step(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        while !rejected.terminal_is_empty() {
            let _ = rejected.close_step(1, 0);
        }
        scope.begin_close();
        assert!(scope.terminal_is_empty());
    }

    struct HostileJob {
        backing: Option<Box<u8>>,
        steps: Option<Arc<AtomicUsize>>,
        panic: bool,
        closing: bool,
    }

    impl InteractiveJob for HostileJob {
        fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
            let step = self.steps.as_ref().expect("hostile step counter").fetch_add(1, AtomicOrdering::AcqRel);
            if self.panic {
                panic!("hostile worker panic");
            }
            if step == 0 {
                return StepOutcome::Yield;
            }
            let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, &[**self.backing.as_ref().expect("hostile backing")]).expect("hostile output page");
            StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output })
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
            self.begin_close();
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if self.backing.take().is_some() {
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if self.steps.take().is_some() {
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.backing.is_none() && self.steps.is_none()
        }
    }

    #[test]
    fn worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let operation = OperationId(90_004);
        let generation = Generation(10);
        let steps = Arc::new(AtomicUsize::new(0));
        let session =
            WorkerJobSession::try_new(HostileJob { backing: Some(Box::new(91)), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(operation, generation, root_cancel_token())).unwrap_or_else(|_| panic!("worker session slot"));
        let first = session.try_submit_step(&pool, Lane::Interactive).expect("first opportunity submitted");
        assert!(matches!(session.try_submit_step(&pool, Lane::Interactive), Err(WorkerJobSubmitFault::Contention(WorkerJobContention::Submitted(_)))));
        wait_for(&session, WorkerJobPoll::Outcome);
        let mut first_owner = session.take_outcome(first).expect("first exact outcome");
        assert!(matches!(first_owner.take_outcome(), StepOutcome::Yield));
        first_owner.resume().unwrap_or_else(|_| panic!("yield owner resumes exact generation"));
        let second = session.try_submit_step(&pool, Lane::Interactive).expect("second opportunity submitted");
        wait_for(&session, WorkerJobPoll::Terminal);
        let terminal = session.take_terminal().expect("terminal owner is take-only");
        let terminal_pointer = terminal.job().backing.as_deref().expect("terminal hostile backing") as *const u8;
        drop(terminal);
        let terminal = session.take_terminal().expect("dropped checkout hands exact terminal back");
        assert_eq!(terminal.job().backing.as_deref().expect("returned hostile backing") as *const u8, terminal_pointer);
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
        let session = WorkerJobSession::try_new(HostileJob { backing: Some(backing), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_005), Generation(11), root_cancel_token()))
            .unwrap_or_else(|_| panic!("worker session slot"));
        assert_eq!(session.try_submit_step(&pool, Lane::Interactive), Err(WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
        let rejected = session.take_rejected().expect("pool rejection retained exact owner");
        assert_eq!(rejected.job().backing.as_deref().expect("rejected hostile backing") as *const u8, backing_pointer);
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
        let session = WorkerJobSession::try_new(HostileJob { backing: Some(Box::new(1)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: true, closing: false }, params(OperationId(90_006), Generation(12), root_cancel_token()))
            .unwrap_or_else(|_| panic!("worker session slot"));
        let preadmitted_fault_pointer = unsafe {
            (&*session.inner.authority.get())
                .as_ref()
                .and_then(|authority| authority.preadmitted_fault.as_ref())
                .and_then(|payload| payload.pages[0].as_ref())
                .map(|page| page.source.backing_identity())
                .expect("panic fault backing is admitted before submission")
        };
        session.register_wake(Waker::noop()).expect("quiet wake registration");
        let _ = session.try_submit_step(&pool, Lane::Interactive).expect("panic opportunity submitted");
        wait_for(&session, WorkerJobPoll::Terminal);
        assert!(session.take_wake());
        assert!(!session.take_wake(), "redundant quiet poll raises no wake");
        let terminal = session.take_terminal().expect("panic becomes retained terminal");
        let StepOutcome::Fault(fault) = terminal.outcome() else { panic!("panic publishes the pre-admitted fault") };
        let returned_fault_pointer = fault.detail.pages[0].as_ref().map(|page| page.source.backing_identity()).expect("terminal fault retains exact backing");
        assert_eq!(returned_fault_pointer, preadmitted_fault_pointer);
        terminal.begin_close();
        while !session.terminal_is_empty() {
            let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
        pool.shutdown();
    }

    #[test]
    fn worker_quiet_wake_sequence_exhaustion_is_permanent_and_typed() {
        let session = WorkerJobSession::try_new(HostileJob { backing: Some(Box::new(2)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_009), Generation(15), root_cancel_token()))
            .unwrap_or_else(|_| panic!("worker session slot"));
        session.inner.wake_sequence.store(u64::MAX, Ordering::Release);
        session.inner.wake_pending.store(false, Ordering::Release);
        session.inner.raise_wake();
        assert_eq!(session.register_wake(Waker::noop()), Err(WorkerJobContention::WakeExhausted(Generation(15))));
        session.begin_close();
        while !session.terminal_is_empty() {
            let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
    }

    #[test]
    fn batch_session_advances_exactly_one_external_opportunity() {
        let steps = Arc::new(AtomicUsize::new(0));
        let mut batch = BatchJobSession::try_new(HostileJob { backing: Some(Box::new(7)), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(OperationId(90_007), Generation(13), root_cancel_token()))
            .unwrap_or_else(|_| panic!("batch fault page is pre-admitted"));
        assert_eq!(batch.step(), Ok(WorkerJobPoll::Outcome));
        assert_eq!(steps.load(AtomicOrdering::Acquire), 1);
        assert!(matches!(batch.take_outcome(), Some(StepOutcome::Yield)));
        batch.resume().expect("caller explicitly resumes after first opportunity");
        assert_eq!(steps.load(AtomicOrdering::Acquire), 1, "batch adapter never drains itself to terminal");
        batch.begin_close();
        assert_eq!(batch.close_step(0, 0), WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        while !batch.terminal_is_empty() {
            let _ = batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
    }

    #[test]
    fn checked_out_and_worker_begin_close_transitions_report_exact_zero_release() {
        let mut batch = BatchJobSession::try_new(HostileJob { backing: Some(Box::new(7)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_010), Generation(16), root_cancel_token()))
            .unwrap_or_else(|_| panic!("batch session authority"));
        assert_eq!(batch.step(), Ok(WorkerJobPoll::Outcome));
        assert!(batch.checkout_outcome());
        assert_eq!(batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES), WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES), WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        while !batch.terminal_is_empty() {
            let _ = batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
    }

    #[test]
    fn worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned() {
        let mut sessions = Vec::with_capacity(WORKER_JOB_SESSION_SLOTS);
        for index in 0..WORKER_JOB_SESSION_SLOTS {
            let job = HostileJob { backing: Some(Box::new(index as u8)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false };
            sessions.push(WorkerJobSession::try_new(job, params(OperationId(91_000 + index as u64), Generation(index as u64 + 1), root_cancel_token())).unwrap_or_else(|_| panic!("each fixed session slot admits once")));
        }
        let rejected_backing = Box::new(211u8);
        let rejected_pointer = rejected_backing.as_ref() as *const u8;
        let mut rejected = match WorkerJobSession::try_new(HostileJob { backing: Some(rejected_backing), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(92_000), Generation(500), root_cancel_token())) {
            Ok(_) => panic!("session maximum plus one must retain exact rejected job"),
            Err(rejected) => rejected,
        };
        assert_eq!(rejected.job().backing.as_deref().expect("session max plus one backing") as *const u8, rejected_pointer);
        assert_eq!(rejected.params.as_ref().expect("session max plus one parameters").operation, OperationId(92_000));
        assert_eq!(rejected.params.as_ref().expect("session max plus one parameters").generation, Generation(500));
        rejected.begin_close();
        assert_eq!(rejected.close_step(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        while !rejected.terminal_is_empty() {
            let _ = rejected.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
        let dropped = sessions.pop().expect("last fixed session");
        drop(dropped);
        assert!(take_worker_job_retirement_wake());
        for _ in 0..8 {
            let _ = pump_worker_job_retirements(1, 1, JOB_PAYLOAD_PAGE_BYTES);
        }
        let replacement = WorkerJobSession::try_new(HostileJob { backing: Some(Box::new(17)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(92_001), Generation(501), root_cancel_token()))
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
