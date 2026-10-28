//! 🪪️ Direct original IDs stay exact while empty/default identity work keeps its original frontier.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/normalization-identity.json")).unwrap()
}
fn source(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    let global = match row["globalId"]["kind"].as_str() {
        Some("string") => IfcValue::String(row["globalId"]["value"].as_str().unwrap().into()),
        Some("enum") => IfcValue::Enum(row["globalId"]["value"].as_str().unwrap().into()),
        _ => IfcValue::Unset,
    };
    let args = if row["missing"] == true { vec![] } else { vec![global, IfcValue::Unset, IfcValue::String("建物🌍".into()), IfcValue::Unset] };
    let complex = contract()["parts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|part| IfcComplexType {
            name: part["name"].as_str().unwrap().into(),
            args: part["args"].as_array().unwrap().iter().map(|arg| if arg["kind"] == "enum" { IfcValue::Enum(arg["value"].as_str().unwrap().into()) } else { IfcValue::Unset }).collect(),
        })
        .collect();
    Arc::new(IfcSnapshot { entities: vec![IfcEntity { id: row["entityId"].as_str().unwrap_or("1").parse().unwrap(), name: "IFCROOT".into(), args, complex }], ..Default::default() })
}
fn event() -> editing::SnapshotEditEvent {
    editing::SnapshotEditEvent::InsertValue { path: "/entities/0/complex/-".into(), value: DslValue::Object(vec![("name".into(), DslValue::String("IFCREPRESENTATIONITEM".into())), ("args".into(), DslValue::Array(vec![]))]) }
}
#[test]
fn original_direct_identity_guard_matches_neutral_roles_and_central_inverse_under_denied_axes() {
    for row in contract()["cases"].as_array().unwrap() {
        let snapshot = source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::INSERT_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for _ in 0..100000 {
                if work.constituent_roster.fallback() {
                    assert_eq!(row["retained"], false);
                    assert!(work.parts.is_none() && work.pending.is_none());
                    break;
                }
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
                        let command = work.original_event.as_ref().unwrap().to_value();
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
                if let ArtifactCommandWorkStep::Complete(emit) = step.unwrap() {
                    result = Some(emit);
                    break;
                }
                assert_eq!(work.reduction_ticks(), 0);
                assert!(work.entity.is_none() && work.semantic_emit.is_none());
            }
            if row["retained"] == true {
                let emit = result.expect("populated original identity must reach retained central output");
                assert_eq!(emit.artifact_mutations.len(), 1);
                let mutation = &emit.artifact_mutations[0];
                let mut next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                assert!(ifc_arguments_identical(&next.entities[0].args, &snapshot.entities[0].args));
                assert_eq!(next.entities[0].complex.len(), snapshot.entities[0].complex.len() + 1);
                for inverse in mutation.inverse(&snapshot).unwrap() {
                    next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
                }
                assert!(ifc_arguments_identical(&next.entities[0].args, &snapshot.entities[0].args));
                assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
            } else {
                assert!(work.constituent_roster.fallback());
            }
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_direct_identity_guard_cancels_and_closes_every_metadata_and_name_prefix() {
    let contract = contract();
    for row in contract["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("populated-original-id" | "empty-id-requires-generation"))) {
        let snapshot = source(row);
        let mut total = 0;
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::INSERT_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(), &mut caller, &mut work, |input, caller, work| {
            while work.pending.is_none() && !work.constituent_roster.fallback() {
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
            inputs(&snapshot, event(), &mut caller, &mut work, |input, caller, work| {
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
