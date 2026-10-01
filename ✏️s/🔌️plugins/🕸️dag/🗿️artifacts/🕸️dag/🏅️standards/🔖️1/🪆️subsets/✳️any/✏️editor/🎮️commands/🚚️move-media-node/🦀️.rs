//! 🕸️ 🕸️ DAG play app commands command — `move-media-node`: one node dropped at a canvas position, committed through the
//! node-drag machine as ONE relative `move-nodes` leaf.

use crate::editor::dag::commands::node_graph_edit::dag_node_drag_emit;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::mutations::move_nodes;
use crate::op::DagMutation;
use crate::DagSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

/// 🪪️ The verb a `moveMediaNode` tool transaction is scoped by.
pub const MOVE_MEDIA_NODE_VERB: &str = "moveMediaNode";

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "move-media-node")]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

/// ✋️ Drops `node_id` at `(x, y)`: the offset from its BASE position is the leaf, so editing the move in history replays it
/// on whatever base it lands on. A node the graph lacks is refused by name; a drop where it already sits is no edit.
pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let node = doc.snapshot.nodes().into_iter().find(|node| node.id == payload.node_id).ok_or_else(|| Fault::from(format!("moveMediaNode refusal: the graph has no node {:?}", payload.node_id)))?;
    let (dx, dy) = (payload.x - node.x, payload.y - node.y);
    if !dx.is_finite() || !dy.is_finite() {
        return Err(Fault::from("moveMediaNode refusal: the dropped position must yield a finite offset".to_owned()));
    }
    if (dx, dy) == (0.0, 0.0) {
        return Ok(Emit::default());
    }
    Ok(dag_node_drag_emit(doc, MOVE_MEDIA_NODE_VERB, MOVE_MEDIA_NODE_VERB, vec![move_nodes(vec![payload.node_id.clone()], dx, dy)]))
}
