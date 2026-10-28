//! ↩️ Inverse for `ChangeFenestrationSillHeight` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeFenestrationSillHeight, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeFenestrationSillHeight>,
{
    Ok((|| {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_sill_height_m.is_finite() || payload.new_sill_height_m < 0.0 || existing.sill_height_m == payload.new_sill_height_m {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_sill_height(payload.id, existing.sill_height_m)]

    })())
}
//#endregion 🔖️Inverse
