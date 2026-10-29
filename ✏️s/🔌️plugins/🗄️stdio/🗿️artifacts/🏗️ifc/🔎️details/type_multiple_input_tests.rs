//! 🧭️ Multiple original reference fields share one source and strict central mutation authority.
use super::*;
fn multiple_contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/type-multiple-input.json")).unwrap()
}
fn multiple_source(law: &serde_json::Value, name: &str) -> Arc<IfcSnapshot> {
    Arc::new(IfcSnapshot { entities: law["sources"][name]["entities"].as_array().unwrap().iter().map(|row| super::super::equal_arity_type_tests::entity(row, row["id"].as_str().unwrap().parse().unwrap())).collect(), ..Default::default() })
}
fn multiple_request(target: &str, rows: &serde_json::Value) -> semio_framework_plugin::action_staging::StagedFormRequest {
    let mut request = guidance_request();
    request.args = DslValue::Object(vec![("path".into(), DslValue::String("/entities/0".into())), ("value".into(), DslValue::Object(vec![("$ifcType".into(), DslValue::String(target.into())), ("$ifcInputs".into(), raw(rows))]))]);
    request
}
#[test]
fn original_multiple_reference_guidance_keeps_all_fields_and_denies_each_published_axis() {
    use semio_framework_plugin_decl_staged_form::StagedBodySource;
    let law = multiple_contract();
    for row in law["cases"].as_array().unwrap() {
        let snapshot = multiple_source(&law, row["source"].as_str().unwrap());
        let mut caller = OriginalWorkFixture::new();
        let mut guide = Guidance::default();
        let mut donor = Some(multiple_request(row["target"].as_str().unwrap(), &row["initial"]));
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
        let prepared = output.as_ref().unwrap();
        assert_eq!(prepared.controls(), row["required"].as_array().unwrap().len());
        for index in 0..prepared.controls() {
            let control = prepared.control(index).unwrap();
            assert!(control.required && control.binding.is_some());
            assert_eq!(control.scalar.address, index as u64);
        }
        retire_guidance(&mut caller, &mut output);
        retire_guidance(&mut caller, &mut Some(guide));
        caller.finish();
    }
}
#[test]
fn original_multiple_reference_guidance_cancels_and_retires_every_accepted_prefix() {
    let law = multiple_contract();
    let row = &law["cases"][1];
    let snapshot = multiple_source(&law, row["source"].as_str().unwrap());
    let mut total = 0;
    for prefix in 0..100000 {
        let mut caller = OriginalWorkFixture::new();
        let mut guide = Guidance::default();
        let mut donor = Some(multiple_request(row["target"].as_str().unwrap(), &row["initial"]));
        let mut output: Option<PreparedGuidance> = None;
        assert!(caller.work_turn(grant(), 1, |cx| guide.receive(&mut donor, &snapshot, [0; 32], cx)).unwrap());
        let mut completed = false;
        for _ in 0..prefix {
            if caller.work_turn(grant(), 1, |cx| guide.advance(&snapshot, [0; 32], cx)).unwrap() {
                completed = true;
                break;
            }
        }
        if completed {
            assert!(caller.work_turn(grant(), 1, |cx| guide.take(&mut output, cx)).unwrap());
            total = prefix;
        } else {
            caller.cancel().cancel_now();
            assert!(caller.work_turn(grant(), 1, |cx| guide.advance(&snapshot, [0; 32], cx)).is_err());
        }
        retire_guidance(&mut caller, &mut output);
        retire_guidance(&mut caller, &mut Some(guide));
        caller.finish();
        if completed {
            break;
        }
    }
    assert!(total > 0 && total < 100000);
}

#[test]
fn original_multiple_reference_inputs_keep_exact_central_output_and_progressive_inverse() {
    let law = multiple_contract();
    for row in law["cases"].as_array().unwrap() {
        let snapshot = multiple_source(&law, row["source"].as_str().unwrap());
        let rows = serde_json::Value::Array(row["required"].as_array().unwrap().iter().map(|index| serde_json::json!({"index":index.as_u64().unwrap().to_string(),"value":row["args"][index.as_u64().unwrap()as usize]})).collect());
        let request = multiple_request(row["target"].as_str().unwrap(), &rows);
        let DslValue::Object(mut fields) = request.args else { panic!("args") };
        let value = fields.remove(fields.iter().position(|(key, _)| key == "value").unwrap()).1;
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
            let expected = row["args"].as_array().unwrap().iter().map(super::super::equal_arity_type_tests::value).collect::<Vec<_>>();
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
