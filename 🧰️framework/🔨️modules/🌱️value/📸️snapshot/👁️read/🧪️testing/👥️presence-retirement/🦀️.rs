//! 👥️ Presence retirement witnesses that need the snapshot-read registry: boxed closes, returned-string admission and registry alias closure; the generic factory and observers are re-exported from the testing crate.
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneStep}, ErasedSnapshotRetirement, RetainedWorkOutcome};
pub use semio_framework_value_testing_presence_retirement::CLOSE_GRANT;
use std::sync::Arc;

pub use semio_framework_value_testing_presence_retirement::*;

pub fn observed_box_close(owner: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> RetainedCloneStep {
    semio_framework_value_testing_presence_retirement::observed_step(grant, |effect| settled(crate::artifact_retirement_box_close_step(owner, effect)))
}

pub fn admit_returned_string(registry: &crate::SnapshotReadRegistryHandle, expected: &str) -> Option<Box<dyn ErasedSnapshotRetirement>> {
    let (result, heap) = with_effect(CLOSE_GRANT, 1 << 20, &mut None, |effect| semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.try_admit_one_returned::<String, _>(effect, |original, effect| {
        assert_eq!(original.as_deref().map(String::as_str), Some(expected));
        assert_eq!(original.as_ref().map(Arc::strong_count), Some(1));
        let turn = semio_framework_value::retirement::arc_custody::admit_shared_retirement(original, effect, false);
        RetainedWorkOutcome { result: turn.result.map_err(Into::into), work: turn.work }
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
