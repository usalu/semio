use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::{add_widget, patch_flow_widgets};
use crate::editor_domain::editor_laws::context::{app, dispatch};
use semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dCommand;
use semio_framework_artifact_flow_flow::Widget;
use crate::editor_domain::editor_laws::context;

#[semio_framework_async_macros::async_test]
async fn add_widget_action_appends_widget() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app).host_snapshot.widgets.len();
    dispatch(&mut app, Generation3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, format: None, action: None, x: None, y: None })).await;
    assert!(context::snapshot(&app).host_snapshot.widgets.len() > before);
}

#[semio_framework_async_macros::async_test]
async fn patch_flow_widgets_edits_slider_value() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5), gesture: None })).await;
    let value = context::snapshot(&app).host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::InputSlider { id, value, .. } if id == "height" => Some(*value),
        _ => None,
    });
    assert_eq!(value, Some(9.5));
}

#[semio_framework_async_macros::async_test]
async fn patch_flow_widgets_recomputes_preview_geometry() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let before_fixture = context::snapshot(&app).host_snapshot.clone();
    let mut before_session = FlowEvalSession::new().with_geometry_port(crate::flow_operators::geometry_session().port());
    let mut before_host = semio_framework_os_flow::flow_host_with_session(&before_fixture, &before_session);
    before_session.sync(&before_host);
    while before_session.tick(&mut before_host, None) {}
    before_host.retire_cold();
    let before_eval = before_session.eval_json().to_string();
    let config = Generation3dConfig::default();
    crate::flow_operators::resolve_preview_geometry(&before_eval, &before_fixture, &config.lod_mode, &mut before_session);
    let before_meshes = semio_s_artifact_procedural_generation3d::editor::generation3d::preview_payload(&before_eval, &before_fixture, &config, Some(&before_session), &Default::default()).meshes_json;

    dispatch(&mut app, Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5), gesture: None })).await;
    let after_fixture = context::snapshot(&app).host_snapshot.clone();
    let mut after_session = FlowEvalSession::new().with_geometry_port(crate::flow_operators::geometry_session().port());
    let mut after_host = semio_framework_os_flow::flow_host_with_session(&after_fixture, &after_session);
    after_session.sync(&after_host);
    while after_session.tick(&mut after_host, None) {}
    after_host.retire_cold();
    let after_eval = after_session.eval_json().to_string();
    crate::flow_operators::resolve_preview_geometry(&after_eval, &after_fixture, &config.lod_mode, &mut after_session);
    let after_meshes = semio_s_artifact_procedural_generation3d::editor::generation3d::preview_payload(&after_eval, &after_fixture, &config, Some(&after_session), &Default::default()).meshes_json;

    assert_ne!(before_eval, after_eval, "slider mutation must invalidate the evaluated flow");
    assert_ne!(before_meshes, after_meshes, "slider mutation must change the tessellated preview mesh");
    // 🧹️ Both owners reject a live drop: `FlowEvalSession` demands the explicit close boundary its
    // production owner walks, and a cloned `FlowHostSnapshot` carries the fail-closed
    // `OrderedMap<WidgetLayout>` root (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    context::retire_flow_eval_session(after_session);
    context::retire_flow_eval_session(before_session);
    after_fixture.retire_cold();
    before_fixture.retire_cold();
}

#[semio_framework_async_macros::async_test]
async fn remove_widget_action_deletes_by_id() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    assert!(context::snapshot(&app).host_snapshot.widgets.iter().any(|widget| semio_s_artifact_procedural_generation3d::widget_id(widget) == "sides"));
    dispatch(&mut app, Generation3dCommand::RemoveWidget(RemoveWidget { widget_id: "sides".into() })).await;
    assert!(!context::snapshot(&app).host_snapshot.widgets.iter().any(|widget| semio_s_artifact_procedural_generation3d::widget_id(widget) == "sides"));
}
