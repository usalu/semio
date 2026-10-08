use super::*;

#[test]
fn artifact_store_resident_empty_backings_retain_undergrant_and_report_actual_free() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for bytes in fixture["emptyVectorExtents"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let mut vector = Vec::<u8>::with_capacity(bytes);
        let mut deque = VecDeque::<u8>::with_capacity(bytes);
        for is_deque in [false, true] {
            let demand = if is_deque { deque_demand(&deque) } else { vec_demand(&vector) };
            assert!(demand <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
            for (items, grant) in [(0, demand), (1, demand - 1)] {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| if is_deque { close_deque(&mut deque, items, grant) } else { close_vec(&mut vector, items, grant) });
                assert_eq!(step.unwrap(), denied());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(if is_deque { deque_demand(&deque) } else { vec_demand(&vector) }, demand);
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| if is_deque { close_deque(&mut deque, 1, demand) } else { close_vec(&mut vector, 1, demand) });
            assert_eq!(step.unwrap(), released(demand));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
            assert_eq!(if is_deque { deque_demand(&deque) } else { vec_demand(&vector) }, 0);
        }
    }
    let mut history = crate::os_vcs::HistoryPageStack::<String>::new();
    while history.capacity() != 0 {
        let demand = history.next_empty_page_release_byte_demand().unwrap();
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| close_history(&mut history, 1, demand - 1));
        assert_eq!(step.unwrap(), denied());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| close_history(&mut history, 1, demand));
        assert_eq!(step.unwrap(), released(demand));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
    }
    let registry = SnapshotReadLeaseRegistry::new();
    assert_eq!(fixture["retainedSlots"].as_u64().unwrap() as usize, SNAPSHOT_READ_LEASE_CAPACITY);
    let demand = registry.next_empty_backing_byte_demand();
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_empty_backing_step(1, demand - 1));
    assert_eq!(step.unwrap(), denied());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_empty_backing_step(1, demand));
    assert_eq!(step.unwrap(), released(demand));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
    assert_eq!(registry.next_empty_backing_byte_demand(), 0);
    eprintln!("[DEBUG] Store resident metadata original backing exact; vector/deque/history/readers zero-birth one-below retained; reader slots={} physical={}", SNAPSHOT_READ_LEASE_CAPACITY, demand);
}

#[test]
fn artifact_store_resident_detached_dag_forwards_exact_backing_then_cursor_terminal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let maximum = fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize;
    let mut retirement = ArtifactStoreMutationDagRetirement::new(crate::os_spr::MutationDag::default());
    let mut turns = 0;
    let mut total = 0;
    while !retirement.terminal_is_empty() {
        let demand = retirement.next_close_byte_demand();
        assert!(demand <= maximum);
        if demand != 0 {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| retirement.close_step(1, demand - 1));
            assert_eq!(step.unwrap(), denied());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(retirement.next_close_byte_demand(), demand);
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| retirement.close_step(1, demand));
        let reported = match step.unwrap() { SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1); released_bytes }, SnapshotRetirementStep::Complete => 0, SnapshotRetirementStep::Blocked => panic!("detached empty DAG has no foreign owner") };
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, reported));
        assert!(heap.largest_release_bytes <= maximum);
        total += reported;
        turns += 1;
        assert!(turns <= 4096);
    }
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(retirement));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    eprintln!("[DEBUG] Store detached DAG exact forwarding turns={} physical={} final payload/frame inline drop=0", turns, total);
}

#[test]
fn artifact_store_resident_registry_terminal_drop_has_no_unreported_native_lock_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut registry = Some(SnapshotReadLeaseRegistry::new());
    let demand = registry.as_ref().unwrap().next_empty_backing_byte_demand();
    assert_eq!(registry.as_ref().unwrap().close_empty_backing_step(1, demand).unwrap(), released(demand));
    assert_eq!(registry.as_ref().unwrap().next_empty_backing_byte_demand(), 0);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(registry.take()));
    assert_eq!(heap.requested_bytes, 0);
    assert_eq!(heap.released_bytes, fixture["registryFinalDropBytes"].as_u64().unwrap() as usize);
    eprintln!("[DEBUG] snapshot registry terminal inline lock after exact slot close: birth={} physical={}", heap.requested_bytes, heap.released_bytes);
}

#[test]
fn artifact_store_resident_registry_lock_matches_standard_contention_and_poison_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let oracle = std::sync::Mutex::new(());
    let registry = SnapshotReadLeaseRegistry::new();
    let standard = oracle.try_lock().unwrap();
    let actual = registry.state.try_lock().unwrap();
    assert_eq!(fixture["registryLockStates"][0], "available");
    assert!(matches!(oracle.try_lock(), Err(std::sync::TryLockError::WouldBlock)));
    assert!(matches!(registry.state.try_lock(), Err(std::sync::TryLockError::WouldBlock)));
    assert_eq!(fixture["registryLockStates"][1], "busy");
    drop((standard, actual));
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = oracle.try_lock().unwrap();
        panic!("standard registry poison witness");
    })).is_err());
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = registry.state.try_lock().unwrap();
        panic!("actual registry poison witness");
    })).is_err());
    assert!(matches!(oracle.try_lock(), Err(std::sync::TryLockError::Poisoned(_))));
    assert!(matches!(registry.state.try_lock(), Err(std::sync::TryLockError::Poisoned(_))));
    assert_eq!(fixture["registryLockStates"][2], "poisoned");
    eprintln!("[DEBUG] registry inline lock states equal std::sync::Mutex: available,busy,poisoned");
}

#[test]
fn artifact_store_resident_registry_arc_frame_retains_original_alias_until_whole_funding() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut owner = SnapshotReadLeaseRegistryOwner::new();
    let original = Arc::as_ptr(owner.0.as_ref().unwrap());
    let alias = owner.0.as_ref().unwrap().clone();
    assert_eq!(Arc::strong_count(&alias), 1 + fixture["registryExternalAliases"].as_u64().unwrap() as usize);
    let slots = owner.next_empty_backing_byte_demand();
    assert_eq!(owner.close_empty_backing_step(1, slots).unwrap(), released(slots));
    let extent = owner.frame_byte_demand();
    assert!(extent <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_frame_step(1, extent));
    assert_eq!(step.unwrap(), SnapshotRetirementStep::Blocked);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(alias));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_frame_step(1, extent - 1));
    assert_eq!(step.unwrap(), denied());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(Arc::as_ptr(owner.0.as_ref().unwrap()), original);
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_frame_step(1, extent));
    assert_eq!(step.unwrap(), released(extent));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, extent));
    assert_eq!(owner.frame_byte_demand(), 0);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    eprintln!("[DEBUG] registry original Arc frame={} shared/one-below retained; exact free={} finalDrop=0", extent, extent);
}

#[test]
fn artifact_store_resident_uninstalled_catalog_terminal_drop_has_no_unreported_factory_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut owners = super::super::tests::demo_closable_store_owners();
    assert_eq!(owners.store_disposer.close_uninstalled_step(1).unwrap(), SnapshotRetirementStep::Complete);
    assert!(owners.store_disposer.uninstalled_terminal_is_empty());
    let mut turns = 0;
    let mut released = 0;
    while !owners.uninstalled_owners_terminal_is_empty() {
        let demand = owners.next_close_byte_demand();
        assert!(demand <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
        if demand != 0 {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owners.close_uninstalled_owners_step(1, demand - 1));
            assert_eq!(step.unwrap(), denied());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(owners.next_close_byte_demand(), demand);
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owners.close_uninstalled_owners_step(1, demand));
        let physical = match step.unwrap() { SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1); released_bytes }, SnapshotRetirementStep::Complete => 0, SnapshotRetirementStep::Blocked => panic!("catalog has no external owner") };
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, physical));
        released += physical;
        turns += 1;
        assert!(turns <= 64);
    }
    eprintln!("[DEBUG] original catalog controlled turns={} physical={} final terminal drop follows", turns, released);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owners));
    eprintln!("[DEBUG] uninstalled catalog terminal original System birth={} physical={}", heap.requested_bytes, heap.released_bytes);
    assert_eq!(heap.requested_bytes, 0);
    assert_eq!(heap.released_bytes, fixture["catalogTerminalDropBytes"].as_u64().unwrap() as usize);
}

#[test]
fn artifact_store_resident_consumed_envelope_keeps_catalog_until_exact_terminal_release() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let owners = super::super::tests::demo_closable_store_owners();
    let envelope = create_document_envelope::<super::super::tests::DemoSnapshot, super::super::fixture_mutations::demo::DemoMutation>("demo/v1", "catalog-cancel", super::super::tests::DemoSnapshot::default(), None);
    let mut owner = Some(owners.retire_envelope_uninstalled(envelope).unwrap());
    let mut turns = 0;
    let mut total = 0;
    while owner.is_some() {
        let demand = artifact_retirement_box_byte_demand(owner.as_ref().unwrap());
        assert!(demand <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| artifact_retirement_box_close_step(&mut owner, 1, demand));
        let physical = match step.unwrap() { SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1); released_bytes }, SnapshotRetirementStep::Complete => 0, SnapshotRetirementStep::Blocked => panic!("consumed envelope has no external owner") };
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, physical));
        total += physical;
        turns += 1;
        assert!(turns <= 1024);
    }
    eprintln!("[DEBUG] consumed envelope and original catalog exact physical={} turns={} terminal owner absent", total, turns);
}

#[test]
fn artifact_store_resident_metadata_string_reports_whole_physical_allocation_only() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for extent in fixture["emptyVectorExtents"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let mut owner = ArtifactStoreStringRetirement::new(String::from_utf8(vec![b'a'; extent]).unwrap());
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(1, extent - 1));
        eprintln!("[DEBUG] metadata String original extent={} denied step={:?} birth={} physical={}", extent, step, heap.requested_bytes, heap.released_bytes);
        assert_eq!(step.unwrap(), denied());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(owner.next_close_byte_demand(), extent);
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(1, extent));
        assert_eq!(step.unwrap(), released(extent));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, extent));
        assert!(owner.terminal_is_empty());
    }
}

#[test]
fn artifact_store_resident_metadata_wrappers_forward_exact_original_string_extent() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let extent = fixture["emptyVectorExtents"][0].as_u64().unwrap() as usize;
    let child = || std::mem::ManuallyDrop::new(Some(ArtifactStoreStringRetirement::new(String::from_utf8(vec![b'a'; extent]).unwrap())));
    let owners: [Box<dyn ErasedSnapshotRetirement>; 3] = [
        Box::new(ArtifactStoreStringVectorRetirement { values: std::mem::ManuallyDrop::new(None), active: child() }),
        Box::new(ArtifactStoreRevisionAccumulatorRetirement { accumulator: std::mem::ManuallyDrop::new(None), active: child() }),
        Box::new(ArtifactStoreCursorRetirement { cursor: std::mem::ManuallyDrop::new(None), active: child() }),
    ];
    for (name, mut owner) in ["string-vector", "revision", "cursor"].into_iter().zip(owners) {
        assert_eq!(owner.next_close_byte_demand(), extent, "{name} forwards its retained physical owner");
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(1, extent - 1));
        assert_eq!(step.unwrap(), denied());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(1, extent));
        assert_eq!(step.unwrap(), released(extent));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, extent));
        for _ in 0..3 { if owner.terminal_is_empty() { break; } owner.close_step(1, owner.next_close_byte_demand()).unwrap(); }
        assert!(owner.terminal_is_empty());
        eprintln!("[DEBUG] metadata wrapper={} forwards original String extent={} denied/exact System parity", name, extent);
    }
}
