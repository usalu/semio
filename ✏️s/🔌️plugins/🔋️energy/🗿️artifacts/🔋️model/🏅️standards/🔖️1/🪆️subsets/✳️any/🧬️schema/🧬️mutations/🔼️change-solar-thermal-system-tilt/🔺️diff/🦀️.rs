//! 🔺️ Sparse diff builder for `ChangeSolarThermalSystemTilt` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSolarThermalSystemTilt, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solar thermal system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_tilt_deg.is_finite() || !(0.0..=90.0).contains(&payload.new_tilt_deg) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Solar thermal system {}: tilt must be a zenith angle in [0, 90] degrees, got {}.", payload.id.0, payload.new_tilt_deg), [payload.id.0.to_string()]);
    }
    if existing.tilt_deg == payload.new_tilt_deg {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Solar thermal system {} already carries this tilt_deg: {}.", payload.id.0, payload.new_tilt_deg));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.solar_thermal_systems.iter_mut().find(|item| item.id == payload.id) {
        item.tilt_deg = payload.new_tilt_deg;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
