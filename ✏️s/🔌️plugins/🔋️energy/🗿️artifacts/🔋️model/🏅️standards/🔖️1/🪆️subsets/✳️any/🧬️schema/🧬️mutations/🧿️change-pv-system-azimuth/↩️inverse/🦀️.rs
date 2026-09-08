//! ↩️ Inverse for `ChangePvSystemAzimuth` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangePvSystemAzimuth, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.pv_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.azimuth_deg != payload.new_azimuth_deg && !(!payload.new_azimuth_deg.is_finite() || !(0.0..=360.0).contains(&payload.new_azimuth_deg)) => vec![vocabulary::change_pv_system_azimuth(payload.id, item.azimuth_deg)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
