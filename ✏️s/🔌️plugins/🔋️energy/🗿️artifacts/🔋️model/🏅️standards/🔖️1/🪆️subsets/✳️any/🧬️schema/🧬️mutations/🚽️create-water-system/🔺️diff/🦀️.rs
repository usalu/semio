//! 🔺️ Sparse diff builder for `CreateWaterSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, WaterSystemConfigPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateWaterSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.water_systems.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Water system {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.water_systems.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} water_systems.", payload.index, base.model.water_systems.len()), [payload.id.0.to_string()]);
    }
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} does not exist.", payload.schedule_id.0), [payload.schedule_id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { water_systems: Rows::inserting(payload.index as usize, crate::model::WaterSystemConfig { id: payload.id, fixture_count: payload.fixture_count, peak_flow_l_s: payload.peak_flow_l_s, schedule_id: payload.schedule_id }), ..Default::default() }))
}
//#endregion 🔖️Diff
