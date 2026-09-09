//! 🔺️ Sparse diff builder for `CreateMechanicalVentilation` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateMechanicalVentilation, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.mechanical_ventilations.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Mechanical Ventilation {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.mechanical_ventilations.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} mechanical_ventilations.", payload.index, base.model.mechanical_ventilations.len()), [payload.id.0.to_string()]);
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
    model.mechanical_ventilations.insert(
        payload.index as usize,
        crate::model::MechanicalVentilation {
            id: payload.id,
            zone_id: payload.zone_id,
            schedule_id: payload.schedule_id,
            design_flow_m3_s: payload.design_flow_m3_s,
            fan_total_efficiency: payload.fan_total_efficiency,
            fan_delta_pressure_pa: payload.fan_delta_pressure_pa,
        },
    );
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
