//! ↩️ Inverse for `ChangeInfiltrationEffectiveLeakageArea` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeInfiltrationEffectiveLeakageArea, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.effective_leakage_area_m2 != payload.new_effective_leakage_area_m2 && !(!payload.new_effective_leakage_area_m2.is_finite() || payload.new_effective_leakage_area_m2 < 0.0) => vec![vocabulary::change_infiltration_effective_leakage_area(payload.id, item.effective_leakage_area_m2)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
