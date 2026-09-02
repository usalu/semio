//! 🧩️ 🧩️ S Studio app command — `patch-media-nodes`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::workflow::MoveNode;
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-media-nodes")]
pub struct PatchMediaNodes {
    pub node_ids: Vec<String>,
    pub field: String,
    pub axis: Option<String>,
    pub value: String,
}

pub fn handle(payload: &PatchMediaNodes, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let projection = doc.snapshot;
    let numeric = payload.value.parse::<f64>().ok();
    if payload.field == "position" {
        let Some(numeric) = numeric else { return Ok(Emit::default()) };
        let artifact_mutations = payload
            .node_ids
            .iter()
            .filter_map(|node_id| {
                let node = projection.graph.nodes.iter().find(|row| &row.id == node_id)?;
                let x = if payload.axis.as_deref() == Some("x") { numeric } else { node.x };
                let y = if payload.axis.as_deref() == Some("y") { numeric } else { node.y };
                Some(WorkflowMutation::MoveNode(MoveNode { node_id: node_id.clone(), x, y }))
            })
            .collect();
        Ok(Emit::mutations(artifact_mutations))
    } else {
        Ok(Emit::default())
    }
}
