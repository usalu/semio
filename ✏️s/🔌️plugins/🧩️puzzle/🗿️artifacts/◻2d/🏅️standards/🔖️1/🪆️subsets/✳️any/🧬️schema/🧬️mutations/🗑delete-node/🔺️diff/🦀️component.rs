//! 🔺️ Sparse diff builder for `DeleteNode` — a real cascade-aware removal (node + any edge that
//! touches one of its handles), never a whole-snapshot capture.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta, Puzzle2dNodesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteNode, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node", payload.id), vec![payload.id.clone()]);
    };
    let handle_ids: Vec<&str> = node.handles.iter().map(|handle| handle.id.as_str()).collect();
    let severed: Vec<String> = base
        .edges
        .iter()
        .filter(|edge| handle_ids.contains(&edge.source.as_str()) || handle_ids.contains(&edge.target.as_str()))
        .map(|edge| edge.id.clone())
        .collect();
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta { removed: vec![payload.id.clone()], ..Default::default() }),
        edges: if severed.is_empty() { None } else { Some(Puzzle2dEdgesDelta { removed: severed, ..Default::default() }) },
        ..Default::default()
    })
}
//#endregion 🔖️Diff
