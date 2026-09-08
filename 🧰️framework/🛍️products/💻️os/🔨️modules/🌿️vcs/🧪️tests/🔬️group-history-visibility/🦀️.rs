use super::*;

fn observed(ledger: &ArtifactHistoryLedger<i32>) -> serde_json::Value {
    serde_json::from_str(&crate::os_pack::json::to_json_string(ledger)).expect("independent parser sees the selected exact history")
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
    for row in fixture["ordered"].as_array().expect("ordered fixture") {
        let ledger = if row["member"] == "a" { &mut first } else { &mut second };
        let reservation = ledger.reserve_group_one(&view).expect("exact suffix reservation");
        ledger.stage_group_reserved(reservation, row["value"].as_i64().expect("value") as i32, &view).expect("one prepared history owner");
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
    for value in [17, 42] {
        let reservation = ledger.reserve_group_one(&view).expect("exact group reservation");
        ledger.stage_group_reserved(reservation, value, &view).expect("exact staged owner");
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
    assert!(!ledger.terminal_is_empty());
    assert_eq!(ledger.abort_group_one(&view), Ok(None));
    assert_eq!(ledger.pop(), Some(0));
    assert!(ledger.terminal_is_empty());
}
