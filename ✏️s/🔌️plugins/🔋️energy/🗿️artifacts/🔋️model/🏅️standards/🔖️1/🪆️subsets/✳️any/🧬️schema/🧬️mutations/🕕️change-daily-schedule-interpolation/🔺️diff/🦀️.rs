//! 🔺️ Sparse diff builder for `ChangeDailyScheduleInterpolation` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeDailyScheduleInterpolation, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.daily.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.interpolation == payload.new_interpolation {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Daily schedule {} already carries this interpolation: {:?}.", payload.id.0, payload.new_interpolation));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.daily.iter_mut().find(|item| item.id == payload.id) {
        item.interpolation = payload.new_interpolation;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
