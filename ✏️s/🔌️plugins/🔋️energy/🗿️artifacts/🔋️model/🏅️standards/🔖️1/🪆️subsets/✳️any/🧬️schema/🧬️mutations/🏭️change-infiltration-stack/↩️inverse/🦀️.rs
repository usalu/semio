//! ↩️ Inverse for `ChangeInfiltrationStackHeight` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeInfiltrationStackHeight, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.stack_height_m != payload.new_stack_height_m && !(!payload.new_stack_height_m.is_finite() || payload.new_stack_height_m < 0.0) => vec![vocabulary::change_infiltration_stack_height(payload.id, item.stack_height_m)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
