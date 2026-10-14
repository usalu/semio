//! ↩️ Inverse for `ChangeEquipmentGainLatentFraction` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeEquipmentGainLatentFraction, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeEquipmentGainLatentFraction>,
{
    Ok((|| {
    match base.model.equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.latent_fraction != payload.new_latent_fraction && !(!(0.0..=1.0).contains(&payload.new_latent_fraction)) => vec![vocabulary::change_equipment_gain_latent_fraction(payload.id, item.latent_fraction)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
