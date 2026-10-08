//! 🔺️ Sparse diff builder for `RotateSelection` — every unlocked addressed node turns about the pivot (its
//! position and each of its handle angles), read off the BASE, so the leaf replays on any base. Target
//! regions are axis-aligned rectangles and are skipped.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle2dDiff, Puzzle2dHandlePatch, Puzzle2dHandlePatchEntry, Puzzle2dHandlesDelta, Puzzle2dNodePatch, Puzzle2dNodePatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_rotated, puzzle2d_selection, puzzle2d_selection_outcome};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RotateSelection, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.angle.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a rotation pivot and angle must be finite", payload.targets.iter().map(|id| id.to_string_owner()).collect::<Vec<_>>());
    }
    let selection = match puzzle2d_selection(base, &payload.targets, false) {
        Ok(selection) => selection,
        Err(outcome) => return outcome,
    };
    let (pivot, radians) = ((payload.pivot_x, payload.pivot_y), payload.angle);
    let (sin, cos) = radians.sin_cos();
    let nodes = selection
        .nodes
        .iter()
        .map(|node| {
            let (x, y) = puzzle2d_rotated((node.x, node.y), pivot, sin, cos);
            let turned: Vec<Puzzle2dHandlePatchEntry> = node
                .handles
                .iter()
                .map(|handle| Puzzle2dHandlePatchEntry { id: handle.id.clone(), patch: Puzzle2dHandlePatch { angle: Some(handle.angle + radians).filter(|angle| *angle != handle.angle), ..Default::default() } })
                .filter(|entry| !entry.patch.is_empty())
                .collect();
            let patch = Puzzle2dNodePatch {
                x: Some(x).filter(|x| *x != node.x),
                y: Some(y).filter(|y| *y != node.y),
                handles: (!turned.is_empty()).then(|| Puzzle2dHandlesDelta { patched: turned, ..Default::default() }),
                ..Default::default()
            };
            Puzzle2dNodePatchEntry { id: node.id.clone(), patch }
        })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle2d_selection_outcome(selection, &payload.targets, nodes, Vec::new())
}
//#endregion 🔖️Diff
