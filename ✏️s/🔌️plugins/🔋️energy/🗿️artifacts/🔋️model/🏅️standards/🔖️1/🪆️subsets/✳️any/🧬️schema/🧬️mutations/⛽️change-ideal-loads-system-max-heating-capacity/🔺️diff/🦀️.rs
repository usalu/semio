//! 🔺️ Sparse diff builder for `ChangeIdealLoadsSystemMaxHeatingCapacity` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeIdealLoadsSystemMaxHeatingCapacity, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.ideal_loads.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Ideal loads system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_capacity_present && payload.new_max_heating_capacity_w != 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", "An absent maximum heating capacity carries the value zero.".to_string(), [payload.id.0.to_string()]);
    }
    if payload.new_capacity_present && (!payload.new_max_heating_capacity_w.is_finite() || payload.new_max_heating_capacity_w <= 0.0) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A stated heating capacity must be a positive finite number, got {}.", payload.new_max_heating_capacity_w), [payload.id.0.to_string()]);
    }
    let value = payload.new_capacity_present.then_some(payload.new_max_heating_capacity_w);
    if existing.max_heating_capacity_w == value {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Ideal loads system {} already has that maximum heating capacity.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.ideal_loads.iter_mut().find(|item| item.id == payload.id) {
        item.max_heating_capacity_w = value;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
