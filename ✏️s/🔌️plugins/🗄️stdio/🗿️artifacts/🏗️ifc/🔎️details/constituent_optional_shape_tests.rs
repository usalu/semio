//! 🧩️ Empty and optional declaration shape changes retain original displaced tuples through close.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/constituent-optional-shape.json")).unwrap()
}
fn source(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    Arc::new(IfcSnapshot { entities: vec![super::equal_arity_type_tests::entity(&contract()["sources"][row["source"].as_str().unwrap()], row["entityId"].as_str().unwrap_or("1").parse().unwrap())], ..Default::default() })
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    let path = row["path"].as_str().unwrap();
    let name = row["target"].as_str().unwrap();
    editing::SnapshotEditEvent::SetValue { path: path.into(), value: if path.ends_with("/name") { DslValue::String(name.into()) } else { DslValue::Object(vec![("$ifcType".into(), DslValue::String(name.into()))]) } }
}
pub(super) fn close_with_denials(caller: &mut OriginalWorkFixture, work: &mut Work) {
    work.begin_close();
    for _ in 0..8192 {
        if work.terminal_is_empty() {
            return;
        }
        let demand = work.close_demand(4096).unwrap();
        for axis in 0..5 {
            let mut denied = grant();
            match axis {
                0 => denied.maximum_items = 0,
                1 if demand.copy_bytes > 0 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1,
                3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1,
                4 if demand.depth > 0 => denied.maximum_depth = demand.depth - 1,
                _ => continue,
            }
            let metadata = work.constituent_roster.metadata();
            caller
                .turn(denied, |cx| {
                    cx.with_original_retirement(|cx| {
                        let receipt = cx.retained_progress();
                        let result = work.close_step(cx)?;
                        assert_eq!(receipt, cx.retained_progress());
                        Ok(result)
                    })
                })
                .unwrap();
            assert_eq!(metadata, work.constituent_roster.metadata());
        }
        caller.close(work, grant()).unwrap();
    }
    panic!("original tuple close did not drain");
}
#[test]
fn original_constituent_optional_shape_defaults_and_drops_match_neutral_output_inverse_and_denied_axes() {
    for row in contract()["cases"].as_array().unwrap() {
        let snapshot = source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for _ in 0..100000 {
                if work.constituent_roster.fallback() {
                    assert_eq!(row["retained"], false);
                    assert!(work.parts.is_none() && work.entity.is_none() && work.pending.is_none());
                    break;
                }
                let bridge = work.prepared && work.reference.complete() && work.enumeration.complete() && work.pending.is_none() && work.constituent_roster.phase() != 2;
                let domain = work.pending.is_some() && !work.domain_validated;
                if bridge || domain {
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
                        let command = work.original_event.as_ref().unwrap().to_value();
                        let turns = work.domain.turns();
                        caller
                            .work_turn(denied, fuel, |cx| {
                                let receipt = cx.retained_progress();
                                assert!(!matches!(work.step(input, cx)?, ArtifactCommandWorkStep::Complete(_)));
                                assert_eq!(receipt, cx.retained_progress());
                                Ok(())
                            })
                            .unwrap();
                        assert_eq!(metadata, work.constituent_roster.metadata());
                        assert_eq!(turns, work.domain.turns());
                        assert_eq!(command, work.original_event.as_ref().unwrap().to_value());
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
                assert!(work.entity.is_none() && work.semantic_emit.is_none());
            }
            if row["retained"] == true {
                let emit = result.expect("retained optional shape must complete").unwrap();
                assert!(work.constituent_roster.type_admission().is_some());
                assert!(work.domain.turns() > 3);
                let [IfcMutation::SetEntityComplex(change)] = emit.artifact_mutations.as_slice() else { panic!("one central complex primitive required") };
                assert_eq!(change.id, snapshot.entities[0].id);
                assert_eq!(change.complex[2].name, row["target"].as_str().unwrap().to_ascii_uppercase());
                let expected: Vec<_> = row["args"].as_array().unwrap().iter().map(super::equal_arity_type_tests::value).collect();
                assert!(ifc_arguments_identical(&change.complex[2].args, &expected));
                assert!(ifc_complex_identical(&change.complex[..2], &snapshot.entities[0].complex[..2]));
                let mutation = &emit.artifact_mutations[0];
                let mut next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                assert!(ifc_arguments_identical(&next.entities[0].args, &snapshot.entities[0].args));
                for inverse in mutation.inverse(&snapshot).unwrap() {
                    next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
                }
                assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
            } else {
                assert!(work.constituent_roster.fallback());
            }
            close_with_denials(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_optional_shape_cancels_and_closes_every_generated_and_displaced_tuple_prefix() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("drop-aggregate" | "grow-two-optional"))) {
        let snapshot = source(row);
        let mut total = 0;
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            for _ in 0..100000 {
                total += 1;
                if matches!(turn(caller, 1, false, |cx| work.step(input, cx)).unwrap(), ArtifactCommandWorkStep::Complete(_)) {
                    break;
                }
            }
            assert!(work.complete);
            close(caller, work);
        });
        caller.finish();
        for prefix in 0..total {
            let mut caller = OriginalWorkFixture::new();
            let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
            inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
                for _ in 0..prefix {
                    turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
                }
                assert!(turn(caller, 1, true, |cx| work.step(input, cx)).is_err());
                close(caller, work);
                assert!(work.terminal_is_empty());
                assert!(work.constituent_roster.type_admission().is_none());
            });
            caller.finish();
        }
    }
}
