//! 🔺️ Sparse diff builder for `CreateRoomAirModelAssignment` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, RoomAirModelAssignmentPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateRoomAirModelAssignment, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.room_air_models.iter().any(|item| item.zone_id == payload.zone_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Room air model assignment for zone {} already exists.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { room_air_models: Rows::inserting(base.model.room_air_models.len(), crate::model::RoomAirModelAssignment { zone_id: payload.zone_id, model: payload.model }), ..Default::default() }))
}
//#endregion 🔖️Diff
