//! ↩️ Inverse for `RenameElectricalLoadCenter` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RenameElectricalLoadCenter, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) {
        Some(item) if item.name != payload.new_name && !payload.new_name.trim().is_empty() && !base.model.electrical_load_centers.iter().any(|other| other.id != payload.id && other.name == payload.new_name) => vec![vocabulary::rename_electrical_load_center(payload.id, item.name.clone())],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
