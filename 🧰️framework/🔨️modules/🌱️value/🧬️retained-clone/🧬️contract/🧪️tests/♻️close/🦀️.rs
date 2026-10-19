//! ♻️ Actual clone close-frame private custody preserves original terminal release laws.
use semio_framework_test_fixtures::fixture_text;
use super::*;
#[test]
fn cold_clone_terminal_frame_denies_partial_funding_and_reports_actual_same_turn_release() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🗺️ordered-map/🧪️tests/🧫️fixtures/📏️close/🔣️.json")).unwrap();
    let admission = fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize;
    let mut mismatches = 0;
    for row in fixture["cases"].as_array().unwrap() {
        let mut text = String::with_capacity(row["capacity"].as_u64().unwrap() as usize);
        text.push_str(row["text"].as_str().unwrap());
        let mut close = RetainedCloneClose::default();
        let mut text=Some(text);
        let birth=crate::owned_retirement_birth_bytes::<String>();
        assert!(birth<=admission);
        close.begin_granted(&mut text,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth,maximum_depth:1,..Default::default()}).unwrap();
        assert!(text.is_none());
        for _ in 0..10_000 {
            if close.retirement.as_ref().unwrap().terminal_is_empty() { break; }
            let capacity=close.next_capacity_byte_demand(7).unwrap();
            let release=close.next_release_byte_demand().unwrap();
            assert!(capacity<=admission&&release<=admission);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:close.next_depth_demand().unwrap()};
            let (step,events)=crate::observe_retirement_allocations(||close.step_granted(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(events,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        }
        assert!(close.retirement.as_ref().unwrap().terminal_is_empty());
        let pointer = close.retirement.as_ref().unwrap().as_ref() as *const dyn crate::ErasedSnapshotRetirement;
        let (physical, query_events) = crate::observe_retirement_allocations(|| close.next_release_byte_demand().unwrap());
        assert_eq!(query_events, (0, 0));
        assert!(physical > 0 && physical <= admission);
        for (items, bytes) in [(0, physical), (1, physical - 1)] {
            let (step, events) = crate::observe_retirement_allocations(|| close.step_granted(RetainedCloneGrant{maximum_items:items,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()}).unwrap());
            let retained = close.retirement.as_ref().is_some_and(|owner| std::ptr::addr_eq(pointer, owner.as_ref() as *const dyn crate::ErasedSnapshotRetirement));
            if step != (RetainedCloneStep::Progress(Default::default())) || events != (0, 0) || !retained {
                mismatches += 1;
            }
            if close.is_empty() { break; }
        }
        if !close.is_empty() {
            let (step, events) = crate::observe_retirement_allocations(|| close.step_granted(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:physical,maximum_depth:1,..Default::default()}).unwrap());
            if step != (RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress{copied_items:1,released_bytes:physical,..Default::default()})) || events != (0, physical) { mismatches += 1; }
        }
        assert!(close.is_empty());
        assert_eq!(close.step_granted(Default::default()).unwrap(), RetainedCloneStep::Complete(Default::default()));
    }
    assert_eq!(mismatches, 0, "cold terminal retirement Box requires its complete same-turn physical grant");
}

