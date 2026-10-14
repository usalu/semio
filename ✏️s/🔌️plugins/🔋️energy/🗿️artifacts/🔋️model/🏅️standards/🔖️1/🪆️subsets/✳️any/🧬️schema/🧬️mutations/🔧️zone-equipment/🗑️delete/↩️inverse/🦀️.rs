//! ↩️ Inverse for `DeleteZoneEquipment` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteZoneEquipment, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.model.zone_equipment.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((index, item)) => vec![vocabulary::create_zone_equipment(item.id, item.zone_id, item.equipment_type.clone(), item.priority, item.heating_capacity_w, item.cooling_capacity_w, Some(index as u32))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
