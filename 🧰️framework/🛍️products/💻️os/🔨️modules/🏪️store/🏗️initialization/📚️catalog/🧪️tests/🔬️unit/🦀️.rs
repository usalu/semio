use super::*;

#[test]
fn retained_empty_initialization_catalog_releases_one_exact_native_page_without_ungranted_heap_work() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let lanes = fixture["lanes"].as_array().unwrap();
    let page_slots = fixture["pageSlots"].as_u64().unwrap() as usize;
    let mut catalog = ArtifactStoreInitializationOwnerCatalog::try_new().unwrap();
    assert_eq!(catalog.admitted_items(), lanes.len() * page_slots);
    let mut total_released = 0;
    for (index, lane) in lanes.iter().enumerate() {
        let (demand, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.next_release_byte_demand().unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(demand > 0);
        let demands = catalog.close_demands().unwrap();
        assert_eq!(demands.copy_bytes, fixture["currencies"]["copyBytes"].as_u64().unwrap() as usize);
        assert_eq!(demands.capacity_bytes, fixture["currencies"]["capacityBytes"].as_u64().unwrap() as usize);
        assert_eq!(demands.depth, fixture["currencies"]["depth"].as_u64().unwrap() as usize);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: demand, maximum_depth: demands.depth, ..Default::default() };
        let (refused, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.close_step(RetainedCloneGrant { maximum_depth: 0, ..grant }));
        assert_eq!(refused.unwrap_err().kind, ValueRefusalKind::DepthLimit);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: demand - 1, ..grant }] {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.close_step(denied).unwrap());
            assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(catalog.next_release_byte_demand().unwrap(), demand);
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.close_step(grant).unwrap());
        assert_eq!(step.progress(), RetainedCloneProgress { copied_items: 1, released_bytes: demand, ..Default::default() });
        assert_eq!(matches!(step, RetainedCloneStep::Complete(_)), catalog.terminal_is_empty());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
        assert_eq!(catalog.admitted_items(), (lanes.len() - index - 1) * page_slots);
        let resident = [catalog.applied_edit_ids.capacity(), catalog.redo_edit_ids.capacity(), catalog.cursor_applied_edit_ids.capacity(), catalog.cursor_redo_edit_ids.capacity(), catalog.applied_revision.capacity(), catalog.redo_revision.capacity()];
        assert_eq!(resident, std::array::from_fn(|lane| if lane <= index { 0 } else { page_slots }));
        total_released += demand;
        println!("[DEBUG] Empty initialization native catalog lane={lane} exact-demand={demand} zero/one-below retained, one native page released0birth");
    }
    assert!(catalog.terminal_is_empty());
    assert_eq!(catalog.next_release_byte_demand().unwrap(), 0);
    assert_eq!(catalog.close_step(Default::default()).unwrap(), RetainedCloneStep::Complete(Default::default()));
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(catalog));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    println!("[DEBUG] Empty initialization native catalog terminal0heap, exact six-page release-total={total_released}; catalog birth cold/uncredited");
}

#[test]
fn retained_initialization_catalog_birth_denial_and_cancellation_preserve_every_original_page() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["birthPolicy"];
    let policy = RetainedCloneGrant { maximum_items: row["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: row["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: row["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: row["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: row["maximumDepth"].as_u64().unwrap() as usize };
    for cut in fixture["birthCancelCuts"].as_array().unwrap() {
        let cancel_after = cut.as_u64().unwrap() as usize;
        let (mut catalog, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(ArtifactStoreInitializationOwnerCatalog::empty);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut born = 0;
        for _ in 0..cancel_after {
            let (demand, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.admission_demands().unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(demand.capacity_bytes > 0 && demand.depth == 1);
            let original = catalog.admitted_items();
            for denied in [RetainedCloneGrant { maximum_items: 0, ..policy }, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..policy }, RetainedCloneGrant { maximum_depth: 0, ..policy }] {
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.admit_next(denied));
                assert!(result.is_err() || result.unwrap() == RetainedCloneProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(catalog.admitted_items(), original);
                assert_eq!(catalog.admission_demands().unwrap(), demand);
            }
            let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.admit_next(policy).unwrap());
            assert!(progress.fits(policy));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, 0));
            assert_eq!(progress.retained_capacity_bytes, demand.capacity_bytes);
            assert_eq!(progress.copied_items, 1);
            born += progress.retained_capacity_bytes;
        }
        assert_eq!(catalog.admission_is_complete(), cancel_after == 6);
        let mut freed = 0;
        for _ in 0..6 {
            if catalog.terminal_is_empty() { break; }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.close_step(policy).unwrap());
            assert!(step.progress().fits(policy));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, step.progress().released_bytes));
            freed += step.progress().released_bytes;
        }
        assert!(catalog.terminal_is_empty());
        assert_eq!(freed, born);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(catalog));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] Initialization catalog cancel-after={cancel_after} original admitted birth={born} paid release={freed} terminal0heap");
    }
}
