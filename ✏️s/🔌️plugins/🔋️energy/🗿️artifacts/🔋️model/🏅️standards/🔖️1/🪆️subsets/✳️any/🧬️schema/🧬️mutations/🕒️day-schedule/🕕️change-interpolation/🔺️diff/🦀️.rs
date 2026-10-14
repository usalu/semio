//! 🔺️ Sparse diff builder for `ChangeDailyScheduleInterpolation` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, DailySchedulePatch, ModelPatch, Rows, ScheduleSetPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeDailyScheduleInterpolation, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.daily.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.interpolation == payload.new_interpolation {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Daily schedule {} already carries this interpolation: {:?}.", payload.id.0, payload.new_interpolation));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { schedules: ScheduleSetPatch { daily: Rows::modifying(DailySchedulePatch { interpolation: Some(payload.new_interpolation), ..DailySchedulePatch::of(payload.id) }), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
