//! ↩️ Inverse for `ChangeZoneEquipmentZone` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeZoneEquipmentZone, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeZoneEquipmentZone>,
{
    Ok((|| {
    match base.model.zone_equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.zone_id != payload.new_zone_id && !(!base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id)) => vec![vocabulary::change_zone_equipment_zone(payload.id, item.zone_id)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
