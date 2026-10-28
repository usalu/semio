//! ↩️ Inverse for `ChangeFaultTargetEquipment` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeFaultTargetEquipment, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeFaultTargetEquipment>,
{
    Ok((|| {
    match base.model.faults.iter().find(|item| item.id == payload.id) {
        Some(item) if item.target_equipment_id != payload.new_target_equipment_id && base.model.ideal_loads.iter().any(|row| row.id == payload.new_target_equipment_id) => {
            vec![vocabulary::change_fault_target_equipment(payload.id, item.target_equipment_id)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
