/// 🪄️ Mesh component laws: the mesh gumball, component edits and quick actions.

#[test]
fn mesh_gumball_splices_typed_transforms_and_preserves_analysis_consumers() {
    let _serial = test_serial();
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["gumball"].as_array().unwrap() {
        let snapshot = semio_s_artifact_procedural_3d_model::example_snapshot(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
        semio_s_artifact_procedural_3d_model::with_host(&snapshot.host_snapshot, |host| {
            let operation = case["operation"].as_str().unwrap();
            let id = semio_s_artifact_procedural_3d_transforms::ensure_gumball_node(host, "extrude@meshOut#0", operation).unwrap();
            let transform = host.host_snapshot.widgets.iter().find(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget) == id).unwrap();
            assert!(matches!(transform, semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, preview: true, .. } if neuron_kind == case["operator"].as_str().unwrap()));
            assert!(host.host_snapshot.synapses.iter().any(|wire| wire.from == "extrude" && wire.from_port == "meshOut" && wire.to == id && wire.to_port == case["input"].as_str().unwrap()));
            assert!(host.host_snapshot.synapses.iter().any(|wire| wire.from == id && wire.from_port == case["output"].as_str().unwrap() && wire.to == "analysis"));
            assert_eq!(semio_s_artifact_procedural_3d_transforms::ensure_gumball_node(host, &id, operation).unwrap(), id);
        });
        snapshot.retire_cold();
    }
}

fn mesh_selection_fixture_snapshot(operation: &str) -> Generation3dSnapshot {
    let mut snapshot = Generation3dSnapshot::default();
    let graph = semio_s_artifact_procedural_3d_model::with_host(&snapshot.host_snapshot, |host| {
        let kind = if operation == "dissolveVertices" { "brep.mesh.construct" } else { "brep.mesh.box" };
        host.add_widget(&serde_json::json!({"kind":"neuron","id":"extrude","neuronKind":kind}).to_string(), 0.0, 0.0).unwrap();
        host.add_widget(r#"{"kind":"neuron","id":"analysis","neuronKind":"brep.mesh.analyze"}"#, 220.0, 0.0).unwrap();
        host.connect_ports("extrude", "meshOut", "analysis", "mesh").unwrap();
        host.host_snapshot.clone()
    });
    let empty = std::mem::replace(&mut snapshot.host_snapshot, graph);
    empty.retire_cold();
    if operation == "dissolveVertices" {
        let data = serde_json::json!({"vertices":[[0,0,0],[1,0,0],[1,1,0],[0,1,0],[0.5,0.5,0]],"faces":[[0,1,4],[1,2,4],[2,3,4],[3,0,4]]}).to_string();
        let input = semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::change_widget_input("extrude", "data", semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputValue::Text(data));
        semio_s_artifact_procedural_3d_mutations::central_apply::apply_generation3d_mutation(&mut snapshot, &input).unwrap();
        input.retire_cold();
    }
    snapshot
}

#[test]
fn mesh_component_edits_insert_typed_widgets_and_update_downstream_analysis() {
    let _serial = test_serial();
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥽️edit-mesh-selection/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["valid"].as_array().unwrap() {
        let operation = case["name"].as_str().unwrap();
        let snapshot = mesh_selection_fixture_snapshot(operation);
        let ids: Vec<String> = serde_json::from_value(case["ids"].clone()).unwrap();
        let payload: edit_mesh_selection::EditMeshSelection = semio_framework_pack_json::from_json_str(&case["payload"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let (id, rows) = edit_mesh_selection::edit_rows(&payload, &snapshot.host_snapshot, &ids).unwrap();
        let field = match operation { "moveVertices" => "vertices", "loopCut" | "bevel" | "dissolveEdges" => "edges", "dissolveVertices" | "mergeVertices" | "moveProportional" | "snapVertices" => "selection", _ => "faces" };
        let inputs: Vec<&semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::ChangeWidgetInput> = rows.iter().filter_map(|row| match row { semio_s_artifact_procedural_3d_mutations::Generation3dMutation::ChangeWidgetInput(input) => Some(input), _ => None }).collect();
        assert!(inputs.iter().all(|input| input.id == id) && inputs[0].channel == field, "{operation}: the user's inputs are absolute leaves on the inserted operator: {inputs:?}");
        assert!(rows.iter().rev().take(inputs.len()).all(|row| matches!(row, semio_s_artifact_procedural_3d_mutations::Generation3dMutation::ChangeWidgetInput(_))), "{operation}: the splice precedes the inputs");
        let mut landed = snapshot.clone();
        for row in rows {
            semio_s_artifact_procedural_3d_mutations::central_apply::apply_generation3d_mutation(&mut landed, &row).unwrap();
            row.retire_cold();
        }
        let outcome = semio_s_artifact_procedural_3d_model::with_host(&landed.host_snapshot, |host| {
            let params = host.host_snapshot.widgets.iter().find_map(|widget| match widget { semio_framework_artifact_flow_flow::Widget::Neuron { id: widget_id, params, preview: true, .. } if widget_id == &id => Some(semio_framework_pack_json::to_json_string(params)), _ => None });
            let feeds_analysis = host.host_snapshot.synapses.iter().any(|wire| wire.from == id && wire.from_port == "meshOut" && wire.to == "analysis" && wire.to_port == "mesh");
            let evaluation = host.evaluate().map_err(|error| error.to_string())?;
            Ok::<_, String>((params, feeds_analysis, evaluation, semio_framework_pack_json::to_json_string(&host.host_snapshot.synapses)))
        });
        landed.retire_cold();
        snapshot.retire_cold();
        let (params, feeds_analysis, evaluation, wires) = outcome.unwrap();
        let params: Value = serde_json::from_str(&params.unwrap()).unwrap();
        let indices: Value = serde_json::from_str(params[field]["value"].as_str().unwrap()).unwrap();
        assert_eq!(indices, serde_json::from_str::<Value>(case["inputs"][field].as_str().unwrap()).unwrap());
        for (channel, expected) in case["inputs"].as_object().unwrap() {
            let actual = if expected.is_array() { serde_json::json!([params[channel]["x"], params[channel]["y"], params[channel]["z"]]) } else { params[channel]["value"].clone() };
            assert_eq!(&actual, expected, "{operation}/{channel}");
        }
        assert!(feeds_analysis, "{operation}: {id} must feed downstream analysis: {wires}");
        let evaluation: Value = serde_json::from_str(&evaluation).unwrap();
        assert_eq!(evaluation[&id]["out"]["meshOut"]["$schema"], "mesh", "{operation}: {evaluation}");
        assert_eq!(evaluation["analysis"]["out"]["vertices"]["$schema"], "number", "{operation}: downstream analysis evaluated");
    }
}

#[test]
fn mesh_component_gumball_pins_the_complete_component_set() {
    let _serial = test_serial();
    let snapshot = semio_s_artifact_procedural_3d_model::example_snapshot(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
    let fixtures: Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️transforms/🧪️tests/🧫️fixtures/🧷️gesture/🔣️.json")).unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let pinned: Vec<String> = serde_json::from_value(case["pinned"].clone()).unwrap();
        let selected: Vec<String> = serde_json::from_value(case["selected"].clone()).unwrap();
        assert_eq!(transform_commands::validate_component_gesture(&snapshot.host_snapshot, &pinned, &selected).is_ok(), case["valid"].as_bool().unwrap(), "{}", case["name"]);
    }
    semio_s_artifact_procedural_3d_model::with_host(&snapshot.host_snapshot, |host| {
        let pinned = vec!["extrude@meshOut#0.face.0".into(), "extrude@meshOut#0.face.1".into()];
        let mut selected = pinned.clone();
        for operation in ["translate", "rotate", "scale"] {
            let (id, mode, components) = transform_commands::ensure_component_node(host, &selected, operation).unwrap();
            land_component_inputs(host, &id, &mode, &components, operation);
            selected = components.iter().map(|component| format!("{id}@meshOut#0.{mode}.{component}")).collect();
            assert!(transform_commands::validate_component_gesture(&host.host_snapshot, &pinned, &selected).is_ok());
        }
    });
    snapshot.retire_cold();
}

/// 🎛️ Lands a component gesture's channels on the operator `ensure_component_node` inserted with its default params, the way
/// the gesture's appended `change-widget-input` leaves do (design §19.4).
fn land_component_inputs(host: &mut semio_framework_os_flow::FlowHost, id: &str, mode: &str, components: &[u32], operation: &str) {
    for (channel, value) in transform_commands::component_gesture_inputs(mode, components, operation) {
        let patch = semio_framework_value::DslValue::object([(channel.to_string(), value.literal())]);
        host.set_neuron_params(id, &semio_framework_pack_json::to_json_string(&patch)).expect("the inserted record declares the channel");
    }
}

#[test]
fn mesh_component_gumball_reuses_only_the_same_selection_and_operation() {
    let _serial = test_serial();
    let snapshot = semio_s_artifact_procedural_3d_model::example_snapshot(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
    semio_s_artifact_procedural_3d_model::with_host(&snapshot.host_snapshot, |host| {
        for operation in ["translate", "rotate", "scale"] {
            let ids = ["extrude@meshOut#0.face.0".into(), "extrude@meshOut#0.face.0".into()];
            let (id, mode, components) = transform_commands::ensure_component_node(host, &ids, operation).unwrap();
            assert_eq!(mode, "face");
            assert_eq!(components, vec![0]);
            land_component_inputs(host, &id, &mode, &components, operation);
            let selected = [format!("{id}@meshOut#0.face.0")];
            assert_eq!(transform_commands::ensure_component_node(host, &selected, operation).unwrap().0, id);
            assert!(transform_commands::validate_component_gesture(&host.host_snapshot, &["extrude@meshOut#0.face.0".into()], &selected).is_ok());
            assert!(transform_commands::validate_component_gesture(&host.host_snapshot, &["other@meshOut#0.face.0".into()], &selected).is_err());
            assert!(host.host_snapshot.synapses.iter().any(|wire| wire.to == id && wire.to_port == "mesh" && wire.from_port == "meshOut"));
            let changed = [format!("{id}@meshOut#0.face.1")];
            assert!(transform_commands::validate_component_gesture(&host.host_snapshot, &["extrude@meshOut#0.face.0".into()], &changed).is_err());
            let (next, next_mode, next_components) = transform_commands::ensure_component_node(host, &changed, operation).unwrap();
            land_component_inputs(host, &next, &next_mode, &next_components, operation);
            assert_ne!(next, id);
            assert!(host.host_snapshot.synapses.iter().any(|wire| wire.to == next && wire.from == id));
            assert!(transform_commands::ensure_component_node(host, &["extrude@meshOut#1.face.0".into()], operation).is_err());
        }
    });
    snapshot.retire_cold();
}

/// ⚖️ LAW (design §19.4, §20.9): a component gumball's first grab inserts its operator with the kind's DEFAULT params and
/// sets every channel the gesture chooses — granularity, component set, pivot, the transform identity — as its own appended
/// `change-widget-input`, each individually editable in history; the folded rows hold the gesture's channels, and a re-grab
/// of that operator appends no row.
#[test]
fn a_first_component_grab_inserts_defaults_then_one_input_per_chosen_channel() {
    use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputValue;
    use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;

    let _serial = test_serial();
    let committed = semio_s_artifact_procedural_3d_model::example_snapshot(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
    let (rows, targets, _) = transform_commands::gumball_splice(&committed.host_snapshot, &["extrude@meshOut#0.face.1".into()], "translate").expect("the first grab splices");
    let operator = targets[0].clone();
    let literal = |widget: &semio_framework_artifact_flow_flow::Widget, key: &str| match widget {
        semio_framework_artifact_flow_flow::Widget::Neuron { params, .. } => params.get(key).and_then(|value| WidgetInputValue::of_literal(&semio_framework_value::ToValue::to_value(value))),
        _ => None,
    };
    let inserted = rows.iter().find_map(|row| match row { Generation3dMutation::CreateWidget(create) if semio_s_artifact_procedural_3d_model::widget_id(&create.widget) == operator => Some(create.widget.clone()), _ => None }).expect("the operator is inserted");
    assert_eq!(literal(&inserted, "mode"), Some(WidgetInputValue::Text("vertex".into())), "inserted with the kind's default, never the gesture's channel");
    let channels: Vec<(String, WidgetInputValue)> = rows.iter().filter_map(|row| match row { Generation3dMutation::ChangeWidgetInput(input) if input.id == operator => Some((input.channel.clone(), input.input.clone())), _ => None }).collect();
    assert_eq!(channels, vec![("mode".to_string(), WidgetInputValue::Text("face".into())), ("selection".to_string(), WidgetInputValue::Text("[1]".into())), ("offset".to_string(), WidgetInputValue::Vector([0.0; 3]))], "one input per chosen channel, after the insert");
    let mut landed = committed.clone();
    for row in rows {
        apply_generation3d_mutation(&mut landed, &row).expect("the splice folds");
        row.retire_cold();
    }
    let record = landed.host_snapshot.widgets.iter().find(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget) == operator).expect("the operator landed").clone();
    assert_eq!((literal(&record, "mode"), literal(&record, "selection")), (Some(WidgetInputValue::Text("face".into())), Some(WidgetInputValue::Text("[1]".into()))));
    let (again, _, _) = transform_commands::gumball_splice(&landed.host_snapshot, &[format!("{operator}@meshOut#0.face.1")], "translate").expect("a re-grab reuses the operator");
    assert!(again.is_empty(), "a re-grab appends no row: {again:?}");
    record.retire_cold();
    for snapshot in [landed, committed] {
        snapshot.retire_cold();
    }
}

#[test]
fn mesh_component_gumball_projects_a_topology_pivot_and_live_dispatch() {
    let _serial = test_serial();
    let snapshot = semio_s_artifact_procedural_3d_model::example_snapshot(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
    let evaluation = semio_s_artifact_procedural_3d_model::with_host(&snapshot.host_snapshot, |host| host.evaluate().unwrap());
    let marks = PreviewInteractionMarks {
        components: selection::ComponentSelection { granularity: "face".into(), selected: vec!["extrude@meshOut#0.face.0".into()], hovered: None },
        ..Default::default()
    };
    let cfg = Generation3dConfig::default();
    let payload = preview_payload(&evaluation, &snapshot.host_snapshot, &cfg, None, &marks);
    assert!(payload.component_pivot.is_some());
    let value: Value = serde_json::from_str(&preview_selection_json(&cfg, "translate", &payload)).unwrap();
    assert_eq!(value["gumballActive"], true);
    assert_eq!(value["gumballLiveDispatch"], true);
    assert_eq!(value["gumballTarget"], serde_json::json!(payload.component_pivot.unwrap()));
    assert_eq!(value["componentIds"], serde_json::json!([0]));
    assert_eq!(value["gumballSelectionIds"], serde_json::json!(["extrude@meshOut#0.face.0"]));
    let value: Value = serde_json::from_str(&preview_selection_json(&cfg, "", &payload)).unwrap();
    assert_eq!(value["gumballActive"], false);
    let invalid = PreviewInteractionMarks { components: selection::ComponentSelection { selected: vec!["extrude@meshOut#0.face.9999".into()], ..marks.components.clone() }, ..Default::default() };
    assert!(preview_payload(&evaluation, &snapshot.host_snapshot, &cfg, None, &invalid).component_pivot.is_none());
    snapshot.retire_cold();
}



#[semio_framework_async_macros::async_test]
async fn mesh_component_edit_undo_redo_restores_geometry_and_analysis_connections() {
    let _serial = test_serial();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let fixtures: Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥽️edit-mesh-selection/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixtures["valid"].as_array().unwrap() {
        let operation = case["name"].as_str().unwrap();
        let granularity = case["granularity"].as_str().unwrap();
        let mut app = app().await;
        let snapshot = mesh_selection_fixture_snapshot(operation);
        let payload = semio_framework_pack_json::to_json_string(&snapshot);
        snapshot.retire_cold();
        context::dispatch(&mut app, Generation3dCommand::ImportDocument(import_document::ImportDocument { name: "mesh-selection.json".into(), payload, widget_id: None, channel: None, texture_id: None })).await;
        context::drain_flow_eval_ticks(&mut app).await;
        let selected: Vec<String> = serde_json::from_value(case["ids"].clone()).unwrap();
        context::select_domain(&mut app, selection::DOMAIN, granularity, &selected.iter().map(String::as_str).collect::<Vec<_>>()).await;
        let probe = |app: &context::Generation3dApp| {
            let snapshot = context::snapshot(app);
            (snapshot.host_snapshot.widgets.len(), snapshot.host_snapshot.synapses.iter().find(|wire| wire.to == "analysis" && wire.to_port == "mesh").unwrap().from.clone())
        };
        let before = probe(&app);
        let before_preview = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
        let before_world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&before_preview).unwrap();
        let before_meshes: Value = serde_json::from_str(&before_world.meshes_json).unwrap();
        let after = (before.0 + 1, format!("extrude__{operation}"));
        let change = context::act_with_view(&mut app, "editMeshSelection", case["payload"].clone(), context::preview_views("procedural-preview-test", "procedural-preview-test-other").0).await.unwrap();
        assert_eq!(probe(&app), after);
        let row = semio_framework_plugin::PluginApp::history_snapshot(&mut *app).await.expect("history").upserts.into_iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).max_by_key(|entry| entry.seq).expect("the mesh edit's row");
        assert_eq!(row.transaction.as_ref().map(|transaction| transaction.tool.as_str()), Some("s.procedural.generation3d@1/*#editor#editMeshSelection"), "{operation}: the edit is ONE tool transaction (audit P4)");
        let label = row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En);
        assert!(label.starts_with("Insert \"") && label.contains(&format!("({})", after.1)), "{operation}: the row reads the operator it inserts, never its splice: {label}");
        let selected = app.interaction_state().await;
        assert!(selected.selection.get(selection::DOMAIN).unwrap().ids.is_empty(), "{operation}: invalidated mesh components are cleared");
        assert_eq!(selected.selection.get("graph").unwrap().ids, vec![after.1.clone()]);
        context::drive_preview_run(&mut app, &context::preview_views("procedural-preview-test", "procedural-preview-test-other").0, &change.effects).await;
        let after_preview = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
        let after_world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&after_preview).unwrap();
        let after_meshes: Value = serde_json::from_str(&after_world.meshes_json).unwrap();
        assert!(!after_meshes.as_array().unwrap().is_empty(), "{operation}: the edit renders geometry before history traversal");
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "undo", semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
        assert_eq!(probe(&app), before);
        context::drain_flow_eval_ticks(&mut app).await;
        let undo_preview = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
        let undo_world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&undo_preview).unwrap();
        assert_eq!(serde_json::from_str::<Value>(&undo_world.meshes_json).unwrap(), before_meshes, "{operation}: undo restores actual viewport geometry");
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "redo", semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
        assert_eq!(probe(&app), after);
        context::drain_flow_eval_ticks(&mut app).await;
        let graph = context::render(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN).await;
        let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&graph).unwrap();
        let status = scene.status_json.unwrap();
        assert_eq!(node_eval_status(&status, &after.1), "ok", "{operation}: {status}");
        assert_eq!(node_eval_status(&status, "analysis"), "ok", "{operation}: {status}");
        let preview = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
        let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&preview).unwrap();
        let meshes: Value = serde_json::from_str(&world.meshes_json).unwrap();
        assert!(!meshes.as_array().unwrap().is_empty(), "{operation}: the new mesh reaches the runtime viewport");
        assert_eq!(meshes, after_meshes, "{operation}: redo restores actual viewport geometry");
        let expected_area = serde_json::from_str::<Value>(scene.eval_json.as_ref().unwrap()).unwrap()["analysis"]["out"]["area"]["value"].as_f64().expect("evaluated analysis area").to_string();
        let committed = context::snapshot(&app);
        context::select_graph(&mut app, "node", &["analysis"]).await;
        for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
            let view = semio_framework_plugin::ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native);
            let rendered = context::render_with_view(&mut app, semio_s_artifact_procedural_3d_editor_panels::inspection::GENERATION_3D_PLAY_BODY_INSPECTION, &view).await;
            fn output<'a>(node: &'a Value) -> Option<&'a Value> { if node["key"] == "procedural-play-inspector.output.area" { return Some(node); } node["children"].as_array()?.iter().find_map(output) }
            let tree: Value = serde_json::from_str(&rendered).unwrap(); let row = output(&tree).unwrap_or_else(|| panic!("{operation}: missing selected analysis: {rendered}"));
            assert!(row.to_string().contains(&expected_area), "{operation}: selected analysis area reflects existing evaluation"); assert!(!row.to_string().contains("setWidgetInput"));
            assert_eq!(context::snapshot(&app), committed, "selected output inspection preserves the document");
        }
        println!("[DEBUG] retained mesh edit {operation}: geometry and selected analysis evaluated, selection cleared, undo/redo restored graph");
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_quick_actions_execute_through_the_registered_preview_app() {
    let _serial = test_serial();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let fixtures: Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥽️edit-mesh-selection/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixtures["valid"].as_array().unwrap() {
        let operation = case["name"].as_str().unwrap();
        let granularity = case["granularity"].as_str().unwrap();
        let mut app = app().await;
        let snapshot = mesh_selection_fixture_snapshot(operation);
        let payload = semio_framework_pack_json::to_json_string(&snapshot);
        snapshot.retire_cold();
        context::dispatch(&mut app, Generation3dCommand::ImportDocument(import_document::ImportDocument { name: "mesh-quick-action.json".into(), payload, widget_id: None, channel: None, texture_id: None })).await;
        context::drain_flow_eval_ticks(&mut app).await;
        let args: semio_framework_value::DslValue = serde_json::json!({"domainId":selection::DOMAIN,"granularityId":granularity}).into();
        let admitted = app.handle_action("setInteractionGranularity", Some(&args), &semio_framework_plugin::artifact_app_laws::meta("local")).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app, admitted).await.unwrap();
        let ids: Vec<String> = serde_json::from_value(case["ids"].clone()).unwrap();
        let before = context::snapshot(&app).host_snapshot.widgets.len();
        for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
            context::select_domain(&mut app, selection::DOMAIN, granularity, &ids.iter().map(String::as_str).collect::<Vec<_>>()).await;
            let mut view = context::preview_views("procedural-preview-test", "procedural-preview-test-other").0;
            view.locale = locale;
            let engagements = app.window_engagements(&view).await;
            let action = engagements.get(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW).unwrap().possible_engagements.as_ref().unwrap().iter().find(|action| action.id == format!("procedural.mesh-{operation}")).unwrap();
            assert!(!action.label.is_empty());
            let descriptor = action.action.as_ref().unwrap();
            assert_eq!(descriptor.action, "editMeshSelection");
            let args = descriptor.args.as_ref().unwrap();
            for key in case["payload"].as_object().unwrap().keys() { assert!(args.get(key).is_some(), "{operation}: quick action declares {key}"); }
            let meta = semio_framework_plugin::ActionMeta { view_state: Some(view), ..semio_framework_plugin::artifact_app_laws::meta("local") };
            app.handle_action(&descriptor.action, descriptor.args.as_ref(), &meta).await.unwrap();
            let change = context::settle(&mut app).await;
            let snapshot = context::snapshot(&app);
            let id = format!("extrude__{operation}");
            assert_eq!(snapshot.host_snapshot.widgets.len(), before + 1);
            assert!(snapshot.host_snapshot.synapses.iter().any(|wire| wire.from == id && wire.to == "analysis" && wire.to_port == "mesh"));
            drop(snapshot);
            assert!(app.interaction_state().await.selection.get(selection::DOMAIN).unwrap().ids.is_empty());
            context::drive_preview_run(&mut app, &context::preview_views("procedural-preview-test", "procedural-preview-test-other").0, &change.effects).await;
            let preview = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
            let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&preview).unwrap();
            assert!(!serde_json::from_str::<Value>(&world.meshes_json).unwrap().as_array().unwrap().is_empty());
            semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "undo", semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
            assert_eq!(context::snapshot(&app).host_snapshot.widgets.len(), before);
            context::drain_flow_eval_ticks(&mut app).await;
            println!("[DEBUG] retained mesh quick action {operation}/{locale:?}: emitted args executed, viewport rendered, selection cleared, undo restored source");
        }
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_component_commands_reject_stale_topology_before_publication() {
    let _serial = test_serial();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let mut app = app().await;
    context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH.into() })).await;
    context::drain_flow_eval_ticks(&mut app).await;
    context::select_domain(&mut app, selection::DOMAIN, "face", &["extrude@meshOut#0.face.999999"]).await;
    let before = semio_framework_pack_json::to_json_string(&context::snapshot(&app).host_snapshot);
    let (view, _) = context::preview_views("procedural-preview-test", "procedural-preview-test-other");
    let (flow_view, _) = context::shell_views("procedural-main-test", "procedural-preview-test");
    for command in [
        Generation3dCommand::EditMeshSelection(edit_mesh_selection::EditMeshSelection { cuts: 1, operation: "inset".into(), amount: 0.1, dx: 0.0, dy: 0.0, dz: 0.0, ..Default::default() }),
        Generation3dCommand::KnifeMeshSelection(knife_mesh_selection::KnifeMeshSelection { start: [0.0, -1.0, 0.0], end: [0.0, 1.0, 0.0] }),
    ] {
        let result = context::dispatch_with_view(&mut app, command, view.clone()).await;
        assert!(result.is_err(), "stale topology must reject the command");
        assert_eq!(semio_framework_pack_json::to_json_string(&context::snapshot(&app).host_snapshot), before);
    }
    let command = Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec!["extrude@meshOut#0.vertex.999999".into()], dx: 1.0, dy: 0.0, dz: 0.0, phase: None, reason: None, window_id: None });
    assert!(context::dispatch_with_view(&mut app, command, flow_view).await.is_err());
    assert_eq!(semio_framework_pack_json::to_json_string(&context::snapshot(&app).host_snapshot), before);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

#[semio_framework_async_macros::async_test]
async fn mesh_component_gumball_rejects_changed_selection_before_and_during_drag() {
    let _serial = test_serial();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    for operation in ["translate", "rotate", "scale"] {
        let mut app = app().await;
        context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH.into() })).await;
        context::drain_flow_eval_ticks(&mut app).await;
        let command = || match operation {
            "translate" => Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec!["extrude@meshOut#0.face.0".into()], dx: 0.1, dy: 0.0, dz: 0.0, phase: None, reason: None, window_id: None }),
            "rotate" => Generation3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude@meshOut#0.face.0".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 0.1, phase: None, reason: None, window_id: None }),
            _ => Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude@meshOut#0.face.0".into()], sx: 1.1, sy: 1.0, sz: 1.0, phase: None, reason: None, window_id: None }),
        };
        let view = context::preview_views("procedural-preview-test", "procedural-preview-test-other").0;
        let args: semio_framework_value::DslValue = serde_json::json!({"domainId":selection::DOMAIN,"granularityId":"face"}).into();
        let admitted = app.handle_action("setInteractionGranularity", Some(&args), &semio_framework_plugin::artifact_app_laws::meta("local")).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app, admitted).await.unwrap();
        context::select_domain(&mut app, selection::DOMAIN, "face", &["extrude@meshOut#0.face.1"]).await;
        let before = semio_framework_pack_json::to_json_string(&context::snapshot(&app).host_snapshot);
        assert!(context::dispatch_with_view(&mut app, command(), view.clone()).await.is_err());
        assert_eq!(semio_framework_pack_json::to_json_string(&context::snapshot(&app).host_snapshot), before);
        context::select_domain(&mut app, selection::DOMAIN, "face", &["extrude@meshOut#0.face.0"]).await;
        context::dispatch_with_view(&mut app, command(), view.clone()).await.unwrap();
        let id = format!("extrude__{operation}Components");
        context::select_domain(&mut app, selection::DOMAIN, "face", &[&format!("{id}@meshOut#0.face.1")]).await;
        let before = semio_framework_pack_json::to_json_string(&context::snapshot(&app).host_snapshot);
        assert!(context::dispatch_with_view(&mut app, command(), view).await.is_err());
        assert_eq!(semio_framework_pack_json::to_json_string(&context::snapshot(&app).host_snapshot), before);
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_component_gumball_retains_selection_coalesces_drags_and_round_trips_history() {
    use semio_framework_plugin::artifact_app_laws::meta;
    let _serial = test_serial();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    for operation in ["translate", "rotate", "scale"] {
        let mut app = app().await;
        context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH.into() })).await;
        context::drain_flow_eval_ticks(&mut app).await;
        let args: semio_framework_value::DslValue = serde_json::json!({"domainId":selection::DOMAIN,"granularityId":"face"}).into();
        let admitted = app.handle_action("setInteractionGranularity", Some(&args), &meta("local")).await.unwrap();
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app, admitted).await.unwrap();
        context::select_domain(&mut app, selection::DOMAIN, "face", &["extrude@meshOut#0.face.0"]).await;
        let probe = |app: &context::Generation3dApp| {
            let snapshot = context::snapshot(app);
            (snapshot.host_snapshot.widgets.len(), snapshot.host_snapshot.synapses.iter().find(|wire| wire.to == "analysis" && wire.to_port == "mesh").unwrap().from.clone())
        };
        let before = probe(&app);
        let id = format!("extrude__{operation}Components");
        let command = || match operation {
            "translate" => Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec!["extrude@meshOut#0.face.0".into()], dx: 0.1, dy: 0.0, dz: 0.0, phase: None, reason: None, window_id: None }),
            "rotate" => Generation3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude@meshOut#0.face.0".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 0.1, phase: None, reason: None, window_id: None }),
            _ => Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude@meshOut#0.face.0".into()], sx: 1.1, sy: 1.0, sz: 1.0, phase: None, reason: None, window_id: None }),
        };
        let view = context::preview_views("procedural-preview-test", "procedural-preview-test-other").0;
        let mut preview_effects = Vec::new();
        for _ in 0..2 {
            let change = context::dispatch_with_view(&mut app, command(), view.clone()).await.unwrap();
            preview_effects.extend(change.effects);
            assert_eq!(probe(&app), (before.0 + 1, id.clone()));
            assert_eq!(app.interaction_state().await.selection.get(selection::DOMAIN).unwrap().ids, vec![format!("{id}@meshOut#0.face.0")]);
        }
        let snapshot = context::snapshot(&app);
        semio_s_artifact_procedural_3d_model::with_host(&snapshot.host_snapshot, |host| {
            let widget = semio_s_artifact_procedural_3d_model::gumball_widget_json(host, &id).unwrap();
            let params = widget.get("params").unwrap();
            let actual = match operation {
                "translate" => params.get("offset").unwrap().get("x").unwrap().as_f64().unwrap(),
                "rotate" => params.get("angle").unwrap().get("value").unwrap().as_f64().unwrap(),
                _ => params.get("factor").unwrap().get("x").unwrap().as_f64().unwrap(),
            };
            assert!((actual - if operation == "scale" { 1.21 } else { 0.2 }).abs() < 1e-9);
        });
        context::drive_preview_run(&mut app, &view, &preview_effects).await;
        let graph = context::render(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN).await;
        let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&graph).unwrap();
        let evaluation: Value = serde_json::from_str(scene.eval_json.as_ref().unwrap()).unwrap();
        assert_eq!(evaluation[&id]["out"]["meshOut"]["$schema"], "mesh", "{evaluation}");
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "undo", meta("local").instance_id).await;
        assert_eq!(probe(&app), before);
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "redo", meta("local").instance_id).await;
        assert_eq!(probe(&app), (before.0 + 1, id));
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    }
}
