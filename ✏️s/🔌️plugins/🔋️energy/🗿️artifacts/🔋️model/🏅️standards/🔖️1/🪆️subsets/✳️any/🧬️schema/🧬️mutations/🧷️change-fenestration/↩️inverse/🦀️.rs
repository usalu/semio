//! ↩️ Inverse for `ChangeFenestrationDividerConductance` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFenestrationDividerConductance, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_divider_conductance_w_k.is_finite() || payload.new_divider_conductance_w_k < 0.0 || existing.divider_conductance_w_k == payload.new_divider_conductance_w_k {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_divider_conductance(payload.id, existing.divider_conductance_w_k)]
}
//#endregion 🔖️Inverse
