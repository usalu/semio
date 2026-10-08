//! 🔺️ Sparse diff builder for `CreateTargetRegion` — a real append-only insert (never a
//! whole-snapshot capture). Fatal when the id already exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dTargetRegionsDelta};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_region_invariant;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateTargetRegion, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_region_invariant(&payload.target_region) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.target_region.id.to_string_owner()]);
    }
    if base.target_regions.iter().any(|entry| entry.id == payload.target_region.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "target region"), vec![payload.target_region.id.to_string_owner()]);
    }
    let index = payload.index.map_or(base.target_regions.len(), |index| index.min(base.target_regions.len()));
    let delta = Puzzle2dTargetRegionsDelta::insertion(index, payload.target_region.clone());
    protocol::MutationOutcome::new(Puzzle2dDiff { target_regions: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
