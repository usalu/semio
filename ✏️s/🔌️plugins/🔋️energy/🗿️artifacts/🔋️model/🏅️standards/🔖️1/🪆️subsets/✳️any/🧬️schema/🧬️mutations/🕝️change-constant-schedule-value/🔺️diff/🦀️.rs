//! 🔺️ Sparse diff builder for `ChangeConstantScheduleValue` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeConstantScheduleValue, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.constants.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Constant schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule {} needs a finite value, got {}.", payload.id.0, payload.new_value), [payload.id.0.to_string()]);
    }
    if existing.value == payload.new_value {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Constant schedule {} already carries this value: {}.", payload.id.0, payload.new_value));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.constants.iter_mut().find(|item| item.id == payload.id) {
        item.value = payload.new_value;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
