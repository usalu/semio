//! ↩️ Inverse for `ChangeWaterSystemPeakFlow` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeWaterSystemPeakFlow, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.water_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.peak_flow_l_s != payload.new_peak_flow_l_s && !(!payload.new_peak_flow_l_s.is_finite() || payload.new_peak_flow_l_s <= 0.0) => vec![vocabulary::change_water_system_peak_flow(payload.id, item.peak_flow_l_s)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
