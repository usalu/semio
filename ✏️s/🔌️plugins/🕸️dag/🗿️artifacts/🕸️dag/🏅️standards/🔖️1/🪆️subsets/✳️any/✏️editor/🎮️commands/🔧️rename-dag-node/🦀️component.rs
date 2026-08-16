//! 🔧️ 🔧️ DAG play app commands command — `rename-dag-node`.

use crate::artifacts::dag::schema;
use crate::artifacts::dag::mutations::{change_node_name, create_node, rename_node, replace_node_kind, resize_node};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "rename-dag-node")]
pub struct RenameDagNode {
    pub old_id: String,
    pub value: String,
}

/// 🕹️ No longer re-selects the node under its new id — no `Emit` channel writes `graph`'s selection
/// directly anymore (the framework owns it exclusively; ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &RenameDagNode, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    let trimmed = payload.value.trim();
    if trimmed.is_empty() || trimmed == payload.old_id.as_str() || document.nodes().iter().any(|node| node.id == trimmed) {
        return Ok(Emit::default());
    }
    // 🏷️ `rename-node` already cascades the id change to every edge endpoint string that
    // referenced the old id — no manual node/edge rebuild needed here any more.
    Ok(Emit::mutations(vec![rename_node(payload.old_id.clone(), trimmed.to_string())]))
}
