//! 🗂️ 🗂️ DAG play app commands command — `node-graph-select`.

use crate::apps::dag::config::{DagConfig, DagConfigMutation};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "node-graph-select")]
pub struct NodeGraphSelect {
    pub node_ids: Vec<String>,
}

pub fn handle(payload: &NodeGraphSelect, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(Emit::config(vec![DagConfigMutation::SetSelection { node_ids: payload.node_ids.clone() }]))
}
