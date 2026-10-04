use super::*;

#[test]
fn mesh_component_vertices_match_evaluated_topology_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🧫️fixtures/🥽️topology/🔣️.json")).unwrap();
    let data = fixture["mesh"].to_string();
    for case in fixture["valid"].as_array().unwrap() {
        let ids = case["ids"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        let vertices = selected_mesh_vertices(&ids, &data).unwrap();
        assert_eq!(vertices, serde_json::from_value::<Vec<usize>>(case["vertices"].clone()).unwrap());
        let (target, components) = component_group(&ids).unwrap();
        assert_eq!(component_pivot(&data, target.granularity, &components).unwrap(), serde_json::from_value::<[f64; 3]>(case["pivot"].clone()).unwrap());
    }
    for case in fixture["invalid"].as_array().unwrap() {
        let ids = case.as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        assert!(selected_mesh_vertices(&ids, &data).is_err());
    }
}

#[test]
fn mesh_component_cached_validation_rejects_stale_indices_without_evaluation() {
    let _serial = crate::test_serial::lock();
    let snapshot = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::example_snapshot(semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
    let mut owned_session = FlowEvalSession::new().with_geometry_port(crate::flow_operators::geometry_session().port());
    let session = &mut owned_session;
    {
        let ids = ["extrude@meshOut#0.face.0".into()];
        assert!(validate_cached_components(&snapshot.host_snapshot, session, &ids, None).is_err());
        semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::with_host_session(&snapshot.host_snapshot, session, |host, session| {
            host.evaluate().unwrap();
            session.capture_baseline_from(host);
        });
        validate_cached_components(&snapshot.host_snapshot, session, &ids, None).unwrap();
        for mode in ["face", "edge", "vertex"] {
            assert!(validate_cached_components(&snapshot.host_snapshot, session, &[format!("extrude@meshOut#0.{mode}.999999")], None).is_err());
        }
        semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::with_host_session(&snapshot.host_snapshot, session, |host, session| {
            let (id, _, _) = semio_s_artifact_procedural_generation3d::editor::generation3d::transform_commands::ensure_component_node(host, &ids, "translate").unwrap();
            validate_cached_components(&host.host_snapshot, session, &[format!("{id}@meshOut#0.face.0")], None).unwrap();
            host.set_neuron_params("extrude", r#"{"distance":{"$schema":"number","value":0.7}}"#).unwrap();
            assert!(validate_cached_components(&host.host_snapshot, session, &ids, None).is_err());
            assert!(validate_cached_components(&host.host_snapshot, session, &[format!("{id}@meshOut#0.face.0")], None).is_err());
        });
    }
    crate::flow_operators::retire_flow_eval_session(owned_session);
    snapshot.retire_cold();
}

#[test]
fn mesh_component_pivot_uses_unique_topology_vertices() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../../../../../../../../../../../🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧫️fixtures/🧭️component-transform/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mesh = semio_framework_pack_json::to_string(fixture.get("mesh").unwrap());
    for case in fixture.get("cases").unwrap().as_array().unwrap() {
        let transform = case.get("transform").unwrap();
        if transform.get("pivot").and_then(semio_framework_pack_json::Value::as_str) != Some("selection") { continue; }
        let ids = transform.get("selection").unwrap().as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect::<Vec<_>>();
        let pivot = component_pivot(&mesh, transform.get("mode").unwrap().as_str().unwrap(), &ids).unwrap();
        for axis in 0..3 { assert!((pivot[axis] - case.get("pivot").unwrap().as_array().unwrap()[axis].as_f64().unwrap()).abs() < 1e-7); }
    }
    assert!(component_pivot(&mesh, "vertex", &[999]).is_none());
    assert!(component_pivot(&mesh, "edge", &[]).is_none());
    assert!(component_pivot(&mesh, "object", &[0]).is_none());
}

#[test]
fn mesh_quick_actions_match_component_kind_and_locale() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixtures["quickActions"].as_array().unwrap() {
        let granularity = case["granularity"].as_str().unwrap();
        let selection = ComponentSelection { granularity: granularity.into(), selected: vec![format!("box@meshOut#0.{granularity}.0")], hovered: None };
        for is_de in [false, true] {
            let actions = engagement(&selection, is_de).possible_engagements.unwrap();
            let expected = case["operations"].as_array().unwrap().iter().map(|operation| format!("procedural.mesh-{}", operation.as_str().unwrap())).collect::<Vec<_>>();
            assert_eq!(actions.iter().map(|action| action.id.clone()).collect::<Vec<_>>(), expected);
            assert!(actions.iter().all(|action| !action.label.is_empty() && action.action.is_some()));
            if granularity == "edge" { assert_eq!(actions[0].label, if is_de { "Schleife schneiden" } else { "Cut Loop" }); }
        }
    }
}

#[test]
fn mesh_component_targets_match_shared_contract() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🧫️fixtures/🔣️.json")).unwrap();
    for target in fixtures["valid"].as_array().unwrap() {
        let actual = ComponentTarget::parse(target["id"].as_str().unwrap()).unwrap();
        assert_eq!(actual.instance, target["instance"].as_str().unwrap());
        assert_eq!(actual.widget, target["widget"].as_str().unwrap());
        assert_eq!(actual.channel, target["channel"].as_str().unwrap());
        assert_eq!(actual.index, target["index"].as_u64().unwrap() as usize);
        assert_eq!(actual.granularity, target["granularity"].as_str().unwrap());
        assert_eq!(actual.component, target["component"].as_u64().unwrap() as u32);
    }
    for id in fixtures["invalid"].as_array().unwrap() {
        assert!(ComponentTarget::parse(id.as_str().unwrap()).is_none(), "{id}");
    }
    for group in fixtures["groups"].as_array().unwrap() {
        let ids = group["ids"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_string()).collect::<Vec<_>>();
        let actual = component_group(&ids);
        if group["error"].as_bool().unwrap_or(false) {
            assert!(actual.is_err());
        } else {
            let (target, components) = actual.unwrap();
            assert_eq!(target.instance, group["instance"].as_str().unwrap());
            assert_eq!(target.granularity, group["granularity"].as_str().unwrap());
            assert_eq!(components, group["components"].as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u32).collect::<Vec<_>>());
        }
    }
}

#[test]
fn mesh_component_projection_preserves_instance_and_filters_hidden_targets() {
    let selection = ComponentSelection { granularity: "face".into(), selected: vec!["box@meshOut#0.face.2".into(), "gone@meshOut#0.face.3".into()], hovered: Some("box@meshOut#0.face.1".into()) };
    let instances = semio_framework_pack_json::parse(r#"[{"id":"box@meshOut#0"}]"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut value = semio_framework_pack_json::Object::new();
    selection.project(&instances, &semio_framework_pack_json::Value::Null, &mut value);
    let json: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(value))).unwrap();
    assert_eq!(json["selectionMode"], "face");
    assert_eq!(json["activeObjectId"], "box@meshOut#0");
    assert_eq!(json["componentIds"], serde_json::json!([2]));
    assert_eq!(json["ids"], serde_json::json!(["box@meshOut#0"]));
    assert_eq!(json["gumballSelectionIds"], serde_json::json!(["box@meshOut#0.face.2"]));
    assert_eq!(json["hoveredComponent"], serde_json::json!({"objectId":"box@meshOut#0","mode":"face","id":1}));
    assert_eq!(json["gumballActive"], false);
    for is_de in [false, true] {
        let controls = engagement(&selection, is_de).options.unwrap();
        assert_eq!(controls.len(), 4);
        assert_eq!(controls.iter().filter(|item| item.pressed == Some(true)).map(|item| item.id.as_str()).collect::<Vec<_>>(), vec!["procedural.select-face"]);
        assert!(controls.iter().all(|item| item.label.as_ref().is_some_and(|label| !label.is_empty())));
    }
}

#[test]
fn mesh_component_edit_context_is_scoped_to_the_addressed_editor_preview() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixtures["contexts"].as_array().unwrap() {
        let kind = case["window"].as_str().unwrap();
        let view = semio_framework_plugin::ViewModel {
            active_window_kind_id: Some("procedural-preview".into()),
            window_id: Some("target".into()),
            window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "target".into(), window_kind_id: kind.into() }],
            ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        };
        assert_eq!(edits_components(Some(&view), case["granularity"].as_str()), case["components"].as_bool().unwrap(), "{case}");
    }
    assert!(!edits_components(None, Some("face")));
}
