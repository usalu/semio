//! 🔺️ Sparse diff builder for `ChangeDaylightZoneIlluminanceTarget` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeDaylightZoneIlluminanceTarget, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.daylight_zones.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daylight zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_illuminance_target_lux.is_finite() || payload.new_illuminance_target_lux <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("An illuminance target must be a positive finite number, got {}.", payload.new_illuminance_target_lux), [payload.id.0.to_string()]);
    }
    if existing.illuminance_target_lux == payload.new_illuminance_target_lux {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Daylight zone {} already has that illuminance target.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.daylight_zones.iter_mut().find(|item| item.id == payload.id) {
        item.illuminance_target_lux = payload.new_illuminance_target_lux;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
