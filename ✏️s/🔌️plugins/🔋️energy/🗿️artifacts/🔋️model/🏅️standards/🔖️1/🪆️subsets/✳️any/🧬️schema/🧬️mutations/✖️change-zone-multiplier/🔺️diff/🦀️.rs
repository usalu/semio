//! 🔺️ Sparse diff builder for `ChangeZoneMultiplier` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeZoneMultiplier, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zones.iter().find(|zone| zone.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_multiplier == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Zone {} needs at least one instance.", payload.id.0), [payload.id.0.to_string()]);
    }
    if existing.multiplier == payload.new_multiplier {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone {} already has multiplier {}.", payload.id.0, payload.new_multiplier));
    }
    let mut model = base.model.clone();
    if let Some(zone) = model.zones.iter_mut().find(|zone| zone.id == payload.id) {
        zone.multiplier = payload.new_multiplier;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
