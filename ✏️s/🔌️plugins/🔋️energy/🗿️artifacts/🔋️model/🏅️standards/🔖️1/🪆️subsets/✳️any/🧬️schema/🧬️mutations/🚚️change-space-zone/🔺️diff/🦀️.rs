//! 🔺️ Sparse diff builder for `ChangeSpaceZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SpacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSpaceZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.spaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.zones.iter().any(|item| item.id == payload.new_zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.new_zone_id.0), [payload.id.0.to_string()]);
    }
    if existing.zone_id == payload.new_zone_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Space {} already has this owning zone.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { spaces: Rows::modifying(SpacePatch { zone_id: Some(payload.new_zone_id), ..SpacePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
