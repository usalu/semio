//! ↩️ Inverse for `DeleteEquipmentGain` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteEquipmentGain, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteEquipmentGain> + From<crate::CreateEquipmentGain>,
{
    Ok((|| {
    let Some(index) = base.model.equipment.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.equipment[index];
    vec![vocabulary::create_equipment_gain(index as u32, existing.id, existing.zone_id, existing.schedule_id, existing.watts_per_area, existing.radiant_fraction, existing.latent_fraction)]

    })())
}
//#endregion 🔖️Inverse
