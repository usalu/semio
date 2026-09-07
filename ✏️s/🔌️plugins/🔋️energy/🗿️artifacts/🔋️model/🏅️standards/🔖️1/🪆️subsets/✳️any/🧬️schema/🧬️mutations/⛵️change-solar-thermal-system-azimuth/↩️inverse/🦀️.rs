//! ↩️ Inverse for `ChangeSolarThermalSystemAzimuth` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSolarThermalSystemAzimuth, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.azimuth_deg != payload.new_azimuth_deg && !(!payload.new_azimuth_deg.is_finite() || !(0.0..=360.0).contains(&payload.new_azimuth_deg)) => vec![vocabulary::change_solar_thermal_system_azimuth(payload.id, item.azimuth_deg)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
