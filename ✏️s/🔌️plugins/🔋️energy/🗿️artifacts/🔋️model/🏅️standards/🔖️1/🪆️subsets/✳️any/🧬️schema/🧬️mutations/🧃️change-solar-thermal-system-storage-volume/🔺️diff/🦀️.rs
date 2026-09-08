//! 🔺️ Sparse diff builder for `ChangeSolarThermalSystemStorageVolume` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSolarThermalSystemStorageVolume, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solar thermal system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_storage_volume_m3.is_finite() || payload.new_storage_volume_m3 <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Solar thermal system {}: buffer volume (m³) must be a positive finite value, got {}.", payload.id.0, payload.new_storage_volume_m3), [payload.id.0.to_string()]);
    }
    if existing.storage_volume_m3 == payload.new_storage_volume_m3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Solar thermal system {} already carries this buffer volume (m³): {}.", payload.id.0, payload.new_storage_volume_m3));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.solar_thermal_systems.iter_mut().find(|item| item.id == payload.id) {
        item.storage_volume_m3 = payload.new_storage_volume_m3;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
