
use super::*;
use crate::engine::space::SpaceCommand;
use crate::engine::space::commands::spawn_app;
use crate::engine::space::unit_tests::context::{app_with_registry, dispatch, seed_draw_plugin, test_surface_id};
use semio_framework_plugin::{INTERACTION_SELECT_ACTION_ID, InteractionTarget, PluginApp, artifact_app_laws::meta};

/// 🕹️ End-to-end proof the `graph` domain's live selection actually drives `deleteSelection` —
/// spawns a node, selects it via the framework's real `interactionSelect` action (the only way a
/// downstream crate can populate a genuine `InteractionView`, see `context::app`'s own doc
/// comment), then confirms `deleteSelection` removes exactly that node.
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_the_live_selected_node() {
    seed_draw_plugin().await;
    let mut app = app_with_registry().await;
    // 🔁️ A retained tool command only returns an admission receipt — the document is not published
    // until the operation settles, so every dispatch here is followed by its settle step.
    let receiver = meta("local").instance_id;
    dispatch(&mut app, SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: test_surface_id("draw").await, x: 10.0, y: 10.0 })).await;
    let _ = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, receiver).await.expect("spawnApp publication");
    let before = app.snapshot().expect("snapshot");
    let node_id = before.graph.nodes.first().expect("spawned node").id.clone();
    let targets = pack::to_json_string(&vec![InteractionTarget { granularity: "instance".into(), id: node_id.clone() }]);
    let args = semio_framework::DslValue::object(vec![
        ("domainId".to_string(), semio_framework::DslValue::String("graph".into())),
        ("targets".to_string(), semio_framework::DslValue::String(targets)),
        ("merge".to_string(), semio_framework::DslValue::String("replace".into())),
        ("method".to_string(), semio_framework::DslValue::String("pick".into())),
    ]);
    let admitted = app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&args), &meta("local")).await.expect("interactionSelect");
    let _ = semio_framework_plugin::app::settle_framework_reserved_admission(&mut app, admitted).await.expect("interactionSelect admission");
    dispatch(&mut app, SpaceCommand::DeleteSelection(DeleteSelection {})).await;
    let _ = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, receiver).await.expect("deleteSelection publication");
    let after = app.snapshot().expect("snapshot");
    assert!(!after.graph.nodes.iter().any(|node| node.id == node_id), "selected node must be deleted");
}
