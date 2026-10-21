//! 🧪️ Test-only Presence factories report their original physical ownership receipts (the generic factory and the heap observers; the snapshot-read bound witnesses live with value-snapshot-read).
use semio_framework_value::{ErasedSnapshotRetirement, SnapshotRetirementFactory, FactoryAuthority, ValueError, RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::{mem::ManuallyDrop, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

pub const CLOSE_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 };

pub trait CountedPresence: Copy + Send + Sync + 'static { fn counted(self) -> bool; }
impl CountedPresence for i32 { fn counted(self) -> bool { self == 41 } }

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Factory(#[factory_child] pub Arc<AtomicUsize>);

struct Retirement<P: CountedPresence> {
    root: ManuallyDrop<Option<Arc<P>>>,
    count: ManuallyDrop<Option<Arc<AtomicUsize>>>,
    counter_close: ManuallyDrop<Option<FactoryAuthority>>,
}

impl<P: CountedPresence> SnapshotRetirementFactory<P> for Factory {
    fn retirement_birth_bytes(&self, _: &Arc<P>) -> usize { size_of::<Retirement<P>>() }
    fn retire(&self, original: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<P>)> {
        semio_framework_value::retirement::frame::admit_retirement_frame(original, grant, |root| Retirement { root: ManuallyDrop::new(Some(root)), count: ManuallyDrop::new(Some(Arc::clone(&self.0))), counter_close: ManuallyDrop::new(None) })
    }
}

impl<P: CountedPresence> Retirement<P> {
    fn demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if self.root.is_some() { return Ok(RetirementDemand { release_bytes: semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<P>(), depth: 1, ..Default::default() }); }
        if self.count.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        self.counter_close.as_ref().map_or(Ok(Default::default()), |owner| owner.demands(copy))
    }
}

impl<P: CountedPresence> ErasedSnapshotRetirement for Retirement<P> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "Presence test owner requires its original depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(root) = self.root.as_ref() {
            if Arc::weak_count(root) != 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let released_bytes = if let Some(value) = Arc::into_inner(self.root.take().unwrap()) {
                if value.counted() { self.count.as_ref().unwrap().fetch_add(1, Ordering::Relaxed); }
                demand.release_bytes
            } else { 0 };
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }));
        }
        if let Some(counter) = self.count.take() {
            let counter: Arc<dyn semio_framework_value::FactoryRetirement> = counter;
            *self.counter_close = Some(FactoryAuthority::new(counter));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        let owner = self.counter_close.as_mut().unwrap();
        let step = owner.step(grant)?;
        if owner.terminal_is_empty() { self.counter_close.take(); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
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

pub fn observed_step(grant: RetainedCloneGrant, close: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| close().unwrap());
    assert!(step.progress().fits(grant));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    step
}

pub fn observed_close(owner: &mut (impl ErasedSnapshotRetirement + ?Sized), grant: RetainedCloneGrant) -> RetainedCloneStep { observed_step(grant, || owner.close_step(grant)) }

pub fn finish(owner: &mut (impl ErasedSnapshotRetirement + ?Sized)) -> usize {
    let mut released = 0;
    for _ in 0..4096 {
        let step = observed_close(owner, CLOSE_GRANT);
        released += step.progress().released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty()); return released; }
    }
    panic!("Presence test owner exhausted its original bounded close turns")
}
