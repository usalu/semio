//! 🔺️ Sparse diff builder for `ChangeRoomAirModel` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRoomAirModel, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.room_air_models.iter().find(|item| item.zone_id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Room air model assignment for zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    };

    if existing.model == payload.new_model {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Room air model assignment for zone {} already has that room air model.", payload.zone_id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.room_air_models.iter_mut().find(|item| item.zone_id == payload.zone_id) {
        item.model = payload.new_model;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
