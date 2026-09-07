//! 🔺️ Sparse diff builder for `CreateSetpointManager` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateSetpointManager, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.setpoint_managers.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Setpoint manager {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A setpoint manager name must not be blank.".to_string(), [payload.id.0.to_string()]);
    }
    if base.model.setpoint_managers.iter().any(|item| item.name == payload.name) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Another setpoint manager is already named {:?}.", payload.name), [payload.id.0.to_string()]);
    }
    if !matches!(payload.kind.as_str(), "Scheduled" | "OutdoorAirReset" | "WarmestZone" | "ColdestZone") {
        return protocol::MutationOutcome::error("mutation.invariant", format!("{:?} is not a setpoint manager kind.", payload.kind), [payload.id.0.to_string()]);
    }
    if payload.kind != "OutdoorAirReset" && !(payload.low_outdoor_c == 0.0 && payload.high_outdoor_c == 0.0 && payload.low_setpoint_c == 0.0 && payload.high_setpoint_c == 0.0) {
        return protocol::MutationOutcome::error("mutation.invariant", "Only an OutdoorAirReset setpoint manager carries reset limits.".to_string(), [payload.id.0.to_string()]);
    }
    if payload.kind == "OutdoorAirReset" && payload.high_outdoor_c <= payload.low_outdoor_c {
        return protocol::MutationOutcome::error("mutation.invariant", "An outdoor air reset needs a high outdoor temperature above its low one.".to_string(), [payload.id.0.to_string()]);
    }
    if !payload.schedule_present && payload.schedule_id.0 != 0 {
        return protocol::MutationOutcome::error("mutation.invariant", "An absent setpoint manager schedule carries the id zero.".to_string(), [payload.id.0.to_string()]);
    }
    if payload.schedule_present && (!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.schedule_id))) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} is not defined by this model.", payload.schedule_id.0), [payload.schedule_id.0.to_string()]);
    }
    let kind = if payload.kind == "OutdoorAirReset" {
        crate::model::SetpointManagerKind::OutdoorAirReset { low_outdoor_c: payload.low_outdoor_c, high_outdoor_c: payload.high_outdoor_c, low_setpoint_c: payload.low_setpoint_c, high_setpoint_c: payload.high_setpoint_c }
    } else if payload.kind == "WarmestZone" {
        crate::model::SetpointManagerKind::WarmestZone
    } else if payload.kind == "ColdestZone" {
        crate::model::SetpointManagerKind::ColdestZone
    } else {
        crate::model::SetpointManagerKind::Scheduled
    };
    let mut model = base.model.clone();
    model.setpoint_managers.push(crate::model::SetpointManager { id: payload.id, name: payload.name.clone(), kind, schedule_id: payload.schedule_present.then_some(payload.schedule_id) });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
