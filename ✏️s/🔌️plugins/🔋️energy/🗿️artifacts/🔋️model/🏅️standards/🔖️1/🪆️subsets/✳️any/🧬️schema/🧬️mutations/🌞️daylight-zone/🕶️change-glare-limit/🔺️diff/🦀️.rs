//! 🔺️ Sparse diff builder for `ChangeDaylightZoneGlareLimit` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, DaylightZoneConfigPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeDaylightZoneGlareLimit, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.daylight_zones.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daylight zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_glare_limit.is_finite() || payload.new_glare_limit <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A glare limit must be a positive finite number, got {}.", payload.new_glare_limit), [payload.id.0.to_string()]);
    }
    if existing.glare_limit == payload.new_glare_limit {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Daylight zone {} already has that glare limit.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { daylight_zones: Rows::modifying(DaylightZoneConfigPatch { glare_limit: Some(payload.new_glare_limit), ..DaylightZoneConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
