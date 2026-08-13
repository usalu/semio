//! 🔧️ 🔧️ DAG play app commands command — `remove-node`.

use crate::artifacts::dag::schema;
use crate::artifacts::dag::mutations::{change_node_name, create_node, rename_node, replace_node_kind, resize_node};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::apps::dag::config::{DagConfig, DagConfigMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-node")]
pub struct RemoveNode {
    pub node_id: String,
}

pub fn handle(payload: &RemoveNode, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let removes = crate::artifacts::dag::schema::remove_nodes_operations(document, std::slice::from_ref(&payload.node_id));
    if removes.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit { artifact_mutations: removes, config_mutations: vec![DagConfigMutation::SetSelection { node_ids: config.selected_node_ids.iter().filter(|id| *id != &payload.node_id).cloned().collect() }], ..Default::default() })
    }
}
