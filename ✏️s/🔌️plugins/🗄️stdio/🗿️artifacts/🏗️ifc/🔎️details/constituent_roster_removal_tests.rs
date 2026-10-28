//! 🗑️ Whole constituent removal retains exact source payloads and central reversibility.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/constituent-roster-removal.json")).unwrap()
}
pub(super) fn source(row: &serde_json::Value) -> Arc<IfcSnapshot> {
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
    editing::SnapshotEditEvent::RemoveValue { path: row["path"].as_str().unwrap().into() }
}
#[test]
fn original_whole_constituent_removal_preserves_normalization_frontier() {
    for kind in contract()["fallback"].as_array().unwrap() {
        let row = serde_json::json!({"path":"/entities/0/complex/0"});
        let mut snapshot = source(&row);
        let selected = &mut Arc::get_mut(&mut snapshot).unwrap().entities[0];
        match kind.as_str().unwrap() {
            "last-constituent" => selected.complex.truncate(1),
            "derived-declaration" => selected.complex[1].name = "IFCSIUNIT".into(),
            "global-id-declaration" => selected.name = "IFCROOT".into(),
            "derived-direct-value" => selected.complex[2].args[0] = IfcValue::Derived,
            "lowercase-survivor-name" => selected.complex[1].name = "IfcPoint".into(),
            "unknown-declaration" => selected.complex[1].name = "IFCUNKNOWN".into(),
            _ => panic!("unknown fallback law"),
        };
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(&row), &mut caller, &mut work, |input, caller, work| {
            for turn_index in 0..10000 {
                if work.constituent_roster.fallback() {
                    break;
                }
                turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
                assert!(work.pending.is_none() && work.entity.is_none());
                assert!(turn_index < 9999);
            }
            assert!(work.constituent_roster.fallback());
            assert!(work.parts.is_none());
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_whole_constituent_removal_matches_neutral_central_diff_inverse_and_denied_axes() {
    for row in contract()["cases"].as_array().unwrap() {
        let snapshot = source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for _ in 0..100000 {
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
                assert_eq!(work.reduction_ticks(), 0);
                assert!(work.entity.is_none() && work.semantic_emit.is_none());
            }
            let result = result.unwrap();
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(emit) = result {
                assert_eq!(emit.artifact_mutations.len(), 1);
                let mutation = &emit.artifact_mutations[0];
                let IfcMutation::SetEntityComplex(change) = mutation else { panic!("central complex mutation required") };
                let expected = row["order"].as_array().unwrap().iter().map(|index| snapshot.entities[0].complex[index.as_u64().unwrap() as usize].clone()).collect::<Vec<_>>();
                assert!(ifc_complex_identical(&change.complex, &expected));
                let mut next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                assert!(ifc_complex_identical(&next.entities[0].complex, &expected));
                for inverse in mutation.inverse(&snapshot).unwrap() {
                    next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
                }
                assert_eq!(&next, snapshot.as_ref());
            }
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_whole_constituent_removal_cancels_and_closes_every_preparation_prefix() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("first" | "middle" | "last"))) {
        let snapshot = source(row);
        let mut total = 0;
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            while work.pending.is_none() {
                turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
                total += 1;
                assert!(total < 100000);
            }
            close(caller, work);
        });
        caller.finish();
        for prefix in 0..=total {
            let mut caller = OriginalWorkFixture::new();
            let mut work = Work::new(editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID);
            inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
                for _ in 0..prefix {
                    turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
                }
                assert!(turn(caller, 1, true, |cx| work.step(input, cx)).is_err());
                close(caller, work);
                assert!(work.terminal_is_empty());
            });
            caller.finish();
        }
    }
}
