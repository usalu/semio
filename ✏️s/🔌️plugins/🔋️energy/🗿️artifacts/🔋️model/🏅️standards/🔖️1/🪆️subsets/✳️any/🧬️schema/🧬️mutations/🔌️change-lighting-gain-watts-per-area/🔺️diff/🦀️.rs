//! 🔺️ Sparse diff builder for `ChangeLightingGainWattsPerArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeLightingGainWattsPerArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.lighting.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Lighting Gain {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_watts_per_area.is_finite() || payload.new_watts_per_area < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Lighting Gain {}: installed power density (W/m²) must be a finite non-negative value, got {}.", payload.id.0, payload.new_watts_per_area), [payload.id.0.to_string()]);
    }
    if existing.watts_per_area == payload.new_watts_per_area {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Lighting Gain {} already carries this installed power density (W/m²): {}.", payload.id.0, payload.new_watts_per_area));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.lighting.iter_mut().find(|item| item.id == payload.id) {
        item.watts_per_area = payload.new_watts_per_area;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
