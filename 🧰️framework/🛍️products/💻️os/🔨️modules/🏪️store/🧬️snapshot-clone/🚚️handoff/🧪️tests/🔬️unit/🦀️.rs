use super::*;
use semio_framework_value::{
    list::PagedList,
    retained_clone::{RetainedClone, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep},
};


fn source(entries: usize) -> PagedList<String, 64> {
    let mut values = PagedList::default();
    for ordinal in 0..entries {
        while !values.has_reserved_slot() {
            let required = values.next_allocation_bytes().expect("handoff fixture capacity");
            assert!(values.reserve_one(required).expect("handoff fixture allocation").progressed);
        }
        values.push_reserved(format!("handoff-{ordinal:02}")).expect("handoff fixture reserved slot");
    }
    values
}

#[test]
fn partial_paged_cursor_is_transferred_and_closed_under_driver_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/📦️lifecycle/🔣️.json")).expect("retained cursor handoff fixture");
    let handoff = &fixture["handoff"];
    let entries = handoff["sourceEntries"].as_u64().expect("handoff source entries") as usize;
    let maximum_items = handoff["closeGrant"]["maximumItems"].as_u64().expect("handoff maximum items") as usize;
    let maximum_bytes = handoff["closeGrant"]["maximumBytes"].as_u64().expect("handoff maximum bytes") as usize;
    let source = source(entries);
    let capacity = RetainedCloneSource::<PagedList<String, 64>>::constructor_capacity_bytes::<()>();
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RetainedCloneSource::<PagedList<String, 64>>::constructor_copy_bytes(), maximum_capacity_bytes: capacity, maximum_depth: 1, ..Default::default() };
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| RetainedCloneSource::admit_owned(source, (), birth));
    let (mut retained, receipt) = result.unwrap_or_else(|_| panic!("handoff original source admission"));
    assert!(receipt.fits(birth));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (capacity, 0));
    let mut cursor = PagedList::<String, 64>::retained_clone_cursor();
    let step = cursor.advance(retained.borrow(), RetainedCloneGrant { maximum_items: 4, maximum_copy_bytes: 16, maximum_capacity_bytes: 4096, maximum_depth: 16, maximum_release_bytes: 4096 }).expect("partial retained cursor progresses");
    assert!(matches!(step, RetainedCloneStep::Progress(_)));

    let mut handoff = RetainedCloneCursorHandoff::<PagedList<String, 64>>::new(cursor);
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns < 256, "driver handoff must reach terminal emptiness");
        let copy = handoff.next_copy_byte_demand().unwrap();
        let capacity = handoff.next_capacity_byte_demand(copy).unwrap();
        assert!(copy + capacity <= maximum_bytes);
        let grant = RetainedCloneGrant { maximum_items, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: handoff.next_release_byte_demand().unwrap(), maximum_depth: handoff.next_depth_demand().unwrap() };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| handoff.close_step(grant).expect("handoff close respects its grant"));
        assert!(step.progress().fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(turns > 1);
    assert!(handoff.terminal_is_empty());
    for turn in 0..100_000 {
        if retained.terminal_is_empty() { println!("[DEBUG] handoff retained native cursor and original source reach exact independent-axis terminal custody in {turn} source turns"); return; }
        let copy = retained.next_close_copy_byte_demand().unwrap();
        let capacity = retained.next_close_capacity_byte_demand(copy).unwrap();
        assert!(copy + capacity <= maximum_bytes);
        let grant = RetainedCloneGrant { maximum_items, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: retained.next_close_release_byte_demand().unwrap(), maximum_depth: retained.next_close_depth_demand().unwrap() };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| retained.close_step(grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    panic!("handoff original source failed bounded closure");
}
