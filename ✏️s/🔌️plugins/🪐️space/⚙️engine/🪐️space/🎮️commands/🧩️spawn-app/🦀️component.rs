//! 🧩️ 🧩️ S Studio app command — `spawn-app`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "spawn-app")]
pub struct SpawnApp {
    pub plugin_id: String,
    pub app_id: String,
    pub x: f64,
    pub y: f64,
}

pub async fn handle(payload: &SpawnApp, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    match crate::engine::space::engine::add_workflow_node_operation(&payload.plugin_id, &payload.app_id, None, payload.x, payload.y) {
        Some((operation, node_id)) => Ok(Emit { artifact_mutations: vec![operation], config_mutations: vec![SpaceConfigMutation::SetActiveNode { node_id: Some(node_id) }], ..Default::default() }),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::space::testkit::{apply_mutations, seed_draw_plugin, seed_multi_port_plugins, studio_emit, test_surface_id};
    use crate::engine::space::SpaceCommand;
    use crate::demo_space_projection;
    use std::collections::HashSet;

    #[test]
    async fn space_command_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::SpawnApp(SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::MoveMediaNode(crate::engine::space::commands::move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveAppInstance(crate::engine::space::commands::remove_app_instance::RemoveAppInstance { node_id: Some("n1".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveAppInstance(crate::engine::space::commands::remove_app_instance::RemoveAppInstance { node_id: None }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::DeleteSelection(crate::engine::space::commands::delete_selection::DeleteSelection {}));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CopyAppInstance(crate::engine::space::commands::copy_app_instance::CopyAppInstance {}));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::DuplicateAppInstance(crate::engine::space::commands::duplicate_app_instance::DuplicateAppInstance {}));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PasteAppInstance(crate::engine::space::commands::paste_app_instance::PasteAppInstance {}));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::RenameAppInstance(crate::engine::space::commands::rename_app_instance::RenameAppInstance { label: Some("Renamed".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchMediaNodes(crate::engine::space::commands::patch_media_nodes::PatchMediaNodes {
            node_ids: vec!["n1".into()],
            field: "position".into(),
            axis: Some("x".into()),
            value: "120".into(),
        }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchAppInstances(crate::engine::space::commands::patch_app_instances::PatchAppInstances {
            node_ids: vec!["n1".into()],
            field: "label".into(),
            value: "Batch Label".into(),
        }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ReorganizeWorkflow(crate::engine::space::commands::reorganize_workflow::ReorganizeWorkflow {}));
    }

    #[test]
    async fn move_media_node_emits_coalesced_move_operation() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node_id = projection.graph.nodes.first().expect("node").id.clone();
        let emit = studio_emit(&projection, &config, &SpaceCommand::MoveMediaNode(crate::engine::space::commands::move_media_node::MoveMediaNode { node_id: node_id.clone(), x: 120.0, y: 160.0 })).expect("handle");
        assert_eq!(emit.coalesce_key.as_deref(), Some(format!("moveMediaNode:{node_id}").as_str()));
        let node = apply_mutations(&projection, &emit.artifact_mutations).graph.nodes.into_iter().find(|row| row.id == node_id).expect("node");
        assert!((node.x - 120.0).abs() < 0.01);
        assert!((node.y - 160.0).abs() < 0.01);
    }

    #[test]
    async fn spawns_draw_app_instance() {
        seed_draw_plugin();
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(SpawnApp { plugin_id: "draw".into(), app_id: test_surface_id("draw"), x: 80.0, y: 80.0 })).expect("handle");
        assert!(!emit.artifact_mutations.is_empty());
        let next = apply_mutations(&projection, &emit.artifact_mutations);
        assert_eq!(next.graph.nodes.len(), projection.graph.nodes.len() + 1);
        let expected_active = next.graph.nodes.last().map(|node| node.id.clone());
        assert_eq!(emit.config_mutations, vec![SpaceConfigMutation::SetActiveNode { node_id: expected_active }]);
    }

    #[test]
    async fn spawns_draw_app_instance_at_drop_position() {
        seed_draw_plugin();
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let existing: HashSet<String> = projection.graph.nodes.iter().map(|node| node.id.clone()).collect();
        let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(SpawnApp { plugin_id: "draw".into(), app_id: test_surface_id("draw"), x: 321.0, y: 654.0 })).expect("handle");
        let next = apply_mutations(&projection, &emit.artifact_mutations);
        let node = next.graph.nodes.iter().find(|node| node.plugin_id == "draw" && !existing.contains(&node.id)).expect("newly spawned draw node");
        assert!((node.x - 321.0).abs() < 0.01);
        assert!((node.y - 654.0).abs() < 0.01);
    }

    #[test]
    async fn patch_app_instances_updates_labels() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let ids: Vec<String> = projection.graph.nodes.iter().take(2).map(|node| node.id.clone()).collect();
        let emit =
            studio_emit(&projection, &config, &SpaceCommand::PatchAppInstances(crate::engine::space::commands::patch_app_instances::PatchAppInstances { node_ids: ids.clone(), field: "label".into(), value: "Batch Label".into() })).expect("handle");
        let next = apply_mutations(&projection, &emit.artifact_mutations);
        let labels: Vec<String> = next.graph.nodes.iter().filter(|node| ids.contains(&node.id)).map(|node| node.label.clone()).collect();
        assert!(labels.iter().all(|label| label == "Batch Label"));
    }

    #[test]
    async fn spawns_puzzle5d_and_shooting_with_multi_port_registrations() {
        seed_multi_port_plugins();
        let mut projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(SpawnApp { plugin_id: "puzzle.5d".into(), app_id: test_surface_id("puzzle5d"), x: 200.0, y: 100.0 })).expect("handle");
        projection = apply_mutations(&projection, &emit.artifact_mutations);
        let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(SpawnApp { plugin_id: "shooting".into(), app_id: test_surface_id("shooting"), x: 300.0, y: 100.0 })).expect("handle");
        projection = apply_mutations(&projection, &emit.artifact_mutations);
        let puzzle_node = projection.graph.nodes.iter().find(|node| node.plugin_id == "puzzle.5d").expect("puzzle node");
        let shooting_node = projection.graph.nodes.iter().find(|node| node.plugin_id == "shooting").expect("shooting node");
        assert_eq!(puzzle_node.outputs.len(), 3, "document:out + out-a + out-b");
        assert_eq!(shooting_node.inputs.len(), 2, "document:in + scene-in");
    }

    #[test]
    async fn undo_redo_round_trip_on_spawn() {
        use semio_framework_plugin::{testkit, VcsArtifactApp};
        seed_draw_plugin();
        let mut app = VcsArtifactApp::new(crate::engine::space::SpaceApp::default());
        let before = app.snapshot().expect("projection").graph.nodes.len();
        testkit::assert_undo_redo_round_trip(&mut app, SpaceCommand::SpawnApp(SpawnApp { plugin_id: "draw".into(), app_id: test_surface_id("draw"), x: 80.0, y: 80.0 }), |app| app.snapshot().expect("projection").graph.nodes.len(), before, before + 1);
    }
}
//#endregion 🧪️Tests
