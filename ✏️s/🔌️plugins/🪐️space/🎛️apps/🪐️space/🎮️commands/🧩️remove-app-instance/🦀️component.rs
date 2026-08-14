//! 🧩️ 🧩️ S Studio app command — `remove-app-instance`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-app-instance")]
pub struct RemoveAppInstance {
    pub node_id: Option<String>,
}

fn remove_with_selection(payload: &RemoveAppInstance, config: &SpaceConfig, selected: &[String]) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    match payload.node_id.clone().or_else(|| crate::apps::space::primary_selected_node_id(selected, config)) {
        Some(node_id) => {
            let mut config_mutations = Vec::new();
            if config.active_node_id.as_deref() == Some(node_id.as_str()) {
                config_mutations.push(SpaceConfigMutation::SetActiveNode { node_id: None });
            }
            if config.focused_node_id.as_deref() == Some(node_id.as_str()) {
                config_mutations.push(SpaceConfigMutation::SetFocusedNode { node_id: None });
            }
            Emit { artifact_mutations: vec![WorkflowMutation::RemoveNode { node_id }], config_mutations, ..Default::default() }
        }
        None => Emit::default(),
    }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable
/// only through that macro-generated path (`SpaceApp::handle` always routes this command through
/// `apply` below instead); `payload.node_id` (when set) is unaffected, and the fallback still honors
/// `config.active_node_id` — only the "fall back to the live selection" step degrades to empty.
pub fn handle(payload: &RemoveAppInstance, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(remove_with_selection(payload, cfg.snapshot, &[]))
}

pub fn apply(payload: &RemoveAppInstance, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(remove_with_selection(payload, cfg.snapshot, &interaction.selection("graph").ids))
}
