//! 🔺️ Sparse diff builder for `DeleteRoomAirModelAssignment` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, RoomAirModelAssignmentPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteRoomAirModelAssignment, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.room_air_models.iter().find(|item| item.zone_id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Room air model assignment for zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    };
    let _ = existing;

    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { room_air_models: Rows::removing(&base.model.room_air_models, &payload.zone_id), ..Default::default() }))
}
//#endregion 🔖️Diff
