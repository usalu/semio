//! 🔺️ Sparse diff builder for `CreateDaylightZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateDaylightZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.daylight_zones.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Daylight zone {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if !payload.illuminance_target_lux.is_finite() || payload.illuminance_target_lux <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("An illuminance target must be a positive finite number, got {}.", payload.illuminance_target_lux), [payload.id.0.to_string()]);
    }
    if !payload.glare_limit.is_finite() || payload.glare_limit <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A glare limit must be a positive finite number, got {}.", payload.glare_limit), [payload.id.0.to_string()]);
    }
    if !payload.window_transmittance.is_finite() || !(0.0..=1.0).contains(&payload.window_transmittance) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A window transmittance must lie between 0.0 and 1.0, got {}.", payload.window_transmittance), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.daylight_zones.push(crate::model::DaylightZoneConfig {
        id: payload.id,
        zone_id: payload.zone_id,
        illuminance_target_lux: payload.illuminance_target_lux,
        glare_limit: payload.glare_limit,
        window_transmittance: payload.window_transmittance,
    });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
