//! ↩️ Inverse for `ChangeSizingObjectZone` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSizingObjectZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.sizing_objects.iter().find(|item| item.id == payload.id) {
        Some(item) if item.zone_id != payload.new_zone_id && !(!base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id)) => vec![vocabulary::change_sizing_object_zone(payload.id, item.zone_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
