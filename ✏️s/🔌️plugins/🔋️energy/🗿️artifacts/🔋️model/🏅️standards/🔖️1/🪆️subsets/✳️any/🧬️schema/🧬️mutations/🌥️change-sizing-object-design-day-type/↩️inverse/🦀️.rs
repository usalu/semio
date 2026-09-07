//! ↩️ Inverse for `ChangeSizingObjectDesignDayType` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSizingObjectDesignDayType, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.sizing_objects.iter().find(|item| item.id == payload.id) {
        Some(item) if item.design_day_type != payload.new_design_day_type => vec![vocabulary::change_sizing_object_design_day_type(payload.id, item.design_day_type)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
