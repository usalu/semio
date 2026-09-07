//! 🔺️ Sparse diff builder for `DeleteAirLoop` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteAirLoop, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.air_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.outdoor_air_systems.iter().any(|system| system.air_loop_id == payload.id) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Air loop {} still serves an outdoor air system.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.air_loops.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
