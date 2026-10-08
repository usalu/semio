use super::*;

fn observed(ledger: &ArtifactHistoryLedger<i32>) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(ledger)).expect("independent parser sees the selected exact history")
}

#[test]
fn retained_group_history_switches_every_direct_reader_at_one_decision() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️group-history.json")).expect("group history fixture");
    let mut first = ArtifactHistoryLedger::new();
    let mut second = ArtifactHistoryLedger::new();
    first.try_push(0).expect("first seed");
    second.try_push(-1).expect("second seed");
    let mut owner = ArtifactGroupVisibilityOwner::new();
    let view = owner.view();
    let mut keys = Vec::new();
    for row in fixture["ordered"].as_array().expect("ordered fixture") {
        let ledger = if row["member"] == "a" { &mut first } else { &mut second };
        let reservation = ledger.reserve_group_one(&view).expect("exact suffix reservation");
        let key = ledger.stage_group_reserved(reservation, row["value"].as_i64().expect("value") as i32, &view).expect("one prepared history owner");
        assert_eq!(ledger.get_key(key), None);
        keys.push((row["member"].as_str().expect("member"), key, row["value"].as_i64().expect("value") as i32));
        let seed = ledger.key_at(0).expect("visible seed generation");
        assert!(ledger.get_key(seed).is_some());
        assert_eq!(observed(&first), fixture["members"][0]["before"]);
        assert_eq!(observed(&second), fixture["members"][1]["before"]);
        assert_eq!((first.len(), first.first(), first.last(), first.get(1)), (1, Some(&0), Some(&0), None));
        assert_eq!(first.iter().rev().copied().collect::<Vec<_>>(), vec![0]);
        assert_eq!(first.reserve_one().unwrap_err(), ArtifactHistoryReservationFault::GroupUnavailable);
        assert_eq!(first.last_mut(), None);
    }
    assert!(owner.commit());
    assert!(!owner.commit());
    assert!(!owner.abort());
    for (member, key, value) in keys {
        let ledger = if member == "a" { &first } else { &second };
        assert_eq!(ledger.get_key(key), Some(&value));
    }
    assert_eq!(observed(&first), fixture["members"][0]["after"]);
    assert_eq!(observed(&second), fixture["members"][1]["after"]);
    assert_eq!((first.len(), first.last(), first.get(1)), (3, Some(&42), Some(&17)));
    assert_eq!(first.iter().rev().copied().collect::<Vec<_>>(), vec![42, 17, 0]);
    assert!(first.abort_group_one(&view).is_err());
    first.adopt_group(&view).expect("first non-publishing adoption");
    assert_eq!(observed(&second), fixture["members"][1]["after"]);
    second.adopt_group(&view).expect("second non-publishing adoption");
    assert_eq!(observed(&first), fixture["members"][0]["after"]);
    while first.pop().is_some() {}
    while second.pop().is_some() {}
    assert!(first.terminal_is_empty() && second.terminal_is_empty());
}

#[test]
fn retained_group_history_abort_transfers_one_exact_owner_and_rejects_foreign_decisions() {
    let mut ledger = ArtifactHistoryLedger::new();
    ledger.try_push(0).expect("seed");
    let mut owner = ArtifactGroupVisibilityOwner::new();
    let view = owner.view();
    let mut foreign = ArtifactGroupVisibilityOwner::new();
    let wrong = foreign.view();
    let mut keys = Vec::new();
    for value in [17, 42] {
        let reservation = ledger.reserve_group_one(&view).expect("exact group reservation");
        keys.push(ledger.stage_group_reserved(reservation, value, &view).expect("exact staged owner"));
    }
    assert!(ledger.reserve_group_one(&wrong).is_err());
    assert!(foreign.abort());
    assert!(ledger.abort_group_one(&wrong).is_err());
    assert!(ledger.abort_group_one(&view).is_err());
    assert!(owner.abort());
    assert!(!owner.commit());
    assert_eq!(ledger.abort_group_one(&view), Ok(Some(42)));
    assert_eq!(observed(&ledger), serde_json::json!([0]));
    assert_eq!(ledger.abort_group_one(&view), Ok(Some(17)));
    for key in keys { assert_eq!(ledger.get_key(key), None); }
    assert!(!ledger.terminal_is_empty());
    assert_eq!(ledger.abort_group_one(&view), Ok(None));
    assert_eq!(ledger.pop(), Some(0));
    assert!(ledger.terminal_is_empty());
}

#[test]
fn original_history_ledger_retires_slot_visibility_before_whole_pages() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/📸️paged-history-ledger/🔣️.json")).unwrap();
    let count = fixture["pushes"].as_u64().unwrap() as usize;
    let mut publisher = ArtifactGroupVisibilityOwner::new();
    let view = publisher.view();
    let original = std::sync::Arc::as_ptr(&view);
    let (mut ledger, born, freed) = crate::test_allocation::observe_backing(|| {
        let mut ledger = ArtifactHistoryLedger::<i32>::new();
        for value in 0..count {
            let reservation = ledger.reserve_group_one(&view).unwrap();
            ledger.stage_group_reserved(reservation, value as i32, &view).unwrap();
        }
        ledger
    });
    assert_eq!(freed, 0);
    assert!(publisher.commit());
    ledger.adopt_group(&view).unwrap();
    let oracle = serde_json::to_value((0..count).map(|value| value as i32).collect::<Vec<_>>()).unwrap();
    assert_eq!(observed(&ledger), oracle);
    for expected in (0..count).rev() { assert_eq!(ledger.pop(), Some(expected as i32)); }
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 };
    while ledger.retained_terminal_slot_count() != 0 {
        assert_eq!(std::sync::Arc::as_ptr(ledger.next_terminal_slot_visibility().unwrap().unwrap()), original);
        let slots = ledger.retained_terminal_slot_count();
        let (_, allocated, released) = crate::test_allocation::observe_backing(|| assert!(ledger.retire_terminal_slot(RetainedCloneGrant { maximum_items: 0, ..grant }).is_err()));
        assert_eq!((allocated, released), (0, 0));
        assert_eq!(ledger.retained_terminal_slot_count(), slots);
        let (result, allocated, released) = crate::test_allocation::observe_backing(|| ledger.retire_terminal_slot(grant));
        let (visibility, progress) = result.unwrap();
        assert_eq!(std::sync::Arc::as_ptr(visibility.as_ref().unwrap()), original);
        assert_eq!(progress, RetainedCloneProgress { copied_items: 1, ..Default::default() });
        assert_eq!((allocated, released), (0, 0));
        let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(visibility));
        assert_eq!((allocated, released), (0, 0));
    }
    assert_eq!(std::sync::Arc::strong_count(&view), 2);
    let mut physical = 0;
    while !ledger.backing_is_empty() {
        let demand = ledger.next_empty_page_release_byte_demand().unwrap();
        assert!(demand != 0);
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| ledger.release_empty_page(RetainedCloneGrant { maximum_release_bytes: demand - 1, ..grant }));
        assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
        assert_eq!((allocated, released), (0, 0));
        assert_eq!(ledger.next_empty_page_release_byte_demand().unwrap(), demand);
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| ledger.release_empty_page(RetainedCloneGrant { maximum_release_bytes: demand, ..grant }));
        assert_eq!(step.unwrap().progress().released_bytes, demand);
        assert_eq!((allocated, released), (0, demand));
        physical += demand;
    }
    assert_eq!(physical, born);
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(ledger));
    assert_eq!((allocated, released), (0, fixture["retirement"]["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] original ledger slots={count} same visibility pointers; System birth={born} exact pages={physical} serde_json rows equal; terminalDrop0heap");
}
