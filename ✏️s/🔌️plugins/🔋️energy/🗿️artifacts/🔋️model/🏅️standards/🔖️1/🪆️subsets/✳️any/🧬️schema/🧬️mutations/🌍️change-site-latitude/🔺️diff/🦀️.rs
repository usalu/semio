//! 🔺️ Sparse diff builder for `ChangeSiteLatitude` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSiteLatitude, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !(-90.0..=90.0).contains(&payload.new_latitude_deg) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A site latitude of {}° is not admissible.", payload.new_latitude_deg), Vec::<String>::new());
    }
    if base.model.site.latitude_deg == payload.new_latitude_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The site latitude is already {}°.", payload.new_latitude_deg));
    }
    let mut model = base.model.clone();
    model.site.latitude_deg = payload.new_latitude_deg;
    protocol::MutationOutcome::new(crate::standards::v1::subsets::any::schema::diff::diff_from_model(model))
}
//#endregion 🔖️Diff
