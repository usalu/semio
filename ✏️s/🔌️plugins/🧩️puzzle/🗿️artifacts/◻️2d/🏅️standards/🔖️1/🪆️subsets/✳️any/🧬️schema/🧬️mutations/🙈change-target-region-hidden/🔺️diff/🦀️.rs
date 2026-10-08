//! 🔺️ Sparse diff builder for `ChangeTargetRegionHidden` — patches the one addressed target region in place.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle2dDiff, Puzzle2dTargetRegionPatch, Puzzle2dTargetRegionsDelta};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeTargetRegionHidden, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(region) = base.target_regions.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target region", payload.id), vec![payload.id.to_string_owner()]);
    };
    let patch = Puzzle2dTargetRegionPatch {
        hidden: (payload.new_hidden != region.hidden).then_some(payload.new_hidden),
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
