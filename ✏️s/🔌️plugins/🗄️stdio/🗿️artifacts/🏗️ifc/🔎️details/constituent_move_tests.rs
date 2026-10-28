//! 🔀️ Whole constituent moves preserve original values, exact identity and central reversibility.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/constituent-move.json")).unwrap()
}
fn source(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    let parts = contract()["names"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(index, name)| IfcComplexType { name: name.as_str().unwrap().into(), args: if index == 2 { vec![IfcValue::Aggregate(vec![IfcValue::Real(f64::from_bits(9223372036854775808)), IfcValue::Real(1.0)])] } else { vec![] } })
        .collect();
    let selected = IfcEntity { id: row["entityId"].as_str().unwrap_or("1").parse().unwrap(), name: "IFCREPRESENTATIONITEM".into(), args: vec![], complex: parts };
    let mut other = selected.clone();
    if row["duplicate"] != true {
        other.id = 2;
    }
    Arc::new(IfcSnapshot { entities: vec![selected, other], ..Default::default() })
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    editing::SnapshotEditEvent::MoveValue { path: row["path"].as_str().unwrap().into(), from: row["from"].as_str().unwrap().into() }
}
#[test]
fn original_constituent_move_preserves_neutral_words_and_central_inverse_under_denied_axes() {
    for row in contract()["cases"].as_array().unwrap() {
        let snapshot = source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::MOVE_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for turn in 0..100000 {
                if work.prepared && work.reference.complete() && work.enumeration.complete() && work.pending.is_none() && work.constituent_roster.phase() != 2 {
                    let demand = work.work_demands(input, 4096).unwrap();
                    for axis in 0..6 {
                        let mut denied = grant();
                        let mut fuel = 1;
                        match axis {
                            0 => denied.maximum_items = 0,
                            1 if demand.copy_bytes > 0 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                            2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1,
                            3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1,
                            4 => denied.maximum_depth = 0,
                            5 => fuel = 0,
                            _ => continue,
                        }
                        let state = work.constituent_roster.metadata();
                        caller
                            .work_turn(denied, fuel, |cx| {
                                let before = cx.retained_progress();
                                let step = work.step(input, cx)?;
                                assert!(!matches!(step, ArtifactCommandWorkStep::Complete(_)));
                                assert_eq!(before, cx.retained_progress());
                                Ok(())
                            })
                            .unwrap();
                        assert_eq!(state, work.constituent_roster.metadata());
                    }
                }
                let bridge = work.prepared && work.reference.complete() && work.enumeration.complete() && work.pending.is_none() && work.constituent_roster.phase() != 2;
                let step = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.step(input, cx));
                    if bridge {
                        let after = cx.retained_progress();
                        assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                    }
                    result
                });
                assert_eq!(work.reduction_ticks(), 0);
                assert!(work.entity.is_none() && work.semantic_emit.is_none() && work.cursor.terminal_is_empty());
                match step {
                    Ok(ArtifactCommandWorkStep::Complete(emit)) => {
                        result = Some(Ok(emit));
                        break;
                    }
                    Err(error) => {
                        result = Some(Err(error));
                        break;
                    }
                    _ => {}
                }
                assert!(turn < 99999);
            }
            let result = result.unwrap();
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(emit) = result {
                assert_eq!(emit.artifact_mutations.len(), 1);
                let mutation = &emit.artifact_mutations[0];
                let IfcMutation::SetEntityComplex(change) = mutation else { panic!("central complex primitive required") };
                assert_eq!(change.id, snapshot.entities[0].id);
                let expected = row["order"].as_array().unwrap().iter().map(|index| snapshot.entities[0].complex[index.as_u64().unwrap() as usize].clone()).collect::<Vec<_>>();
                assert!(ifc_complex_identical(&change.complex, &expected));
                let next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                assert!(ifc_complex_identical(&next.entities[0].complex, &expected));
                assert!(ifc_complex_identical(&next.entities[1].complex, &snapshot.entities[1].complex));
                let mut restored = next;
                for inverse in mutation.inverse(&snapshot).unwrap() {
                    restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap();
                }
                assert!(ifc_complex_identical(&restored.entities[0].complex, &snapshot.entities[0].complex));
                assert_eq!(restored, *snapshot);
            }
            caller.cancel().cancel_now();
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
        assert_eq!(Arc::strong_count(&snapshot), 1);
    }
}
#[test]
fn original_constituent_move_cancels_every_roster_prefix_and_closes_partial_owners() {
    let contract = contract();
    for row in contract["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("forward" | "backward" | "dash"))) {
        let snapshot = source(row);
        let original = snapshot.as_ref().clone();
        let mut turns = 0;
        for cancel_at in std::iter::once(None).chain((1..100000).map(Some)) {
            if cancel_at.is_some_and(|turn| turn > turns) {
                break;
            }
            let mut caller = OriginalWorkFixture::new();
            let mut work = Work::new(editing::MOVE_SNAPSHOT_VALUE_ACTION_ID);
            inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
                for turn in 1..100000 {
                    if cancel_at == Some(turn) {
                        caller.cancel().cancel_now();
                    }
                    let result = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx));
                    assert_eq!(snapshot.as_ref(), &original);
                    assert_eq!(work.reduction_ticks(), 0);
                    if cancel_at == Some(turn) {
                        assert!(result.is_err());
                        break;
                    }
                    if let ArtifactCommandWorkStep::Complete(emit) = result.unwrap() {
                        assert_eq!(emit.artifact_mutations.len(), 1);
                        turns = turn;
                        break;
                    }
                    assert!(turn < 99999);
                }
                caller.cancel().cancel_now();
                close(caller, work);
                assert!(work.terminal_is_empty());
            });
            caller.finish();
            assert_eq!(Arc::strong_count(&snapshot), 1);
        }
        assert!(turns > 20);
    }
}

#[test]
fn original_constituent_move_rejects_substitute_event_snapshot_address_receiver_and_caller() {
    let contract = contract();
    let row = &contract["cases"][0];
    let snapshot = source(row);
    let alternate = Arc::new(snapshot.as_ref().clone());
    let mut alternate_event = event(row);
    let alternate_address = pointer::PathAdmission::default();
    let mut alternate_pending = None;
    let mut caller = OriginalWorkFixture::new();
    let mut work = Work::new(editing::MOVE_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
        for turn in 0..100000 {
            if work.constituent_roster.phase() == 1 {
                break;
            }
            caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx)).unwrap();
            assert!(turn < 99999);
        }
        for variant in 0..4 {
            let before = work.constituent_roster.metadata();
            caller
                .work_turn(grant(), 1, |cx| {
                    let receipt = cx.retained_progress();
                    let result = work.constituent_roster.advance::<DetailsTestProfile>(
                        if variant == 0 { &mut alternate_event } else { work.original_event.as_mut().unwrap() },
                        if variant == 1 { alternate.as_ref() } else { snapshot.as_ref() },
                        if variant == 2 { &alternate_address } else { &work.address },
                        work.source.as_ref(),
                        &mut work.parts_cursor,
                        &mut work.parts,
                        if variant == 3 { &mut alternate_pending } else { &mut work.pending },
                        &mut work.value_cursor,
                        &mut work.value,
                        cx,
                    );
                    assert!(result.is_err());
                    assert_eq!(receipt, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            assert_eq!(before, work.constituent_roster.metadata());
        }
        let mut other = OriginalWorkFixture::new();
        other
            .work_turn(grant(), 1, |cx| {
                let receipt = cx.retained_progress();
                assert!(work.constituent_roster.advance::<DetailsTestProfile>(work.original_event.as_mut().unwrap(), snapshot.as_ref(), &work.address, work.source.as_ref(), &mut work.parts_cursor, &mut work.parts, &mut work.pending, &mut work.value_cursor, &mut work.value, cx).is_err());
                assert_eq!(receipt, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        other.finish();
        caller.cancel().cancel_now();
        close(caller, work);
        assert!(work.terminal_is_empty());
    });
    caller.finish();
    assert_eq!(Arc::strong_count(&snapshot), 1);
    assert!(alternate_pending.is_none());
}
