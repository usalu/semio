//! 🔺️ Sparse diff builder for `ChangePvSystemArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePvSystemArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.pv_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("PV system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_area_m2.is_finite() || payload.new_area_m2 <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("PV system {}: aperture area (m²) must be a positive finite value, got {}.", payload.id.0, payload.new_area_m2), [payload.id.0.to_string()]);
    }
    if existing.area_m2 == payload.new_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("PV system {} already carries this aperture area (m²): {}.", payload.id.0, payload.new_area_m2));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.pv_systems.iter_mut().find(|item| item.id == payload.id) {
        item.area_m2 = payload.new_area_m2;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
