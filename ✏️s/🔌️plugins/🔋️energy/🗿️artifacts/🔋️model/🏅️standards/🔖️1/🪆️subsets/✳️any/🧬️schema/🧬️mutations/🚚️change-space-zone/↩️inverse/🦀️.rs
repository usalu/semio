//! ↩️ Inverse for `ChangeSpaceZone` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSpaceZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.spaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !base.model.zones.iter().any(|item| item.id == payload.new_zone_id) || existing.zone_id == payload.new_zone_id {
        return Vec::new();
    }
    vec![vocabulary::change_space_zone(payload.id, existing.zone_id)]
}
//#endregion 🔖️Inverse
