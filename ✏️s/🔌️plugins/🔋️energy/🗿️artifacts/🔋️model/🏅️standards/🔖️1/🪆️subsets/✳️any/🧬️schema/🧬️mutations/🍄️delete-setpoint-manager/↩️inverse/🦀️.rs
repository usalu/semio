//! ↩️ Inverse for `DeleteSetpointManager` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteSetpointManager, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.setpoint_managers.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![vocabulary::create_setpoint_manager(item.id, item.name.clone(), match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { .. } => "OutdoorAirReset".to_string(), crate::model::SetpointManagerKind::WarmestZone => "WarmestZone".to_string(), crate::model::SetpointManagerKind::ColdestZone => "ColdestZone".to_string(), crate::model::SetpointManagerKind::Scheduled => "Scheduled".to_string() }, match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { low_outdoor_c, .. } => *low_outdoor_c, _ => 0.0 }, match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { high_outdoor_c, .. } => *high_outdoor_c, _ => 0.0 }, match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { low_setpoint_c, .. } => *low_setpoint_c, _ => 0.0 }, match &item.kind { crate::model::SetpointManagerKind::OutdoorAirReset { high_setpoint_c, .. } => *high_setpoint_c, _ => 0.0 }, item.schedule_id.is_some(), item.schedule_id.unwrap_or(crate::model::ScheduleId(0)))],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
