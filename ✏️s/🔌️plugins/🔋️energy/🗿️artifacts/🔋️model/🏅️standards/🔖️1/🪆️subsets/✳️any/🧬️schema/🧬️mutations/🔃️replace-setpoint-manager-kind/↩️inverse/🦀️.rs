//! ↩️ Inverse for `ReplaceSetpointManagerKind` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ReplaceSetpointManagerKind, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let kind = if payload.new_kind == "OutdoorAirReset" {
        crate::model::SetpointManagerKind::OutdoorAirReset { low_outdoor_c: payload.new_low_outdoor_c, high_outdoor_c: payload.new_high_outdoor_c, low_setpoint_c: payload.new_low_setpoint_c, high_setpoint_c: payload.new_high_setpoint_c }
    } else if payload.new_kind == "WarmestZone" {
        crate::model::SetpointManagerKind::WarmestZone
    } else if payload.new_kind == "ColdestZone" {
        crate::model::SetpointManagerKind::ColdestZone
    } else {
        crate::model::SetpointManagerKind::Scheduled
    };
    match base.model.setpoint_managers.iter().find(|item| item.id == payload.id) {
        Some(item) if item.kind != kind && !((!matches!(payload.new_kind.as_str(), "Scheduled" | "OutdoorAirReset" | "WarmestZone" | "ColdestZone")) || (payload.new_kind != "OutdoorAirReset" && !(payload.new_low_outdoor_c == 0.0 && payload.new_high_outdoor_c == 0.0 && payload.new_low_setpoint_c == 0.0 && payload.new_high_setpoint_c == 0.0)) || (payload.new_kind == "OutdoorAirReset" && payload.new_high_outdoor_c <= payload.new_low_outdoor_c)) => vec![vocabulary::replace_setpoint_manager_kind(payload.id, match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { .. } => "OutdoorAirReset".to_string(), crate::model::SetpointManagerKind::WarmestZone => "WarmestZone".to_string(), crate::model::SetpointManagerKind::ColdestZone => "ColdestZone".to_string(), crate::model::SetpointManagerKind::Scheduled => "Scheduled".to_string() }, match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { low_outdoor_c, .. } => *low_outdoor_c, _ => 0.0 }, match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { high_outdoor_c, .. } => *high_outdoor_c, _ => 0.0 }, match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { low_setpoint_c, .. } => *low_setpoint_c, _ => 0.0 }, match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { high_setpoint_c, .. } => *high_setpoint_c, _ => 0.0 })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
