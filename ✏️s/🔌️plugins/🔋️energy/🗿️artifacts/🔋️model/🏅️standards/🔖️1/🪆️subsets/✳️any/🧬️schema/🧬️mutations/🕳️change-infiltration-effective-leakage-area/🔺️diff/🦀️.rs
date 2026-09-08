//! 🔺️ Sparse diff builder for `ChangeInfiltrationEffectiveLeakageArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationEffectiveLeakageArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_effective_leakage_area_m2.is_finite() || payload.new_effective_leakage_area_m2 < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Infiltration {}: effective leakage area (m²) must be a finite non-negative value, got {}.", payload.id.0, payload.new_effective_leakage_area_m2), [payload.id.0.to_string()]);
    }
    if existing.effective_leakage_area_m2 == payload.new_effective_leakage_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Infiltration {} already carries this effective leakage area (m²): {}.", payload.id.0, payload.new_effective_leakage_area_m2));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.infiltrations.iter_mut().find(|item| item.id == payload.id) {
        item.effective_leakage_area_m2 = payload.new_effective_leakage_area_m2;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
