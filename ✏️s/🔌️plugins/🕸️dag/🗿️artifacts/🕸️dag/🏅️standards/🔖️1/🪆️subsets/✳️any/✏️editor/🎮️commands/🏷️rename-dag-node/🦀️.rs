//! 🔧️ 🔧️ DAG play app commands command — `rename-dag-node`.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::{DagMutation, DagSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "rename-dag-node")]
pub struct RenameDagNode {
    pub old_id: String,
    pub value: String,
}

/// 🏷️ One graph `rename-node` child leaf: the graph cascades the id change to every edge endpoint naming the node.
/// 🕹️ No longer re-selects the node under its new id — no `Emit` channel writes `graph`'s selection
/// directly anymore (the framework owns it exclusively; ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &RenameDagNode, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let trimmed = payload.value.trim();
    let scene = crate::dag_scene(doc)?;
    if trimmed.is_empty() || trimmed == payload.old_id.as_str() || scene.nodes.iter().any(|node| node.id == trimmed) || !scene.nodes.iter().any(|node| node.id == payload.old_id) {
        return Ok(Emit::default());
    }
    Ok(crate::dag_child_emit(doc.snapshot, &[crate::rename_node_leaf(&payload.old_id, trimmed)]))
}
