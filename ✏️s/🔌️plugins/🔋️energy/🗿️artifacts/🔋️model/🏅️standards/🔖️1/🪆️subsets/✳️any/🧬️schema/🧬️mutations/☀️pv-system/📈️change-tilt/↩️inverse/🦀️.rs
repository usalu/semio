//! ↩️ Inverse for `ChangePvSystemTilt` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangePvSystemTilt, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangePvSystemTilt>,
{
    Ok((|| {
    match base.model.pv_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.tilt_deg != payload.new_tilt_deg && !(!payload.new_tilt_deg.is_finite() || !(0.0..=90.0).contains(&payload.new_tilt_deg)) => vec![vocabulary::change_pv_system_tilt(payload.id, item.tilt_deg)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
