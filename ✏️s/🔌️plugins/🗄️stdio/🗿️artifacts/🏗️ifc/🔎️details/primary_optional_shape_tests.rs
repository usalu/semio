//! 🧩️ Primary optional shape changes publish only complete original-owned argument batches.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/primary-optional-shape.json")).unwrap()
}
fn source(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    Arc::new(IfcSnapshot { entities: vec![super::equal_arity_type_tests::entity(&contract()["sources"][row["source"].as_str().unwrap()], row["entityId"].as_str().unwrap_or("1").parse().unwrap())], ..Default::default() })
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: DslValue::Object(vec![("$ifcType".into(), DslValue::String(row["target"].as_str().unwrap().into()))]) }
}
#[test]
fn original_primary_optional_shapes_admit_complete_batches_under_denied_axes_and_preserve_central_inverse() {
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
                if work.constituent_roster.receiving_primitives() {
                    assert_eq!(work.domain.turns(), 0);
                    assert!(!work.publication_validated);
                    assert!(work.constituent_roster.type_admission().is_none());
                }
                let bridge = work.prepared && work.reference.complete() && work.enumeration.complete() && (work.pending.is_none() || work.constituent_roster.receiving_primitives());
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
                        let length = work.pending.as_ref().map(Vec::len);
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
                        assert_eq!(length, work.pending.as_ref().map(Vec::len));
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
                        result = Some(emit);
                        break;
                    }
                    Err(error) => panic!("{}: {error:?}", row["id"]),
                    _ => {}
                }
                assert_eq!(work.reduction_ticks(), 0);
                assert!(work.entity.is_none() && work.parts.is_none() && work.semantic_emit.is_none());
                assert!(work.cursor.terminal_is_empty() && work.parts_cursor.terminal_is_empty());
            }
            if row["retained"] == true {
                let emit = result.expect("primary shape must finish");
                assert!(work.constituent_roster.type_admission().is_some());
                assert!(work.domain.turns() > 3);
                assert_eq!(emit.artifact_mutations.len(), row["mutations"].as_u64().unwrap() as usize);
                let IfcMutation::SetEntityName(change) = &emit.artifact_mutations[0] else { panic!("central name first") };
                assert_eq!(change.name, row["target"].as_str().unwrap().to_ascii_uppercase());
                assert_eq!(change.id, snapshot.entities[0].id);
                for (offset, mutation) in emit.artifact_mutations[1..].iter().enumerate() {
                    match mutation {
                        IfcMutation::InsertEntityArg(value) => {
                            assert_eq!(value.index, offset);
                            assert!(matches!(value.value, IfcValue::Unset));
                        }
                        IfcMutation::RemoveEntityArg(value) => assert_eq!(value.index, snapshot.entities[0].args.len() - 1 - offset),
                        _ => panic!("only ordered root argument primitives"),
                    }
                }
                let mut next = snapshot.as_ref().clone();
                let mut inverse = Vec::new();
                for mutation in &emit.artifact_mutations {
                    inverse.push(mutation.inverse(&next).unwrap());
                    next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
                }
                let expected: Vec<_> = row["args"].as_array().unwrap().iter().map(super::equal_arity_type_tests::value).collect();
                assert!(ifc_arguments_identical(&next.entities[0].args, &expected));
                assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
                for mutations in inverse.into_iter().rev() {
                    for mutation in mutations {
                        next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
                    }
                }
                assert_eq!(next.entities[0].name, snapshot.entities[0].name);
                assert!(ifc_arguments_identical(&next.entities[0].args, &snapshot.entities[0].args));
                assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
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
fn original_primary_optional_shapes_cancel_and_close_every_partial_batch_and_domain_prefix() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("drop-aggregate" | "drop-two-slots" | "grow-two-optional"))) {
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
#[test]
fn original_primary_shape_witness_borrows_actual_prepared_arguments_and_rejects_alternate_batch_or_policy() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("grow-two-optional" | "drop-two-slots"))) {
        let snapshot = source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            for index in 0..100000 {
                if work.constituent_roster.type_admission().is_some() {
                    break;
                }
                turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
                assert!(index < 99999);
            }
            let length = row["args"].as_array().unwrap().len();
            let admission = work.constituent_roster.type_admission().unwrap();
            assert_eq!(admission.primary_arity(), Some(length));
            for index in 0..length {
                let IfcMutation::InsertEntityArg(value) = &work.pending.as_ref().unwrap()[index + 1] else { panic!("prepared inserted payload") };
                assert!(std::ptr::eq(admission.primary_argument(&snapshot.entities[0], work.pending.as_deref().unwrap(), index).unwrap(), &value.value));
            }
            assert!(admission.primary_argument(&snapshot.entities[0], work.pending.as_deref().unwrap(), length).is_none());
            let alternate = work.pending.as_ref().unwrap().clone();
            let mut domain = domain_work::DomainWork::default();
            caller
                .work_turn(grant(), 1, |cx| {
                    let receipt = cx.retained_progress();
                    assert!(domain.advance::<DetailsTestProfile>(work.original_event.as_ref().unwrap(), snapshot.as_ref(), &alternate, &work.address, work.constituent_roster.type_admission(), cx).is_err());
                    assert_eq!(receipt, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            assert_eq!(domain.turns(), 0);
            caller.work_turn(grant(), 1, |cx| domain.advance::<DetailsTestProfile>(work.original_event.as_ref().unwrap(), snapshot.as_ref(), work.pending.as_deref().unwrap(), &work.address, work.constituent_roster.type_admission(), cx)).unwrap();
            let turns = domain.turns();
            caller
                .work_turn(grant(), 1, |cx| {
                    let receipt = cx.retained_progress();
                    assert!(domain.advance::<DetailsTestProfile>(work.original_event.as_ref().unwrap(), snapshot.as_ref(), work.pending.as_deref().unwrap(), &work.address, None, cx).is_err());
                    assert_eq!(receipt, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            assert_eq!(domain.turns(), turns);
            super::equal_arity_type_tests::close_domain(caller, &mut domain);
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
