//! 🔺️ Sparse diff builder for `ChangeSolarThermalSystemAzimuth` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSolarThermalSystemAzimuth, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solar thermal system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_azimuth_deg.is_finite() || !(0.0..=360.0).contains(&payload.new_azimuth_deg) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Solar thermal system {}: azimuth must be a compass bearing in [0, 360] degrees, got {}.", payload.id.0, payload.new_azimuth_deg), [payload.id.0.to_string()]);
    }
    if existing.azimuth_deg == payload.new_azimuth_deg {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Solar thermal system {} already carries this azimuth_deg: {}.", payload.id.0, payload.new_azimuth_deg));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.solar_thermal_systems.iter_mut().find(|item| item.id == payload.id) {
        item.azimuth_deg = payload.new_azimuth_deg;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
