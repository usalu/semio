//! ↩️ Inverse for `ChangeZoneEquipmentHeatingCapacity` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeZoneEquipmentHeatingCapacity, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeZoneEquipmentHeatingCapacity>,
{
    Ok((|| {
    match base.model.zone_equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.heating_capacity_w != payload.new_heating_capacity_w && !(!payload.new_heating_capacity_w.is_finite() || payload.new_heating_capacity_w < 0.0) => {
            vec![vocabulary::change_zone_equipment_heating_capacity(payload.id, item.heating_capacity_w)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
