//! 🔺️ Sparse diff builder for `ChangeConstantScheduleValue` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ConstantSchedulePatch, ModelPatch, Rows, ScheduleSetPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeConstantScheduleValue, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.constants.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Constant schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Schedule {} needs a finite value, got {}.", payload.id.0, payload.new_value), [payload.id.0.to_string()]);
    }
    if existing.value == payload.new_value {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Constant schedule {} already carries this value: {}.", payload.id.0, payload.new_value));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { schedules: ScheduleSetPatch { constants: Rows::modifying(ConstantSchedulePatch { value: Some(payload.new_value), ..ConstantSchedulePatch::of(payload.id) }), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
