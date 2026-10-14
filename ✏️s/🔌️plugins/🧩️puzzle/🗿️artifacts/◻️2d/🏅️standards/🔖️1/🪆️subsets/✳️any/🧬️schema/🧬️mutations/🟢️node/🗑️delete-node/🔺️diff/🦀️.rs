//! 🔺️ Sparse diff builder for `DeleteNode` — a real cascade-aware removal (node + any edge that
//! touches one of its handles), never a whole-snapshot capture.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta, Puzzle2dNodesDelta};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteNode, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some((at, node)) = base.nodes.iter().enumerate().find(|(_, entry)| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node", payload.id), vec![payload.id.to_string_owner()]);
    };
    let handle_ids: Vec<&semio_framework_value::paged::PagedUtf8<{ usize::MAX }>> = node.handles.iter().map(|handle| &handle.id).collect();
    let severed: Vec<(semio_framework_value::paged::PagedUtf8<{ usize::MAX }>, usize)> = base.edges.iter().enumerate().filter(|(_, edge)| handle_ids.contains(&&edge.source) || handle_ids.contains(&&edge.target)).map(|(index, edge)| (edge.id.clone(), index)).collect();
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta::removal_by_id(payload.id.clone(), at)),
        edges: if severed.is_empty() { None } else { Some(Puzzle2dEdgesDelta::removals_by_id(severed)) },
        ..Default::default()
    })
}
//#endregion 🔖️Diff
