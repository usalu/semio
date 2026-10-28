//! 🏷️ Primary complex zero-slot type variants preserve graph payloads without entity clones.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/primary-zero-slot-type.json")).unwrap()
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: DslValue::Object(vec![("$ifcType".into(), DslValue::String(row["value"]["$ifcType"].as_str().unwrap().into()))]) }
}
#[test]
fn original_primary_zero_slot_types_match_neutral_central_diff_inverse_and_denied_axes() {
    for row in contract()["cases"].as_array().unwrap() {
        let snapshot = super::constituent_roster_removal_tests::source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for _ in 0..100000 {
                let bridge = work.prepared && work.reference.complete() && work.enumeration.complete() && work.pending.is_none();
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
                assert!(work.entity.is_none() && work.parts.is_none() && work.semantic_emit.is_none());
                assert!(work.cursor.terminal_is_empty() && work.parts_cursor.terminal_is_empty());
            }
            let result = result.unwrap();
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(emit) = result {
                assert_eq!(emit.artifact_mutations.len(), row["mutations"].as_u64().unwrap() as usize);
                let mut next = snapshot.as_ref().clone();
                for mutation in &emit.artifact_mutations {
                    let IfcMutation::SetEntityName(change) = mutation else { panic!("central name primitive required") };
                    assert_eq!(change.id, snapshot.entities[0].id);
                    assert_eq!(change.name, row["name"].as_str().unwrap());
                    let editing::SnapshotEditEvent::SetValue { value, .. } = work.original_event.as_ref().unwrap() else { panic!("original marker retained") };
                    assert_eq!(requested_type(value), Some(""));
                    next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
                }
                assert_eq!(next.entities[0].name, row["name"].as_str().unwrap());
                assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
                for mutation in emit.artifact_mutations.iter().rev() {
                    for inverse in mutation.inverse(&snapshot).unwrap() {
                        next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
                    }
                }
                assert_eq!(next.entities[0].name, snapshot.entities[0].name);
                assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
            }
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_primary_zero_slot_types_leave_layout_and_default_generation_on_explicit_frontier() {
    for kind in contract()["fallback"].as_array().unwrap() {
        let mut row = serde_json::json!({"value":{"$ifcType":"IFCGEOMETRICREPRESENTATIONITEM"}});
        let mut snapshot = super::constituent_roster_removal_tests::source(&row);
        let entity = &mut Arc::get_mut(&mut snapshot).unwrap().entities[0];
        match kind.as_str().unwrap() {
            "simple-root-layout" => entity.complex.clear(),
            "new-own-attributes" => row["value"]["$ifcType"] = serde_json::json!("IFCCARTESIANPOINT"),
            "old-own-attributes" => entity.name = "IFCCARTESIANPOINT".into(),
            "source-surplus-arguments" => entity.args.push(IfcValue::Unset),
            "unknown-new-type" => row["value"]["$ifcType"] = serde_json::json!("IFCUNKNOWN"),
            "derived-survivor" => entity.complex[0].name = "IFCSIUNIT".into(),
            "global-id-survivor" => entity.complex[0].name = "IFCROOT".into(),
            "derived-direct-value" => entity.complex[2].args[0] = IfcValue::Derived,
            "lowercase-survivor-name" => entity.complex[0].name = "IfcGeometricRepresentationItem".into(),
            _ => panic!("unknown fallback"),
        };
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
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
fn original_primary_zero_slot_type_cancels_and_closes_every_preparation_prefix() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("primary-variant" | "same-type"))) {
        let snapshot = super::constituent_roster_removal_tests::source(row);
        let mut total = 0;
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
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
            let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
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
