//! 🔺️ Sparse diff builder for `ChangeDaylightZoneIlluminanceTarget` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, DaylightZoneConfigPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeDaylightZoneIlluminanceTarget, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.daylight_zones.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daylight zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_illuminance_target_lux.is_finite() || payload.new_illuminance_target_lux <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("An illuminance target must be a positive finite number, got {}.", payload.new_illuminance_target_lux), [payload.id.0.to_string()]);
    }
    if existing.illuminance_target_lux == payload.new_illuminance_target_lux {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Daylight zone {} already has that illuminance target.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { daylight_zones: Rows::modifying(DaylightZoneConfigPatch { illuminance_target_lux: Some(payload.new_illuminance_target_lux), ..DaylightZoneConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
