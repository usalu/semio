//! 🔺️ Sparse diff builder for `ChangeFenestrationArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFenestrationArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_area_m2.is_finite() || payload.new_area_m2 <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs a positive finite area, got {}.", payload.id.0, payload.new_area_m2), [payload.id.0.to_string()]);
    }
    if existing.area_m2 == payload.new_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fenestration {} already has this area.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.fenestrations.iter_mut().find(|item| item.id == payload.id) {
        item.area_m2 = payload.new_area_m2;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
