//! 🔺️ Sparse diff builder for `ChangeZoneConditioned` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ZonePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeZoneConditioned, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zones.iter().find(|zone| zone.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.conditioned == payload.new_conditioned {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Zone {} is already conditioned={}.", payload.id.0, payload.new_conditioned));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { zones: Rows::modifying(ZonePatch { conditioned: Some(payload.new_conditioned), ..ZonePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
