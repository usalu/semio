//! 🔧️ 🔧️ DAG play app commands command — `remove-node`.

use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-node")]
pub struct RemoveNode {
    pub node_id: String,
}

/// 🕹️ No longer filters the removed id out of a config selection field — `graph`'s selection now auto-
/// prunes any deleted node id via `DagPlayApp::interaction_topology` (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so no config mutation is needed here at all.
pub async fn handle(payload: &RemoveNode, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    let removes = crate::artifacts::dag::schema::remove_nodes_operations(document, std::slice::from_ref(&payload.node_id));
    if removes.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(removes))
    }
}
