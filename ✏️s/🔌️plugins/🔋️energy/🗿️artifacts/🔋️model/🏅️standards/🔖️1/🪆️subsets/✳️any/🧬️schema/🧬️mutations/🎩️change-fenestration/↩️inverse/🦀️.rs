//! ↩️ Inverse for `ChangeFenestrationOverhangOffset` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFenestrationOverhangOffset, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_overhang_offset_m.is_finite() || payload.new_overhang_offset_m < 0.0 || existing.overhang_offset_m == payload.new_overhang_offset_m {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_overhang_offset(payload.id, existing.overhang_offset_m)]
}
//#endregion 🔖️Inverse
