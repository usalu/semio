//! 🔺️ Sparse diff builder for `RenameZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zones.iter().find(|zone| zone.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A zone name must not be blank.", [payload.id.0.to_string()]);
    }
    if base.model.zones.iter().any(|zone| zone.id != payload.id && zone.name == payload.new_name) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Another zone is already named \"{}\".", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone {} is already named \"{}\".", payload.id.0, payload.new_name));
    }
    let mut model = base.model.clone();
    if let Some(zone) = model.zones.iter_mut().find(|zone| zone.id == payload.id) {
        zone.name = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
