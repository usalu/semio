//! ↩️ Inverse for `DeleteSizingObject` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteSizingObject, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.sizing_objects.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![vocabulary::create_sizing_object(item.id, item.zone_id, item.sizing_type, item.design_day_type)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
