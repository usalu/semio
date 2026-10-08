//! 🔺️ Sparse diff builder for `ChangeIdealLoadsSystemZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, IdealLoadsSystemPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeIdealLoadsSystemZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.ideal_loads.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Ideal loads system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.new_zone_id.0), [payload.new_zone_id.0.to_string()]);
    }
    if existing.zone_id == payload.new_zone_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Ideal loads system {} already has that zone.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { ideal_loads: Rows::modifying(IdealLoadsSystemPatch { zone_id: Some(payload.new_zone_id), ..IdealLoadsSystemPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
