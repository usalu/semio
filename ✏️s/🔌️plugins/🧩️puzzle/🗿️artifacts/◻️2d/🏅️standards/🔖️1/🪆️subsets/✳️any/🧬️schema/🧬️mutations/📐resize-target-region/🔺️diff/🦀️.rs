//! 🔺️ Sparse diff builder for `ResizeTargetRegion` — patches the one addressed target region in place.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle2dDiff, Puzzle2dTargetRegionPatch, Puzzle2dTargetRegionsDelta};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_finite;

//#region 🔖️Diff
pub fn diff(payload: &super::ResizeTargetRegion, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_finite(&[("newWidth", payload.new_width), ("newHeight", payload.new_height)]) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.to_string_owner()]);
    }
    let Some(region) = base.target_regions.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target region", payload.id), vec![payload.id.to_string_owner()]);
    };
    let patch = Puzzle2dTargetRegionPatch {
        width: (payload.new_width != region.width).then_some(payload.new_width),
        height: (payload.new_height != region.height).then_some(payload.new_height),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.to_string_owner()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        target_regions: Some(Puzzle2dTargetRegionsDelta::patching(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
