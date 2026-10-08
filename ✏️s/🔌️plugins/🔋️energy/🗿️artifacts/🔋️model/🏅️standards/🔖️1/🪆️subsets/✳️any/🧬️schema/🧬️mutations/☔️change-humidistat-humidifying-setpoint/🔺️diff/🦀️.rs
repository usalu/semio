//! 🔺️ Sparse diff builder for `ChangeHumidistatHumidifyingSetpointSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, HumidistatPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeHumidistatHumidifyingSetpointSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.humidistats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Humidistat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_humidifying_setpoint_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_humidifying_setpoint_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_humidifying_setpoint_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_humidifying_setpoint_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_humidifying_setpoint_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} is not defined by this model.", payload.new_humidifying_setpoint_schedule_id.0), [payload.new_humidifying_setpoint_schedule_id.0.to_string()]);
    }
    if existing.humidifying_setpoint_schedule_id == payload.new_humidifying_setpoint_schedule_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Humidistat {} already has that humidifying setpoint schedule.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { humidistats: Rows::modifying(HumidistatPatch { humidifying_setpoint_schedule_id: Some(payload.new_humidifying_setpoint_schedule_id), ..HumidistatPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
