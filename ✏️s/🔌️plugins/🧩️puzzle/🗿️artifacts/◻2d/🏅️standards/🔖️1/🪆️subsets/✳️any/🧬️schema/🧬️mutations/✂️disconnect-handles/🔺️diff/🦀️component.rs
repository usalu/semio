//! 🔺️ Sparse diff builder for `DisconnectHandles` — a real removal, never a whole-snapshot capture.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DisconnectHandles, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if !base.edges.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handles", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff { edges: Some(Puzzle2dEdgesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
