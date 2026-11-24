//! 👥️ Presence retirement witnesses that need the snapshot-read registry: boxed closes, returned-string admission and registry alias closure; the generic factory and observers are re-exported from the testing crate.
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneStep}, ErasedSnapshotRetirement, RetainedWorkOutcome};
pub use semio_framework_value_testing_presence_retirement::CLOSE_GRANT;
use std::sync::Arc;

pub use semio_framework_value_testing_presence_retirement::*;

pub fn observed_box_close(owner: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> RetainedCloneStep {
    semio_framework_value_testing_presence_retirement::observed_step(grant, |effect| settled(crate::artifact_retirement_box_close_step(owner, effect.grant().maximum_copy_bytes, effect)))
}

/// 🪑️ Admits one returned Arc root as its shared controlled lease owner without moving it on refusal.
pub fn admit_returned_arc<T: semio_framework_value::retirement::RetireOwned + Sync>(root: &mut Option<Arc<T>>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<Option<(Box<dyn ErasedSnapshotRetirement>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::OriginalRetirementFailure> {
    let mut lease = root.take().map(semio_framework_value::retirement::arc_custody::ArcLease::new);
    let turn = semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(&mut lease, effect);
    if let Some(lease) = lease.take() { *root = lease.0; }
    RetainedWorkOutcome { result: turn.result.map_err(Into::into).map(|admitted| admitted.map(|(owner, progress)| (owner as Box<dyn ErasedSnapshotRetirement>, progress))), work: turn.work }
}

pub fn admit_returned_string(registry: &crate::SnapshotReadRegistryHandle, expected: &str) -> Option<Box<dyn ErasedSnapshotRetirement>> {
    let (result, heap) = with_effect(CLOSE_GRANT, 1 << 20, &mut None, |effect| semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.try_admit_one_returned::<String, _>(effect, |original, effect| {
        assert_eq!(original.as_deref().map(String::as_str), Some(expected));
        assert_eq!(original.as_ref().map(Arc::strong_count), Some(1));
        admit_returned_arc(original, effect)
    }).result));
    match result {
        Ok((owner, receipt)) => { assert!(receipt.fits(CLOSE_GRANT)); assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes)); owner }
        Err(crate::SnapshotReadReturnedRefusal::Lease(crate::SnapshotReadLeaseRefusal::Busy)) => { assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); None }
        Err(_) => panic!("original returned String admission failed"),
    }
}

pub fn finish_box(owner: &mut Option<Box<dyn ErasedSnapshotRetirement>>) -> usize {
    let mut released = 0;
    for _ in 0..4096 {
        let step = observed_box_close(owner, CLOSE_GRANT);
        released += step.progress().released_bytes;
        if owner.is_none() { return released; }
    }
    panic!("original Presence test frame exhausted funded close turns")
}

pub fn finish_registry(registry: crate::SnapshotReadRegistryHandle) {
    let mut original = Some(crate::SnapshotReadRegistryAliasRetirement::new(registry));
    for _ in 0..4096 {
        semio_framework_value_testing_presence_retirement::observed_step(CLOSE_GRANT, |effect| settled(crate::snapshot_registry_alias_close_step(&mut original, effect)));
        if original.is_none() { return; }
    }
    panic!("original Presence registry failed funded physical closure")
}
