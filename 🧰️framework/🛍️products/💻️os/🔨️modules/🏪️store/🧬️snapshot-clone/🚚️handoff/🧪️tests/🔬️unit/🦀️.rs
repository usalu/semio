use super::*;
use semio_framework_value::{
    list::PagedList,
    retained_clone::{RetainedClone, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep},
};
use std::sync::Arc;

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
    let source = Arc::new(source(entries));
    let retained = RetainedCloneSource::from_authority(Arc::clone(&source), ());
    let mut cursor = PagedList::<String, 64>::retained_clone_cursor();
    let step = cursor.advance(retained.borrow(), RetainedCloneGrant { maximum_items: 4, maximum_copy_bytes: 16, maximum_capacity_bytes: 4096, maximum_depth: 16, maximum_release_bytes: 4096 }).expect("partial retained cursor progresses");
    assert!(matches!(step, RetainedCloneStep::Progress(_)));

    let mut handoff = RetainedCloneCursorHandoff::<PagedList<String, 64>>::new(cursor);
    drop(retained);
    drop(source);
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns < 256, "driver handoff must reach terminal emptiness");
        match handoff.close_step(maximum_items, maximum_bytes).expect("handoff close respects its grant") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= maximum_items);
                assert!(released_bytes <= maximum_bytes);
            }
            SnapshotRetirementStep::Blocked => panic!("exact handoff grant unexpectedly blocked"),
            SnapshotRetirementStep::Complete => break,
        }
    }
    assert!(turns > 1);
    assert!(handoff.terminal_is_empty());
}
