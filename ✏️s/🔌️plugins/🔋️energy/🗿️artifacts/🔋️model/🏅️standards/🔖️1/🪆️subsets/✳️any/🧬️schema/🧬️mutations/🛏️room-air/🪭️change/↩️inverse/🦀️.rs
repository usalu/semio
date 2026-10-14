//! ↩️ Inverse for `ChangeRoomAirModel` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeRoomAirModel, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeRoomAirModel>,
{
    Ok((|| {
    match base.model.room_air_models.iter().find(|item| item.zone_id == payload.zone_id) {
        Some(item) if item.model != payload.new_model => vec![vocabulary::change_room_air_model(payload.zone_id, item.model)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
