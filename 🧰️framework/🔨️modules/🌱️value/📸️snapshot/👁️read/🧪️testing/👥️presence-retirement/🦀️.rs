//! 👥️ Presence retirement witnesses that need the snapshot-read registry: boxed closes, returned-string admission and registry alias closure; the generic factory and observers are re-exported from the testing crate.
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneStep}, ErasedSnapshotRetirement};
use semio_framework_value_testing_presence_retirement::observed_step;
pub use semio_framework_value_testing_presence_retirement::CLOSE_GRANT;
use std::sync::Arc;

pub use semio_framework_value_testing_presence_retirement::*;

pub fn observed_box_close(owner: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> RetainedCloneStep {
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| crate::artifact_retirement_box_close_step(owner, grant).unwrap());
    assert!(step.progress().fits(grant));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    step
}

pub fn admit_returned_string(registry: &crate::SnapshotReadRegistryHandle, expected: &str) -> Option<Box<dyn ErasedSnapshotRetirement>> {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.try_admit_one_returned::<String, _>(CLOSE_GRANT, |original, grant| {
        assert_eq!(original.as_str(), expected);
        assert_eq!(Arc::strong_count(&original), 1);
        semio_framework_value::retirement::arc_custody::admit_shared_retirement(original, grant, false)
    }));
    match result {
        Ok((owner, receipt)) => { assert!(receipt.fits(CLOSE_GRANT)); assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes)); owner }
        Err(crate::SnapshotReadLeaseRefusal::Busy) => { assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); None }
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
        observed_step(CLOSE_GRANT, || crate::snapshot_registry_alias_close_step(&mut original, CLOSE_GRANT));
        if original.is_none() { return; }
    }
    panic!("original Presence registry failed funded physical closure")
}
