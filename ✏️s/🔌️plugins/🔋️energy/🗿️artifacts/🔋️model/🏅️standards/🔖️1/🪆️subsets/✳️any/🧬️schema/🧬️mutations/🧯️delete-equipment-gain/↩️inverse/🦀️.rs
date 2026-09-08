//! ↩️ Inverse for `DeleteEquipmentGain` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteEquipmentGain, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.equipment.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.equipment[index];
    vec![vocabulary::create_equipment_gain(index as u32, existing.id, existing.zone_id, existing.schedule_id, existing.watts_per_area, existing.radiant_fraction, existing.latent_fraction)]
}
//#endregion 🔖️Inverse
