//! ↩️ Inverse for `ChangeZoneFloorAreaParticipation` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeZoneFloorAreaParticipation, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.zones.iter().find(|zone| zone.id == payload.id) {
        Some(zone) if zone.part_of_total_floor_area != payload.new_part_of_total_floor_area => vec![vocabulary::change_zone_floor_area_participation(payload.id, zone.part_of_total_floor_area)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
