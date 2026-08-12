//! 🔺️ Sparse diff builder for `DeleteNode` — a real cascade-aware removal (node + any edge that
//! touches it), never a whole-snapshot capture.
use crate::artifacts::dag::diff::{DagDiff, DagEdgesDelta, DagNodesDelta};
use crate::artifacts::dag::engine::split_endpoint;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteNode, base: &DagSnapshot) -> DagDiff {
    let severed: Vec<String> = base
        .edges
        .iter()
        .filter(|edge| split_endpoint(&edge.source).0 == payload.id || split_endpoint(&edge.target).0 == payload.id)
        .map(|edge| edge.id.clone())
        .collect();
    DagDiff {
        nodes: Some(DagNodesDelta { removed: vec![payload.id.clone()], ..Default::default() }),
        edges: if severed.is_empty() { None } else { Some(DagEdgesDelta { removed: severed, ..Default::default() }) },
        ..Default::default()
    }
}
//#endregion 🔖️Diff
