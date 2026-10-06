//! 🔧️ 🔧️ DAG play app commands command — `remove-node`.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::{DagMutation, DagSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "remove-node")]
pub struct RemoveNode {
    pub node_id: String,
}

/// 🕹️ No longer filters the removed id out of a config selection field — `graph`'s selection now auto-
/// prunes any deleted node id via `DagPlayApp::interaction_topology` (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so no config mutation is needed here at all.
pub fn handle(payload: &RemoveNode, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let leaves = crate::schema::remove_nodes_leaves(&crate::dag_scene(doc)?, std::slice::from_ref(&payload.node_id));
    Ok(crate::dag_child_emit(doc.snapshot, leaves))
}
