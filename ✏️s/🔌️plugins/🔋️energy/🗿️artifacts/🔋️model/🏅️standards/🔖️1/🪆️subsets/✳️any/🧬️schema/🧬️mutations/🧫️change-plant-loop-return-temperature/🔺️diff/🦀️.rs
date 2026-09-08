//! 🔺️ Sparse diff builder for `ChangePlantLoopReturnTemperature` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePlantLoopReturnTemperature, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.plant_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Plant loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_return_temperature_c.is_finite() || !(-100.0..=300.0).contains(&payload.new_return_temperature_c) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A return temperature must lie between -100.0 and 300.0, got {}.", payload.new_return_temperature_c), [payload.id.0.to_string()]);
    }
    if existing.return_temperature_c == payload.new_return_temperature_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Plant loop {} already has that return temperature.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.plant_loops.iter_mut().find(|item| item.id == payload.id) {
        item.return_temperature_c = payload.new_return_temperature_c;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
