//! 📨️ Required reference inputs retain original receiving and strict candidate authority.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/type-input.json")).unwrap()
}
pub(super) fn raw(value: &serde_json::Value) -> DslValue {
    match value {
        serde_json::Value::String(value) => DslValue::String(value.clone()),
        serde_json::Value::Array(values) => DslValue::Array(values.iter().map(raw).collect()),
        serde_json::Value::Object(values) => DslValue::Object(values.iter().map(|(key, value)| (key.clone(), raw(value))).collect()),
        _ => panic!("closed reference fixture has only arrays, objects and exact text"),
    }
}
fn source(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/primary-remap.json")).unwrap();
    let mut entities = vec![super::equal_arity_type_tests::entity(&fixture["sources"]["constituent"], 1)];
    entities.extend(
        fixture["tail"].as_array().unwrap().iter().map(|value| super::equal_arity_type_tests::entity(value, if value["id"] == "7" { row["profileId"].as_str().unwrap_or("7").parse().unwrap() } else { value["id"].as_str().unwrap().parse().unwrap() })),
    );
    Arc::new(IfcSnapshot { entities, ..Default::default() })
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: DslValue::Object(vec![("$ifcType".into(), DslValue::String("IFCMATERIALPROFILE".into())), ("$ifcInputs".into(), raw(&row["inputs"]))]) }
}
#[test]
fn original_required_type_inputs_match_neutral_admission_and_exact_central_inverse_under_denied_axes() {
    for row in contract()["cases"].as_array().unwrap() {
        let snapshot = source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for _ in 0..100000 {
                let bridge = work.prepared && work.reference.complete() && work.enumeration.complete() && (work.pending.is_none() || work.constituent_roster.receiving_primitives());
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
                        let before = work.constituent_roster.metadata();
                        let turns = work.domain.turns();
                        caller
                            .work_turn(denied, fuel, |cx| {
                                let receipt = cx.retained_progress();
                                assert!(!matches!(work.step(input, cx)?, ArtifactCommandWorkStep::Complete(_)));
                                assert_eq!(receipt, cx.retained_progress());
                                Ok(())
                            })
                            .unwrap();
                        assert_eq!(before, work.constituent_roster.metadata());
                        assert_eq!(turns, work.domain.turns());
                    }
                }
                if work.constituent_roster.receiving_primitives() {
                    assert_eq!(work.domain.turns(), 0);
                    assert!(!work.publication_validated);
                    assert!(work.constituent_roster.type_admission().is_none());
                }
                match turn(caller, 1, false, |cx| work.step(input, cx)) {
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
                assert!(work.entity.is_none() && work.parts.is_none() && work.semantic_emit.is_none());
                assert_eq!(work.reduction_ticks(), 0);
            }
            let result = result.expect("explicit input completes or refuses without eager fallback");
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(emit) = result {
                assert_eq!(emit.artifact_mutations.len(), 4);
                let mut next = snapshot.as_ref().clone();
                let mut inverse = Vec::new();
                for mutation in &emit.artifact_mutations {
                    inverse.push(mutation.inverse(&next).unwrap());
                    next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
                }
                assert_eq!(next.entities[0].name, "IFCMATERIALPROFILE");
                assert!(matches!(next.entities[0].args[3],IfcValue::Reference(id)if id==row["profileId"].as_str().unwrap_or("7").parse::<u64>().unwrap()));
                for batch in inverse.into_iter().rev() {
                    for mutation in batch {
                        next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
                    }
                }
                assert_eq!(next.entities[0].name, snapshot.entities[0].name);
                assert!(ifc_arguments_identical(&next.entities[0].args, &snapshot.entities[0].args));
                assert!(ifc_complex_identical(&next.entities[0].complex, &snapshot.entities[0].complex));
            }
            super::constituent_optional_shape_tests::close_with_denials(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
    }
}
#[test]
fn original_required_type_input_every_receiving_candidate_and_domain_prefix_cancels_and_closes() {
    for row in contract()["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("caller-profile" | "wrong-profile-type" | "duplicate-index"))) {
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
fn guidance_request() -> semio_framework_plugin::action_staging::StagedFormRequest {
    use semio_framework_plugin::action_staging::*;
    StagedFormRequest {
        token: StagedFormToken { action: editing::SET_SNAPSHOT_VALUE_ACTION_ID.into(), window: "original-type-window".into(), generation: "1".into(), revision: "0".repeat(64) },
        args: DslValue::Object(vec![("path".into(), DslValue::String("/entities/0".into())), ("value".into(), DslValue::Object(vec![("$ifcType".into(), DslValue::String("IFCMATERIALPROFILE".into()))]))]),
        locale: semio_framework_ui_locale::Locale::En,
        page: None,
    }
}
type Guidance = super::super::super::type_guidance::IfcTypeInputSourceWork<DetailsTestProfile>;
type PreparedGuidance = super::super::super::type_guidance::IfcPreparedTypeInputSource;
fn retire_guidance<T: semio_framework_value::retirement::RetireOwned + 'static>(caller: &mut OriginalWorkFixture, owner: &mut Option<T>) {
    let mut close = semio_framework_value::retained_clone::RetainedCloneClose::default();
    for _ in 0..100000 {
        if owner.is_none() && close.is_empty() {
            break;
        }
        let demand = close.next_owner_demands::<T>(owner.is_some(), &None, 4096).unwrap();
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                        if close.is_empty() {
                            close.begin_granted(owner, grant)?.ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "missing guidance owner"))
                        } else {
                            close.step_granted(grant)
                        }
                    })?;
                    Ok(())
                })
            })
            .unwrap();
    }
    assert!(owner.is_none() && close.is_empty());
}
#[test]
fn original_type_guidance_denies_each_published_axis_before_source_or_output_changes() {
    use semio_framework_plugin_decl_staged_form::StagedBodySource;
    let snapshot = source(&contract()["cases"][0]);
    let alternate = Arc::new(snapshot.as_ref().clone());
    let mut caller = OriginalWorkFixture::new();
    let mut guide = Guidance::default();
    let mut donor = Some(guidance_request());
    let mut output: Option<PreparedGuidance> = None;
    assert!(caller.work_turn(grant(), 1, |cx| guide.receive(&mut donor, &snapshot, [0; 32], cx)).unwrap());
    for index in 0..100000 {
        let demand = guide.demands().unwrap();
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
            caller
                .work_turn(denied, fuel, |cx| {
                    let receipt = cx.retained_progress();
                    assert!(!guide.advance(&snapshot, [0; 32], cx)?);
                    assert_eq!(receipt, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
        }
        caller
            .work_turn(grant(), 1, |cx| {
                let receipt = cx.retained_progress();
                assert!(guide.advance(&alternate, [0; 32], cx).is_err());
                assert_eq!(receipt, cx.retained_progress());
                Ok(())
            })
            .unwrap();
        if caller.work_turn(grant(), 1, |cx| guide.advance(&snapshot, [0; 32], cx)).unwrap() {
            break;
        }
        assert!(index < 99999);
    }
    assert!(caller.work_turn(grant(), 1, |cx| guide.take(&mut output, cx)).unwrap());
    let source = output.as_ref().unwrap();
    assert!(!source.status().ready);
    assert_eq!(source.controls(), 1);
    let field = source.control(0).unwrap();
    assert!(field.required && field.binding.is_some());
    assert_eq!(field.scalar.label, "Profile reference");
    retire_guidance(&mut caller, &mut output);
    retire_guidance(&mut caller, &mut Some(guide));
    caller.finish();
}
#[test]
fn original_type_guidance_closes_every_accepted_metadata_domain_binding_and_take_prefix() {
    let snapshot = source(&contract()["cases"][0]);
    let mut total = 0;
    let mut caller = OriginalWorkFixture::new();
    let mut guide = Guidance::default();
    let mut donor = Some(guidance_request());
    let mut output: Option<PreparedGuidance> = None;
    let mut received = false;
    let mut prepared = false;
    for _ in 0..100000 {
        total += 1;
        caller
            .work_turn(grant(), 1, |cx| {
                if !received {
                    received = guide.receive(&mut donor, &snapshot, [0; 32], cx)?;
                } else if !prepared {
                    prepared = guide.advance(&snapshot, [0; 32], cx)?;
                } else if output.is_none() {
                    guide.take(&mut output, cx)?;
                }
                Ok(())
            })
            .unwrap();
        if output.is_some() {
            break;
        }
    }
    assert!(output.is_some());
    retire_guidance(&mut caller, &mut output);
    retire_guidance(&mut caller, &mut Some(guide));
    caller.finish();
    for prefix in 0..=total {
        let mut caller = OriginalWorkFixture::new();
        let mut guide = Guidance::default();
        let mut donor = Some(guidance_request());
        let mut output: Option<PreparedGuidance> = None;
        let mut received = false;
        let mut prepared = false;
        for _ in 0..prefix {
            caller
                .work_turn(grant(), 1, |cx| {
                    if !received {
                        received = guide.receive(&mut donor, &snapshot, [0; 32], cx)?;
                    } else if !prepared {
                        prepared = guide.advance(&snapshot, [0; 32], cx)?;
                    } else if output.is_none() {
                        guide.take(&mut output, cx)?;
                    }
                    Ok(())
                })
                .unwrap();
        }
        caller.cancel().cancel_now();
        if received {
            assert!(caller.work_turn(grant(), 1, |cx| guide.advance(&snapshot, [0; 32], cx)).is_err());
        }
        retire_guidance(&mut caller, &mut output);
        retire_guidance(&mut caller, &mut donor);
        retire_guidance(&mut caller, &mut Some(guide));
        caller.finish();
    }
}
#[path = "type_multiple_input_tests.rs"]
mod multiple;
#[path = "type_scalar_input_tests.rs"]
mod scalar;

#[path="type_enum_input_tests.rs"]
mod enumeration;
