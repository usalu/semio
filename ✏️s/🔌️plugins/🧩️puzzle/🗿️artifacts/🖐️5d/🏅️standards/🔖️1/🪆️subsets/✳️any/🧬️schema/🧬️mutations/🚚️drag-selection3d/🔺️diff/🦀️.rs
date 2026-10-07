//! 🔺️ Sparse diff builder for `DragSelection3d` — every unlocked addressed part's world origin and target volume's
//! origin moves by the payload offset, read off the BASE origin, so the leaf replays on any base. A part's board pin
//! tracks the same ground-plane motion through [`PUZZLE5D_FLAT_TO_WORLD`] (board y points the other way), so the two
//! poses of a part never drift apart.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_diff,PUZZLE5D_FLAT_TO_WORLD};

use crate::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dSnapshot, Puzzle5dTargetVolume};

//#region 🔖️Diff
pub fn diff(payload: &super::DragSelection3d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !payload.offset.iter().all(|value| value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.clone());
    }
    let [dx, dy, dz] = payload.offset;
    let moved = |origin: [f64; 3]| [origin[0] + dx, origin[1] + dy, origin[2] + dz];
    let volume = |entry: &Puzzle5dTargetVolume| Puzzle5dTargetVolume { origin: moved(entry.origin), ..entry.clone() };
    let part = |entry: &Puzzle5dPart| Puzzle5dPart {
        part_2d: Puzzle5dPart2d { x: entry.part_2d.x + dx / PUZZLE5D_FLAT_TO_WORLD, y: entry.part_2d.y - dy / PUZZLE5D_FLAT_TO_WORLD, ..entry.part_2d.clone() },
        part_3d: Puzzle5dPart3d { origin: moved(entry.part_3d.origin), ..entry.part_3d.clone() },
        ..entry.clone()
    };
    puzzle5d_selection_diff(base, &payload.targets, payload.offset == [0.0; 3], part, Some(&volume))
}
//#endregion 🔖️Diff
