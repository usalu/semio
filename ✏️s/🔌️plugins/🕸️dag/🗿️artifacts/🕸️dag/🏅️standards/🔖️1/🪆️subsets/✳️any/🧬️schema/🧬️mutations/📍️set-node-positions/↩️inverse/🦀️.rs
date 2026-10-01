//! ↩️ Inverse for `SetNodePositions` — ONE `set-node-positions` row with every placed node's BASE position; a payload that
//! moves nothing has no inverse.
use crate::mutations::{dag_targets_invariant, set_node_positions, DagMutation, DagNodePosition};
use crate::{dag_working_scene, DagSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::SetNodePositions, base: &DagSnapshot) -> Vec<DagMutation> {
    if dag_targets_invariant(&payload.ids()).is_err() || payload.positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
        return Vec::new();
    }
    let scene = dag_working_scene(base);
    let placed: Vec<(DagNodePosition, bool)> = payload
        .positions
        .iter()
        .filter_map(|position| scene.nodes.iter().find(|node| node.id == position.id).map(|node| (DagNodePosition { id: node.id.clone(), x: node.x, y: node.y }, (node.x, node.y) != (position.x, position.y))))
        .collect();
    if !placed.iter().any(|(_, moves)| *moves) {
        return Vec::new();
    }
    vec![set_node_positions(placed.into_iter().map(|(position, _)| position).collect())]
}
//#endregion 🔖️Inverse
