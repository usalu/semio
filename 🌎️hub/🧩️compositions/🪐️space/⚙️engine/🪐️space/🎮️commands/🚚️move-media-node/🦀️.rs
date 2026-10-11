//! 🧩️ 🧩️ S Studio app command — `move-media-node`: one workflow node dropped at a canvas position, committed through the
//! node-drag machine as ONE relative `move-nodes` leaf.

use crate::engine::space::commands::node_graph_edit::space_node_drag_emit;
use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_artifact_workflow_workflow::MoveNodes;
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

/// 🪪️ The verb a `moveMediaNode` tool transaction is scoped by.
pub const MOVE_MEDIA_NODE_VERB: &str = "moveMediaNode";

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "move-media-node")]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

/// ✋️ Drops `node_id` at `(x, y)`: the offset from its BASE position is the leaf, so editing the move in history replays it
/// on whatever base it lands on. A node the workflow lacks is refused by name; a drop where it already sits is no edit.
pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let node = doc.snapshot.graph.nodes.iter().find(|node| node.id == payload.node_id).ok_or_else(|| Fault::from(format!("moveMediaNode refusal: the workflow has no node {:?}", payload.node_id)))?;
    let (dx, dy) = (payload.x - node.x, payload.y - node.y);
    if !dx.is_finite() || !dy.is_finite() {
        return Err(Fault::from("moveMediaNode refusal: the dropped position must yield a finite offset".to_owned()));
    }
    if (dx, dy) == (0.0, 0.0) {
        return Ok(Emit::default());
    }
    Ok(space_node_drag_emit(doc, MOVE_MEDIA_NODE_VERB, MOVE_MEDIA_NODE_VERB, vec![WorkflowMutation::MoveNodes(MoveNodes { node_ids: vec![payload.node_id.clone()], dx, dy })]))
}
