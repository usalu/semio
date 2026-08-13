//! 🧩️ 🧩️ S Studio app command — `reorganize-workflow`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "reorganize-workflow")]
pub struct ReorganizeWorkflow {}

pub fn handle(_payload: &ReorganizeWorkflow, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let node_ids: Vec<String> = if config.selected_node_ids.is_empty() { doc.snapshot.graph.nodes.iter().map(|node| node.id.clone()).collect() } else { config.selected_node_ids.clone() };
    let artifact_mutations = node_ids
        .iter()
        .enumerate()
        .map(|(index, node_id)| {
            let col = (index % 4) as f64;
            let row = (index / 4) as f64;
            WorkflowMutation::MoveNode { node_id: node_id.clone(), x: 80.0 + col * 220.0, y: 80.0 + row * 160.0 }
        })
        .collect();
    Ok(Emit::mutations(artifact_mutations))
}
