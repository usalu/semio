//! ↩️ Inverse for `ChangePvSystemTilt` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangePvSystemTilt, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.pv_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.tilt_deg != payload.new_tilt_deg && !(!payload.new_tilt_deg.is_finite() || !(0.0..=90.0).contains(&payload.new_tilt_deg)) => vec![vocabulary::change_pv_system_tilt(payload.id, item.tilt_deg)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
