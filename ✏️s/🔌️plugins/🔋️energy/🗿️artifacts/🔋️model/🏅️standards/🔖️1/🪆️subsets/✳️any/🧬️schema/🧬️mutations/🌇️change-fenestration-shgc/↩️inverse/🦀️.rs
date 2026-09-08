//! ↩️ Inverse for `ChangeFenestrationShgc` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFenestrationShgc, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_shgc.is_finite() || !(0.0..=1.0).contains(&payload.new_shgc) || existing.shgc == payload.new_shgc {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_shgc(payload.id, existing.shgc)]
}
//#endregion 🔖️Inverse
