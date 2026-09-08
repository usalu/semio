//! 🔺️ Sparse diff builder for `UpdateSite` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateSite, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let site = crate::model::Site { latitude_deg: payload.latitude_deg, longitude_deg: payload.longitude_deg, elevation_m: payload.elevation_m, time_zone_hours: payload.time_zone_hours, north_axis_deg: payload.north_axis_deg };
    if !(-90.0..=90.0).contains(&site.latitude_deg) || !(-180.0..=180.0).contains(&site.longitude_deg) || !(-12.0..=14.0).contains(&site.time_zone_hours) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Site {:.4}, {:.4} at UTC{:+} is outside the representable earth.", site.latitude_deg, site.longitude_deg, site.time_zone_hours), Vec::<String>::new());
    }
    if base.model.site == site {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The site facet already has this value.");
    }
    let mut model = base.model.clone();
    model.site = site;
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
