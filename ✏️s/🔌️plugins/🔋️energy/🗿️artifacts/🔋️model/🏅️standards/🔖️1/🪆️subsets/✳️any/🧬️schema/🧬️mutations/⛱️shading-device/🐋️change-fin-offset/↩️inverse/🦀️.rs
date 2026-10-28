//! ↩️ Inverse for `ChangeFenestrationFinOffset` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeFenestrationFinOffset, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeFenestrationFinOffset>,
{
    Ok((|| {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_fin_offset_m.is_finite() || payload.new_fin_offset_m < 0.0 || existing.fin_offset_m == payload.new_fin_offset_m {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_fin_offset(payload.id, existing.fin_offset_m)]

    })())
}
//#endregion 🔖️Inverse
