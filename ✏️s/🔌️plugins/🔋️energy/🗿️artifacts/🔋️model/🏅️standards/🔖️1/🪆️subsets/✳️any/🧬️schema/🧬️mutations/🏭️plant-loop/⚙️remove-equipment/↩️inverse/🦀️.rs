//! ↩️ Inverse for `RemovePlantLoopEquipment` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::RemovePlantLoopEquipment, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::RemovePlantLoopEquipment> + From<crate::AddPlantLoopEquipment>,
{
    Ok((|| {
    match base.model.plant_loops.iter().find(|item| item.id == payload.id) {
        Some(existing) if !(!existing.equipment_ids.contains(&payload.equipment_id)) => vec![vocabulary::add_plant_loop_equipment(payload.id, payload.equipment_id, existing.equipment_ids.iter().position(|entry| *entry == payload.equipment_id).map(|index| index as u32))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
