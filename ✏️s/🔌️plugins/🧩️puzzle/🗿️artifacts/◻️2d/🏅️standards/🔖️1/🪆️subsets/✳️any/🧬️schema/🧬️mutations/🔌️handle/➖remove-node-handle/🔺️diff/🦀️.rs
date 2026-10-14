//! 🔺️ Sparse diff builder for `RemoveNodeHandle` — removes one handle from the owner node's `🐙️handles` and
//! severs any edge referencing the removed handle.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta, Puzzle2dHandlesDelta, Puzzle2dNodePatch, Puzzle2dNodesDelta};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveNodeHandle, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node-handle", payload.node_id), vec![payload.node_id.to_string_owner()]);
    };
    let Some(handle_at) = node.handles.iter().position(|handle| handle.id == payload.handle_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Handle \"{}\" not found on node \"{}\".", payload.handle_id, payload.node_id), vec![payload.handle_id.to_string_owner()]);
    };
    let severed: Vec<(semio_framework_value::paged::PagedUtf8<{ usize::MAX }>, usize)> = base.edges.iter().enumerate().filter(|(_, edge)| edge.source == payload.handle_id || edge.target == payload.handle_id).map(|(index, edge)| (edge.id.clone(), index)).collect();
    let patch = Puzzle2dNodePatch { handles: Some(Puzzle2dHandlesDelta::removal_by_id(payload.handle_id.clone(), handle_at)), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta::modification(payload.node_id.clone(), patch)),
        edges: (!severed.is_empty()).then(|| Puzzle2dEdgesDelta::removals_by_id(severed)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
