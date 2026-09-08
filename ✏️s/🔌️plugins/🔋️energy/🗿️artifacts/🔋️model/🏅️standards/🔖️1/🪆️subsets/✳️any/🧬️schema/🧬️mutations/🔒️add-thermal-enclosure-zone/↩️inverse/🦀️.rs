//! ↩️ Inverse for `AddThermalEnclosureZone` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::AddThermalEnclosureZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.thermal_enclosures.iter().find(|item| item.id == payload.id) {
        Some(item) if !item.zone_ids.contains(&payload.zone_id) && payload.index as usize <= item.zone_ids.len() && base.model.zones.iter().any(|row| row.id == payload.zone_id) => vec![vocabulary::remove_thermal_enclosure_zone(payload.id, payload.zone_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
