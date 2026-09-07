//! ↩️ Inverse for `ChangeFenestrationOverhangDepth` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFenestrationOverhangDepth, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_overhang_depth_m.is_finite() || payload.new_overhang_depth_m < 0.0 || existing.overhang_depth_m == payload.new_overhang_depth_m {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_overhang_depth(payload.id, existing.overhang_depth_m)]
}
//#endregion 🔖️Inverse
