//! 🔺️ Sparse diff builder for `ScaleSelection3d` — every unlocked addressed part and target volume keeps its origin
//! and multiplies its BASE scale (uniform broadcast, absent reads as one) by the payload's per-axis factors, so the leaf
//! replays on any base.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_scaled, puzzle5d_selection_diff};
use crate::{Puzzle5dPart, Puzzle5dPart3d, Puzzle5dSnapshot, Puzzle5dTargetVolume};

//#region 🔖️Diff
pub fn diff(payload: &super::ScaleSelection3d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !payload.factors.iter().all(|value| value.is_finite() && *value > 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "every scale factor must be a finite number greater than 0", payload.targets.clone());
    }
    let factors = payload.factors;
    let volume = |entry: &Puzzle5dTargetVolume| Puzzle5dTargetVolume { scale: Some(puzzle5d_scaled(entry.scale, factors)), ..entry.clone() };
    puzzle5d_selection_diff(base, &payload.targets, factors == [1.0; 3], |entry: &Puzzle5dPart| Puzzle5dPart { part_3d: Puzzle5dPart3d { scale: Some(puzzle5d_scaled(entry.part_3d.scale, factors)), ..entry.part_3d.clone() }, ..entry.clone() }, Some(&volume))
}
//#endregion 🔖️Diff
