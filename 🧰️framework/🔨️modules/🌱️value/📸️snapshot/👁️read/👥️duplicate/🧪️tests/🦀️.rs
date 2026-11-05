//! 🧫️ Exact-root duplication currency, denial and original registry identity laws.
use crate::*;

#[test]
fn typed_original_duplicate_preserves_root_and_registry_without_heap_birth() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../fixture.json")).unwrap();
    let registry = SnapshotReadRegistryHandle::new();
    let root = Arc::new(71u64);
    let read = SnapshotRead::new(root.clone(), registry.try_issue(root.clone()).unwrap());
    let identity = read.lease_identity().unwrap();
    let demand = SnapshotRead::<u64>::duplicate_original_demands();
    for case in fixture["cases"].as_array().unwrap() {
        let grant = RetainedCloneGrant { maximum_items: case["items"].as_u64().unwrap() as usize, maximum_copy_bytes: demand.copy_bytes - usize::from(case["copy"] == "short"), maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: case["depth"].as_u64().unwrap() as usize };
        let mut output = None;
        let before = Arc::strong_count(&root);
        let (turn, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| read.duplicate_original(&mut output, grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(turn.work, case["work"].as_u64().unwrap());
        let progress = turn.result.unwrap();
        assert_eq!(output.is_some(), case["issued"].as_bool().unwrap());
        assert_eq!(read.lease_identity(), Some(identity));
        if let Some(duplicate) = output {
            let next = duplicate.lease_identity().unwrap();
            assert_eq!(next.0, identity.0);
            assert_ne!((next.1, next.2), (identity.1, identity.2));
            assert!(std::ptr::eq(read.get(), duplicate.get()));
            assert_eq!(progress.copied_bytes, demand.copy_bytes);
            assert_eq!(progress.retained_capacity_bytes, 0);
            let lease = duplicate.lease.as_ref().unwrap();
            let guard = registry.try_take(lease.index, lease.generation).unwrap();
            let mut duplicate = duplicate;
            duplicate.lease.take();
            drop((duplicate, guard));
        } else {
            assert_eq!(progress, RetainedCloneProgress::default());
            assert_eq!(Arc::strong_count(&root), before);
        }
    }
    let mut output = None;
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 2 };
    let held = registry.state.try_lock().unwrap();
    let turn = read.duplicate_original(&mut output, grant);
    assert!(turn.result.is_err());
    assert_eq!(turn.work, 1);
    assert!(output.is_none());
    assert_eq!(read.lease_identity(), Some(identity));
    drop(held);
    let turn = read.duplicate_original(&mut output, grant);
    assert!(turn.result.is_ok());
    let retained = output.as_ref().unwrap().lease_identity();
    let turn = read.duplicate_original(&mut output, grant);
    assert!(turn.result.is_err());
    assert_eq!(turn.work, 0);
    assert_eq!(output.as_ref().unwrap().lease_identity(), retained);
    let mut duplicate = output.take().unwrap();
    let lease = duplicate.lease.take().unwrap();
    drop(registry.try_take(lease.index, lease.generation).unwrap());
    drop(duplicate);
    let mut read = read;
    let lease = read.lease.take().unwrap();
    drop(registry.try_take(lease.index, lease.generation).unwrap());
    drop(read);
    assert!(registry.terminal_is_empty());
}
