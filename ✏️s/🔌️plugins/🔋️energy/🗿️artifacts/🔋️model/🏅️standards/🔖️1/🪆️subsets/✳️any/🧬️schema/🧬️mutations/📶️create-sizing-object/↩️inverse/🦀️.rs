//! ↩️ Inverse for `CreateSizingObject` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateSizingObject, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if (base.model.sizing_objects.iter().any(|item| item.id == payload.id)) || (!base.model.zones.iter().any(|zone| zone.id == payload.zone_id)) {
        return Vec::new();
    }
    vec![vocabulary::delete_sizing_object(payload.id)]
}
//#endregion 🔖️Inverse
