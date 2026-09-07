//! ↩️ Inverse for `ChangeSolarThermalSystemStorageVolume` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSolarThermalSystemStorageVolume, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.storage_volume_m3 != payload.new_storage_volume_m3 && !(!payload.new_storage_volume_m3.is_finite() || payload.new_storage_volume_m3 <= 0.0) => vec![vocabulary::change_solar_thermal_system_storage_volume(payload.id, item.storage_volume_m3)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
