//! 🔺️ Sparse diff builder for `ChangePvSystemAzimuth` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePvSystemAzimuth, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.pv_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("PV system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_azimuth_deg.is_finite() || !(0.0..=360.0).contains(&payload.new_azimuth_deg) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("PV system {}: azimuth must be a compass bearing in [0, 360] degrees, got {}.", payload.id.0, payload.new_azimuth_deg), [payload.id.0.to_string()]);
    }
    if existing.azimuth_deg == payload.new_azimuth_deg {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("PV system {} already carries this azimuth_deg: {}.", payload.id.0, payload.new_azimuth_deg));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.pv_systems.iter_mut().find(|item| item.id == payload.id) {
        item.azimuth_deg = payload.new_azimuth_deg;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
