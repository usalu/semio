use super::*;
use crate::editor_domain::editor_laws::context;
use semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dCommand;
use semio_framework_plugin::{ArtifactEditor, PluginApp};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-widget/🧫️fixtures/🔣️.json")).expect("creation fixture")
}

fn payload(value: &serde_json::Value) -> AddWidget {
    AddWidget {
        kind: value["kind"].as_str().unwrap().into(),
        neuron_kind: value["neuronKind"].as_str().map(String::from),
        format: value["format"].as_str().map(String::from),
        action: value["action"].as_str().map(String::from),
        x: value["x"].as_f64(),
        y: value["y"].as_f64(),
    }
}



#[semio_framework_async_macros::async_test]
async fn catalogue_descriptors_survive_action_decode_and_retained_publication() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = context::app().await;
    for value in fixture()["valid"].as_array().unwrap() {
        let args: dsl::DslValue = value.clone().into();
        let before = context::snapshot(&app).host_snapshot.widgets.len();
        let command = semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dPlayApp::command_from_action("addWidget", Some(&args)).expect("command");
        let Generation3dCommand::AddWidget(decoded) = &command else { panic!("add widget command") };
        assert_eq!(decoded, &payload(value));
        context::dispatch(&mut app, command).await;
        let after = context::snapshot(&app);
        assert_eq!(after.host_snapshot.widgets.len(), before + 1);
        let widget = after.host_snapshot.widgets.last().unwrap();
        match widget {
            semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, .. } => assert_eq!(neuron_kind, value["neuronKind"].as_str().unwrap()),
            semio_framework_artifact_flow_flow::Widget::OutputExport { format, .. } => assert_eq!(format, value["format"].as_str().unwrap()),
            semio_framework_artifact_flow_flow::Widget::OutputAction { action, .. } => assert_eq!(action, value["action"].as_str().unwrap()),
            _ => {}
        }
        eprintln!("[DEBUG] catalogue {} publication: {} -> {}", value, before, after.host_snapshot.widgets.len());
    }
}

#[semio_framework_async_macros::async_test]
async fn rejected_creation_returns_a_fault_and_leaves_the_document_unchanged() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = context::app().await;
    let before = context::snapshot(&app);
    let cases = fixture();
    for value in cases["invalid"].as_array().unwrap().iter().chain(cases["unsupportedOperators"].as_array().unwrap()) {
        let args: dsl::DslValue = value.clone().into();
        let command = semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dPlayApp::command_from_action("addWidget", Some(&args)).expect("decoded");
        let admission = app.dispatch_typed(command, &semio_framework_plugin::artifact_app_laws::meta("local")).await;
        let fault = match admission {
            Err(fault) => fault,
            Ok(_) => semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut *app, 1).await.err().expect("creation must fail"),
        };
        eprintln!("[DEBUG] catalogue refused creation: {} {}", fault.code.0, fault.message);
        assert_eq!(fault.code.0, fixture()["faultCode"].as_str().unwrap());
        assert_eq!(context::snapshot(&app), before);
    }
}

#[semio_framework_async_macros::async_test]
async fn repeated_automatic_additions_have_separated_rendered_rectangles() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = context::app().await;
    for _ in 0..3 {
        context::dispatch(&mut app, Generation3dCommand::AddWidget(payload(&serde_json::json!({"kind":"inputNote"})))).await;
    }
    let snapshot = context::snapshot(&app);
    with_host(&snapshot.host_snapshot, |host| {
        let nodes = &host.dag.host_snapshot.nodes;
        for node in nodes.iter().rev().take(3) {
            for other in nodes.iter().filter(|other| other.id != node.id) {
                assert!((node.x - other.x).abs() >= (node.width + other.width) / 2.0 + AUTOMATIC_GAP || (node.y - other.y).abs() >= (node.height + other.height) / 2.0 + AUTOMATIC_GAP, "{} overlaps {}", node.id, other.id);
            }
            eprintln!("[DEBUG] catalogue automatic placement {} at ({}, {}) size ({}, {})", node.id, node.x, node.y, node.width, node.height);
        }
    });
}

#[semio_framework_async_macros::async_test]
async fn automatic_addition_undo_redo_restores_both_content_and_position() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = context::app().await;
    let before = context::snapshot(&app);
    context::dispatch(&mut app, Generation3dCommand::AddWidget(payload(&serde_json::json!({"kind":"inputNote"})))).await;
    let after = context::snapshot(&app);
    assert_eq!(after.host_snapshot.widgets.len(), before.host_snapshot.widgets.len() + 1);
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "undo", 1).await;
    assert_eq!(context::snapshot(&app), before);
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "redo", 1).await;
    assert_eq!(context::snapshot(&app), after);
}
