//! 🧮️ Typed scalar drafts use the same original source, receiving and central inverse.
use super::*;
fn scalar_contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/type-scalar-input.json")).unwrap()
}
fn scalar_value(value: &serde_json::Value) -> IfcValue {
    if value["kind"] == "derived" { IfcValue::Derived } else { super::super::equal_arity_type_tests::value(value) }
}
fn scalar_source(law: &serde_json::Value, name: &str) -> Arc<IfcSnapshot> {
    Arc::new(IfcSnapshot {
        entities: law["sources"][name]["entities"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| IfcEntity {
                id: row["id"].as_str().unwrap().parse().unwrap(),
                name: row["name"].as_str().unwrap().into(),
                args: row["args"].as_array().unwrap().iter().map(scalar_value).collect(),
                complex: row["complex"].as_array().unwrap().iter().map(|part| IfcComplexType { name: part["name"].as_str().unwrap().into(), args: part["args"].as_array().unwrap().iter().map(scalar_value).collect() }).collect(),
            })
            .collect(),
        ..Default::default()
    })
}
fn scalar_request(row: &serde_json::Value, complete: bool) -> semio_framework_plugin::action_staging::StagedFormRequest {
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
fn original_scalar_type_guidance_admits_complete_exact_strings_and_signed_integers_without_defaults() {
    use semio_framework_plugin_decl_staged_form::StagedBodySource;
    let law = scalar_contract();
    for row in law["cases"].as_array().unwrap() {
        for complete in [false, true] {
            let snapshot = scalar_source(&law, row["source"].as_str().unwrap());
            let mut caller = OriginalWorkFixture::new();
            let mut guide = Guidance::default();
            let mut donor = Some(scalar_request(row, complete));
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
            assert_eq!(output.as_ref().unwrap().controls(), row["required"].as_array().unwrap().len());
            assert_eq!(output.as_ref().unwrap().status().ready, complete);
            retire_guidance(&mut caller, &mut output);
            retire_guidance(&mut caller, &mut Some(guide));
            caller.finish();
        }
    }
}
#[test]
fn original_scalar_type_inputs_keep_exact_complete_central_output_and_inverse() {
    let law = scalar_contract();
    for row in law["cases"].as_array().unwrap() {
        let snapshot = scalar_source(&law, row["source"].as_str().unwrap());
        let request = scalar_request(row, true);
        let DslValue::Object(mut fields) = request.args else { panic!("args") };
        let ordinal = fields.iter().position(|(key, _)| key == "value").unwrap();
        let value = fields.remove(ordinal).1;
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value }, &mut caller, &mut work, |input, caller, work| {
            let mut output = None;
            for turn in 0..100000 {
                if let ArtifactCommandWorkStep::Complete(emit) = super::super::turn(caller, 1, false, |cx| work.step(input, cx)).unwrap() {
                    output = Some(emit);
                    break;
                }
                assert!(turn < 99999);
            }
            let emit = output.unwrap();
            assert_eq!(emit.artifact_mutations.len(), row["mutations"].as_u64().unwrap() as usize);
            let mut next = snapshot.as_ref().clone();
            let mut inverse = Vec::new();
            for mutation in &emit.artifact_mutations {
                inverse.push(mutation.inverse(&next).unwrap());
                next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
            }
            let expected = row["args"].as_array().unwrap().iter().map(scalar_value).collect::<Vec<_>>();
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
fn original_scalar_guidance_cancels_and_retires_every_string_copy_and_binding_prefix() {
    let law = scalar_contract();
    let row = &law["cases"][0];
    let snapshot = scalar_source(&law, row["source"].as_str().unwrap());
    let request = || {
        let mut request = guidance_request();
        let text = "a".repeat(63) + &"🧭".repeat(20) + "ß";
        request.args = DslValue::Object(vec![
            ("path".into(), DslValue::String("/entities/0".into())),
            ("value".into(), DslValue::Object(vec![("$ifcType".into(), DslValue::String("IFCAPPLICATION".into())), ("$ifcInputs".into(), raw(&serde_json::json!([{"index":"3","value":{"kind":"string","value":text}}])))])),
        ]);
        request
    };
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
