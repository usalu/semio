//! 🔺️ Sparse diff builder for `ChangeMechanicalVentilationSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, MechanicalVentilationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMechanicalVentilationSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mechanical Ventilation {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} does not exist.", payload.new_schedule_id.0), [payload.new_schedule_id.0.to_string()]);
    }
    if existing.schedule_id == payload.new_schedule_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Mechanical Ventilation {} already carries this schedule reference: {}.", payload.id.0, payload.new_schedule_id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { mechanical_ventilations: Rows::modifying(MechanicalVentilationPatch { schedule_id: Some(payload.new_schedule_id), ..MechanicalVentilationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
