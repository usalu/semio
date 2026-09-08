//! 🔺️ Sparse diff builder for `ReplaceSetpointManagerKind` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceSetpointManagerKind, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.setpoint_managers.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Setpoint manager {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !matches!(payload.new_kind.as_str(), "Scheduled" | "OutdoorAirReset" | "WarmestZone" | "ColdestZone") {
        return protocol::MutationOutcome::error("mutation.invariant", format!("{:?} is not a setpoint manager kind.", payload.new_kind), [payload.id.0.to_string()]);
    }
    if payload.new_kind != "OutdoorAirReset" && !(payload.new_low_outdoor_c == 0.0 && payload.new_high_outdoor_c == 0.0 && payload.new_low_setpoint_c == 0.0 && payload.new_high_setpoint_c == 0.0) {
        return protocol::MutationOutcome::error("mutation.invariant", "Only an OutdoorAirReset setpoint manager carries reset limits.".to_string(), [payload.id.0.to_string()]);
    }
    if payload.new_kind == "OutdoorAirReset" && payload.new_high_outdoor_c <= payload.new_low_outdoor_c {
        return protocol::MutationOutcome::error("mutation.invariant", "An outdoor air reset needs a high outdoor temperature above its low one.".to_string(), [payload.id.0.to_string()]);
    }
    let kind = if payload.new_kind == "OutdoorAirReset" {
        crate::model::SetpointManagerKind::OutdoorAirReset { low_outdoor_c: payload.new_low_outdoor_c, high_outdoor_c: payload.new_high_outdoor_c, low_setpoint_c: payload.new_low_setpoint_c, high_setpoint_c: payload.new_high_setpoint_c }
    } else if payload.new_kind == "WarmestZone" {
        crate::model::SetpointManagerKind::WarmestZone
    } else if payload.new_kind == "ColdestZone" {
        crate::model::SetpointManagerKind::ColdestZone
    } else {
        crate::model::SetpointManagerKind::Scheduled
    };
    if existing.kind == kind {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Setpoint manager {} already runs that control law.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.setpoint_managers.iter_mut().find(|item| item.id == payload.id) {
        item.kind = kind;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
