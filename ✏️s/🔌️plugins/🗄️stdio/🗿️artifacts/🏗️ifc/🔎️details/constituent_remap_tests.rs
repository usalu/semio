//! 🧬️ Metadata remapping preserves compatible original values and refuses missing required slots.
use super::*;
fn expression(input: &serde_json::Value) -> IfcTypeExpression {
    let text = |key: &str| input[key].as_str().map(str::to_owned);
    IfcTypeExpression {
        kind: input["kind"].as_str().unwrap().into(),
        name: text("name"),
        width: input["width"].as_i64(),
        fixed: input["fixed"].as_bool().unwrap_or(false),
        aggregate: text("aggregate"),
        lower: input["lower"].as_i64(),
        upper: input["upper"].as_i64(),
        lower_expression: text("lowerExpression"),
        upper_expression: text("upperExpression"),
        unique: input["unique"].as_bool().unwrap_or(false),
        optional: input["optional"].as_bool().unwrap_or(false),
        item: input.get("item").map(|item| Box::new(expression(item))),
    }
}
#[test]
fn borrowed_type_metadata_comparison_matches_independent_structural_identity_laws() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/type-metadata.json")).unwrap();
    for row in contract["cases"].as_array().unwrap() {
        let left = Box::leak(Box::new(expression(&row["left"])));
        let right = Box::leak(Box::new(expression(&row["right"])));
        let mut comparison = super::super::super::type_metadata::Comparison::default();
        comparison.start(left, right);
        let mut complete = false;
        for _ in 0..10000 {
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| comparison.advance());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if let Some(result) = result.unwrap() {
                assert_eq!(result, row["equal"].as_bool().unwrap(), "{}", row["id"]);
                complete = true;
                break;
            }
        }
        assert!(complete);
    }
}
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/constituent-remap.json")).unwrap()
}
fn source(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    let contract = contract();
    let mut entities = vec![super::equal_arity_type_tests::entity(&contract["sources"][row["source"].as_str().unwrap()], row["entityId"].as_str().unwrap_or("1").parse().unwrap())];
    entities.extend(contract["tail"].as_array().unwrap().iter().map(|input| super::equal_arity_type_tests::entity(input, input["id"].as_str().unwrap().parse().unwrap())));
    Arc::new(IfcSnapshot { entities, ..Default::default() })
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    let path = row["path"].as_str().unwrap();
    let name = row["target"].as_str().unwrap();
    editing::SnapshotEditEvent::SetValue { path: path.into(), value: if path.ends_with("/name") { DslValue::String(name.into()) } else { DslValue::Object(vec![("$ifcType".into(), DslValue::String(name.into()))]) } }
}
#[test]
fn original_constituent_remap_matches_neutral_output_inverse_and_denied_axes_without_eager_defaults() {
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
                if bridge || work.pending.is_some() && !work.domain_validated {
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
                        let turns = work.domain.turns();
                        let command = work.original_event.as_ref().unwrap().to_value();
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
                    if bridge && result.is_ok() {
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
                let result = result.expect("remap must complete or explicitly refuse");
                assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
                if row["phase"] == "required" {
                    assert!(work.parts.is_none() && work.entity.is_none() && work.pending.is_none());
                }
                if let Ok(emit) = result {
                    assert!(work.constituent_roster.type_admission().is_some());
                    let [IfcMutation::SetEntityComplex(change)] = emit.artifact_mutations.as_slice() else { panic!("one central complex primitive") };
                    let expected: Vec<_> = row["args"].as_array().unwrap().iter().map(super::equal_arity_type_tests::value).collect();
                    assert_eq!(change.complex[2].name, row["target"].as_str().unwrap().to_ascii_uppercase());
                    assert!(ifc_arguments_identical(&change.complex[2].args, &expected));
                    assert!(ifc_complex_identical(&change.complex[..2], &snapshot.entities[0].complex[..2]));
                    let mutation = &emit.artifact_mutations[0];
                    let mut next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                    for inverse in mutation.inverse(&snapshot).unwrap() {
                        next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
                    }
                    assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
                }
            } else {
                assert!(work.constituent_roster.fallback());
            }
            super::constituent_optional_shape_tests::close_with_denials(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_remap_cancels_and_closes_every_plan_value_move_and_domain_prefix() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("property-grow" | "property-shrink" | "required-enumeration-refusal" | "invalid-compatible-domain-refusal"))) {
        let snapshot = source(row);
        let mut total = 0;
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            for _ in 0..100000 {
                total += 1;
                match turn(caller, 1, false, |cx| work.step(input, cx)) {
                    Ok(ArtifactCommandWorkStep::Complete(_)) | Err(_) => break,
                    _ => {}
                }
            }
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
            });
            caller.finish();
        }
    }
}
