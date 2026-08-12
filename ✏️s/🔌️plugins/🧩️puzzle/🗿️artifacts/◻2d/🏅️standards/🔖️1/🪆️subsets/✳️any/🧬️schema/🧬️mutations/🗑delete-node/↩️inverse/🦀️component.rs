//! ↩️ Inverse for `DeleteNode` — reconstructs a `create-node` of the captured BASE node, then
//! re-`connect-handles`es every edge BASE shows touching one of its handles (severed cascade).
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteNode, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.nodes.iter().position(|entry| entry.id == payload.id);
    let handle_ids: Vec<&str> = node.handles.iter().map(|handle| handle.id.as_str()).collect();
    let mut mutations = vec![crate::artifacts::puzzle2d::mutations::create_node::mutation::create_node(node.clone(), index)];
    for edge in base.edges.iter().filter(|edge| handle_ids.contains(&edge.source.as_str()) || handle_ids.contains(&edge.target.as_str())) {
        mutations.push(crate::artifacts::puzzle2d::mutations::connect_handles::mutation::connect_handles(
            edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.edge_kind.clone(),
            edge.gap, edge.shift, edge.rise, edge.rotation, edge.turn, edge.tilt, edge.x, edge.y,
            edge.source_tip.clone(), edge.target_tip.clone(),
        ));
    }
    mutations
}
//#endregion 🔖️Inverse
