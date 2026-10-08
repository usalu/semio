//! 🔺️ Sparse diff builder for `ChangeSiteLongitude` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, SitePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSiteLongitude, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !(-180.0..=180.0).contains(&payload.new_longitude_deg) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A site longitude of {}° is not admissible.", payload.new_longitude_deg), Vec::<String>::new());
    }
    if base.model.site.longitude_deg == payload.new_longitude_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The site longitude is already {}°.", payload.new_longitude_deg));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { site: SitePatch { longitude_deg: Some(payload.new_longitude_deg), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
