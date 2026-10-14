//! 🔺️ Sparse diff builder for `ChangeSiteNorthAxis` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, SitePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSiteNorthAxis, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !payload.new_north_axis_deg.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A site north axis of {}° is not admissible.", payload.new_north_axis_deg), Vec::<String>::new());
    }
    if base.model.site.north_axis_deg == payload.new_north_axis_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The site north axis is already {}°.", payload.new_north_axis_deg));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { site: SitePatch { north_axis_deg: Some(payload.new_north_axis_deg), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
