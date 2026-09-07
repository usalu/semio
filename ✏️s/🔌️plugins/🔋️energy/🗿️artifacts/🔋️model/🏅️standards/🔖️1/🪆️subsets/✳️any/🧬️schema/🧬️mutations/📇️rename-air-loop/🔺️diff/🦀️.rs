//! 🔺️ Sparse diff builder for `RenameAirLoop` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameAirLoop, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.air_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A air loop name must not be blank.".to_string(), [payload.id.0.to_string()]);
    }
    if base.model.air_loops.iter().any(|item| item.id != payload.id && item.name == payload.new_name) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Another air loop is already named {:?}.", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Air loop {} already has that name.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.air_loops.iter_mut().find(|item| item.id == payload.id) {
        item.name = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
