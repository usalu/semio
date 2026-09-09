//! 🔺️ Sparse diff builder for `CreateInfiltration` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateInfiltration, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.infiltrations.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Infiltration {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.infiltrations.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} infiltrations.", payload.index, base.model.infiltrations.len()), [payload.id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} does not exist.", payload.schedule_id.0), [payload.schedule_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.infiltrations.insert(
        payload.index as usize,
        crate::model::Infiltration {
            id: payload.id,
            zone_id: payload.zone_id,
            schedule_id: payload.schedule_id,
            method: payload.method,
            design_flow_ach: payload.design_flow_ach,
            flow_per_exterior_area_m3_s_m2: payload.flow_per_exterior_area_m3_s_m2,
            effective_leakage_area_m2: payload.effective_leakage_area_m2,
            discharge_coefficient: payload.discharge_coefficient,
            stack_height_m: payload.stack_height_m,
            constant_term_coefficient: payload.constant_term_coefficient,
            temperature_term_coefficient: payload.temperature_term_coefficient,
            velocity_term_coefficient: payload.velocity_term_coefficient,
            velocity_squared_term_coefficient: payload.velocity_squared_term_coefficient,
        },
    );
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
