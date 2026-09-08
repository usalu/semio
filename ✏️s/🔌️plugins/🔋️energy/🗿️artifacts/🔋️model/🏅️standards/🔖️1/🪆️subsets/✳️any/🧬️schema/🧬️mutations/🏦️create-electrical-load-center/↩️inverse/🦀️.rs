//! ↩️ Inverse for `CreateElectricalLoadCenter` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateElectricalLoadCenter, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.electrical_load_centers.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.electrical_load_centers.len() || payload.pv_ids.iter().any(|candidate| !base.model.pv_systems.iter().any(|row| row.id == *candidate)) || payload.battery_ids.iter().any(|candidate| !base.model.battery_storage.iter().any(|row| row.id == *candidate)) {
        return Vec::new();
    }
    vec![vocabulary::delete_electrical_load_center(payload.id)]
}
//#endregion 🔖️Inverse
