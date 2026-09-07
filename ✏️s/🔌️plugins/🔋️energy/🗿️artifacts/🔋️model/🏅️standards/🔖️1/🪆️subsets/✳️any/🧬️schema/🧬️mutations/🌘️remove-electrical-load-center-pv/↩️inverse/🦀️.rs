//! ↩️ Inverse for `RemoveElectricalLoadCenterPv` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RemoveElectricalLoadCenterPv, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(item) = base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let Some(position) = item.pv_ids.iter().position(|candidate| *candidate == payload.pv_id) else {
        return Vec::new();
    };
    vec![vocabulary::add_electrical_load_center_pv(payload.id, position as u32, payload.pv_id)]
}
//#endregion 🔖️Inverse
