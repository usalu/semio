//! ↩️ Inverse for `RemoveNodeHandle` — reconstructs an `add-node-handle` of the captured BASE
//! handle, then re-`connect-handles`es every edge BASE shows touching it (severed cascade).
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveNodeHandle, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return Vec::new();
    };
    let Some(handle) = node.handles.iter().find(|handle| handle.id == payload.handle_id) else {
        return Vec::new();
    };
    let index = node.handles.iter().position(|h| h.id == payload.handle_id);
    let mut mutations = vec![crate::artifacts::puzzle2d::mutations::add_node_handle::mutation::add_node_handle(payload.node_id.clone(), handle.clone(), index)];
    for edge in base.edges.iter().filter(|edge| edge.source == payload.handle_id || edge.target == payload.handle_id) {
        mutations.push(crate::artifacts::puzzle2d::mutations::connect_handles::mutation::connect_handles(
            edge.id.clone(),
            edge.source.clone(),
            edge.target.clone(),
            edge.edge_kind.clone(),
            edge.gap,
            edge.shift,
            edge.rise,
            edge.rotation,
            edge.turn,
            edge.tilt,
            edge.x,
            edge.y,
            edge.source_tip.clone(),
            edge.target_tip.clone(),
        ));
    }
    mutations
}
//#endregion 🔖️Inverse
