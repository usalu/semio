//! 🔺️ Sparse diff builder for `EditTargetRegionLabel` — patches the one addressed target region in place.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dTargetRegionPatch, Puzzle2dTargetRegionPatchEntry, Puzzle2dTargetRegionsDelta};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::EditTargetRegionLabel, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(region) = base.target_regions.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target region", payload.id), vec![payload.id.clone()]);
    };
    let mut next = region.clone();
    next.label = payload.new_label.clone();
    if next == *region {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        target_regions: Some(Puzzle2dTargetRegionsDelta { patched: vec![Puzzle2dTargetRegionPatchEntry { id: payload.id.clone(), patch: Puzzle2dTargetRegionPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
