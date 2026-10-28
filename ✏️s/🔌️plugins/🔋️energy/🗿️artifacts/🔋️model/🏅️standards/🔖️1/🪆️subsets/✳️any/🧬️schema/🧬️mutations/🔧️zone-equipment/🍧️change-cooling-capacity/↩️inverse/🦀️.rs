//! ↩️ Inverse for `ChangeZoneEquipmentCoolingCapacity` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeZoneEquipmentCoolingCapacity, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeZoneEquipmentCoolingCapacity>,
{
    Ok((|| {
    match base.model.zone_equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.cooling_capacity_w != payload.new_cooling_capacity_w && !(!payload.new_cooling_capacity_w.is_finite() || payload.new_cooling_capacity_w < 0.0) => {
            vec![vocabulary::change_zone_equipment_cooling_capacity(payload.id, item.cooling_capacity_w)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
