//! 🧩️ 🧩️ S Studio app command — `duplicate-app-instance`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️DuplicateAndPaste
/// 🔁️ Shared body for `duplicate_app_instance` (sources = selection) and `paste_app_instance` (sources
/// = clipboard) — both mint a fresh node per source id, offset from the original.
fn duplicate_nodes(source_ids: Vec<String>, projection: &WorkflowSnapshot) -> Emit<WorkflowMutation, SpaceConfigMutation> {
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

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "duplicate-app-instance")]
pub struct DuplicateAppInstance {}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable
/// only through that macro-generated path (`SpaceApp::handle` always routes this command through
/// `apply` below instead), so it degrades to treating the selection as empty (duplicates nothing).
pub fn handle(_payload: &DuplicateAppInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(duplicate_nodes(Vec::new(), doc.snapshot))
}

pub fn apply(_payload: &DuplicateAppInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(duplicate_nodes(interaction.selection("graph").ids.clone(), doc.snapshot))
}
