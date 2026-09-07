//! ↩️ Inverse for `ChangeEquipmentGainWattsPerArea` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeEquipmentGainWattsPerArea, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.watts_per_area != payload.new_watts_per_area && !(!payload.new_watts_per_area.is_finite() || payload.new_watts_per_area < 0.0) => vec![vocabulary::change_equipment_gain_watts_per_area(payload.id, item.watts_per_area)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
