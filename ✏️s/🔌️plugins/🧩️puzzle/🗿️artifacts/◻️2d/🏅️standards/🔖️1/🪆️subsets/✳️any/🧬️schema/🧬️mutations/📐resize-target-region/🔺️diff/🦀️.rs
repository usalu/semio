//! 🔺️ Sparse diff builder for `ResizeTargetRegion` — patches the one addressed target region in place.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dTargetRegionPatch, Puzzle2dTargetRegionPatchEntry, Puzzle2dTargetRegionsDelta};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_finite;

//#region 🔖️Diff
pub fn diff(payload: &super::ResizeTargetRegion, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_finite(&[("newWidth", payload.new_width), ("newHeight", payload.new_height)]) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.clone()]);
    }
    let Some(region) = base.target_regions.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target region", payload.id), vec![payload.id.clone()]);
    };
    let mut next = region.clone();
    next.width = payload.new_width;
    next.height = payload.new_height;
    if next == *region {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        target_regions: Some(Puzzle2dTargetRegionsDelta { patched: vec![Puzzle2dTargetRegionPatchEntry { id: payload.id.clone(), patch: Puzzle2dTargetRegionPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
