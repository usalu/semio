//! 🔺️ Sparse diff builder for `DisconnectGrips` — a real removal, never a whole-snapshot capture.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dFastenersDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectGrips, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !base.fasteners.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grips", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff { fasteners: Some(Puzzle5dFastenersDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
