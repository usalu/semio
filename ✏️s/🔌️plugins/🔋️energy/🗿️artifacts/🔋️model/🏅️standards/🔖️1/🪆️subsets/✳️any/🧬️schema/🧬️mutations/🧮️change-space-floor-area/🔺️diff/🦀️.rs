//! 🔺️ Sparse diff builder for `ChangeSpaceFloorArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSpaceFloorArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.spaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_floor_area_m2.is_finite() || payload.new_floor_area_m2 < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Space {} needs a non-negative finite floor area, got {}.", payload.id.0, payload.new_floor_area_m2), [payload.id.0.to_string()]);
    }
    if existing.floor_area_m2 == payload.new_floor_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Space {} already has this floor area.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.spaces.iter_mut().find(|item| item.id == payload.id) {
        item.floor_area_m2 = payload.new_floor_area_m2;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
