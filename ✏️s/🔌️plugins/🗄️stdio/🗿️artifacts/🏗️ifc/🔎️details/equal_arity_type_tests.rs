//! 🧭️ Nonzero ordinal-preserving variants require the completed original roster's strict domain witness.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/equal-arity-type.json")).unwrap()
}
pub(super) fn value(input: &serde_json::Value) -> IfcValue {
    match input["kind"].as_str().unwrap() {
        "real" => IfcValue::Real(f64::from_bits(input["value"]["bits"].as_str().unwrap().parse().unwrap())),
        "integer" => IfcValue::Integer(input["value"].as_str().unwrap().parse().unwrap()),
        "typedValue" => IfcValue::TypedValue(IfcTypedValue { name: input["value"]["name"].as_str().unwrap().into(), items: input["value"]["items"].as_array().unwrap().iter().map(value).collect() }),
        "reference" => IfcValue::Reference(input["value"].as_str().unwrap().parse().unwrap()),
        "aggregate" => IfcValue::Aggregate(input["value"].as_array().unwrap().iter().map(value).collect()),
        "string" => IfcValue::String(input["value"].as_str().unwrap().into()),
        "enum" => IfcValue::Enum(input["value"].as_str().unwrap().into()),
        "unset" => IfcValue::Unset,
        _ => panic!("unsupported neutral fixture value"),
    }
}
pub(super) fn entity(input: &serde_json::Value, id: u64) -> IfcEntity {
    IfcEntity {
        id,
        name: input["name"].as_str().unwrap().into(),
        args: input["args"].as_array().unwrap().iter().map(value).collect(),
        complex: input["complex"].as_array().unwrap().iter().map(|part| IfcComplexType { name: part["name"].as_str().unwrap().into(), args: part["args"].as_array().unwrap().iter().map(value).collect() }).collect(),
    }
}
fn source(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    let contract = contract();
    let mut entities = vec![entity(&contract["sources"][row["source"].as_str().unwrap()], row["entityId"].as_str().unwrap_or("1").parse().unwrap())];
    entities.extend(contract["tail"].as_array().unwrap().iter().map(|input| entity(input, input["id"].as_str().unwrap().parse().unwrap())));
    Arc::new(IfcSnapshot { entities, ..Default::default() })
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    let name = row["target"].as_str().unwrap();
    editing::SnapshotEditEvent::SetValue { path: row["path"].as_str().unwrap().into(), value: if row["mode"] == "name" { DslValue::String(name.into()) } else { DslValue::Object(vec![("$ifcType".into(), DslValue::String(name.into()))]) } }
}
pub(super) fn close_domain(caller: &mut OriginalWorkFixture, domain: &mut domain_work::DomainWork) {
    domain.begin_close();
    while !domain.terminal_is_empty() {
        let demand = domain.close_demands();
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| domain.close_step(grant)))).unwrap();
    }
}
#[test]
fn original_equal_arity_types_preserve_words_and_require_central_candidate_domain_under_denied_axes() {
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
                                let step = work.step(input, cx)?;
                                assert!(!matches!(step, ArtifactCommandWorkStep::Complete(_)));
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
                let result = result.expect("retained type must finish or refuse candidate domain");
                assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
                assert!(work.domain.turns() > 3);
                assert!(work.constituent_roster.type_admission().is_some());
                if let Ok(emit) = result {
                    assert_eq!(emit.artifact_mutations.len(), 1);
                    let mutation = &emit.artifact_mutations[0];
                    let IfcMutation::SetEntityComplex(change) = mutation else { panic!("central complex primitive required") };
                    assert_eq!(change.complex[2].name, row["target"].as_str().unwrap().to_ascii_uppercase());
                    assert!(ifc_arguments_identical(&change.complex[2].args, &snapshot.entities[0].complex[2].args));
                    let mut next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                    for inverse in mutation.inverse(&snapshot).unwrap() {
                        next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
                    }
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
fn original_equal_arity_types_cancel_and_close_every_preparation_and_domain_prefix() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("aggregate-variant" | "candidate-required-unset"))) {
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
#[test]
fn original_equal_arity_domain_witness_rejects_other_source_event_address_primitive_and_caller() {
    let contract = contract();
    let row = &contract["cases"][0];
    let snapshot = source(row);
    let alternate = Arc::new(snapshot.as_ref().clone());
    let other_address = pointer::PathAdmission::default();
    let mut caller = OriginalWorkFixture::new();
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    assert!(constituent_roster::ConstituentRoster::default().type_admission().is_none());
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
            close_domain(caller, &mut domain);
        }
        let mut legitimate = domain_work::DomainWork::default();
        caller.work_turn(grant(), 1, |cx| legitimate.advance::<DetailsTestProfile>(work.original_event.as_ref().unwrap(), snapshot.as_ref(), work.pending.as_deref().unwrap(), &work.address, work.constituent_roster.type_admission(), cx)).unwrap();
        let turns = legitimate.turns();
        caller
            .work_turn(grant(), 1, |cx| {
                let receipt = cx.retained_progress();
                assert!(legitimate.advance::<DetailsTestProfile>(work.original_event.as_ref().unwrap(), snapshot.as_ref(), work.pending.as_deref().unwrap(), &work.address, None, cx).is_err());
                assert_eq!(receipt, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        assert_eq!(turns, legitimate.turns());
        close_domain(caller, &mut legitimate);
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
        close_domain(&mut other_caller, &mut domain);
        other_caller.finish();
        close(caller, work);
        assert!(work.terminal_is_empty());
        assert!(work.constituent_roster.type_admission().is_none());
    });
    caller.finish();
}
