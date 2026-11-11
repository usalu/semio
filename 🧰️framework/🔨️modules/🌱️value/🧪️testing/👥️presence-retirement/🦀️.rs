//! 🧪️ Test-only Presence factories report their original physical ownership receipts (the generic factory and the heap observers; the snapshot-read bound witnesses live with value-snapshot-read).
use semio_framework_value::factory::{FactoryProducerReply, FactoryProducerReplyComplete};
use semio_framework_value::{ErasedSnapshotRetirement, OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, SnapshotRetirementFactory, FactoryAuthority, ValueError, RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, RetainedWorkOutcome, RetainedWorkPoll};
#[path="effect.rs"]
mod effect;
pub use effect::{settled, with_effect};
use semio_framework_value::retirement::arc_custody::original::{SharedOriginalRelease, SharedOriginalStorage};
use std::{mem::ManuallyDrop, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

pub const CLOSE_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 };

pub trait CountedPresence: Copy + Send + Sync + 'static { fn counted(self) -> bool; }
impl CountedPresence for i32 { fn counted(self) -> bool { self == 41 } }

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Factory(#[factory_child] pub Arc<AtomicUsize>);

struct Retirement<P: CountedPresence> {
    root: ManuallyDrop<Option<SharedOriginalStorage<P>>>,
    count: ManuallyDrop<Option<Arc<AtomicUsize>>>,
    counter_close: ManuallyDrop<Option<FactoryAuthority>>,
}

impl<P: CountedPresence> SnapshotRetirementFactory<P> for Factory {
    fn retirement_birth_bytes(&self, _: &SharedOriginalStorage<P>) -> usize { size_of::<Retirement<P>>() }
    fn retire_demands(&self, _: &SharedOriginalStorage<P>) -> Result<RetirementDemand, ValueError> { Ok(RetirementDemand { capacity_bytes: size_of::<Retirement<P>>(), depth: 1, ..Default::default() }) }
    fn retire_work_demand(&self, _: &SharedOriginalStorage<P>) -> Result<u64, ValueError> { Ok(1) }
    fn retire<'reply>(&self, original: &mut Option<SharedOriginalStorage<P>>, effect: &mut RetainedAllocationEffect<'_>, mut reply: FactoryProducerReply<'reply, dyn ErasedSnapshotRetirement, OriginalRetirementFailure>) -> FactoryProducerReplyComplete<'reply, OriginalRetirementFailure> {
        let grant = effect.grant();
        let bytes = size_of::<Retirement<P>>();
        if original.is_none() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_capacity_bytes < bytes || effect.maximum_work_units() == 0 { return reply.complete(RetainedWorkOutcome { result: Ok(RetainedCloneProgress::default()), work: 0 }); }
        match reply.performed(effect, 1) {
            Ok(true) => {}
            Ok(false) => return reply.complete(RetainedWorkOutcome { result: Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "Presence test frame exceeded its original work").into()), work: 1 }),
            Err(error) => return reply.complete(RetainedWorkOutcome { result: Err(error.into()), work: 1 }),
        }
        let Some(root) = original.take() else { return reply.complete(RetainedWorkOutcome { result: Err(ValueError::invariant("Presence test frame lost its original").into()), work: 1 }) };
        reply.publish(Box::new(Retirement { root: ManuallyDrop::new(Some(root)), count: ManuallyDrop::new(Some(Arc::clone(&self.0))), counter_close: ManuallyDrop::new(None) }));
        reply.complete(RetainedWorkOutcome { result: Ok(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: bytes, ..Default::default() }), work: 1 })
    }
    fn recover_retire<'reply>(&self, _: &mut Option<SharedOriginalStorage<P>>, _: &mut RetainedAllocationEffect<'_>, reply: FactoryProducerReply<'reply, dyn ErasedSnapshotRetirement, OriginalRetirementFailure>) -> FactoryProducerReplyComplete<'reply, OriginalRetirementFailure> { reply.pending() }
}

impl<P: CountedPresence> Retirement<P> {
    fn close(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>, work: &mut u128) -> Result<RetainedCloneStep, OriginalRetirementFailure> {
        let grant = effect.grant();
        let pay = |effect: &mut RetainedAllocationEffect<'_>, work: &mut u128| match semio_framework_value::retirement::record_retirement_work(effect, 1) { Ok(paid) => { *work += paid; Ok(()) } Err((error, paid)) => { *work += paid; Err(error) } };
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || effect.maximum_work_units() == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.demands(body)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "Presence test owner requires its original depth").into()); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(root) = self.root.as_ref() {
            if root.aliases() == 1 && !root.physically_unique() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            pay(effect, work)?;
            let released_bytes = match self.root.take().map(SharedOriginalStorage::retire) {
                Some(SharedOriginalRelease::Original(mut original)) => match original.take_value() {
                    Some(value) => { if value.counted() { if let Some(count) = self.count.as_ref() { count.fetch_add(1, Ordering::Relaxed); } } demand.release_bytes }
                    None => return Err(ValueError::invariant("Presence test original lost its unique payload").into()),
                },
                _ => 0,
            };
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }));
        }
        if let Some(counter) = self.count.take() {
            pay(effect, work)?;
            let counter: Arc<dyn semio_framework_value::FactoryRetirement> = counter;
            *self.counter_close = Some(FactoryAuthority::new(counter));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        let Some(owner) = self.counter_close.as_mut() else { return Err(ValueError::invariant("Presence test owner lost its counter close").into()); };
        let turn = match owner.step(body, effect) { RetainedWorkPoll::Complete(turn) => turn, RetainedWorkPoll::Pending { work: pending } => { *work += pending; return Ok(RetainedCloneStep::Progress(Default::default())); } };
        *work += turn.work;
        let step = turn.result?;
        if owner.terminal_is_empty() { self.counter_close.take(); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if self.root.is_some() { return Ok(RetirementDemand { release_bytes: SharedOriginalStorage::<P>::allocation_bytes(), depth: 1, ..Default::default() }); }
        if self.count.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        self.counter_close.as_ref().map_or(Ok(Default::default()), |owner| owner.demands(copy))
    }
}

impl<P: CountedPresence> ErasedSnapshotRetirement for Retirement<P> {
    fn next_computational_work(&self, _: usize) -> Result<u64, ValueError> { Ok(u64::from(!self.terminal_is_empty())) }
    fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let mut work = 0;
        let result = self.close(body, effect, &mut work);
        RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work })
    }
    fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.count.is_none() && self.counter_close.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> { self.demands(maximum_body_bytes) }
}

impl<P: CountedPresence> Drop for Retirement<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Presence test owner requires funded physical closure");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.root); ManuallyDrop::drop(&mut self.count); ManuallyDrop::drop(&mut self.counter_close); } }
    }
}

pub fn observed_step(grant: RetainedCloneGrant, close: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<RetainedCloneStep, OriginalRetirementFailure>) -> RetainedCloneStep {
    let (turn, heap) = with_effect(grant, 1 << 20, &mut None, |effect| semio_framework_trace::observe_heap_allocations_on_this_thread(|| close(effect)));
    let step = match turn.result { Ok(step) => step, Err(error) => panic!("original Presence close refused: {error}") };
    assert!(step.progress().fits(grant));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    step
}

pub fn observed_close(owner: &mut (impl ErasedSnapshotRetirement + ?Sized), grant: RetainedCloneGrant) -> RetainedCloneStep { observed_step(grant, |effect| settled(owner.close_step(grant.maximum_copy_bytes, effect))) }

pub fn finish(owner: &mut (impl ErasedSnapshotRetirement + ?Sized)) -> usize {
    let mut released = 0;
    for _ in 0..4096 {
        let step = observed_close(owner, CLOSE_GRANT);
        released += step.progress().released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty()); return released; }
    }
    panic!("Presence test owner exhausted its original bounded close turns")
}
