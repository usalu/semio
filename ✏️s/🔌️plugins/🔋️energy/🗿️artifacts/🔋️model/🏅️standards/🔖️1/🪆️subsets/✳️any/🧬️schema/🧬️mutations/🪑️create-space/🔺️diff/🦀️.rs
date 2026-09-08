//! 🔺️ Sparse diff builder for `CreateSpace` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateSpace, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.spaces.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Space {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A space name must not be blank.", [payload.id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|item| item.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.id.0.to_string()]);
    }
    if !payload.floor_area_m2.is_finite() || payload.floor_area_m2 < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Space {} needs a non-negative finite floor area, got {}.", payload.id.0, payload.floor_area_m2), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    let position = model.spaces.iter().position(|item| item.id > payload.id).unwrap_or(model.spaces.len());
    model.spaces.insert(position, crate::model::Space { id: payload.id, name: payload.name.clone(), zone_id: payload.zone_id, floor_area_m2: payload.floor_area_m2 });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
