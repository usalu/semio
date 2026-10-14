//! 🔺️ Sparse diff builder for `ChangeSolarThermalSystemTilt` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SolarThermalConfigPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSolarThermalSystemTilt, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solar thermal system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_tilt_deg.is_finite() || !(0.0..=90.0).contains(&payload.new_tilt_deg) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Solar thermal system {}: tilt must be a zenith angle in [0, 90] degrees, got {}.", payload.id.0, payload.new_tilt_deg), [payload.id.0.to_string()]);
    }
    if existing.tilt_deg == payload.new_tilt_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Solar thermal system {} already carries this tilt_deg: {}.", payload.id.0, payload.new_tilt_deg));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { solar_thermal_systems: Rows::modifying(SolarThermalConfigPatch { tilt_deg: Some(payload.new_tilt_deg), ..SolarThermalConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
