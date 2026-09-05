//! 🧩️ 🧩️ S Studio app command — `delete-selection`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::workflow::RemoveNode;
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

/// 🕹️ Reads the `graph` domain's current selection instead of a deleted config field — no
/// `SetSelection` config mutation needed afterwards, the framework auto-prunes the deleted ids out of
/// `graph`'s selection via `interaction_topology`.
async fn delete_selected(config: &SpaceConfig, selected: &[String]) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    let artifact_mutations = selected.iter().cloned().map(|node_id| WorkflowMutation::RemoveNode(RemoveNode { node_id })).collect();
    let mut config_mutations = Vec::new();
    if config.active_node_id.as_ref().is_some_and(|id| selected.contains(id)) {
        config_mutations.push(SpaceConfigMutation::SetActiveNode { node_id: None });
    }
    if config.focused_node_id.as_ref().is_some_and(|id| selected.contains(id)) {
        config_mutations.push(SpaceConfigMutation::SetFocusedNode { node_id: None });
    }
    Emit { artifact_mutations, config_mutations, ..Default::default() }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable
/// only through that macro-generated path (`SpaceApp::handle` always routes this command through
/// `apply` below instead), so it degrades to treating the selection as empty, matching the same gap
/// `SpaceApp::render`'s own selection-dependent branches already carry.
pub fn handle(_payload: &DeleteSelection, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(crate::engine::space::engine::resolve_future(delete_selected(cfg.snapshot, &[])))
}

pub async fn apply(_payload: &DeleteSelection, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(delete_selected(cfg.snapshot, &interaction.selection("graph").ids).await)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::space::commands::spawn_app;
    use crate::engine::space::testkit::{app_with_registry, dispatch, seed_draw_plugin, test_surface_id};
    use crate::engine::space::SpaceCommand;
    use semio_framework_plugin::{testkit::meta, InteractionTarget, PluginApp, INTERACTION_SELECT_ACTION_ID};

    /// 🕹️ End-to-end proof the `graph` domain's live selection actually drives `deleteSelection` —
    /// spawns a node, selects it via the framework's real `interactionSelect` action (the only way a
    /// downstream crate can populate a genuine `InteractionView`, see `testkit::app`'s own doc
    /// comment), then confirms `deleteSelection` removes exactly that node.
    #[semio_framework_async_macros::async_test]
    async fn delete_selection_removes_the_live_selected_node() {
        seed_draw_plugin().await;
        let mut app = app_with_registry().await;
        dispatch(&mut app, SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: test_surface_id("draw").await, x: 10.0, y: 10.0 })).await;
        let before = app.snapshot().expect("snapshot");
        let node_id = before.graph.nodes.first().expect("spawned node").id.clone();
        let targets = pack::to_json_string(&vec![InteractionTarget { granularity: "instance".into(), id: node_id.clone() }]);
        let args = semio_framework::DslValue::object(vec![
            ("domainId".to_string(), semio_framework::DslValue::String("graph".into())),
            ("targets".to_string(), semio_framework::DslValue::String(targets)),
            ("merge".to_string(), semio_framework::DslValue::String("replace".into())),
            ("method".to_string(), semio_framework::DslValue::String("pick".into())),
        ]);
        app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&args), &meta("local")).await.expect("interactionSelect");
        dispatch(&mut app, SpaceCommand::DeleteSelection(DeleteSelection {})).await;
        let after = app.snapshot().expect("snapshot");
        assert!(!after.graph.nodes.iter().any(|node| node.id == node_id), "selected node must be deleted");
    }
}
//#endregion 🧪️Tests
