//! ↩️ Inverse for `MoveNodes` — ONE absolute `set-node-positions` row putting every moved node back at its BASE position
//! (never a negated offset), so a multi-node drag stays one point-invertible row.
use crate::mutations::{dag_targets_invariant, set_node_positions, DagMutation, DagNodePosition};
use crate::{dag_working_scene, DagSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::MoveNodes, base: &DagSnapshot) -> Vec<DagMutation> {
    if dag_targets_invariant(&payload.ids).is_err() || !payload.dx.is_finite() || !payload.dy.is_finite() || (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    let scene = dag_working_scene(base);
    let positions: Vec<DagNodePosition> = payload.ids.iter().filter_map(|id| scene.nodes.iter().find(|node| &node.id == id).map(|node| DagNodePosition { id: id.clone(), x: node.x, y: node.y })).collect();
    if positions.is_empty() {
        return Vec::new();
    }
    vec![set_node_positions(positions)]
}
//#endregion 🔖️Inverse
