//! 🧪️ Native authoring consumes the same independently validated decimal commands and STEP models.
use super::*;
fn original(value: &serde_json::Value) -> semio_framework_value::DslValue {
    let parsed = semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    semio_framework_pack_json::to_dsl_value(&parsed)
}
#[test]
fn native_template_identity_query_uses_complete_planner_requirements_without_invented_ids(){
    let fixtures:serde_json::Value=serde_json::from_str(include_str!("native-cases.json")).unwrap();
    let cases:serde_json::Value=serde_json::from_str(include_str!("identity-query-cases.json")).unwrap();
    for query in cases["queries"].as_array().unwrap(){
        let case=fixtures["cases"].as_array().unwrap().iter().find(|case|case["name"]==query["sourceCase"]).unwrap();let source=crate::io::parse_ifc4x3_step(case["source"].as_str().unwrap()).unwrap();let mut application=case["application"].clone();application["identities"]=serde_json::json!([]);
        let actual=work::test_identity_query(original(&application),&source,None).unwrap();let expected=query["requirements"].as_array().unwrap();assert_eq!(actual.len(),expected.len());
        for((key,global_id),expected)in actual.into_iter().zip(expected){assert!(key_matches(key,expected["key"].as_str().unwrap()));assert_eq!(global_id,expected["globalId"].as_bool().unwrap());}
        assert!(work::test_plan(original(&application),&source,None).unwrap_err().contains("identityMissing"));
        for cut in [0,1,2,8,16,32]{assert!(work::test_identity_query(original(&application),&source,Some(cut)).is_err());}
    }
}
#[test]
fn retained_native_template_route_matches_all_independent_authoring_models() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("native-cases.json")).unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let source = crate::io::parse_ifc4x3_step(case["source"].as_str().unwrap()).unwrap();
        let expected = crate::io::parse_ifc4x3_step(case["expected"].as_str().unwrap()).unwrap();
        let original = original(&case["application"]);
        let mutations = work::test_plan(original, &source, None).unwrap();
        let mut actual = source.clone();
        let mut inverses = Vec::new();
        for mutation in &mutations {
            inverses.push(protocol::ColdMutation::inverse(mutation, &actual).unwrap());
            let outcome = protocol::ColdMutation::diff(mutation, &actual);
            actual = protocol::apply_diff_cold(outcome.diff(), &actual).unwrap();
        }
        assert_eq!(actual, expected, "{}", case["name"]);
        for inverse in inverses.into_iter().rev() {
            for mutation in inverse {
                let outcome = protocol::ColdMutation::diff(&mutation, &actual);
                actual = protocol::apply_diff_cold(outcome.diff(), &actual).unwrap();
            }
        }
        assert_eq!(actual, source);
        let report = crate::validation::validate_ifc4x3_express(&expected, Default::default());
        assert!(report.complete && report.valid, "{} {:?}", case["name"], report.diagnostics);
    }
}
#[test]
fn retained_native_template_route_refuses_each_cancelled_frontier_before_emission() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("native-cases.json")).unwrap();
    let case = &fixtures["cases"][0];
    let source = crate::io::parse_ifc4x3_step(case["source"].as_str().unwrap()).unwrap();
    for cut in [0, 1, 2, 3, 8, 16, 32, 64, 128] {
        let result = work::test_plan(original(&case["application"]), &source, Some(cut));
        assert!(result.is_err(), "cancel frontier {cut}");
    }
}
#[test]
fn native_template_capacity_covers_every_released_selection_without_invented_profiles() {
    let maximum = super::super::ifc4x3_template_catalog().templates.iter().map(|template| template.entries.len()).max().unwrap();
    assert_eq!(maximum, MAXIMUM_ENTRIES);
    assert_eq!(MAXIMUM_IDENTITIES, maximum * 2 + 4);
}

#[test]
fn native_template_command_matches_its_closed_wire_variant_and_domain_action() {
    use protocol::OpBinary;
    use semio_framework_plugin::ArtifactEditor;
    use semio_framework_value::ToValue;
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("native-cases.json")).unwrap();
    let application = original(&fixtures["cases"][0]["application"]);
    let command = crate::editor::Ifc4x3EditCommand::ApplyTemplate { application: application.clone() };
    assert_eq!(crate::editor::Ifc4x3AnyEditor::command_id(&command), APPLY_TEMPLATE_ACTION);
    let bytes = command.encode_op().unwrap();
    let restored = crate::editor::Ifc4x3EditCommand::decode_op(&bytes).unwrap();
    assert_eq!(restored, command);
    assert_eq!(command.to_value().get("ApplyTemplate").unwrap().get("application"), Some(&application));
    let contracts = <work::factory::IfcTemplateToolJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(contracts.len(), 1);
    assert_eq!(contracts[0].tool_id, APPLY_TEMPLATE_ACTION);
}

#[test]
fn borrowed_inverse_preimages_reject_duplicate_slots_and_insert_then_edit() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("preimages.json")).unwrap();
    let source = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCPROPERTYSINGLEVALUE".into(), args: vec![IfcValue::Unset; 2], complex: Vec::new() }], ..Default::default() };
    for case in fixture["cases"].as_array().unwrap() {
        let mutations: Vec<_> = case["operations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|operation| {
                let id = operation["id"].as_str().unwrap().parse().unwrap();
                let index = operation["index"].as_u64().unwrap() as usize;
                if operation["kind"] == "insert" {
                    IfcMutation::InsertEntity(InsertEntity { index, entity: IfcEntity { id, name: "IFCPROPERTYSINGLEVALUE".into(), args: Vec::new(), complex: Vec::new() } })
                } else {
                    IfcMutation::SetEntityArg(SetEntityArg { id, index, value: IfcValue::Unset })
                }
            })
            .collect();
        assert_eq!(independent_batch(&Reader { snapshot: &source, order: &[0] }, &mutations).is_ok(), case["admitted"].as_bool().unwrap(), "{}", case["name"]);
    }
}

#[test]
fn existing_template_definition_sets_are_checked_before_retained_identity_collisions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("native-select.json")).unwrap();
    let cases: serde_json::Value = serde_json::from_str(include_str!("native-cases.json")).unwrap();
    let case = cases["cases"].as_array().unwrap().iter().find(|case| case["name"] == fixture["sourceCase"]).unwrap();
    let mut source = crate::io::parse_ifc4x3_step(case["expected"].as_str().unwrap()).unwrap();
    let relation = source.entities.iter_mut().find(|entity| entity.name == fixture["relation"].as_str().unwrap()).unwrap();
    let index = attributes(&relation.name).unwrap().iter().position(|attribute| attribute.name == fixture["field"].as_str().unwrap()).unwrap();
    let value = std::mem::take(&mut relation.args[index]);
    relation.args[index] = IfcValue::TypedValue(IfcTypedValue { name: fixture["type"].as_str().unwrap().into(), items: vec![IfcValue::Aggregate(vec![value])] });
    let error = work::test_plan(original(&case["application"]), &source, None).unwrap_err();
    assert!(error.contains(fixture["refusal"].as_str().unwrap()));
}

#[test]
fn native_factory_transfers_original_payload_and_never_publishes_without_semantic_authority() {
    use semio_framework_plugin::{
        app_testing::OriginalWorkFixture,
        retained_command::{ArtifactCommandInputs, ArtifactCommandSnapshotRead, ArtifactCommandWork, ArtifactCommandWorkStep},
        AppOperationContext, HistoryView, NoConfig,
    };
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use std::sync::Arc;
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("native-cases.json")).unwrap();
    let case = &fixtures["cases"][0];
    let source = Arc::new(crate::io::parse_ifc4x3_step(case["source"].as_str().unwrap()).unwrap());
    let saved = source.as_ref().clone();
    for cut in [None, Some(0), Some(1), Some(8), Some(64), Some(128), Some(256)] {
        let mut command = crate::editor::Ifc4x3EditCommand::ApplyTemplate { application: original(&case["application"]) };
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(918), semio_framework_job::Generation(1));
        let mut work = work::IfcTemplateWork::default();
        let grant = RetainedCloneGrant { maximum_items: 1_000_000, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 };
        if cut == Some(0) {
            caller.cancel().cancel_now();
        }
        let received = caller.work_turn(grant, 1_000_000, |cx| work.receive_original_command(&mut command, cx));
        let mut refused = received.is_err();
        if !refused {
            assert!(received.unwrap());
            assert!(matches!(&command, crate::editor::Ifc4x3EditCommand::ApplyTemplate { application: semio_framework_value::DslValue::Null }));
            let config = NoConfig {};
            let history = HistoryView::empty();
            let interaction = Default::default();
            let hover = Default::default();
            let operation = AppOperationContext {
                app_instance_id: 17,
                parent_document_id: "ifc-template-native-test".into(),
                operation_id: 918,
                generation: 1,
                canonical_base_revision: [0; 32],
                retained: grant,
                authoring_seed: "retained-template-identities".into(),
            };
            let input = ArtifactCommandInputs {
                command: &command,
                snapshot: source.as_ref(),
                snapshot_owner: Some(&source),
                snapshot_read: ArtifactCommandSnapshotRead::unavailable(&operation),
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: None,
                operation: &operation,
            };
            for turn in 1..100_000 {
                if cut == Some(turn) {
                    caller.cancel().cancel_now();
                }
                match caller.work_turn(grant, 1_000_000, |cx| work.step(&input, cx)) {
                    Ok(ArtifactCommandWorkStep::Complete(_)) => panic!("Native template emitted without retained EXPRESS authority"),
                    Err(error) => {
                        if cut.is_none() {
                            assert!(format!("{error:?}").contains("template.semantic-admission-unavailable"));
                        }
                        refused = true;
                        break;
                    }
                    _ => {}
                }
            }
        }
        assert!(refused);
        assert_eq!(source.as_ref(), &saved);
        work.begin_close();
        for _ in 0..100_000 {
            if work.terminal_is_empty() {
                break;
            }
            caller.close(&mut work, grant).unwrap();
        }
        assert!(work.terminal_is_empty());
        let mut incoming = Some(command);
        let mut retirement = semio_framework_value::retained_clone::RetainedCloneClose::default();
        for _ in 0..100_000 {
            if incoming.is_none() && retirement.is_empty() {
                break;
            }
            caller
                .turn(grant, |cx| {
                    cx.with_original_retirement(|cx| {
                        let demand = retirement.next_owner_demands::<crate::editor::Ifc4x3EditCommand>(incoming.is_some(), &None, 4096)?;
                        semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                            if retirement.is_empty() {
                                retirement.begin_granted(&mut incoming, grant)?.ok_or_else(|| receive::invalid("Template test incoming command omitted its owner"))
                            } else {
                                retirement.step_granted(grant)
                            }
                        })?;
                        Ok(())
                    })
                })
                .unwrap();
        }
        assert!(incoming.is_none() && retirement.is_empty());
        caller.finish();
    }
}
