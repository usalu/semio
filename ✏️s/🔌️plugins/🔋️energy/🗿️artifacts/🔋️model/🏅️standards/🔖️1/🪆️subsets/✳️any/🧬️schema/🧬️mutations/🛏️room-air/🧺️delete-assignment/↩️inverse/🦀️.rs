//! ↩️ Inverse for `DeleteRoomAirModelAssignment` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteRoomAirModelAssignment, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.model.room_air_models.iter().enumerate().find(|(_, item)| item.zone_id == payload.zone_id) {
        Some((index, item)) => vec![vocabulary::create_room_air_model_assignment(item.zone_id, item.model, Some(index as u32))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
