//! ↩️ Inverse for `ChangePvSystemDcCapacity` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangePvSystemDcCapacity, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangePvSystemDcCapacity>,
{
    Ok((|| {
    match base.model.pv_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.dc_capacity_w != payload.new_dc_capacity_w && !(!payload.new_dc_capacity_w.is_finite() || payload.new_dc_capacity_w <= 0.0) => vec![vocabulary::change_pv_system_dc_capacity(payload.id, item.dc_capacity_w)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
