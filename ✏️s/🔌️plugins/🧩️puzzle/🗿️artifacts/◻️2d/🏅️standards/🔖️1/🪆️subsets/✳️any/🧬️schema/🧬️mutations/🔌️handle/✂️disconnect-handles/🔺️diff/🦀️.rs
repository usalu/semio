//! 🔺️ Sparse diff builder for `DisconnectHandles` — a real removal, never a whole-snapshot capture.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectHandles, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(at) = base.edges.iter().position(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handles", payload.id), vec![payload.id.to_string_owner()]);
    };
    protocol::MutationOutcome::new(Puzzle2dDiff { edges: Some(Puzzle2dEdgesDelta::removal_by_id(payload.id.clone(), at)), ..Default::default() })
}
//#endregion 🔖️Diff
