//! 🔺️ Sparse diff builder for `ChangeSiteNorthAxis` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSiteNorthAxis, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !payload.new_north_axis_deg.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A site north axis of {}° is not admissible.", payload.new_north_axis_deg), Vec::<String>::new());
    }
    if base.model.site.north_axis_deg == payload.new_north_axis_deg {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The site north axis is already {}°.", payload.new_north_axis_deg));
    }
    let mut model = base.model.clone();
    model.site.north_axis_deg = payload.new_north_axis_deg;
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
