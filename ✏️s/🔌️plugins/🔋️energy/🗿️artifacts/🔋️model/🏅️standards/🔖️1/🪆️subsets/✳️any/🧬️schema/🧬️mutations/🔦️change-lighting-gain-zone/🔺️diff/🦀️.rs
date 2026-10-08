//! 🔺️ Sparse diff builder for `ChangeLightingGainZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, LightingGainPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeLightingGainZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.lighting.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Lighting Gain {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.new_zone_id.0), [payload.new_zone_id.0.to_string()]);
    }
    if existing.zone_id == payload.new_zone_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Lighting Gain {} already carries this zone reference: {}.", payload.id.0, payload.new_zone_id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { lighting: Rows::modifying(LightingGainPatch { zone_id: Some(payload.new_zone_id), ..LightingGainPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
