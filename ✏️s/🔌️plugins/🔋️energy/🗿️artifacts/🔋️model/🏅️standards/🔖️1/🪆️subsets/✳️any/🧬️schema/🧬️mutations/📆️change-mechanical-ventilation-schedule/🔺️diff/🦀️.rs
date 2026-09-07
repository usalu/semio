//! 🔺️ Sparse diff builder for `ChangeMechanicalVentilationSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMechanicalVentilationSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mechanical Ventilation {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_schedule_id)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} does not exist.", payload.new_schedule_id.0), [payload.new_schedule_id.0.to_string()]);
    }
    if existing.schedule_id == payload.new_schedule_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Mechanical Ventilation {} already carries this schedule reference: {}.", payload.id.0, payload.new_schedule_id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.mechanical_ventilations.iter_mut().find(|item| item.id == payload.id) {
        item.schedule_id = payload.new_schedule_id;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
