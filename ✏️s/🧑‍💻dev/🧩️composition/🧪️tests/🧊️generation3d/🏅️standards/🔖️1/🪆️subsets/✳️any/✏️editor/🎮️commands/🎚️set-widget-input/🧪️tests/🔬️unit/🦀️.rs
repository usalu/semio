use super::*;
use semio_framework_value::ToValue;

#[semio_framework_async_macros::async_test]
async fn widget_input_graph_selection_projects_the_same_retained_inspector() {
    use crate::editor_domain::editor_laws::context;
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, settle_history_verb};
    use semio_framework_ui_locale::{Locale, Terminology};
    use semio_s_artifact_procedural_generation3d::editor::generation3d::{Generation3dCommand, panels::inspection::GENERATION_3D_PLAY_BODY_INSPECTION};
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["selectionPublication"];
    let mut app = context::app().await;
    let mut snapshot = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
    let projected = with_host(&snapshot.host_snapshot, |host| {
        let id = host.add_widget(&serde_json::json!({"kind":"inputSlider","id":case["widgetId"],"label":""}).to_string(), 0.0, 0.0).unwrap();
        host.set_slider_value(&id, case["value"].as_f64().unwrap());
        host.host_snapshot.clone()
    });
    std::mem::replace(&mut snapshot.host_snapshot, projected).retire_cold();
    let payload = semio_framework_pack_json::to_json_string(&snapshot);
    snapshot.retire_cold();
    context::dispatch(&mut app, Generation3dCommand::ImportDocument(semio_s_artifact_procedural_generation3d::editor::generation3d::commands::import_document::ImportDocument { name: "selected-input.json".into(), payload, widget_id: None, channel: None, texture_id: None })).await;
    let before = context::snapshot(&app);
    let mut binding = serde_json::Value::Null;
    for action in case["actions"].as_array().unwrap() {
        context::act_with_view(&mut app, action.as_str().unwrap(), serde_json::json!({}), context::preview_views("selection", "second").0).await.unwrap();
        for locale in [Locale::En, Locale::De] {
            let view = semio_framework_plugin::ViewModel::new(locale, Terminology::Native);
            let rendered = context::render_with_view(&mut app, GENERATION_3D_PLAY_BODY_INSPECTION, &view).await;
            let tree: serde_json::Value = serde_json::from_str(&rendered).unwrap();
            fn control<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
                if node["key"] == key { return Some(node); }
                node["children"].as_array()?.iter().find_map(|child| control(child, key))
            }
            let input = control(&tree, case["controlKey"].as_str().unwrap()).unwrap_or_else(|| panic!("{rendered}"));
            assert_eq!(input["component"]["value"], case["value"].as_i64().unwrap().to_string());
            assert_eq!(input["component"]["commit"], case["control"]["commit"]);
            assert_eq!(input["accessibility"]["label"], case["control"][if locale == Locale::De { "de" } else { "en" }]);
            assert_eq!(input["bindings"][0]["trigger"], case["control"]["trigger"]);
            assert_eq!(input["bindings"][0]["args"]["widgetIds"], serde_json::json!([case["widgetId"]]));
            binding = input["bindings"][0].clone();
            assert_eq!(context::snapshot(&app), before);
        }
        println!("[DEBUG] {} retained graph selection projected the selected input into both inspector locales", action);
    }
    let mut args = binding["args"].clone(); args["value"] = case["committedValue"].clone();
    context::act_with_view(&mut app, binding["action"]["name"].as_str().unwrap(), args, context::preview_views("selection", "second").0).await.unwrap();
    let after = context::snapshot(&app);
    let value = after.host_snapshot.widgets.iter().find_map(|widget| if let Widget::InputSlider { id, value, .. } = widget { (id == case["widgetId"].as_str().unwrap()).then_some(*value) } else { None }).unwrap();
    assert_eq!(value, case["committedValue"].as_f64().unwrap());
    assert_eq!(after.host_snapshot.synapses, before.host_snapshot.synapses);
    settle_history_verb(&mut *app, "undo", meta("local").instance_id).await; assert_eq!(context::snapshot(&app), before);
    settle_history_verb(&mut *app, "redo", meta("local").instance_id).await; assert_eq!(context::snapshot(&app), after);
    println!("[DEBUG] selected slider emitted commit binding published its absolute mutation and exact undo/redo");
    close_registered_fixture_app(&mut *app);
}

#[semio_framework_async_macros::async_test]
async fn widget_input_mesh_source_actions_publish_evaluate_and_restore_history() {
    use crate::editor_domain::editor_laws::context;
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, decode_fixture_scene_with_lanes, meta, settle_history_verb};
    use semio_s_artifact_procedural_generation3d::editor::generation3d::{Generation3dCommand, modes::edit::windows::flow::GENERATION_3D_PLAY_BODY_MAIN};
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧫️fixtures/🔣️.json")).unwrap();
    for group in ["meshSource", "meshAssets"] {
    for case in fixture[group]["cases"].as_array().unwrap().iter().filter(|case| case["error"] != true) {
        let mut app = context::app().await;
        let mut snapshot = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
        let projected = with_host(&snapshot.host_snapshot, |host| {
            let source = host.add_widget(r#"{"kind":"inputNote","id":"source"}"#, 0.0, 0.0).unwrap();
            host.set_note_text(&source, &fixture[group]["mesh"].to_string());
            let construct = host.add_widget(r#"{"kind":"neuron","id":"construct","neuronKind":"brep.mesh.construct"}"#, 200.0, 0.0).unwrap();
            let preview = host.add_widget(r#"{"kind":"outputPreview","id":"preview"}"#, 400.0, 0.0).unwrap();
            host.connect_ports(&source, "text", &construct, "data").unwrap();
            host.connect_ports(&construct, "meshOut", &preview, "").unwrap();
            host.host_snapshot.clone()
        });
        std::mem::replace(&mut snapshot.host_snapshot, projected).retire_cold();
        let payload = semio_framework_pack_json::to_json_string(&snapshot); snapshot.retire_cold();
        context::dispatch(&mut app, Generation3dCommand::ImportDocument(semio_s_artifact_procedural_generation3d::editor::generation3d::commands::import_document::ImportDocument { name: "mesh-source.json".into(), payload, widget_id: None, channel: None, texture_id: None })).await;
        let before = context::snapshot(&app);
        let view = context::preview_views("mesh-source", "second").0;
        let mut args = case["command"].clone(); args["widgetId"] = "construct".into(); args["channel"] = "data".into(); args["facet"] = "meshSource".into();
        let change = context::act_with_view(&mut app, "setWidgetInput", args, view.clone()).await.unwrap();
        let after = context::snapshot(&app);
        let text = after.host_snapshot.widgets.iter().find_map(|widget| if let Widget::InputNote { id, text } = widget { (id == "source").then_some(text) } else { None }).unwrap();
        let mut expected = case.get("expectedMesh").cloned().unwrap_or_else(|| fixture[group]["mesh"].clone());
        if let Some(edit) = case.get("expected") { let mut target = &mut expected; for part in edit["path"].as_array().unwrap() { let key = part.as_str().unwrap(); target = if target.is_array() { &mut target[key.parse::<usize>().unwrap()] } else { &mut target[key] }; } *target = edit["value"].clone(); }
        assert_eq!(serde_json::from_str::<serde_json::Value>(text).unwrap(), expected, "{}", case["id"]);
        assert_eq!(after.host_snapshot.synapses, before.host_snapshot.synapses);
        let receipt = context::drive_preview_run(&mut app, &view, &change.effects).await;
        eprintln!("[DEBUG] input preview hops={} answered={} state={:?}", receipt.hops, receipt.answered, receipt.state);
        assert!(receipt.answered > 0, "contributed geometry must cross the actual extension transport");
        let graph = context::render_with_view(&mut app, GENERATION_3D_PLAY_BODY_MAIN, &view).await;
        let scene = decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&graph).unwrap();
        assert_eq!(crate::editor_domain::editor_laws::node_eval_status(scene.status_json.as_ref().unwrap(), "construct"), "ok", "{}", case["id"]);
        settle_history_verb(&mut *app, "undo", meta("local").instance_id).await; assert_eq!(context::snapshot(&app), before);
        settle_history_verb(&mut *app, "redo", meta("local").instance_id).await; assert_eq!(context::snapshot(&app), after);
        println!("[DEBUG] structured mesh {} used its retained source note, canonical geometry inference and exact undo/redo", case["id"]);
        close_registered_fixture_app(&mut *app);
    }
    }
}





#[semio_framework_async_macros::async_test]
async fn widget_input_publication_supports_scalar_events_and_undo_redo() {
    use crate::editor_domain::editor_laws::context;
    use semio_s_artifact_procedural_generation3d::editor::generation3d::{Generation3dCommand, Generation3dPlayApp};
    use semio_framework_plugin::ArtifactEditor;
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let mut app = context::app().await;
    context::dispatch(&mut app, Generation3dCommand::AddWidget(semio_s_artifact_procedural_generation3d::editor::generation3d::commands::add_widget::AddWidget { kind: "neuron".into(), neuron_kind: Some("brep.mesh.box".into()), format: None, action: None, x: None, y: None })).await;
    let before = context::snapshot(&app);
    let id = semio_s_artifact_procedural_generation3d::widget_id(before.host_snapshot.widgets.last().unwrap()).to_string();
    let args: semio_framework_value::DslValue = serde_json::json!({"widgetId":id,"channel":"width","value":2.5}).into();
    let command = Generation3dPlayApp::command_from_action("setWidgetInput", Some(&args)).unwrap();
    context::dispatch(&mut app, command).await;
    let after = context::snapshot(&app);
    let params = after.host_snapshot.widgets.iter().find_map(|widget| match widget { Widget::Neuron { id: candidate, params, .. } if candidate == &id => Some(params.to_value()), _ => None }).unwrap();
    assert_eq!(params.get("width").and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_f64), Some(2.5));
    eprintln!("[DEBUG] inspector input {id}.width published 2.5");
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "undo", 1).await;
    assert_eq!(context::snapshot(&app), before);
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "redo", 1).await;
    assert_eq!(context::snapshot(&app), after);
    eprintln!("[DEBUG] inspector input {id}.width undo/redo restored typed parameters");
}

#[semio_framework_async_macros::async_test]
async fn widget_input_collection_actions_publish_evaluate_and_restore_history() {
    use crate::editor_domain::editor_laws::context;
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, decode_fixture_scene_with_lanes, meta, settle_history_verb};
    use semio_s_artifact_procedural_generation3d::editor::generation3d::{Generation3dCommand, modes::edit::windows::flow::GENERATION_3D_PLAY_BODY_MAIN};
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["runtimeCollections"].as_array().unwrap() {
        let mut app = context::app().await;
        let mut snapshot = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
        let projected = with_host(&snapshot.host_snapshot, |host| {
            let source = host.add_widget(r#"{"kind":"neuron","id":"polyline","neuronKind":"brep.curve.polyline"}"#, 0.0, 0.0).unwrap();
            host.set_neuron_params(&source, &serde_json::json!({"points":fixture["runtimeInput"]}).to_string()).unwrap();
            let preview = host.add_widget(r#"{"kind":"outputPreview","id":"preview"}"#, 200.0, 0.0).unwrap();
            host.connect_ports(&source, "wire", &preview, "").unwrap();
            host.host_snapshot.clone()
        });
        std::mem::replace(&mut snapshot.host_snapshot, projected).retire_cold();
        let payload = semio_framework_pack_json::to_json_string(&snapshot);
        snapshot.retire_cold();
        context::dispatch(&mut app, Generation3dCommand::ImportDocument(semio_s_artifact_procedural_generation3d::editor::generation3d::commands::import_document::ImportDocument { name: "collection.json".into(), payload, widget_id: None, channel: None, texture_id: None })).await;
        let before = context::snapshot(&app);
        let view = context::preview_views("collection", "second").0;
        let mut args = case["command"].clone();
        args["widgetId"] = "polyline".into();
        args["channel"] = "points".into();
        let change = context::act_with_view(&mut app, "setWidgetInput", args, view.clone()).await.unwrap();
        let after = context::snapshot(&app);
        let params = after.host_snapshot.widgets.iter().find_map(|widget| match widget { Widget::Neuron { id, params, .. } if id == "polyline" => Some(params.to_value()), _ => None }).unwrap();
        let points = params.get("points").unwrap();
        let expected = case["expected"].as_array().unwrap();
        assert_eq!(points.as_object().unwrap().len(), expected.len() + 1);
        for (index, point) in expected.iter().enumerate() {
            let actual = points.get(&index.to_string()).unwrap();
            for (axis, name) in ["x", "y", "z"].iter().enumerate() { assert_eq!(actual.get(name).unwrap().as_f64(), point[axis].as_f64()); }
        }
        let receipt = context::drive_preview_run(&mut app, &view, &change.effects).await;
        eprintln!("[DEBUG] input preview hops={} answered={} state={:?}", receipt.hops, receipt.answered, receipt.state);
        assert!(receipt.answered > 0, "contributed geometry must cross the actual extension transport");
        let graph = context::render_with_view(&mut app, GENERATION_3D_PLAY_BODY_MAIN, &view).await;
        let scene = decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&graph).unwrap();
        let statuses: serde_json::Value = serde_json::from_str(scene.status_json.as_ref().unwrap()).unwrap();
        assert_eq!(statuses["polyline"]["status"], "ok", "{}: {statuses}", case["id"]);
        settle_history_verb(&mut *app, "undo", meta("local").instance_id).await;
        assert_eq!(context::snapshot(&app), before);
        settle_history_verb(&mut *app, "redo", meta("local").instance_id).await;
        assert_eq!(context::snapshot(&app), after);
        context::drain_flow_eval_ticks_with_view(&mut app, &view).await;
        println!("[DEBUG] retained point-list {} action evaluated through the registered geometry gateway and undo/redo restored its exact document", case["id"]);
        close_registered_fixture_app(&mut *app);
    }
}

#[semio_framework_async_macros::async_test]
async fn widget_input_metadata_actions_preserve_live_connections_and_history() {
    use crate::editor_domain::editor_laws::context;
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, decode_fixture_scene_with_lanes, meta, settle_history_verb};
    use semio_s_artifact_procedural_generation3d::editor::generation3d::{Generation3dCommand, modes::edit::windows::flow::GENERATION_3D_PLAY_BODY_MAIN};
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let mut app = context::app().await;
    let mut snapshot = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
    let projected = with_host(&snapshot.host_snapshot, |host| {
        let source = host.add_widget(r#"{"kind":"inputSlider","id":"source","label":""}"#, 0.0, 0.0).unwrap();
        host.set_slider_value(&source, 1.0);
        let variable = host.add_widget(r#"{"kind":"variable","id":"parameter"}"#, 200.0, 0.0).unwrap();
        host.set_variable_name(&variable, "height");
        host.set_variable_schema(&variable, "number");
        let preview = host.add_widget(r#"{"kind":"outputPreview","id":"preview"}"#, 400.0, 0.0).unwrap();
        host.connect_ports(&source, "number", &variable, "height").unwrap();
        host.connect_ports(&variable, "height", &preview, "").unwrap();
        host.host_snapshot.clone()
    });
    std::mem::replace(&mut snapshot.host_snapshot, projected).retire_cold();
    let payload = semio_framework_pack_json::to_json_string(&snapshot);
    snapshot.retire_cold();
    context::dispatch(&mut app, Generation3dCommand::ImportDocument(semio_s_artifact_procedural_generation3d::editor::generation3d::commands::import_document::ImportDocument { name: "variable.json".into(), payload, widget_id: None, channel: None, texture_id: None })).await;
    let before = context::snapshot(&app);
    let view = context::preview_views("metadata", "second").0;
    let change = context::act_with_view(&mut app, "setWidgetInput", serde_json::json!({"widgetId":"parameter","facet":"variableName","channel":"name","value":"  Width  "}), view.clone()).await.unwrap();
    let after = context::snapshot(&app);
    assert!(after.host_snapshot.synapses.iter().any(|wire| wire.to == "parameter" && wire.to_port == "Width"));
    assert!(after.host_snapshot.synapses.iter().any(|wire| wire.from == "parameter" && wire.from_port == "Width"));
    let receipt = context::drive_preview_run(&mut app, &view, &change.effects).await;
        eprintln!("[DEBUG] input preview hops={} answered={} state={:?}", receipt.hops, receipt.answered, receipt.state);
    let graph = context::render_with_view(&mut app, GENERATION_3D_PLAY_BODY_MAIN, &view).await;
    let scene = decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&graph).unwrap();
    assert_eq!(crate::editor_domain::editor_laws::node_eval_status(scene.status_json.as_ref().unwrap(), "parameter"), "ok", "status={:?} evaluation={:?}",scene.status_json,scene.eval_json);
    settle_history_verb(&mut *app, "undo", meta("local").instance_id).await;
    assert_eq!(context::snapshot(&app), before);
    settle_history_verb(&mut *app, "redo", meta("local").instance_id).await;
    assert_eq!(context::snapshot(&app), after);
    context::dispatch(&mut app, Generation3dCommand::AddWidget(semio_s_artifact_procedural_generation3d::editor::generation3d::commands::add_widget::AddWidget { kind:"outputExport".into(), neuron_kind:None, format:None, action:None, x:None, y:None })).await;
    let before_export = context::snapshot(&app);
    let id = before_export.host_snapshot.widgets.iter().find_map(|widget| match widget { Widget::OutputExport { id, format } => { assert_eq!(format,"gltf"); Some(id.clone()) }, _ => None }).unwrap();
    context::act_with_view(&mut app, "setWidgetInput", serde_json::json!({"widgetId":id,"facet":"exportFormat","channel":"format","value":"obj"}), view.clone()).await.unwrap();
    let after_export = context::snapshot(&app);
    assert!(after_export.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::OutputExport { id: candidate, format } if candidate == &id && format == "obj")));
    settle_history_verb(&mut *app, "undo", meta("local").instance_id).await;
    assert_eq!(context::snapshot(&app), before_export);
    settle_history_verb(&mut *app, "redo", meta("local").instance_id).await;
    assert_eq!(context::snapshot(&app), after_export);
    println!("[DEBUG] variable rename preserved evaluated input/output channels; default glTF export changed through its own facet; undo/redo restored both documents");
    close_registered_fixture_app(&mut *app);
}

#[semio_framework_async_macros::async_test]
async fn widget_input_texture_picker_retains_source_scope_and_restores_history() {
    use crate::editor_domain::editor_laws::context;
    use semio_framework_plugin::{Effect, artifact_app_laws::{close_registered_fixture_app, meta, settle_history_verb}};
    use semio_s_artifact_procedural_generation3d::editor::generation3d::{Generation3dCommand, commands::{import_document, import_document_request}};
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧫️fixtures/🔣️.json")).unwrap();
    let mut app = context::app().await;
    let mut snapshot = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
    let projected = with_host(&snapshot.host_snapshot, |host| {
        let source = host.add_widget(r#"{"kind":"inputNote","id":"source"}"#, 0.0, 0.0).unwrap();
        host.set_note_text(&source, &fixture["meshAssets"]["mesh"].to_string());
        let construct = host.add_widget(r#"{"kind":"neuron","id":"construct","neuronKind":"brep.mesh.construct"}"#, 200.0, 0.0).unwrap();
        host.connect_ports(&source, "text", &construct, "data").unwrap(); host.host_snapshot.clone()
    });
    std::mem::replace(&mut snapshot.host_snapshot, projected).retire_cold();
    let payload = semio_framework_pack_json::to_json_string(&snapshot); snapshot.retire_cold();
    context::dispatch(&mut app, Generation3dCommand::ImportDocument(import_document::ImportDocument { name: "texture-scope.json".into(), payload, widget_id: None, channel: None, texture_id: None })).await;
    let before = context::snapshot(&app);
    let request = import_document_request::ImportDocumentRequest { widget_id: Some("construct".into()), channel: Some("data".into()), texture_id: Some("image".into()) };
    let emitted = import_document_request::emit(&request, &before.host_snapshot).unwrap();
    let Effect::RequestFileOpen { args: Some(args), accept, read_as, import_action, multiple, .. } = &emitted.effects[0] else { panic!("Expected scoped picker") };
    assert_eq!(accept, ".png,.jpg,.jpeg"); assert_eq!(read_as.as_deref(), Some("dataUrl")); assert_eq!(import_action, "importDocument"); assert!(!multiple);
    assert_eq!(args.get("widgetId").and_then(semio_framework_value::DslValue::as_str), Some("source")); assert_eq!(args.get("channel").and_then(semio_framework_value::DslValue::as_str), Some("text"));
    context::dispatch(&mut app, Generation3dCommand::ImportDocumentRequest(request)).await;
    assert_eq!(context::snapshot(&app), before);
    for key in ["textureImport", "textureJpegImport"] {
    let original = context::snapshot(&app); let case = &fixture[key];
    let mut picked = case["target"].clone(); picked["name"] = case["name"].clone(); picked["payload"] = case["payload"].clone();
    context::act_with_view(&mut app, "importDocument", picked, context::preview_views("texture-scope", "second").0).await.unwrap();
    let after = context::snapshot(&app);
    let text = after.host_snapshot.widgets.iter().find_map(|widget| if let Widget::InputNote { id, text } = widget { (id == "source").then_some(text) } else { None }).unwrap();
    let source: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(source["textures"]["image"], serde_json::json!({"mime":case["mime"],"bytes":case["bytes"]}));
    assert_eq!(source["materials"], fixture["meshAssets"]["mesh"]["materials"]); assert_eq!(after.host_snapshot.synapses, before.host_snapshot.synapses);
    settle_history_verb(&mut *app, "undo", meta("local").instance_id).await; assert_eq!(context::snapshot(&app), original);
    settle_history_verb(&mut *app, "redo", meta("local").instance_id).await; assert_eq!(context::snapshot(&app), after);
    println!("[DEBUG] {key}: texture file picker retained source scope, one canonical source-text mutation, and exact undo/redo");
    }
    close_registered_fixture_app(&mut *app);
}
