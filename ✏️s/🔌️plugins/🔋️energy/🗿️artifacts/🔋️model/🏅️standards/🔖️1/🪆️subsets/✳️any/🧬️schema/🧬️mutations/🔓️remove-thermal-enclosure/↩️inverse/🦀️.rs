//! ↩️ Inverse for `RemoveThermalEnclosureZone` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RemoveThermalEnclosureZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(item) = base.model.thermal_enclosures.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let Some(position) = item.zone_ids.iter().position(|candidate| *candidate == payload.zone_id) else {
        return Vec::new();
    };
    vec![vocabulary::add_thermal_enclosure_zone(payload.id, position as u32, payload.zone_id)]
}
//#endregion 🔖️Inverse
