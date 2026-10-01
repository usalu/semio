//! ↩️ Inverse for `ChangeFenestrationUValue` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFenestrationUValue, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_u_value_w_m2k.is_finite() || payload.new_u_value_w_m2k <= 0.0 || existing.u_value_w_m2k == payload.new_u_value_w_m2k {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_u_value(payload.id, existing.u_value_w_m2k)]
}
//#endregion 🔖️Inverse
