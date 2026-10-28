//! 🏷️ Primary direct arguments cross strict candidate domains under the completed original type owner.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/primary-equal-arity-type.json")).unwrap()
}
fn source(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    let contract = contract();
    let mut entities = vec![super::equal_arity_type_tests::entity(&contract["sources"][row["source"].as_str().unwrap()], row["entityId"].as_str().unwrap_or("1").parse().unwrap())];
    entities.extend(contract["tail"].as_array().unwrap().iter().map(|input| super::equal_arity_type_tests::entity(input, input["id"].as_str().unwrap().parse().unwrap())));
    Arc::new(IfcSnapshot { entities, ..Default::default() })
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: DslValue::Object(vec![("$ifcType".into(), DslValue::String(row["target"].as_str().unwrap().into()))]) }
}
#[test]
fn original_primary_equal_arity_types_preserve_arguments_and_require_candidate_domains_under_denied_axes() {
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
                let bridge = work.prepared && work.reference.complete() && work.enumeration.complete() && work.pending.is_none();
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
                assert!(work.entity.is_none() && work.parts.is_none() && work.semantic_emit.is_none());
                assert!(work.cursor.terminal_is_empty() && work.parts_cursor.terminal_is_empty());
            }
            if row["retained"] == true {
                let result = result.expect("primary retained candidate must complete or refuse domain");
                assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
                if row["mutations"] != 0 {
                    assert!(work.domain.turns() > 3);
                    assert!(work.constituent_roster.type_admission().is_some());
                } else {
                    assert!(work.constituent_roster.type_admission().is_none());
                }
                if let Ok(emit) = result {
                    assert_eq!(emit.artifact_mutations.len(), row["mutations"].as_u64().unwrap() as usize);
                    let mut next = snapshot.as_ref().clone();
                    for mutation in &emit.artifact_mutations {
                        let IfcMutation::SetEntityName(change) = mutation else { panic!("central primary name primitive required") };
                        assert_eq!(change.id, snapshot.entities[0].id);
                        assert_eq!(change.name, row["target"].as_str().unwrap().to_ascii_uppercase());
                        let editing::SnapshotEditEvent::SetValue { value, .. } = work.original_event.as_ref().unwrap() else { panic!("original marker retained") };
                        assert_eq!(requested_type(value), Some(""));
                        next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
                    }
                    assert_eq!(next.entities[0].name, row["target"].as_str().unwrap().to_ascii_uppercase());
                    assert!(ifc_arguments_identical(&next.entities[0].args, &snapshot.entities[0].args));
                    assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
                    for mutation in emit.artifact_mutations.iter().rev() {
                        for inverse in mutation.inverse(&snapshot).unwrap() {
                            next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
                        }
                    }
                    assert_eq!(next.entities[0].name, snapshot.entities[0].name);
                    assert!(ifc_arguments_identical(&next.entities[0].args, &snapshot.entities[0].args));
                    assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
                }
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
fn original_primary_equal_arity_types_cancel_and_close_every_preparation_and_domain_prefix() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("aggregate-variant" | "candidate-required-unset" | "same-type-noop"))) {
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
                assert!(work.constituent_roster.type_admission().is_none());
            });
            caller.finish();
        }
    }
}
#[test]
fn original_primary_type_witness_rejects_substitution_and_domain_policy_downgrade() {
    let contract = contract();
    let row = &contract["cases"][0];
    let snapshot = source(row);
    let alternate = Arc::new(snapshot.as_ref().clone());
    let other_address = pointer::PathAdmission::default();
    let mut caller = OriginalWorkFixture::new();
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
        for index in 0..100000 {
            if work.pending.is_some() {
                break;
            }
            turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
            assert!(index < 99999);
        }
        let other_mutations = work.pending.as_ref().unwrap().clone();
        let other_event = work.original_event.as_ref().unwrap().clone();
        for variant in 0..4 {
            let mut domain = domain_work::DomainWork::default();
            caller
                .work_turn(grant(), 1, |cx| {
                    let receipt = cx.retained_progress();
                    assert!(
                        domain
                            .advance::<DetailsTestProfile>(
                                if variant == 0 { &other_event } else { work.original_event.as_ref().unwrap() },
                                if variant == 1 { alternate.as_ref() } else { snapshot.as_ref() },
                                if variant == 2 { &other_mutations } else { work.pending.as_deref().unwrap() },
                                if variant == 3 { &other_address } else { &work.address },
                                work.constituent_roster.type_admission(),
                                cx
                            )
                            .is_err()
                    );
                    assert_eq!(receipt, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            assert_eq!(domain.turns(), 0);
            super::equal_arity_type_tests::close_domain(caller, &mut domain);
        }
        let mut domain = domain_work::DomainWork::default();
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
        let mut other_caller = OriginalWorkFixture::new();
        let mut domain = domain_work::DomainWork::default();
        other_caller
            .work_turn(grant(), 1, |cx| {
                let receipt = cx.retained_progress();
                assert!(domain.advance::<DetailsTestProfile>(work.original_event.as_ref().unwrap(), snapshot.as_ref(), work.pending.as_deref().unwrap(), &work.address, work.constituent_roster.type_admission(), cx).is_err());
                assert_eq!(receipt, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        super::equal_arity_type_tests::close_domain(&mut other_caller, &mut domain);
        other_caller.finish();
        close(caller, work);
        assert!(work.terminal_is_empty());
    });
    caller.finish();
}
