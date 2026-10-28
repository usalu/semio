//! 🧬️ Sparse primary remaps borrow unchanged values and retain moved payload custody.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/primary-remap.json")).unwrap()
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
fn original_primary_remap_denies_axes_before_progress_and_matches_neutral_sparse_output_inverse() {
    for row in contract()["cases"].as_array().unwrap() {
        let snapshot = source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for _ in 0..100000 {
                if work.constituent_roster.fallback() {
                    assert_eq!(row["retained"], false);
                    break;
                }
                let bridge = work.prepared && work.reference.complete() && work.enumeration.complete() && (work.pending.is_none() || work.constituent_roster.receiving_primitives());
                if work.constituent_roster.receiving_primitives() {
                    assert_eq!(work.domain.turns(), 0);
                    assert!(!work.publication_validated);
                    assert!(work.constituent_roster.type_admission().is_none());
                }
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
                    }
                }
                match caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| {
                    let receipt = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.step(input, cx));
                    if bridge && result.is_ok() {
                        let after = cx.retained_progress();
                        assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - receipt.retained_capacity_bytes, after.released_bytes - receipt.released_bytes));
                    }
                    result
                }) {
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
                assert!(work.parts_cursor.terminal_is_empty());
            }
            if row["retained"] == true {
                let result = result.expect("original remap completes or explicitly refuses");
                assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
                if row["phase"] == "required" {
                    assert!(work.pending.is_none() && work.value.is_none() && work.value_cursor.terminal_is_empty());
                }
                if let Ok(emit) = result {
                    assert_eq!(emit.artifact_mutations.len(), row["mutations"].as_u64().unwrap() as usize);
                    let mut next = snapshot.as_ref().clone();
                    let mut inverses = Vec::new();
                    for mutation in &emit.artifact_mutations {
                        inverses.push(mutation.inverse(&next).unwrap());
                        next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
                    }
                    let expected: Vec<_> = row["args"].as_array().unwrap().iter().map(super::equal_arity_type_tests::value).collect();
                    assert!(ifc_arguments_identical(&next.entities[0].args, &expected));
                    assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
                    for inverse in inverses.into_iter().rev() {
                        for mutation in inverse {
                            next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
                        }
                    }
                    assert_eq!(next.entities[0].name, snapshot.entities[0].name);
                    assert!(ifc_arguments_identical(&next.entities[0].args, &snapshot.entities[0].args));
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
fn original_primary_remap_cancels_and_closes_every_metadata_payload_scaffold_and_batch_prefix() {
    for row in
        contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("property-grow" | "material-in-place-and-move" | "aggregate-offset-move" | "required-profile-refusal" | "invalid-compatible-domain-refusal")))
    {
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
                super::constituent_optional_shape_tests::close_with_denials(caller, work);
                assert!(work.terminal_is_empty());
            });
            caller.finish();
        }
    }
}
#[test]
fn original_primary_remap_private_witness_borrows_unchanged_original_slots() {
    let contract = contract();
    let row = contract["cases"].as_array().unwrap().iter().find(|row| row["id"] == "material-in-place-and-move").unwrap();
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
        let admission = work.constituent_roster.type_admission().unwrap();
        for index in 0..3 {
            assert!(std::ptr::eq(admission.primary_argument(&snapshot.entities[0], work.pending.as_deref().unwrap(), index).unwrap(), &snapshot.entities[0].args[index]));
        }
        let IfcMutation::SetEntityArg(value) = &work.pending.as_ref().unwrap()[3] else { panic!("exact moved Category primitive") };
        assert!(std::ptr::eq(admission.primary_argument(&snapshot.entities[0], work.pending.as_deref().unwrap(), 4).unwrap(), &value.value));
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
        super::equal_arity_type_tests::close_domain(caller, &mut domain);
        close(caller, work);
    });
    caller.finish();
}
#[test]
fn original_primary_remap_rejects_substituted_payload_receivers_before_debit() {
    let contract = contract();
    let row = contract["cases"].as_array().unwrap().iter().find(|row| row["id"] == "aggregate-offset-move").unwrap();
    let snapshot = source(row);
    let mut caller = OriginalWorkFixture::new();
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
        for index in 0..100000 {
            if work.constituent_roster.phase() == 19 {
                break;
            }
            turn(caller, 1, false, |cx| work.step(input, cx)).unwrap();
            assert!(index < 99999);
        }
        let mut alternate_cursor = IfcValue::retained_clone_cursor();
        let mut alternate_value = None;
        for variant in 0..2 {
            let metadata = work.constituent_roster.metadata();
            caller
                .work_turn(grant(), 1, |cx| {
                    let receipt = cx.retained_progress();
                    assert!(
                        work.constituent_roster
                            .advance::<DetailsTestProfile>(
                                work.original_event.as_mut().unwrap(),
                                snapshot.as_ref(),
                                &work.address,
                                work.source.as_ref(),
                                &mut work.parts_cursor,
                                &mut work.parts,
                                &mut work.pending,
                                if variant == 0 { &mut alternate_cursor } else { &mut work.value_cursor },
                                if variant == 1 { &mut alternate_value } else { &mut work.value },
                                cx
                            )
                            .is_err()
                    );
                    assert_eq!(receipt, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            assert_eq!(metadata, work.constituent_roster.metadata());
        }
        assert!(alternate_cursor.terminal_is_empty() && alternate_value.is_none());
        close(caller, work);
    });
    caller.finish();
}
