//! 🔺️ Sparse diff builder for `ChangeZoneVolume` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ZonePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeZoneVolume, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zones.iter().find(|zone| zone.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_volume_m3.is_finite() || payload.new_volume_m3 <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Zone {} needs a positive finite volume, got {}.", payload.id.0, payload.new_volume_m3), [payload.id.0.to_string()]);
    }
    if existing.volume_m3 == payload.new_volume_m3 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Zone {} already has volume {} m³.", payload.id.0, payload.new_volume_m3));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { zones: Rows::modifying(ZonePatch { volume_m3: Some(payload.new_volume_m3), ..ZonePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
