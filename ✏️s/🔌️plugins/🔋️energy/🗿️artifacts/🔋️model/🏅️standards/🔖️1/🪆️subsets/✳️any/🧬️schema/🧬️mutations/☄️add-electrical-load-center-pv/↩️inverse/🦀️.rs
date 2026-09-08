//! ↩️ Inverse for `AddElectricalLoadCenterPv` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::AddElectricalLoadCenterPv, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) {
        Some(item) if !item.pv_ids.contains(&payload.pv_id) && payload.index as usize <= item.pv_ids.len() && base.model.pv_systems.iter().any(|row| row.id == payload.pv_id) => vec![vocabulary::remove_electrical_load_center_pv(payload.id, payload.pv_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
