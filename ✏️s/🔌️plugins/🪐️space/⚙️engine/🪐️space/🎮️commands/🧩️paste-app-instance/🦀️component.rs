//! 🧩️ 🧩️ S Studio app command — `paste-app-instance`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️DuplicateAndPaste
/// 🔁️ Shared body for `duplicate_app_instance` (sources = selection) and `paste_app_instance` (sources
/// = clipboard) — both mint a fresh node per source id, offset from the original.
async fn duplicate_nodes(source_ids: Vec<String>, projection: &WorkflowSnapshot) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    let mut artifact_mutations = Vec::new();
    let mut new_active_node_id = None;
    for node_id in source_ids {
        let Some(node) = projection.graph.nodes.iter().find(|row| row.id == node_id) else { continue };
        let label = format!("{} Copy", node.label);
        if let Some((operation, new_id)) = crate::engine::space::engine::add_workflow_node_operation(&node.plugin_id, &node.app_id, Some(&label), node.x + 40.0, node.y + 40.0) {
            new_active_node_id = Some(new_id);
            artifact_mutations.push(operation);
        }
    }
    let config_mutations = new_active_node_id.into_iter().map(|node_id| SpaceConfigMutation::SetActiveNode { node_id: Some(node_id) }).collect();
    Emit { artifact_mutations, config_mutations, ..Default::default() }
}

//#endregion 🔖️DuplicateAndPaste


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "paste-app-instance")]
pub struct PasteAppInstance {}

pub async fn handle(_payload: &PasteAppInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(duplicate_nodes(cfg.snapshot.clipboard_node_ids.clone(), doc.snapshot))
}
