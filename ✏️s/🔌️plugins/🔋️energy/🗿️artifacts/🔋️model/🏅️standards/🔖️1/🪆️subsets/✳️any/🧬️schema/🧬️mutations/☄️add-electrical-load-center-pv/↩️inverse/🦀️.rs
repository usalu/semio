//! ↩️ Inverse for `AddElectricalLoadCenterPv` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::AddElectricalLoadCenterPv, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) {
        Some(item) if !item.pv_ids.contains(&payload.pv_id) && payload.index as usize <= item.pv_ids.len() && base.model.pv_systems.iter().any(|row| row.id == payload.pv_id) => vec![vocabulary::remove_electrical_load_center_pv(payload.id, payload.pv_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
