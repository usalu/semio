//! 🔺️ Sparse diff builder for `ChangeSiteElevation` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, SitePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSiteElevation, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !(-300.0..8900.0).contains(&payload.new_elevation_m) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A site elevation of {} m is not admissible.", payload.new_elevation_m), Vec::<String>::new());
    }
    if base.model.site.elevation_m == payload.new_elevation_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The site elevation is already {} m.", payload.new_elevation_m));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { site: SitePatch { elevation_m: Some(payload.new_elevation_m), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
