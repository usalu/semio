//! 🔺️ Sparse diff builder for `CreateTargetRegion` — a real append-only insert (never a
//! whole-snapshot capture). Fatal when the id already exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dTargetRegionsDelta};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateTargetRegion, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if base.target_regions.iter().any(|entry| entry.id == payload.target_region.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "target region"), vec![payload.target_region.id.clone()]);
    }
    let mut delta = Puzzle2dTargetRegionsDelta { added: vec![payload.target_region.clone()], ..Default::default() };
    if let Some(index) = payload.index {
        let mut order: Vec<String> = base.target_regions.iter().map(|entry| entry.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, payload.target_region.id.clone());
        delta.reordered = Some(order);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff { target_regions: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
