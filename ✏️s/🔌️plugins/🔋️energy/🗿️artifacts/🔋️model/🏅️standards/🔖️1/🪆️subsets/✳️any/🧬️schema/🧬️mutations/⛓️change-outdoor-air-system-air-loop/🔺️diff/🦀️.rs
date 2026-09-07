//! 🔺️ Sparse diff builder for `ChangeOutdoorAirSystemAirLoop` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeOutdoorAirSystemAirLoop, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.outdoor_air_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Outdoor air system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.air_loops.iter().any(|item| item.id == payload.new_air_loop_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.new_air_loop_id.0), [payload.new_air_loop_id.0.to_string()]);
    }
    if existing.air_loop_id == payload.new_air_loop_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Outdoor air system {} already has that air loop.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.outdoor_air_systems.iter_mut().find(|item| item.id == payload.id) {
        item.air_loop_id = payload.new_air_loop_id;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
