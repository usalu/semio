//! 🔺️ Sparse diff builder for `ChangeSpaceFloorArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SpacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSpaceFloorArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.spaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_floor_area_m2.is_finite() || payload.new_floor_area_m2 < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Space {} needs a non-negative finite floor area, got {}.", payload.id.0, payload.new_floor_area_m2), [payload.id.0.to_string()]);
    }
    if existing.floor_area_m2 == payload.new_floor_area_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Space {} already has this floor area.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { spaces: Rows::modifying(SpacePatch { floor_area_m2: Some(payload.new_floor_area_m2), ..SpacePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
