//! 🔺️ Sparse diff builder for `CreateFault` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateFault, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.faults.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Fault {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.faults.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} faults.", payload.index, base.model.faults.len()), [payload.id.0.to_string()]);
    }
    if !base.model.ideal_loads.iter().any(|row| row.id == payload.target_equipment_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Ideal loads system {} does not exist.", payload.target_equipment_id.0), [payload.target_equipment_id.0.to_string()]);
    }
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.start_schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.start_schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.start_schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.start_schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.start_schedule_id)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} does not exist.", payload.start_schedule_id.0), [payload.start_schedule_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.faults.insert(payload.index as usize, crate::model::FaultDefinition { id: payload.id, target_equipment_id: payload.target_equipment_id, fault_type: payload.fault_type, severity: payload.severity, start_schedule_id: payload.start_schedule_id });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
