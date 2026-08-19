//! 🧩️ 🧩️ S Studio app command — `rename-app-instance`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "rename-app-instance")]
pub struct RenameAppInstance {
    pub label: Option<String>,
}

async fn rename_with_selection(payload: &RenameAppInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, config: &SpaceConfig, selected: &[String]) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    match crate::engine::space::primary_selected_node_id(selected, config) {
        Some(node_id) => {
            let next_label = payload.label.clone().or_else(|| doc.snapshot.graph.nodes.iter().find(|row| row.id == node_id).map(|node| format!("{} (renamed)", node.label)));
            match next_label {
                Some(next_label) => Emit::mutations(vec![WorkflowMutation::PatchNode { node_id, label: next_label }]),
                None => Emit::default(),
            }
        }
        None => Emit::default(),
    }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable
/// only through that macro-generated path (`SpaceApp::handle` always routes this command through
/// `apply` below instead); the fallback still honors `config.active_node_id` — only the "fall back to
/// the live selection" step degrades to empty.
pub async fn handle(payload: &RenameAppInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(rename_with_selection(payload, doc, cfg.snapshot, &[]))
}

pub async fn apply(payload: &RenameAppInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(rename_with_selection(payload, doc, cfg.snapshot, &interaction.selection("graph").ids))
}
