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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
