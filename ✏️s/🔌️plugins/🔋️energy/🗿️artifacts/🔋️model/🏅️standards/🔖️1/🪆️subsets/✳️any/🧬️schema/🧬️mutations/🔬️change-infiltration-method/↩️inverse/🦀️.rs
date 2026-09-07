//! ↩️ Inverse for `ChangeInfiltrationMethod` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeInfiltrationMethod, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.method != payload.new_method => vec![vocabulary::change_infiltration_method(payload.id, item.method)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
