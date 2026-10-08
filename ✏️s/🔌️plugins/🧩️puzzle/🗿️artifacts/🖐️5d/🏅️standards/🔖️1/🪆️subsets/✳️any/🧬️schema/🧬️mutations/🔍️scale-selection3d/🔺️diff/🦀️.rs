//! 🔺️ Sparse diff builder for `ScaleSelection3d` — every unlocked addressed part and target volume keeps its origin
//! and multiplies its BASE scale (uniform broadcast, absent reads as one) by the payload's per-axis factors, so the leaf
//! replays on any base.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle5dDiff, Puzzle5dPart3dPatch, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dTargetVolumePatch, Puzzle5dTargetVolumePatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_scaled, puzzle5d_selection, puzzle5d_selection_outcome};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ScaleSelection3d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !payload.factors.iter().all(|value| value.is_finite() && *value > 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "every scale factor must be a finite number greater than 0", payload.targets.clone());
    }
    let selection = match puzzle5d_selection(base, &payload.targets, true) {
        Ok(selection) => selection,
        Err(outcome) => return outcome,
    };
    let factors = payload.factors;
    let identity = factors == [1.0; 3];
    let parts = selection
        .parts
        .iter()
        .filter(|_| !identity)
        .map(|part| {
            let world = Puzzle5dPart3dPatch { scale: Some(Some(puzzle5d_scaled(part.part_3d.scale, factors))).filter(|scale| *scale != part.part_3d.scale), ..Default::default() };
            Puzzle5dPartPatchEntry { id: part.id.clone(), patch: Puzzle5dPartPatch { part_3d: Some(world).filter(|world| !world.is_empty()), ..Default::default() } }
        })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    let volumes = selection
        .volumes
        .iter()
        .filter(|_| !identity)
        .map(|volume| Puzzle5dTargetVolumePatchEntry { id: volume.id.clone(), patch: Puzzle5dTargetVolumePatch { scale: Some(Some(puzzle5d_scaled(volume.scale, factors))).filter(|scale| *scale != volume.scale), ..Default::default() } })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle5d_selection_outcome(selection, &payload.targets, parts, volumes)
}
//#endregion 🔖️Diff
