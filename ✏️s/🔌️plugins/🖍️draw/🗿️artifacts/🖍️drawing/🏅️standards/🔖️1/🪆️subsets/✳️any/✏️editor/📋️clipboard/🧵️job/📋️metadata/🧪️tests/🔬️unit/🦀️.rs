//! 🧪️ Metadata pages and keys are funded before their first allocation and survive logical removal.
use semio_framework_job::OriginalRetirementPump;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::{retirement::controlled::ControlledRetirement, RetainedCloneGrant, RetainedWorkPoll};
use semio_s_artifact_draw_drawing_editor::clipboard_job::metadata::Table;

#[test]
fn clipboard_metadata_replays_neutral_json_patch_membership_and_exact_backing() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../📋️clipboard/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let (mut table, heap) = observe(Table::<()>::new);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let mut keys = Vec::<String>::new();
    let mut retained = 0;
    for row in fixture["metadataCases"].as_array().unwrap() {
        let key = row["key"].as_str().unwrap();
        let inserting = row["operation"] == "insert";
        let fresh = inserting && !keys.iter().any(|candidate| candidate == key);
        let ((added, live), heap) = observe(|| {
            let added = if inserting { table.insert_key(key).unwrap() } else { table.remove(key); false };
            (added, table.len())
        });
        assert!(!heap.overflowed);
        assert_eq!(heap.released_bytes, 0);
        if fresh {
            assert!(heap.requested_bytes >= key.len());
            keys.push(key.into());
        } else {
            assert_eq!(heap.requested_bytes, 0);
        }
        retained += heap.requested_bytes;
        assert_eq!(added, row["added"].as_bool().unwrap());
        assert_eq!(live, row["live"].as_array().unwrap().len());
        let mut actual = keys.iter().filter(|key| table.contains(key)).cloned().collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual, serde_json::from_value::<Vec<String>>(row["live"].clone()).unwrap());
    }
    let mut owner = ControlledRetirement::retain(table);
    let pump = OriginalRetirementPump::new();
    for _ in 0..10_000 {
        if owner.terminal_is_empty() { break; }
        let body = owner.next_copy_byte_demand().unwrap().max(owner.next_release_byte_demand().unwrap());
        let demand = owner.next_demand(body).unwrap();
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: demand.copy_bytes,
            maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes,
            maximum_depth: demand.depth,
        };
        let mut turn = pump.turn(1 << 20, u64::MAX, || Some(0), grant).unwrap();
        let (poll, heap) = observe(|| turn.with_effect(|effect| owner.step(body, effect)).unwrap().unwrap().0);
        let RetainedWorkPoll::Complete(step) = poll else { panic!("Funded metadata retirement stayed pending"); };
        let progress = step.result.unwrap().progress();
        assert!(progress.fits(grant));
        assert!(!heap.overflowed);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        retained = (retained + heap.requested_bytes).checked_sub(heap.released_bytes).unwrap();
    }
    assert!(owner.terminal_is_empty());
    assert_eq!(retained, 0);
    let (_, heap) = observe(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
}
