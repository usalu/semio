//! handle, then re-`connect-handles`es every edge BASE shows touching it (severed cascade).
//! ↩️ Inverse for `RemoveNodeHandle` — reconstructs an `add-node-handle` of the captured BASE
//! Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::ChangeEdgeVisible;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::ChangeEdgeLocked;
use crate::standards::v1::subsets::any::schema::mutations::connect_handles::ConnectHandles;
use crate::standards::v1::subsets::any::schema::mutations::add_node_handle::AddNodeHandle;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<RemoveNodeHandle> + From<AddNodeHandle> + From<ConnectHandles> + From<ChangeEdgeVisible> + From<ChangeEdgeLocked>>(payload: &super::RemoveNodeHandle, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return Vec::new();
    };
    let Some(handle) = node.handles.iter().find(|handle| handle.id == payload.handle_id) else {
        return Vec::new();
    };
    let index = node.handles.iter().position(|h| h.id == payload.handle_id);
    let restored_edges = base.edges.iter().enumerate().filter(|(_, edge)| edge.source == payload.handle_id || edge.target == payload.handle_id).flat_map(|(index, edge)| crate::standards::v1::subsets::any::schema::mutations::connect_handles::restore_edge(edge, index));
    let natural: Vec<Op> = std::iter::once(crate::standards::v1::subsets::any::schema::mutations::add_node_handle::add_node_handle(payload.node_id.clone(), handle.clone(), index)).chain(restored_edges).collect();
    natural.into_iter().rev().collect()

    })())
}
//#endregion 🔖️Inverse
