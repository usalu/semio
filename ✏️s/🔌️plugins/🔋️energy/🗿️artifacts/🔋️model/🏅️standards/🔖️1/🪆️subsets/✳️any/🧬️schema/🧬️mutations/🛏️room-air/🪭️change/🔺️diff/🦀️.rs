//! 🔺️ Sparse diff builder for `ChangeRoomAirModel` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, RoomAirModelAssignmentPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRoomAirModel, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.room_air_models.iter().find(|item| item.zone_id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Room air model assignment for zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    };

    if existing.model == payload.new_model {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Room air model assignment for zone {} already has that room air model.", payload.zone_id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { room_air_models: Rows::modifying(RoomAirModelAssignmentPatch { model: Some(payload.new_model), ..RoomAirModelAssignmentPatch::of(payload.zone_id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
