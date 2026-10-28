//! ➕️ Original whole constituent add controls retain name ownership and central insertion preimages.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/zero-slot-constituent-insertion.json")).unwrap()
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    editing::SnapshotEditEvent::InsertValue {
        path: row["path"].as_str().unwrap().into(),
        value: DslValue::Object(vec![
            ("name".into(), DslValue::String(row["value"]["name"].as_str().unwrap().into())),
            ("args".into(), DslValue::Array(if row["value"]["args"].as_array().unwrap().is_empty() { vec![] } else { vec![IfcValue::Unset.to_value()] })),
        ]),
    }
}
#[test]
fn original_zero_slot_whole_constituent_insertions_match_neutral_diff_inverse_and_denied_axes() {
    for row in contract()["cases"].as_array().unwrap() {
        let snapshot = super::constituent_roster_removal_tests::source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::INSERT_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for _ in 0..100000 {
                let bridge = work.prepared && work.reference.complete() && work.enumeration.complete() && work.pending.is_none() && work.constituent_roster.phase() != 2;
                if bridge {
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
                        let metadata = work.constituent_roster.metadata();
                        let original = work.original_event.as_ref().unwrap().to_value();
                        caller
                            .work_turn(denied, fuel, |cx| {
                                let receipt = cx.retained_progress();
                                let step = work.step(input, cx)?;
                                assert!(!matches!(step, ArtifactCommandWorkStep::Complete(_)));
                                assert_eq!(receipt, cx.retained_progress());
                                Ok(())
                            })
                            .unwrap();
                        assert_eq!(metadata, work.constituent_roster.metadata());
                        assert_eq!(original, work.original_event.as_ref().unwrap().to_value());
                    }
                }
                let step = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| {
                    let receipt = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.step(input, cx));
                    if bridge {
                        let after = cx.retained_progress();
                        assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - receipt.retained_capacity_bytes, after.released_bytes - receipt.released_bytes));
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
                assert!(work.entity.is_none() && work.semantic_emit.is_none() && work.cursor.terminal_is_empty());
            }
            let result = result.unwrap();
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(emit) = result {
                assert_eq!(emit.artifact_mutations.len(), 1);
                let mutation = &emit.artifact_mutations[0];
                let IfcMutation::SetEntityComplex(change) = mutation else { panic!("central complex primitive required") };
                let inserted = IfcComplexType { name: row["value"]["name"].as_str().unwrap().to_ascii_uppercase(), args: vec![] };
                let expected = row["order"].as_array().unwrap().iter().map(|index| if index.as_i64() == Some(-1) { inserted.clone() } else { snapshot.entities[0].complex[index.as_u64().unwrap() as usize].clone() }).collect::<Vec<_>>();
                assert!(ifc_complex_identical(&change.complex, &expected));
                let editing::SnapshotEditEvent::InsertValue { value, .. } = work.original_event.as_ref().unwrap() else { panic!("original event retained") };
                assert_eq!(value.get("name").and_then(DslValue::as_str), Some(""));
                let mut next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                assert!(ifc_complex_identical(&next.entities[0].complex, &expected));
                for inverse in mutation.inverse(&snapshot).unwrap() {
                    next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
                }
                assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
            }
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_zero_slot_whole_constituent_insertion_preserves_layout_and_normalization_frontier() {
    for kind in contract()["fallback"].as_array().unwrap() {
        let mut row = serde_json::json!({"path":"/entities/0/complex/1","value":{"name":"IFCREPRESENTATIONITEM","args":[]}});
        let mut snapshot = super::constituent_roster_removal_tests::source(&row);
        let entity = &mut Arc::get_mut(&mut snapshot).unwrap().entities[0];
        match kind.as_str().unwrap() {
            "first-constituent-layout" => {
                entity.complex.clear();
                row["path"] = serde_json::json!("/entities/0/complex/0");
            }
            "candidate-own-attributes" => row["value"]["name"] = serde_json::json!("IFCCARTESIANPOINT"),
            "candidate-arguments" => row["value"]["args"] = serde_json::json!([{}]),
            "unknown-candidate" => row["value"]["name"] = serde_json::json!("IFCUNKNOWN"),
            "candidate-global-id" => row["value"]["name"] = serde_json::json!("IFCROOT"),
            "derived-survivor" => entity.complex[0].name = "IFCSIUNIT".into(),
            "global-id-root" => entity.name = "IFCROOT".into(),
            "derived-direct-value" => entity.complex[2].args[0] = IfcValue::Derived,
            "lowercase-survivor-name" => entity.complex[0].name = "IfcGeometricRepresentationItem".into(),
            _ => panic!("unknown fallback"),
        };
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::INSERT_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(&row), &mut caller, &mut work, |input, caller, work| {
            for index in 0..10000 {
                if work.constituent_roster.fallback() {
                    break;
                }
                turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
                assert!(work.entity.is_none() && work.parts.is_none() && work.pending.is_none());
                assert!(index < 9999);
            }
            assert!(work.constituent_roster.fallback());
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_zero_slot_whole_constituent_insertion_cancels_and_closes_every_transfer_prefix() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("first" | "middle" | "dash"))) {
        let snapshot = super::constituent_roster_removal_tests::source(row);
        let mut total = 0;
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::INSERT_SNAPSHOT_VALUE_ACTION_ID);
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
            let mut work = Work::new(editing::INSERT_SNAPSHOT_VALUE_ACTION_ID);
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
