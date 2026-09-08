//! ↩️ Inverse for `ChangeEquipmentGainRadiantFraction` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeEquipmentGainRadiantFraction, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.radiant_fraction != payload.new_radiant_fraction && !(!(0.0..=1.0).contains(&payload.new_radiant_fraction)) => vec![vocabulary::change_equipment_gain_radiant_fraction(payload.id, item.radiant_fraction)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
