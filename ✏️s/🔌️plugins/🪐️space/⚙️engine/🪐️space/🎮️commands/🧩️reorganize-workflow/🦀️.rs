//! 🧩️ 🧩️ S Studio app command — `reorganize-workflow`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::workflow::MoveNode;
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "reorganize-workflow")]
pub struct ReorganizeWorkflow {}

async fn reorganize_selected(doc: &ArtifactView<'_, WorkflowSnapshot>, selected: &[String]) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    let node_ids: Vec<String> = if selected.is_empty() { doc.snapshot.graph.nodes.iter().map(|node| node.id.clone()).collect() } else { selected.to_vec() };
    let artifact_mutations = node_ids
        .iter()
        .enumerate()
        .map(|(index, node_id)| {
            let col = (index % 4) as f64;
            let row = (index / 4) as f64;
            WorkflowMutation::MoveNode(MoveNode { node_id: node_id.clone(), x: 80.0 + col * 220.0, y: 80.0 + row * 160.0 })
        })
        .collect();
    Emit::mutations(artifact_mutations)
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable
/// only through that macro-generated path (`SpaceApp::handle` always routes this command through
/// `apply` below instead); an empty selection already reorganizes every node (unchanged behavior), so
/// this degrades identically to a real "nothing selected" dispatch.
pub fn handle(_payload: &ReorganizeWorkflow, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(crate::engine::space::engine::resolve_future(reorganize_selected(doc, &[])))
}

pub async fn apply(_payload: &ReorganizeWorkflow, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(reorganize_selected(doc, &interaction.selection("graph").ids).await)
}
