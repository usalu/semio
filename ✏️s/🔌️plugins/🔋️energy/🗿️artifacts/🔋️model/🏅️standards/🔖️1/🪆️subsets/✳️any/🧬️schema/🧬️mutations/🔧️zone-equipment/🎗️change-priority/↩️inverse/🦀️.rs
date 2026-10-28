//! ↩️ Inverse for `ChangeZoneEquipmentPriority` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeZoneEquipmentPriority, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeZoneEquipmentPriority>,
{
    Ok((|| {
    match base.model.zone_equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.priority != payload.new_priority && !(payload.new_priority == 0) => vec![vocabulary::change_zone_equipment_priority(payload.id, item.priority)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
