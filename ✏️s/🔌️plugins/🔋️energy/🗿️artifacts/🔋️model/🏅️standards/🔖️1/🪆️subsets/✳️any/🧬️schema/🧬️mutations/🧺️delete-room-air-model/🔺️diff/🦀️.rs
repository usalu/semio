//! 🔺️ Sparse diff builder for `DeleteRoomAirModelAssignment` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteRoomAirModelAssignment, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.room_air_models.iter().find(|item| item.zone_id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Room air model assignment for zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    };
    let _ = existing;

    let mut model = base.model.clone();
    model.room_air_models.retain(|item| item.zone_id != payload.zone_id);
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
