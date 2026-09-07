//! ↩️ Inverse for `ChangeFenestrationHeight` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFenestrationHeight, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_height_m.is_finite() || payload.new_height_m <= 0.0 || existing.height_m == payload.new_height_m {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_height(payload.id, existing.height_m)]
}
//#endregion 🔖️Inverse
