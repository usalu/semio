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
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: demand, maximum_depth: 1 };
            for denied_grant in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: demand - 1, ..grant }] {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| if is_deque { close_deque(&mut deque, denied_grant) } else { close_vec(&mut vector, denied_grant) });
                assert_eq!(step.unwrap(), denied());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(if is_deque { deque_demand(&deque) } else { vec_demand(&vector) }, demand);
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| if is_deque { close_deque(&mut deque, grant) } else { close_vec(&mut vector, grant) });
            assert_eq!(step.unwrap(), released(demand, true));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
            assert_eq!(if is_deque { deque_demand(&deque) } else { vec_demand(&vector) }, 0);
        }
    }
    let mut history = crate::os_vcs::HistoryPageStack::<String>::new();
    while history.capacity() != 0 {
        let demand = history.next_empty_page_release_byte_demand().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: demand, maximum_depth: 1 };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| close_history(&mut history, RetainedCloneGrant { maximum_release_bytes: demand - 1, ..grant }));
        assert_eq!(step.unwrap(), denied());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| close_history(&mut history, grant));
        assert_eq!(step.unwrap(), released(demand, history.capacity() == 0));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
    }
    let registry = SnapshotReadLeaseRegistry::new();
    assert_eq!(fixture["retainedSlots"].as_u64().unwrap() as usize, SNAPSHOT_READ_LEASE_CAPACITY);
    let demand = registry.empty_backing_demands().unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_empty_backing_step(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }));
    assert_eq!(step.unwrap(), denied());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_empty_backing_step(grant));
    assert_eq!(step.unwrap(), released(demand.release_bytes, true));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand.release_bytes));
    assert_eq!(registry.empty_backing_demands().unwrap().release_bytes, 0);
    eprintln!("[DEBUG] Store resident original backing whole release independent of copy/capacity: reader slots={} physical={}", SNAPSHOT_READ_LEASE_CAPACITY, demand.release_bytes);
}

#[test]
fn artifact_store_resident_detached_dag_forwards_exact_backing_then_cursor_terminal() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let maximum = fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize;
    let law = &fixture["dagRetirement"];
    for seed in [false, true] {
        let mut dag = crate::os_spr::MutationDag::default();
        if seed { dag.seed_applied(crate::os_spr::MutationId(law["appliedIdentity"].as_str().unwrap().to_owned())).unwrap(); }
        let (mut retirement, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ArtifactStoreMutationDagRetirement::new(dag));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, law["constructorBytes"].as_u64().unwrap() as usize));
        let mut turns = 0;
        let mut total = 0;
        while !retirement.terminal_is_empty() {
            let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize;
            let release = retirement.next_release_byte_demand().unwrap();
            let capacity = retirement.next_capacity_byte_demand(copy).unwrap();
            let depth = retirement.next_depth_demand().unwrap();
            assert!(release <= maximum);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| retirement.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }));
            assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if release != 0 {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| retirement.close_step(RetainedCloneGrant { maximum_release_bytes: release - 1, ..grant }));
                assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(retirement.next_release_byte_demand().unwrap(), release);
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| retirement.close_step(grant));
            let progress = step.unwrap().progress();
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            total += progress.released_bytes;
            turns += 1;
            assert!(turns <= 4096);
        }
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(retirement));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
        eprintln!("[DEBUG] Store original DAG seed={seed} bounded copy=1 independent whole backing releases={total} turns={turns} terminalDrop0heap");
    }
}
#[test]
fn artifact_store_resident_registry_terminal_drop_has_no_unreported_native_lock_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut registry = Some(SnapshotReadLeaseRegistry::new());
    let demand = registry.as_ref().unwrap().empty_backing_demands().unwrap();
    assert_eq!(registry.as_ref().unwrap().close_empty_backing_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth, ..Default::default() }).unwrap(), released(demand.release_bytes, true));
    assert_eq!(registry.as_ref().unwrap().empty_backing_demands().unwrap().release_bytes, 0);
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
    let original = owner.0.as_ref().unwrap().identity();
    let alias = owner.0.as_ref().unwrap().alias_handle().clone();
    assert_eq!(alias.strong_count(), 1 + fixture["registryExternalAliases"].as_u64().unwrap() as usize);
    let slots = owner.empty_backing_demands().unwrap();
    assert_eq!(owner.close_empty_backing_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: slots.release_bytes, maximum_depth: slots.depth, ..Default::default() }).unwrap(), released(slots.release_bytes, true));
    let extent = owner.frame_byte_demand().unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: extent, maximum_depth: 1, ..Default::default() };
    assert!(extent <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_frame_step(grant));
    assert_eq!(step.unwrap(), denied());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(alias));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_frame_step(RetainedCloneGrant { maximum_release_bytes: extent - 1, ..grant }));
    assert_eq!(step.unwrap(), denied());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(owner.0.as_ref().unwrap().identity(), original);
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_frame_step(grant));
    assert_eq!(step.unwrap(), released(extent, true));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, extent));
    assert_eq!(owner.frame_byte_demand().unwrap(), 0);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    eprintln!("[DEBUG] registry original Arc frame={} shared/one-below retained; exact free={} finalDrop=0", extent, extent);
}


fn demand_grant(demand: semio_framework_value::RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

fn undergrants(demand: semio_framework_value::RetirementDemand) -> Vec<RetainedCloneGrant> {
    let grant = demand_grant(demand);
    [(demand.copy_bytes != 0).then(|| RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant }), (demand.capacity_bytes != 0).then(|| RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }), (demand.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant })].into_iter().flatten().collect()
}

#[test]
fn artifact_store_resident_uninstalled_catalog_terminal_drop_has_no_unreported_factory_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let maximum = fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize;
    let mut owners = super::super::tests::demo_closable_store_owners();
    while !owners.store_disposer.uninstalled_terminal_is_empty() {
        let demand = owners.store_disposer.uninstalled_demands(0).unwrap();
        assert!(matches!(owners.store_disposer.close_uninstalled_step(demand_grant(demand)).unwrap(), RetainedCloneStep::Progress(_) | RetainedCloneStep::Complete(_)));
    }
    let mut turns = 0;
    let mut released = 0;
    while !owners.uninstalled_owners_terminal_is_empty() {
        let demand = owners.uninstalled_owners_demands(0).unwrap();
        assert!(demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) <= maximum);
        for denied_grant in undergrants(demand) {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owners.close_uninstalled_owners_step(denied_grant));
            assert_eq!(step.unwrap(), denied());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(owners.uninstalled_owners_demands(0).unwrap(), demand);
        }
        let grant = demand_grant(demand);
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owners.close_uninstalled_owners_step(grant));
        let progress = step.unwrap().progress();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        released += progress.released_bytes;
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
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: usize::MAX / 2, maximum_release_bytes: 0, maximum_depth: 64 };
    let (born, progress) = owners.retire_envelope_uninstalled(envelope, grant).unwrap_or_else(|_| panic!("catalog envelope birth is funded"));
    assert!(progress.fits(grant));
    let mut owner = Some(born);
    let mut turns = 0;
    let mut total = 0;
    while owner.is_some() {
        let demand = artifact_retirement_box_demands(owner.as_ref().unwrap(), 0).unwrap();
        assert!(demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
        let grant = demand_grant(demand);
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| artifact_retirement_box_close_step(&mut owner, grant));
        let progress = step.unwrap().progress();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        total += progress.released_bytes;
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
        let mut born = 0;
        let mut physical = 0;
        let mut payload_releases = 0;
        let mut turns = 0;
        while !owner.terminal_is_empty() {
            let release = owner.next_release_byte_demand().unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1, maximum_capacity_bytes: owner.next_capacity_byte_demand(1).unwrap(), maximum_release_bytes: release, maximum_depth: owner.next_depth_demand().unwrap() };
            for denied_grant in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), (release != 0).then(|| RetainedCloneGrant { maximum_release_bytes: release - 1, ..grant })].into_iter().flatten() {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(denied_grant));
                assert_eq!(step.unwrap(), denied());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(owner.next_release_byte_demand().unwrap(), release);
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant));
            let progress = step.unwrap().progress();
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            if release == extent && progress.released_bytes != 0 { assert_eq!(progress.released_bytes, extent); payload_releases += 1; }
            born += heap.requested_bytes;
            physical += heap.released_bytes;
            turns += 1;
            assert!(turns < extent * 2 + 128);
        }
        assert_eq!(payload_releases, 1);
        assert_eq!(physical, extent + born);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        eprintln!("[DEBUG] metadata String same whole payload={extent} actual births={born} full physical={physical} bounded copy1 turns={turns}");
    }
}

#[test]
fn artifact_store_resident_metadata_wrappers_forward_exact_original_string_extent() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let extent = fixture["emptyVectorExtents"][0].as_u64().unwrap() as usize;
    let body = || String::from_utf8(vec![b'a'; extent]).unwrap();
    let mut revision = CursorRevisionAccumulator { identity_digest: [0; 32], applied: crate::os_vcs::HistoryPageStack::empty(), redo: crate::os_vcs::HistoryPageStack::empty(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() };
    revision.mutation_positions.insert(MutationId(body()), (0, 0, [0; 32]));
    let visibility = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
    let view = visibility.view();
    let mut cursor = ArtifactCursor::new(vec![body()], Vec::new(), None);
    cursor.stage_group_owned(ArtifactCursorOwners { applied_edit_ids: vec![body()].into(), redo_edit_ids: Default::default(), checkpoint_id: Some(body()) }, &view).unwrap();
    let original = Arc::as_ptr(&view);
    let mut cursor = ArtifactStoreCursorRetirement::new(cursor);
    let zero = RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 0 };
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_step(zero).unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(Arc::as_ptr(&cursor.owner.original().unwrap().group.as_ref().unwrap().visibility), original);
    drop((view, visibility));
    let owners: [Box<dyn ErasedSnapshotRetirement>; 3] = [
        Box::new(ArtifactStoreStringVectorRetirement::new(vec![body()])),
        Box::new(ArtifactStoreRevisionAccumulatorRetirement::new(revision)),
        Box::new(cursor),
    ];
    for (row, (name, owner)) in ["string-vector", "revision", "cursor"].into_iter().zip(owners).enumerate() {
        let mut slot = Some(owner);
        let mut string_extents = 0;
        let mut turns = 0;
        while let Some(owner) = slot.as_ref() {
            let demand = semio_framework_value::factory_ticket_demands(owner, 1).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let mut denials = vec![zero];
            if demand.capacity_bytes != 0 { denials.push(RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }); }
            if demand.release_bytes != 0 { denials.push(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }); }
            for denied in denials {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::close_factory_ticket(&mut slot, denied));
                assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(semio_framework_value::factory_ticket_demands(slot.as_ref().unwrap(), 1).unwrap(), demand);
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::close_factory_ticket(&mut slot, grant));
            let progress = step.unwrap().progress();
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            string_extents += usize::from(progress.released_bytes == extent);
            turns += 1;
            assert!(turns < extent * 4 + 1000);
        }
        assert_eq!(string_extents, fixture["metadataWrapperStringCounts"][row].as_u64().unwrap() as usize, "{name} releases each genuine original String backing whole");
        eprintln!("[DEBUG] metadata wrapper={name} originalString={extent} copyBudget=1 exact System receipt, staged group and terminal ticket ownership closed in {turns} turns");
    }
}
#[test]
fn artifact_store_string_history_full_grant_reports_original_physical_heap() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["stringHistory"];
    let count = law["itemCount"].as_u64().unwrap() as usize;
    let bytes = law["bodyBytes"].as_u64().unwrap() as usize;
    let mut values = crate::os_vcs::HistoryPageStack::empty();
    for _ in 0..count { values.push(String::from_utf8(vec![b'a'; bytes]).unwrap()); }
    let original = values.last().unwrap().as_ptr();
    let mut owner = ArtifactStoreStringVectorRetirement::new(values);
    let zero = RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 0 };
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(zero));
    assert_eq!(step.unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress::default()));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(owner.values.as_ref().unwrap().last().unwrap().as_ptr(), original);
    let mut turns = 0;
    let mut physical = 0;
    while !owner.terminal_is_empty() {
        let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize;
        let release = owner.next_release_byte_demand().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: release, maximum_depth: owner.next_depth_demand().unwrap() };
        if release != 0 {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(RetainedCloneGrant { maximum_release_bytes: release - 1, ..grant }));
            assert_eq!(step.unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(owner.next_release_byte_demand().unwrap(), release);
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant));
        let step = step.unwrap();
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty()); }
        physical += progress.released_bytes;
        turns += 1;
        assert!(turns <= count * (bytes + 64));
    }
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
    assert!(physical >= count * bytes);
    eprintln!("[DEBUG] original Store string history fullgrant turns={turns} physical={physical} copyBudget=1 terminalDrop=0 System receipt parity");
}

#[test]
fn artifact_store_original_source_iterator_reports_original_whole_backing_after_prefix_handoff() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["sourceIterator"];
    let expected: Vec<u64> = law["values"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().parse().unwrap()).collect();
    let reference: Vec<u64> = serde_json::from_str("[0,18446744073709551615,7]").unwrap();
    assert_eq!(expected, reference);
    let (source, original) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        let mut source = Vec::with_capacity(law["backingCapacity"].as_u64().unwrap() as usize);
        source.extend(expected.iter().copied());
        source
    });
    let pointer = source.as_ptr();
    let physical = source.capacity() * std::mem::size_of::<u64>();
    assert_eq!(physical, original.requested_bytes);
    let (mut owner, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| SourceIterator::new(source));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (law["constructorBytes"].as_u64().unwrap() as usize, 0));
    assert_eq!(owner.as_slice().as_ptr(), pointer);
    assert_eq!(owner.next(), Some(reference[0]));
    assert_eq!(owner.as_slice(), &reference[1..]);
    let mut born = 0;
    let mut freed = 0;
    let mut terminal_receipt = 0;
    for _ in 0..256 {
        if owner.terminal_is_empty() { break; }
        let demand = owner.demands(law["maximumCopyBytes"].as_u64().unwrap() as usize).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let denied_grant = RetainedCloneGrant { maximum_items: 0, ..grant };
        let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(denied_grant));
        assert_eq!(denied.unwrap().progress(), RetainedCloneProgress::default());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(owner.demands(grant.maximum_copy_bytes).unwrap(), demand);
        if demand.release_bytes != 0 {
            let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }));
            assert_eq!(denied.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant));
        let step = step.unwrap();
        assert!(step.progress().fits(grant));
        assert_eq!(step.progress().retained_capacity_bytes, heap.requested_bytes);
        assert_eq!(step.progress().released_bytes, heap.released_bytes);
        born += heap.requested_bytes;
        freed += heap.released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) { terminal_receipt = step.progress().released_bytes; }
    }
    assert!(owner.terminal_is_empty());
    assert_eq!(terminal_receipt, physical);
    assert_eq!(freed, original.requested_bytes + born);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] Original source iterator prefix pointer preserved; physical={physical} born={born} freed={freed} final={terminal_receipt}");
}
