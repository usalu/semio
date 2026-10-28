//! 🧮️ Declared enum choices retain full source domains, real pages and strict actual argument changes.
use super::*;
fn enum_contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/type-enum-input.json")).unwrap()
}
fn enum_value(value: &serde_json::Value) -> IfcValue {
    if value["kind"] == "derived" { IfcValue::Derived } else { super::super::equal_arity_type_tests::value(value) }
}
fn enum_source(law: &serde_json::Value, name: &str) -> Arc<IfcSnapshot> {
    Arc::new(IfcSnapshot {
        entities: law["sources"][name]["entities"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| IfcEntity {
                id: row["id"].as_str().unwrap().parse().unwrap(),
                name: row["name"].as_str().unwrap().into(),
                args: row["args"].as_array().unwrap().iter().map(enum_value).collect(),
                complex: row["complex"].as_array().unwrap().iter().map(|part| IfcComplexType { name: part["name"].as_str().unwrap().into(), args: part["args"].as_array().unwrap().iter().map(enum_value).collect() }).collect(),
            })
            .collect(),
        ..Default::default()
    })
}
fn enum_request(row: &serde_json::Value, complete: bool) -> semio_framework_plugin::action_staging::StagedFormRequest {
    let mut request = guidance_request();
    let rows = if complete {
        serde_json::Value::Array(row["required"].as_array().unwrap().iter().map(|item| serde_json::json!({"index":item["index"].as_u64().unwrap().to_string(),"value":row["args"][item["index"].as_u64().unwrap()as usize]})).collect())
    } else {
        serde_json::json!([])
    };
    request.args =
        DslValue::Object(vec![("path".into(), DslValue::String("/entities/0".into())), ("value".into(), DslValue::Object(vec![("$ifcType".into(), DslValue::String(row["target"].as_str().unwrap().into())), ("$ifcInputs".into(), raw(&rows))]))]);
    request
}
#[test]
fn original_enum_type_guidance_keeps_declared_options_bounded_and_supplied_values_editable() {
    use semio_framework_plugin::app::staged_form::StagedBodySource;
    let law = enum_contract();
    for row in law["cases"].as_array().unwrap() {
        for complete in [false, true] {
            let snapshot = enum_source(&law, row["source"].as_str().unwrap());
            let mut caller = OriginalWorkFixture::new();
            let mut guide = Guidance::default();
            let mut donor = Some(enum_request(row, complete));
            let mut output = None;
            assert!(caller.work_turn(grant(), 1, |cx| guide.receive(&mut donor, &snapshot, [0; 32], cx)).unwrap());
            for turn in 0..100000 {
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
                            let before = cx.retained_progress();
                            assert!(!guide.advance(&snapshot, [0; 32], cx)?);
                            assert_eq!(before, cx.retained_progress());
                            Ok(())
                        })
                        .unwrap();
                }
                if caller.work_turn(grant(), 1, |cx| guide.advance(&snapshot, [0; 32], cx)).unwrap() {
                    break;
                }
                assert!(turn < 99999);
            }
            assert!(caller.work_turn(grant(), 1, |cx| guide.take(&mut output, cx)).unwrap());
            let page = output.as_ref().unwrap();
            let required = row["required"].as_array().unwrap();
            assert!(page.controls() >= required.len() && page.controls() <= required.len() + 2);
            for (index, field) in required.iter().enumerate() {
                let control = page.control(index).unwrap();
                let semio_framework_plugin::app::staged_form::StagedScalarKind::Choice { value, choices } = control.scalar.kind else { panic!("declared choice") };
                assert!(choices.len() <= 32);
                if complete {
                    assert_eq!(value, row["args"][field["index"].as_u64().unwrap() as usize]["value"].as_str().unwrap());
                    assert!(choices.iter().any(|choice| choice.key == value));
                }
                for choice in choices {
                    assert!(field["members"].as_array().unwrap().iter().any(|word| word.as_str() == Some(choice.key)));
                }
            }
            assert_eq!(output.as_ref().unwrap().status().ready, complete);
            retire_guidance(&mut caller, &mut output);
            retire_guidance(&mut caller, &mut Some(guide));
            caller.finish();
        }
    }
}
#[test]
fn original_enum_same_type_repair_emits_only_changed_args_and_exact_central_inverse() {
    let law = enum_contract();
    for row in law["cases"].as_array().unwrap() {
        let snapshot = enum_source(&law, row["source"].as_str().unwrap());
        let request = enum_request(row, true);
        let DslValue::Object(mut fields) = request.args else { panic!("args") };
        let ordinal = fields.iter().position(|(key, _)| key == "value").unwrap();
        let value = fields.remove(ordinal).1;
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value }, &mut caller, &mut work, |input, caller, work| {
            let mut output = None;
            for turn in 0..100000 {
                if work.constituent_roster.receiving_primitives() || work.pending.is_some() && !work.domain_validated {
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
                        };
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
                if let ArtifactCommandWorkStep::Complete(emit) = super::super::turn(caller, 1, false, |cx| work.step(input, cx)).unwrap() {
                    output = Some(emit);
                    break;
                }
                assert!(turn < 99999);
            }
            let emit = output.unwrap();
            assert_eq!(emit.artifact_mutations.len(), row["mutations"].as_u64().unwrap() as usize);
            assert!(emit.artifact_mutations.iter().all(|mutation| matches!(mutation, IfcMutation::SetEntityArg(_))));
            let mut next = snapshot.as_ref().clone();
            let mut inverse = Vec::new();
            for mutation in &emit.artifact_mutations {
                inverse.push(mutation.inverse(&next).unwrap());
                next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
            }
            let expected = row["args"].as_array().unwrap().iter().map(enum_value).collect::<Vec<_>>();
            assert!(ifc_arguments_identical(&next.entities[0].args, &expected));
            for batch in inverse.into_iter().rev() {
                for mutation in batch {
                    next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
                }
            }
            assert_eq!(next.entities[0].name, snapshot.entities[0].name);
            assert!(ifc_arguments_identical(&next.entities[0].args, &snapshot.entities[0].args));
            super::super::constituent_optional_shape_tests::close_with_denials(caller, work);
        });
        caller.finish();
    }
}
#[test]
fn original_enum_guidance_cancels_and_retires_every_option_page_and_binding_prefix() {
    let law = enum_contract();
    let row = &law["cases"][2];
    let snapshot = enum_source(&law, row["source"].as_str().unwrap());
    let request = || enum_request(row, true);
    let mut settled = false;
    for prefix in 0..100000 {
        let mut caller = OriginalWorkFixture::new();
        let mut guide = Guidance::default();
        let mut donor = Some(request());
        let mut output: Option<PreparedGuidance> = None;
        assert!(caller.work_turn(grant(), 1, |cx| guide.receive(&mut donor, &snapshot, [0; 32], cx)).unwrap());
        let mut complete = false;
        for _ in 0..prefix {
            if caller.work_turn(grant(), 1, |cx| guide.advance(&snapshot, [0; 32], cx)).unwrap() {
                complete = true;
                break;
            }
        }
        if complete {
            assert!(caller.work_turn(grant(), 1, |cx| guide.take(&mut output, cx)).unwrap());
            settled = true;
        } else {
            caller.cancel().cancel_now();
            assert!(caller.work_turn(grant(), 1, |cx| guide.advance(&snapshot, [0; 32], cx)).is_err());
        }
        retire_guidance(&mut caller, &mut output);
        retire_guidance(&mut caller, &mut Some(guide));
        caller.finish();
        if complete {
            break;
        }
    }
    assert!(settled);
}

#[test]
fn original_enum_guidance_genuine_second_window_has_every_remaining_member_and_paid_close() {
    use semio_framework_plugin::app::staged_form::{StagedBodyBinding, StagedBodySource, StagedScalarKind};
    let law = enum_contract();
    for row in law["cases"].as_array().unwrap().iter().filter(|row| row["required"][0]["members"].as_array().unwrap().len() > 32) {
        let snapshot = enum_source(&law, row["source"].as_str().unwrap());
        let mut caller = OriginalWorkFixture::new();
        let mut first = Guidance::default();
        let mut donor = Some(enum_request(row, false));
        let mut output: Option<PreparedGuidance> = None;
        assert!(caller.work_turn(grant(), 1, |cx| first.receive(&mut donor, &snapshot, [0; 32], cx)).unwrap());
        for turn in 0..100000 {
            if caller.work_turn(grant(), 1, |cx| first.advance(&snapshot, [0; 32], cx)).unwrap() {
                break;
            }
            assert!(turn < 99999);
        }
        assert!(caller.work_turn(grant(), 1, |cx| first.take(&mut output, cx)).unwrap());
        let page = output.as_ref().unwrap();
        let mut keys = std::collections::BTreeSet::new();
        let StagedScalarKind::Choice { choices, .. } = page.control(0).unwrap().scalar.kind else { panic!("first choice") };
        for choice in choices {
            keys.insert(choice.key.to_owned());
        }
        let binding = (row["required"].as_array().unwrap().len()..page.controls())
            .find_map(|index| match page.control(index).unwrap().binding {
                Some(StagedBodyBinding::Page(binding)) if binding.page.choice.as_ref().is_some_and(|choice| choice.offset == "31") => Some(binding),
                _ => None,
            })
            .expect("actual second page");
        let mut request = enum_request(row, false);
        request.page = Some(binding.destination(page.status(), &request.token).unwrap().clone());
        request.token.generation = "2".into();
        let mut second = Guidance::default();
        let mut donor = Some(request);
        let mut next: Option<PreparedGuidance> = None;
        assert!(caller.work_turn(grant(), 1, |cx| second.receive(&mut donor, &snapshot, [0; 32], cx)).unwrap());
        for turn in 0..100000 {
            if caller.work_turn(grant(), 1, |cx| second.advance(&snapshot, [0; 32], cx)).unwrap() {
                break;
            }
            assert!(turn < 99999);
        }
        assert!(caller.work_turn(grant(), 1, |cx| second.take(&mut next, cx)).unwrap());
        let StagedScalarKind::Choice { choices, .. } = next.as_ref().unwrap().control(0).unwrap().scalar.kind else { panic!("second choice") };
        for choice in choices {
            keys.insert(choice.key.to_owned());
        }
        assert_eq!(keys.len(), row["required"][0]["members"].as_array().unwrap().len());
        retire_guidance(&mut caller, &mut next);
        retire_guidance(&mut caller, &mut Some(second));
        retire_guidance(&mut caller, &mut output);
        retire_guidance(&mut caller, &mut Some(first));
        caller.finish();
    }
}

#[test]
fn original_enum_repair_refuses_nonmembers_inherited_simple_and_invalid_preserved_aggregate_before_publication() {
    let law = enum_contract();
    for mode in 0..4 {
        let row = &law["cases"][if mode == 2 { 3 } else { 2 }];
        let mut snapshot = enum_source(&law, row["source"].as_str().unwrap());
        if mode == 2 {
            Arc::get_mut(&mut snapshot).unwrap().entities[0].complex.clear();
        }
        if mode == 3 {
            Arc::get_mut(&mut snapshot).unwrap().entities[0].args[0] = IfcValue::Aggregate(Vec::new());
        }
        let mut request = enum_request(row, true);
        if mode < 2 {
            let value = enum_field_mut(enum_field_mut(&mut request.args, "value"), "$ifcInputs");
            let DslValue::Array(rows) = value else { panic!("rows") };
            *enum_field_mut(enum_field_mut(&mut rows[0], "value"), "value") = DslValue::String(if mode == 0 { "userdefined" } else { "UNKNOWN" }.into());
        }
        let DslValue::Object(mut fields) = request.args else { panic!("args") };
        let ordinal = fields.iter().position(|(key, _)| key == "value").unwrap();
        let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: fields.remove(ordinal).1 };
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
            let mut refused = false;
            for turn in 0..100000 {
                match super::super::turn(caller, 1, false, |cx| work.step(input, cx)) {
                    Err(_) => {
                        refused = true;
                        break;
                    }
                    Ok(ArtifactCommandWorkStep::Complete(_)) => panic!("invalid enum candidate published"),
                    _ => {}
                }
                assert!(turn < 99999);
            }
            assert!(refused);
            assert!(!work.publication_validated);
            super::super::constituent_optional_shape_tests::close_with_denials(caller, work);
        });
        caller.finish();
    }
}

fn enum_field_mut<'a>(value: &'a mut DslValue, key: &str) -> &'a mut DslValue {
    let DslValue::Object(fields) = value else { panic!("object") };
    &mut fields.iter_mut().find(|(name, _)| name == key).unwrap().1
}

#[test]
fn original_enum_guidance_retains_multibyte_raw_sibling_when_an_earlier_choice_builds_sorted_rows() {
    use semio_framework_plugin::app::staged_form::{StagedBodyBinding, StagedBodySource};
    let law = enum_contract();
    let row = &law["cases"][1];
    let snapshot = enum_source(&law, "logical");
    let mut request = enum_request(row, false);
    *enum_field_mut(enum_field_mut(&mut request.args, "value"), "$ifcInputs") = raw(&serde_json::json!([{"index":"4","value":{"kind":"enum","value":law["draftSibling"]["text"]}}]));
    let mut caller = OriginalWorkFixture::new();
    let mut guide = Guidance::default();
    let mut donor = Some(request);
    let mut output: Option<PreparedGuidance> = None;
    assert!(caller.work_turn(grant(), 1, |cx| guide.receive(&mut donor, &snapshot, [0; 32], cx)).unwrap());
    for turn in 0..100000 {
        if caller.work_turn(grant(), 1, |cx| guide.advance(&snapshot, [0; 32], cx)).unwrap() {
            break;
        }
        assert!(turn < 99999);
    }
    assert!(caller.work_turn(grant(), 1, |cx| guide.take(&mut output, cx)).unwrap());
    let page = output.as_ref().unwrap();
    assert!(!page.status().ready);
    let Some(StagedBodyBinding::Action(binding)) = page.control(0).unwrap().binding else { panic!("typed field binding") };
    let semio_framework_plugin::action_staging::StagedFormBindingEdit::Fixed { edit: semio_framework_plugin::action_staging::StagedFormEdit::Set { value: DslValue::Array(rows), .. } } = &binding.edits[0] else { panic!("sorted rows") };
    assert_eq!(rows[0].get("index").unwrap().as_str(), Some("3"));
    assert_eq!(rows[1].get("index").unwrap().as_str(), Some("4"));
    assert_eq!(rows[1].get("value").unwrap().get("value").unwrap().as_str(), law["draftSibling"]["text"].as_str());
    caller.cancel().cancel_now();
    retire_guidance(&mut caller, &mut output);
    retire_guidance(&mut caller, &mut Some(guide));
    caller.finish();
}
                                                                                                                                         