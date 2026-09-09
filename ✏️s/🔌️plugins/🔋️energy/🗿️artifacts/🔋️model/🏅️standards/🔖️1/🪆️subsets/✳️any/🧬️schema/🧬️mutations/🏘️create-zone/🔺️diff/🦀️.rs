//! 🔺️ Sparse diff builder for `CreateZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.zones.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Zone {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A zone name must not be blank.", [payload.id.0.to_string()]);
    }
    if base.model.zones.iter().any(|item| item.name == payload.name) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Another zone is already named \"{}\".", payload.name), [payload.id.0.to_string()]);
    }
    if !payload.volume_m3.is_finite() || payload.volume_m3 <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Zone {} needs a positive finite volume, got {}.", payload.id.0, payload.volume_m3), [payload.id.0.to_string()]);
    }
    if payload.multiplier == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Zone {} needs at least one instance.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    let position = model.zones.iter().position(|item| item.id > payload.id).unwrap_or(model.zones.len());
    model
        .zones
        .insert(position, crate::model::Zone { id: payload.id, name: payload.name.clone(), volume_m3: payload.volume_m3, multiplier: payload.multiplier, conditioned: payload.conditioned, part_of_total_floor_area: payload.part_of_total_floor_area });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
