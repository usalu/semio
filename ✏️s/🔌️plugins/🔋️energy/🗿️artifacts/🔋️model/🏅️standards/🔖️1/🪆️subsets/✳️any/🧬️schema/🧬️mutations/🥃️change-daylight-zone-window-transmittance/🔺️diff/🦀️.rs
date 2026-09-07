//! 🔺️ Sparse diff builder for `ChangeDaylightZoneWindowTransmittance` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeDaylightZoneWindowTransmittance, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.daylight_zones.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daylight zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_window_transmittance.is_finite() || !(0.0..=1.0).contains(&payload.new_window_transmittance) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A window transmittance must lie between 0.0 and 1.0, got {}.", payload.new_window_transmittance), [payload.id.0.to_string()]);
    }
    if existing.window_transmittance == payload.new_window_transmittance {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Daylight zone {} already has that window transmittance.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.daylight_zones.iter_mut().find(|item| item.id == payload.id) {
        item.window_transmittance = payload.new_window_transmittance;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
