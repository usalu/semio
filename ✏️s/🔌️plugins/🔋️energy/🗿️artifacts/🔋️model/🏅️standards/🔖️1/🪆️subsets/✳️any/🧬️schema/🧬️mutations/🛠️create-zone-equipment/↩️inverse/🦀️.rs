//! ↩️ Inverse for `CreateZoneEquipment` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateZoneEquipment, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if (base.model.zone_equipment.iter().any(|item| item.id == payload.id))
        || (!base.model.zones.iter().any(|zone| zone.id == payload.zone_id))
        || (payload.priority == 0)
        || (!payload.heating_capacity_w.is_finite() || payload.heating_capacity_w < 0.0)
        || (!payload.cooling_capacity_w.is_finite() || payload.cooling_capacity_w < 0.0)
    {
        return Vec::new();
    }
    vec![vocabulary::delete_zone_equipment(payload.id)]
}
//#endregion 🔖️Inverse
