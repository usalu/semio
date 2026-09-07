//! ↩️ Inverse for `CreateBattery` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateBattery, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.battery_storage.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.battery_storage.len() {
        return Vec::new();
    }
    vec![vocabulary::delete_battery(payload.id)]
}
//#endregion 🔖️Inverse
