//! ↩️ Inverse for `DeleteRoomAirModelAssignment` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteRoomAirModelAssignment, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteRoomAirModelAssignment> + From<crate::CreateRoomAirModelAssignment>,
{
    Ok((|| {
    match base.model.room_air_models.iter().enumerate().find(|(_, item)| item.zone_id == payload.zone_id) {
        Some((index, item)) => vec![vocabulary::create_room_air_model_assignment(item.zone_id, item.model, Some(index as u32))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
