//! ↩️ Inverse for `RemoveNodeHandle` — reconstructs an `add-node-handle` of the captured BASE
//! handle, then re-`connect-handles`es every edge BASE shows touching it (severed cascade).
//! Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveNodeHandle, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return Vec::new();
    };
    let Some(handle) = node.handles.iter().find(|handle| handle.id == payload.handle_id) else {
        return Vec::new();
    };
    let index = node.handles.iter().position(|h| h.id == payload.handle_id);
    let mut mutations = vec![crate::standards::v1::subsets::any::schema::mutations::add_node_handle::add_node_handle(payload.node_id.clone(), handle.clone(), index)];
    for (index, edge) in base.edges.iter().enumerate().filter(|(_, edge)| edge.source == payload.handle_id || edge.target == payload.handle_id) {
        crate::standards::v1::subsets::any::schema::mutations::connect_handles::restore_edge(edge, index, &mut mutations);
    }
    mutations

    })())
}
//#endregion 🔖️Inverse
