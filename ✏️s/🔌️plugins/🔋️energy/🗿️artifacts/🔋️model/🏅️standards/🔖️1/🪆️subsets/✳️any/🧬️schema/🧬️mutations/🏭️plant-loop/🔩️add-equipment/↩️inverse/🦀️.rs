//! ↩️ Inverse for `AddPlantLoopEquipment` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::AddPlantLoopEquipment, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::AddPlantLoopEquipment> + From<crate::RemovePlantLoopEquipment>,
{
    Ok((|| {
    match base.model.plant_loops.iter().find(|item| item.id == payload.id) {
        Some(existing) if !((payload.equipment_id.0 == 0) || (existing.equipment_ids.contains(&payload.equipment_id)) || payload.index.is_some_and(|index| index as usize > existing.equipment_ids.len())) => vec![vocabulary::remove_plant_loop_equipment(payload.id, payload.equipment_id)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
