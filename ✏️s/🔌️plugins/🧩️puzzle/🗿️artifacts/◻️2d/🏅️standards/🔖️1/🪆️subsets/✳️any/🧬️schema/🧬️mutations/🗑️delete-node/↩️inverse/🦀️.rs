//! ↩️ Inverse for `DeleteNode` — reconstructs a `create-node` of the captured BASE node, then
//! re-`connect-handles`es every edge BASE shows touching one of its handles (severed cascade).
//! Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteNode, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.nodes.iter().position(|entry| entry.id == payload.id);
    let handle_ids: Vec<&semio_framework_value::paged::PagedUtf8<{ usize::MAX }>> = node.handles.iter().map(|handle| &handle.id).collect();
    let restored_edges = base.edges.iter().enumerate().filter(|(_, edge)| handle_ids.contains(&&edge.source) || handle_ids.contains(&&edge.target)).flat_map(|(index, edge)| crate::standards::v1::subsets::any::schema::mutations::connect_handles::restore_edge(edge, index));
    let natural: Vec<Puzzle2dMutation> = std::iter::once(crate::standards::v1::subsets::any::schema::mutations::create_node::create_node(node.clone(), index)).chain(restored_edges).collect();
    natural.into_iter().rev().collect()

    })())
}
//#endregion 🔖️Inverse
