//! 🔺️ Sparse diff builder for `RemoveNodeHandle` — removes one handle from the owner node's `🐙️handles` and
//! severs any edge referencing the removed handle.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta, Puzzle2dHandlesDelta, Puzzle2dNodePatch, Puzzle2dNodesDelta};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveNodeHandle, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node-handle", payload.node_id), vec![payload.node_id.to_string_owner()]);
    };
    if !node.handles.iter().any(|handle| handle.id == payload.handle_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Handle \"{}\" not found on node \"{}\".", payload.handle_id, payload.node_id), vec![payload.handle_id.to_string_owner()]);
    }
    let severed: Vec<semio_framework_value::paged::PagedUtf8<{ usize::MAX }>> = base.edges.iter().filter(|edge| edge.source == payload.handle_id || edge.target == payload.handle_id).map(|edge| edge.id.clone()).collect();
    let patch = Puzzle2dNodePatch { handles: Some(Puzzle2dHandlesDelta::removing(vec![payload.handle_id.clone()])), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta::patching(payload.node_id.clone(), patch)),
        edges: (!severed.is_empty()).then(|| Puzzle2dEdgesDelta::removing(severed)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
