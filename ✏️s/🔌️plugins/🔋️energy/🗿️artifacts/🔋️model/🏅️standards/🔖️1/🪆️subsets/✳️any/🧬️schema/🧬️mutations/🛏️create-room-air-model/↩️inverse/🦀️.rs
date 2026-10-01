//! ↩️ Inverse for `CreateRoomAirModelAssignment` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateRoomAirModelAssignment, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if (base.model.room_air_models.iter().any(|item| item.zone_id == payload.zone_id)) || (!base.model.zones.iter().any(|zone| zone.id == payload.zone_id)) {
        return Vec::new();
    }
    vec![vocabulary::delete_room_air_model_assignment(payload.zone_id)]
}
//#endregion 🔖️Inverse
