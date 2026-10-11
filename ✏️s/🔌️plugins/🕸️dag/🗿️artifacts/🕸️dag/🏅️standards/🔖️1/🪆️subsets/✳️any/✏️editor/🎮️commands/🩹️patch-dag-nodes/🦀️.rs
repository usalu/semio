//! 🔧️ 🔧️ DAG play app commands command — `patch-dag-nodes`: the inspector's name and slider fields.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::{DagMutation, DagSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-dag-nodes")]
pub struct PatchDagNodes {
    pub node_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

/// 🩹️ `name` relabels every addressed node whose name differs (`change-node-label`); `value`/`min`/`max` yield the ABSOLUTE
/// `set-node-property` of that slider field per addressed slider (plus `resize-node` when the widget refits). A held number field carries its press as the dispatch's top-level `gesture`/`commit`, so the
/// framework scrub machine keeps every tick provisional and commits the release as ONE edit; this handler never reads a
/// gesture and the same dispatch serves a one-shot (MCP, keyboard) unchanged.
pub fn handle(payload: &PatchDagNodes, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let leaves: Vec<_> = crate::dag_scene(doc)?.nodes.iter().filter(|node| payload.node_ids.contains(&node.id)).flat_map(|node| crate::schema::node_field_leaves(node, &payload.field, &payload.value)).collect();
    Ok(crate::dag_child_emit(doc.snapshot, leaves))
}
