//! ↩️ Inverse for `ChangeFenestrationFrameConductance` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFenestrationFrameConductance, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_frame_conductance_w_k.is_finite() || payload.new_frame_conductance_w_k < 0.0 || existing.frame_conductance_w_k == payload.new_frame_conductance_w_k {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_frame_conductance(payload.id, existing.frame_conductance_w_k)]
}
//#endregion 🔖️Inverse
