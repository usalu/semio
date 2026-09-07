//! 🔺️ Sparse diff builder for `CreateSolarThermalSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateSolarThermalSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.solar_thermal_systems.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Solar thermal system {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.solar_thermal_systems.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} solar_thermal_systems.", payload.index, base.model.solar_thermal_systems.len()), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.solar_thermal_systems.insert(payload.index as usize, crate::model::SolarThermalConfig { id: payload.id, collector_area_m2: payload.collector_area_m2, efficiency: payload.efficiency, storage_volume_m3: payload.storage_volume_m3, tilt_deg: payload.tilt_deg, azimuth_deg: payload.azimuth_deg });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
