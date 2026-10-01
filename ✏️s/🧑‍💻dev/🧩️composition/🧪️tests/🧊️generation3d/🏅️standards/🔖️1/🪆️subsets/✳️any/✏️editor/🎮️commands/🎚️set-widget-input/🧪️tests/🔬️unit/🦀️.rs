use super::*;
use dsl::ToValue;





#[semio_framework_async_macros::async_test]
async fn widget_input_publication_supports_scalar_events_and_undo_redo() {
    use crate::editor_domain::editor_laws::context;
    use semio_s_artifact_procedural_generation3d::editor::generation3d::{Generation3dCommand, Generation3dPlayApp};
    use semio_framework_plugin::ArtifactEditor;
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = context::app().await;
    context::dispatch(&mut app, Generation3dCommand::AddWidget(semio_s_artifact_procedural_generation3d::editor::generation3d::commands::add_widget::AddWidget { kind: "neuron".into(), neuron_kind: Some("brep.mesh.box".into()), format: None, action: None, x: None, y: None })).await;
    let before = context::snapshot(&app);
    let id = semio_s_artifact_procedural_generation3d::widget_id(before.host_snapshot.widgets.last().unwrap()).to_string();
    let args: dsl::DslValue = serde_json::json!({"widgetId":id,"channel":"width","value":2.5}).into();
    let command = Generation3dPlayApp::command_from_action("setWidgetInput", Some(&args)).unwrap();
    context::dispatch(&mut app, command).await;
    let after = context::snapshot(&app);
    let params = after.host_snapshot.widgets.iter().find_map(|widget| match widget { Widget::Neuron { id: candidate, params, .. } if candidate == &id => Some(params.to_value()), _ => None }).unwrap();
    assert_eq!(params.get("width").and_then(|value| value.get("value")).and_then(dsl::DslValue::as_f64), Some(2.5));
    eprintln!("[DEBUG] inspector input {id}.width published 2.5");
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "undo", 1).await;
    assert_eq!(context::snapshot(&app), before);
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "redo", 1).await;
    assert_eq!(context::snapshot(&app), after);
    eprintln!("[DEBUG] inspector input {id}.width undo/redo restored typed parameters");
}
