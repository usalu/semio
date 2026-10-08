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
        let (demand, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.next_close_byte_demand().unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(demand > 0);
        for (items, bytes) in [(0, demand), (1, demand - 1)] {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.close_step(items, bytes).unwrap());
            assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(catalog.next_close_byte_demand().unwrap(), demand);
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.close_step(1, demand).unwrap());
        assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 1, released_bytes: demand });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
        assert_eq!(catalog.admitted_items(), (lanes.len() - index - 1) * page_slots);
        let resident = [catalog.applied_edit_ids.capacity(), catalog.redo_edit_ids.capacity(), catalog.cursor_applied_edit_ids.capacity(), catalog.cursor_redo_edit_ids.capacity(), catalog.applied_revision.capacity(), catalog.redo_revision.capacity()];
        assert_eq!(resident, std::array::from_fn(|lane| if lane <= index { 0 } else { page_slots }));
        total_released += demand;
        println!("[DEBUG] Empty initialization native catalog lane={lane} exact-demand={demand} zero/one-below retained, one native page released0birth");
    }
    assert!(catalog.terminal_is_empty());
    assert_eq!(catalog.next_close_byte_demand().unwrap(), 0);
    assert_eq!(catalog.close_step(1, 0).unwrap(), SnapshotRetirementStep::Complete);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(catalog));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    println!("[DEBUG] Empty initialization native catalog terminal0heap, exact six-page release-total={total_released}; catalog birth cold/uncredited");
}
