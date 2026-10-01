//! 🔍️ 🔍️ S Studio app command — `open-instance`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Effect, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "open-instance")]
pub struct OpenInstance {
    pub node_id: Option<String>,
}

/// 🕹️ Selection now only informs which node opens, not a `SetSelection` config mutation (the
/// framework owns `graph`'s selection state now — ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub async fn open_with_selection(payload: &OpenInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, config: &SpaceConfig, selected: &[String]) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    let resolved_node_id = match payload.node_id.clone() {
        Some(node_id) => Some(node_id),
        None => crate::engine::space::primary_selected_node_id(selected, config).await,
    };
    match resolved_node_id {
        Some(node_id) => match doc.snapshot.graph.nodes.iter().find(|row| row.id == node_id) {
            Some(node) => Emit {
                config_mutations: vec![SpaceConfigMutation::SetFocusedNode { node_id: Some(node_id.clone()) }, SpaceConfigMutation::SetActiveNode { node_id: Some(node_id.clone()) }],
                effects: vec![Effect::OpenPluginInstance { plugin_id: node.plugin_id.clone(), app_id: node.app_id.clone(), os_instance_id: Some(node.id.clone()) }],
                ..Default::default()
            },
            None => Emit::default(),
        },
        None => Emit::default(),
    }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable
/// only through that macro-generated path (`SpaceApp::handle` always routes this command through
/// `apply` below instead); `payload.node_id` (when set) is unaffected — only the "fall back to the
/// live selection" step degrades to empty.
pub fn handle(payload: &OpenInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(crate::engine::space::engine::resolve_future(open_with_selection(payload, doc, cfg.snapshot, &[])))
}

pub async fn apply(payload: &OpenInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(open_with_selection(payload, doc, cfg.snapshot, &interaction.selection("graph").ids).await)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
