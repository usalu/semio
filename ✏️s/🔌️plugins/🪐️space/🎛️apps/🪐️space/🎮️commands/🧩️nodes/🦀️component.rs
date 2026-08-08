//! 🧩️ S Studio app — workflow node/app-instance lifecycle commands: spawn/move/remove/copy/rename/patch.
//!
//! One nested `pub mod` per payload (the `app_commands!` shape — see `apps::space::🦀️component.rs`'s
//! `🔖️SpaceCommand` region, which `use`s each of these modules flat).

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowDocument, WorkflowMutation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};

//#region 🔖️SpawnApp
pub mod spawn_app {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "spawn-app")]
    pub struct SpawnApp {
        pub plugin_id: String,
        pub app_id: String,
        pub x: f64,
        pub y: f64,
    }

    pub fn handle(payload: &SpawnApp, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        match crate::apps::space::engine::add_workflow_node_operation(&payload.plugin_id, &payload.app_id, None, payload.x, payload.y) {
            Some((operation, node_id)) => Ok(Emit { document_mutations: vec![operation], config_mutations: vec![SpaceConfigMutation::SetActiveNode { node_id: Some(node_id) }], ..Default::default() }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SpawnApp

//#region 🔖️MoveMediaNode
pub mod move_media_node {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-media-node")]
    pub struct MoveMediaNode {
        pub node_id: String,
        pub x: f64,
        pub y: f64,
    }

    pub fn handle(payload: &MoveMediaNode, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(Emit::amend(vec![WorkflowMutation::MoveNode { node_id: payload.node_id.clone(), x: payload.x, y: payload.y }], format!("moveMediaNode:{}", payload.node_id)))
    }
}
//#endregion 🔖️MoveMediaNode

//#region 🔖️RemoveAppInstance
pub mod remove_app_instance {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-app-instance")]
    pub struct RemoveAppInstance {
        pub node_id: Option<String>,
    }

    pub fn handle(payload: &RemoveAppInstance, _doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let config = cfg.projection;
        match payload.node_id.clone().or_else(|| crate::apps::space::primary_selected_node_id(config)) {
            Some(node_id) => {
                let mut config_mutations = Vec::new();
                if config.active_node_id.as_deref() == Some(node_id.as_str()) {
                    config_mutations.push(SpaceConfigMutation::SetActiveNode { node_id: None });
                }
                if config.focused_node_id.as_deref() == Some(node_id.as_str()) {
                    config_mutations.push(SpaceConfigMutation::SetFocusedNode { node_id: None });
                }
                Ok(Emit { document_mutations: vec![WorkflowMutation::RemoveNode { node_id }], config_mutations, ..Default::default() })
            }
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️RemoveAppInstance

//#region 🔖️DeleteSelection
pub mod delete_selection {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, _doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let config = cfg.projection;
        let document_mutations = config.selected_node_ids.iter().cloned().map(|node_id| WorkflowMutation::RemoveNode { node_id }).collect();
        Ok(Emit {
            document_mutations,
            config_mutations: vec![SpaceConfigMutation::SetSelection { node_ids: Vec::new() }, SpaceConfigMutation::SetActiveNode { node_id: None }, SpaceConfigMutation::SetFocusedNode { node_id: None }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️DeleteSelection

//#region 🔖️CopyAppInstance
pub mod copy_app_instance {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "copy-app-instance")]
    pub struct CopyAppInstance {}

    pub fn handle(_payload: &CopyAppInstance, _doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SpaceConfigMutation::SetClipboard { node_ids: cfg.projection.selected_node_ids.clone() }]))
    }
}
//#endregion 🔖️CopyAppInstance

//#region 🔖️DuplicateAndPaste
/// 🔁️ Shared body for `duplicate_app_instance` (sources = selection) and `paste_app_instance` (sources
/// = clipboard) — both mint a fresh node per source id, offset from the original.
fn duplicate_nodes(source_ids: Vec<String>, projection: &WorkflowDocument) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    let mut document_mutations = Vec::new();
    let mut new_active_node_id = None;
    for node_id in source_ids {
        let Some(node) = projection.graph.nodes.iter().find(|row| row.id == node_id) else { continue };
        let label = format!("{} Copy", node.label);
        if let Some((operation, new_id)) = crate::apps::space::engine::add_workflow_node_operation(&node.plugin_id, &node.app_id, Some(&label), node.x + 40.0, node.y + 40.0) {
            new_active_node_id = Some(new_id);
            document_mutations.push(operation);
        }
    }
    let config_mutations = new_active_node_id.into_iter().map(|node_id| SpaceConfigMutation::SetActiveNode { node_id: Some(node_id) }).collect();
    Emit { document_mutations, config_mutations, ..Default::default() }
}

pub mod duplicate_app_instance {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "duplicate-app-instance")]
    pub struct DuplicateAppInstance {}

    pub fn handle(_payload: &DuplicateAppInstance, doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(duplicate_nodes(cfg.projection.selected_node_ids.clone(), doc.projection))
    }
}

pub mod paste_app_instance {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "paste-app-instance")]
    pub struct PasteAppInstance {}

    pub fn handle(_payload: &PasteAppInstance, doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(duplicate_nodes(cfg.projection.clipboard_node_ids.clone(), doc.projection))
    }
}
//#endregion 🔖️DuplicateAndPaste

//#region 🔖️RenameAppInstance
pub mod rename_app_instance {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "rename-app-instance")]
    pub struct RenameAppInstance {
        pub label: Option<String>,
    }

    pub fn handle(payload: &RenameAppInstance, doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        match crate::apps::space::primary_selected_node_id(cfg.projection) {
            Some(node_id) => {
                let next_label = payload.label.clone().or_else(|| doc.projection.graph.nodes.iter().find(|row| row.id == node_id).map(|node| format!("{} (renamed)", node.label)));
                match next_label {
                    Some(next_label) => Ok(Emit::mutations(vec![WorkflowMutation::PatchNode { node_id, label: next_label }])),
                    None => Ok(Emit::default()),
                }
            }
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️RenameAppInstance

//#region 🔖️PatchMediaNodes
pub mod patch_media_nodes {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-media-nodes")]
    pub struct PatchMediaNodes {
        pub node_ids: Vec<String>,
        pub field: String,
        pub axis: Option<String>,
        pub value: String,
    }

    pub fn handle(payload: &PatchMediaNodes, doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let projection = doc.projection;
        let numeric = payload.value.parse::<f64>().ok();
        if payload.field == "position" {
            let Some(numeric) = numeric else { return Ok(Emit::default()) };
            let document_mutations = payload
                .node_ids
                .iter()
                .filter_map(|node_id| {
                    let node = projection.graph.nodes.iter().find(|row| &row.id == node_id)?;
                    let x = if payload.axis.as_deref() == Some("x") { numeric } else { node.x };
                    let y = if payload.axis.as_deref() == Some("y") { numeric } else { node.y };
                    Some(WorkflowMutation::MoveNode { node_id: node_id.clone(), x, y })
                })
                .collect();
            Ok(Emit::mutations(document_mutations))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️PatchMediaNodes

//#region 🔖️PatchAppInstances
pub mod patch_app_instances {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-app-instances")]
    pub struct PatchAppInstances {
        pub node_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchAppInstances, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        if payload.field == "label" {
            Ok(Emit::mutations(payload.node_ids.iter().cloned().map(|node_id| WorkflowMutation::PatchNode { node_id, label: payload.value.clone() }).collect()))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️PatchAppInstances

//#region 🔖️ReorganizeWorkflow
pub mod reorganize_workflow {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reorganize-workflow")]
    pub struct ReorganizeWorkflow {}

    pub fn handle(_payload: &ReorganizeWorkflow, doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let config = cfg.projection;
        let node_ids: Vec<String> = if config.selected_node_ids.is_empty() { doc.projection.graph.nodes.iter().map(|node| node.id.clone()).collect() } else { config.selected_node_ids.clone() };
        let document_mutations = node_ids
            .iter()
            .enumerate()
            .map(|(index, node_id)| {
                let col = (index % 4) as f64;
                let row = (index / 4) as f64;
                WorkflowMutation::MoveNode { node_id: node_id.clone(), x: 80.0 + col * 220.0, y: 80.0 + row * 160.0 }
            })
            .collect();
        Ok(Emit::mutations(document_mutations))
    }
}
//#endregion 🔖️ReorganizeWorkflow

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::space::testkit::{apply_mutations, seed_draw_plugin, seed_multi_port_plugins, studio_emit};
    use crate::apps::space::SpaceCommand;
    use crate::demo_space_projection;
    use std::collections::HashSet;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveAppInstance(remove_app_instance::RemoveAppInstance { node_id: Some("n1".into()) }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveAppInstance(remove_app_instance::RemoveAppInstance { node_id: None }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::DeleteSelection(delete_selection::DeleteSelection {}));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::CopyAppInstance(copy_app_instance::CopyAppInstance {}));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::DuplicateAppInstance(duplicate_app_instance::DuplicateAppInstance {}));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::PasteAppInstance(paste_app_instance::PasteAppInstance {}));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::RenameAppInstance(rename_app_instance::RenameAppInstance { label: Some("Renamed".into()) }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchMediaNodes(patch_media_nodes::PatchMediaNodes { node_ids: vec!["n1".into()], field: "position".into(), axis: Some("x".into()), value: "120".into() }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchAppInstances(patch_app_instances::PatchAppInstances { node_ids: vec!["n1".into()], field: "label".into(), value: "Batch Label".into() }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ReorganizeWorkflow(reorganize_workflow::ReorganizeWorkflow {}));
    }

    #[test]
    fn move_media_node_emits_coalesced_move_operation() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node_id = projection.graph.nodes.first().expect("node").id.clone();
        let emit = studio_emit(&projection, &config, &SpaceCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: node_id.clone(), x: 120.0, y: 160.0 })).expect("handle");
        assert_eq!(emit.coalesce_key.as_deref(), Some(format!("moveMediaNode:{node_id}").as_str()));
        let node = apply_mutations(&projection, &emit.document_mutations).graph.nodes.into_iter().find(|row| row.id == node_id).expect("node");
        assert!((node.x - 120.0).abs() < 0.01);
        assert!((node.y - 160.0).abs() < 0.01);
    }

    #[test]
    fn spawns_draw_app_instance() {
        seed_draw_plugin();
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 })).expect("handle");
        assert!(!emit.document_mutations.is_empty());
        let next = apply_mutations(&projection, &emit.document_mutations);
        assert_eq!(next.graph.nodes.len(), projection.graph.nodes.len() + 1);
        let expected_active = next.graph.nodes.last().map(|node| node.id.clone());
        assert_eq!(emit.config_mutations, vec![SpaceConfigMutation::SetActiveNode { node_id: expected_active }]);
    }

    #[test]
    fn spawns_draw_app_instance_at_drop_position() {
        seed_draw_plugin();
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let existing: HashSet<String> = projection.graph.nodes.iter().map(|node| node.id.clone()).collect();
        let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 321.0, y: 654.0 })).expect("handle");
        let next = apply_mutations(&projection, &emit.document_mutations);
        let node = next.graph.nodes.iter().find(|node| node.plugin_id == "draw" && !existing.contains(&node.id)).expect("newly spawned draw node");
        assert!((node.x - 321.0).abs() < 0.01);
        assert!((node.y - 654.0).abs() < 0.01);
    }

    #[test]
    fn patch_app_instances_updates_labels() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let ids: Vec<String> = projection.graph.nodes.iter().take(2).map(|node| node.id.clone()).collect();
        let emit = studio_emit(&projection, &config, &SpaceCommand::PatchAppInstances(patch_app_instances::PatchAppInstances { node_ids: ids.clone(), field: "label".into(), value: "Batch Label".into() })).expect("handle");
        let next = apply_mutations(&projection, &emit.document_mutations);
        let labels: Vec<String> = next.graph.nodes.iter().filter(|node| ids.contains(&node.id)).map(|node| node.label.clone()).collect();
        assert!(labels.iter().all(|label| label == "Batch Label"));
    }

    #[test]
    fn spawns_puzzle5d_and_shooting_with_multi_port_registrations() {
        seed_multi_port_plugins();
        let mut projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "puzzle.5d".into(), app_id: "puzzle5d".into(), x: 200.0, y: 100.0 })).expect("handle");
        projection = apply_mutations(&projection, &emit.document_mutations);
        let emit = studio_emit(&projection, &config, &SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "shooting".into(), app_id: "shooting".into(), x: 300.0, y: 100.0 })).expect("handle");
        projection = apply_mutations(&projection, &emit.document_mutations);
        let puzzle_node = projection.graph.nodes.iter().find(|node| node.plugin_id == "puzzle.5d").expect("puzzle node");
        let shooting_node = projection.graph.nodes.iter().find(|node| node.plugin_id == "shooting").expect("shooting node");
        assert_eq!(puzzle_node.outputs.len(), 3, "document:out + out-a + out-b");
        assert_eq!(shooting_node.inputs.len(), 2, "document:in + scene-in");
    }

    #[test]
    fn undo_redo_round_trip_on_spawn() {
        use semio_framework_plugin::{testkit, VcsDocumentApp};
        seed_draw_plugin();
        let mut app = VcsDocumentApp::new(crate::apps::space::SpaceApp);
        let before = app.projection().expect("projection").graph.nodes.len();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 }),
            |app| app.projection().expect("projection").graph.nodes.len(),
            before,
            before + 1,
        );
    }
}
//#endregion 🧪️Tests
