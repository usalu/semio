//! ↩️ Inverse for `CreatePvSystem` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreatePvSystem, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreatePvSystem> + From<crate::DeletePvSystem>,
{
    Ok((|| {
    if base.model.pv_systems.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.pv_systems.len() || !payload.temperature_coefficient.is_finite() || !payload.faiman_heat_loss_constant_w_m2k.is_finite() || payload.faiman_heat_loss_constant_w_m2k <= 0.0 || !payload.faiman_heat_loss_wind_w_s_m3k.is_finite() || payload.faiman_heat_loss_wind_w_s_m3k < 0.0 {
        return Vec::new();
    }
    vec![vocabulary::delete_pv_system(payload.id)]

    })())
}
//#endregion 🔖️Inverse
