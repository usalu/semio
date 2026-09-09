//! 🔺️ Sparse diff builder for `CreateRefrigerationSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateRefrigerationSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.refrigeration_systems.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Refrigeration system {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.refrigeration_systems.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} refrigeration_systems.", payload.index, base.model.refrigeration_systems.len()), [payload.id.0.to_string()]);
    }
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.defrost_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.defrost_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.defrost_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.defrost_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.defrost_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} does not exist.", payload.defrost_schedule_id.0), [payload.defrost_schedule_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.refrigeration_systems.insert(payload.index as usize, crate::model::RefrigerationConfig { id: payload.id, case_count: payload.case_count, design_load_w: payload.design_load_w, defrost_schedule_id: payload.defrost_schedule_id });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
