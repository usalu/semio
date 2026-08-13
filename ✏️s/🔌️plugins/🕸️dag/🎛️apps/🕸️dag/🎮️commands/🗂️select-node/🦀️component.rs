//! 🗂️ 🗂️ DAG play app commands command — `select-node`.

use crate::apps::dag::config::{DagConfig, DagConfigMutation};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "select-node")]
pub struct SelectNode {
    pub node_id: String,
}

pub fn handle(payload: &SelectNode, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(Emit::config(vec![DagConfigMutation::SetSelection { node_ids: vec![payload.node_id.clone()] }]))
}
