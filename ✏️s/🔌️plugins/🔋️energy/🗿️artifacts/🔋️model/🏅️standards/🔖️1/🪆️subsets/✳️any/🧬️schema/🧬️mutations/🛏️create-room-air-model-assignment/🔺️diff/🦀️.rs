//! 🔺️ Sparse diff builder for `CreateRoomAirModelAssignment` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateRoomAirModelAssignment, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.room_air_models.iter().any(|item| item.zone_id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Room air model assignment for zone {} already exists.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.room_air_models.push(crate::model::RoomAirModelAssignment { zone_id: payload.zone_id, model: payload.model });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
