//! 🔺️ Sparse diff builder for `DragSelection` — every unlocked addressed object and target volume moves
//! by the payload offset, read off the BASE origin, so the leaf replays on any base. The attraction graph is
//! re-solved: attracted objects are re-placed from their moved parents, other touched attractions re-derive.
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::puzzle3d_selection_diff;
use crate::{Puzzle3dObject, Puzzle3dSnapshot, Puzzle3dTargetVolume};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DragSelection, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if !payload.offset.iter().all(|value| value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.clone());
    }
    let [dx, dy, dz] = payload.offset;
    let moved = |origin: [f64; 3]| [origin[0] + dx, origin[1] + dy, origin[2] + dz];
    puzzle3d_selection_diff(base, &payload.targets, payload.offset == [0.0; 3], |entry: &Puzzle3dObject| Puzzle3dObject { origin: moved(entry.origin), ..entry.clone() }, |entry: &Puzzle3dTargetVolume| Puzzle3dTargetVolume { origin: moved(entry.origin), ..entry.clone() }, true)
}
//#endregion 🔖️Diff
