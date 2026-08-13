//! 🗂️ 🗂️ S Studio app command — `set-media-node-selection`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️GraphSelection
/// 🔁️ Shared body for `node_graph_select` and `set_media_node_selection` — both replace the node
/// selection wholesale (optionally "select all"), publish presence, and set a single active node when
/// exactly one is selected.
fn select_nodes(node_ids: Vec<String>, select_all: bool, projection: &WorkflowSnapshot, _config: &SpaceConfig) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    let node_ids = if select_all { projection.graph.nodes.iter().map(|node| node.id.clone()).collect() } else { node_ids };
    let mut config_mutations = vec![SpaceConfigMutation::SetSelection { node_ids: node_ids.clone() }];
    if node_ids.len() == 1 {
        config_mutations.push(SpaceConfigMutation::SetActiveNode { node_id: node_ids.first().cloned() });
    }
    Emit::config(config_mutations)
}

//#endregion 🔖️GraphSelection

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-media-node-selection")]
pub struct SetMediaNodeSelection {
    pub node_ids: Vec<String>,
    pub select_all: bool,
}

pub fn handle(payload: &SetMediaNodeSelection, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(select_nodes(payload.node_ids.clone(), payload.select_all, doc.snapshot, cfg.snapshot))
}
