//! 🔧️ 🔧️ DAG play app commands command — `patch-dag-nodes`.

use crate::artifacts::dag::mutations::{change_node_name, replace_node_kind, resize_node};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-dag-nodes")]
pub struct PatchDagNodes {
    pub node_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

pub async fn handle(payload: &PatchDagNodes, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    // 🩹️ `node_patch_for_field` only ever fills `name` (a scalar rename) or `kind`+`width`+
    // `height` together (a Slider's live-dragged value/min/max, which also refits the widget
    // size) — re-expressed here as the matching targeted mutations instead of a generic patch.
    let nodes = document.nodes();
    let operations: Vec<DagMutation> = nodes
        .iter()
        .filter(|node| payload.node_ids.contains(&node.id))
        .flat_map(|node| {
            let patch = crate::artifacts::dag::schema::node_patch_for_field(node, &payload.field, Some(payload.value.as_str()));
            let mut ops = Vec::new();
            if let Some(patch) = patch {
                if let Some(name) = patch.name {
                    ops.push(change_node_name(node.id.clone(), name));
                }
                if let Some(kind) = patch.kind {
                    ops.push(replace_node_kind(node.id.clone(), kind));
                }
                if let (Some(width), Some(height)) = (patch.width, patch.height) {
                    ops.push(resize_node(node.id.clone(), width, height));
                }
            }
            ops
        })
        .collect();
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::amend(operations, format!("patch-{}-{}", payload.field, payload.node_ids.join(","))))
    }
}
