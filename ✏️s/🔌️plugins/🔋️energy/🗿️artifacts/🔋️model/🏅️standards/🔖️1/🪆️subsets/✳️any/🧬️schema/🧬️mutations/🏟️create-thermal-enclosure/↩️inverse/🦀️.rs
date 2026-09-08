//! ↩️ Inverse for `CreateThermalEnclosure` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateThermalEnclosure, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.thermal_enclosures.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.thermal_enclosures.len() || payload.zone_ids.iter().any(|candidate| !base.model.zones.iter().any(|row| row.id == *candidate)) {
        return Vec::new();
    }
    vec![vocabulary::delete_thermal_enclosure(payload.id)]
}
//#endregion 🔖️Inverse
