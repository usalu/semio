//! ↩️ Inverse for `ChangeEquipmentGainLatentFraction` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeEquipmentGainLatentFraction, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.latent_fraction != payload.new_latent_fraction && !(!(0.0..=1.0).contains(&payload.new_latent_fraction)) => vec![vocabulary::change_equipment_gain_latent_fraction(payload.id, item.latent_fraction)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
