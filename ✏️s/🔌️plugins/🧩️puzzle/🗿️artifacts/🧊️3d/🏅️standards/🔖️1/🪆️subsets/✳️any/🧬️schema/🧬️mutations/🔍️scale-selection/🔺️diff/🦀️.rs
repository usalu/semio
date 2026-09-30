//! 🔺️ Sparse diff builder for `ScaleSelection` — every unlocked addressed object and target volume keeps
//! its origin and multiplies its BASE scale (uniform broadcast, absent reads as one) by the payload's
//! per-axis factors, so the leaf replays on any base.
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_scaled, puzzle3d_selection_diff};
use crate::{Puzzle3dObject, Puzzle3dSnapshot, Puzzle3dTargetVolume};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ScaleSelection, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if !payload.factors.iter().all(|value| value.is_finite() && *value > 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "every scale factor must be a finite number greater than 0", payload.targets.clone());
    }
    let factors = payload.factors;
    puzzle3d_selection_diff(base, &payload.targets, factors == [1.0; 3], |entry: &Puzzle3dObject| Puzzle3dObject { scale: Some(puzzle3d_scaled(entry.scale, factors)), ..entry.clone() }, |entry: &Puzzle3dTargetVolume| Puzzle3dTargetVolume { scale: Some(puzzle3d_scaled(entry.scale, factors)), ..entry.clone() })
}
//#endregion 🔖️Diff
