//! 🔺️ Sparse diff builder for `ChangeSiteTimeZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSiteTimeZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !(-12.0..=14.0).contains(&payload.new_time_zone_hours) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A site time zone of {} h is not admissible.", payload.new_time_zone_hours), Vec::<String>::new());
    }
    if base.model.site.time_zone_hours == payload.new_time_zone_hours {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The site time zone is already {} h.", payload.new_time_zone_hours));
    }
    let mut model = base.model.clone();
    model.site.time_zone_hours = payload.new_time_zone_hours;
    protocol::MutationOutcome::new(crate::standards::v1::subsets::any::schema::diff::diff_from_model(model))
}
//#endregion 🔖️Diff
