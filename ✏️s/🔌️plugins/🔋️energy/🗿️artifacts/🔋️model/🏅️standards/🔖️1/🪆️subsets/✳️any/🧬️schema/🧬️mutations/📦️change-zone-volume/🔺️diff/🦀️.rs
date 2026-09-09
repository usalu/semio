//! 🔺️ Sparse diff builder for `ChangeZoneVolume` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeZoneVolume, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zones.iter().find(|zone| zone.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_volume_m3.is_finite() || payload.new_volume_m3 <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Zone {} needs a positive finite volume, got {}.", payload.id.0, payload.new_volume_m3), [payload.id.0.to_string()]);
    }
    if existing.volume_m3 == payload.new_volume_m3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone {} already has volume {} m³.", payload.id.0, payload.new_volume_m3));
    }
    let mut model = base.model.clone();
    if let Some(zone) = model.zones.iter_mut().find(|zone| zone.id == payload.id) {
        zone.volume_m3 = payload.new_volume_m3;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
