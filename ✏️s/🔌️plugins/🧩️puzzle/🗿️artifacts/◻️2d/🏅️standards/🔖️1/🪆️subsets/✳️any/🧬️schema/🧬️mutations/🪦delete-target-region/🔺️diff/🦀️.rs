//! 🔺️ Sparse diff builder for `DeleteTargetRegion` — a real removal, never a whole-snapshot
//! capture. A region owns no dependents, so nothing cascades.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dTargetRegionsDelta};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteTargetRegion, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(at) = base.target_regions.iter().position(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target region", payload.id), vec![payload.id.to_string_owner()]);
    };
    protocol::MutationOutcome::new(Puzzle2dDiff { target_regions: Some(Puzzle2dTargetRegionsDelta::removal_by_id(payload.id.clone(), at)), ..Default::default() })
}
//#endregion 🔖️Diff
