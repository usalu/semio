//! 🧭️ The actual original GuidedProducer renders every required reference without selecting IDs.
use super::*;
fn multiple_value(input: &serde_json::Value) -> IfcValue {
    match input["kind"].as_str().unwrap() {
        "aggregate" => IfcValue::Aggregate(input["value"].as_array().unwrap().iter().map(multiple_value).collect()),
        "string" => IfcValue::String(input["value"].as_str().unwrap().into()),
        "reference" => IfcValue::Reference(input["value"].as_str().unwrap().parse().unwrap()),
        "enum" => IfcValue::Enum(input["value"].as_str().unwrap().into()),
        "real" => IfcValue::Real(f64::from_bits(input["value"]["bits"].as_str().unwrap().parse().unwrap())),
        "unset" => IfcValue::Unset,
        _ => panic!("unhandled neutral value"),
    }
}
#[test]
fn multiple_required_references_use_the_original_controller_and_keep_supplied_fields_editable() {
    use semio_framework_plugin::{AppRenderOperationContext, ChildContentView};
    use semio_framework_ui_contract::{Component, ContainerRole, OriginalBuiltTreeRetirement};
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcComplexType, IfcEntity};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔎️details/🧪️tests/type-multiple-input.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let entities = law["sources"][row["source"].as_str().unwrap()]["entities"]
            .as_array()
            .unwrap()
            .iter()
            .map(|input| IfcEntity {
                id: input["id"].as_str().unwrap().parse().unwrap(),
                name: input["name"].as_str().unwrap().into(),
                args: input["args"].as_array().unwrap().iter().map(multiple_value).collect(),
                complex: input["complex"].as_array().unwrap().iter().map(|part| IfcComplexType { name: part["name"].as_str().unwrap().into(), args: part["args"].as_array().unwrap().iter().map(multiple_value).collect() }).collect(),
            })
            .collect();
        let snapshot = std::sync::Arc::new(IfcSnapshot { entities, ..Default::default() });
        let history = semio_framework_plugin::HistoryView::empty();
        let view = staged_view();
        let mut request = staged_request(&serde_json::json!({"path":"/entities/0","value":{"$ifcType":row["target"],"$ifcInputs":row["initial"]}}));
        request.token.action = "setSnapshotValue".into();
        let operation = AppRenderOperationContext {
            app_instance_id: 17,
            base_revision: semio_framework_job::RevisionId(0),
            generation: semio_framework_job::Generation(1),
            canonical_base_revision: [0; 32],
            mounted_policy: semio_framework_plugin::app_testing::artifact_app_laws::fixture_mounted_policy(),
        };
        let doc = semio_framework_async::poll::resolve_ready(ArtifactView::with_render_context(snapshot.as_ref(), &history, ChildContentView::EMPTY, operation, "multiple-source", None));
        let mut caller = OriginalWorkFixture::new();
        let mut work = caller.work_turn(staged_grant(), 1000, |cx| Ifc4x3AnyEditor::begin_render_work(semio_framework_plugin::action_staging::STAGED_FORM_BODY_KEY, cx)).unwrap().unwrap();
        let mut donor = Some(request);
        let mut received = false;
        for _ in 0..16 {
            if caller.work_turn(staged_grant(), 1000, |cx| Ifc4x3AnyEditor::receive_render_work_request(&mut work, &mut donor, &view, cx)).unwrap().is_some() {
                received = true;
                break;
            }
        }
        assert!(received && donor.is_none());
        let mut output = None;
        for _ in 0..100000 {
            match caller.work_turn(staged_grant(), 1000, |cx| Ok(work.render(semio_framework_plugin::action_staging::STAGED_FORM_BODY_KEY, &snapshot, &doc, &view, cx))).unwrap() {
                Ok(Some(tree)) => {
                    output = Some(tree);
                    break;
                }
                Ok(None) => {}
                Err(_) => panic!("multiple neutral guidance refuses"),
            }
        }
        let tree = output.take().expect("original multiple reference body");
        let Component::Container(form) = &tree.root.component else { panic!("missing form") };
        assert_eq!(form.role, ContainerRole::Form);
        let count = row["required"].as_array().unwrap().len();
        assert_eq!(tree.root.children.len(), count + 1);
        for field in 0..count {
            let node = tree.root.children.get(field + 1).unwrap().children.get(0).unwrap();
            let Component::Input(props) = &node.component else { panic!("required input") };
            let index = &row["required"][field];
            let raw = row["initial"].as_array().unwrap().iter().find(|item| item["index"].as_str().unwrap() == index.as_u64().unwrap().to_string());
            assert_eq!(props.value.as_ref(), raw.and_then(|item| item["value"]["value"].as_str()).unwrap_or(""));
            assert!(!node.bindings.is_empty());
        }
        let mut close = OriginalBuiltTreeRetirement::new(tree.root);
        for _ in 0..100000 {
            if close.terminal_is_empty() {
                break;
            }
            caller
                .turn(staged_grant(), |cx| {
                    cx.with_original_retirement(|cx| {
                        let demand = close.next_demand(32768)?;
                        semio_framework_job::close_original_retained_step(cx, demand, |grant| close.step(grant))?;
                        Ok(())
                    })
                })
                .unwrap();
        }
        assert!(close.terminal_is_empty());
        caller.cancel().cancel_now();
        retire_staged(&mut caller, &mut Some(work));
        caller.finish();
    }
}
