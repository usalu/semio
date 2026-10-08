//! 🔺️ Sparse diff builder for `DisconnectGrips` — a real removal, never a whole-snapshot capture.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dFastenersDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectGrips, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(at) = base.fasteners.iter().position(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grips", payload.id), vec![payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Puzzle5dDiff { fasteners: Some(Puzzle5dFastenersDelta::removal_by_id(payload.id.clone(), at)), ..Default::default() })
}
//#endregion 🔖️Diff
