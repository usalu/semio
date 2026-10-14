//! ↩️ Inverse for `ChangeFenestrationVlt` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeFenestrationVlt, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeFenestrationVlt>,
{
    Ok((|| {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_vlt.is_finite() || !(0.0..=1.0).contains(&payload.new_vlt) || existing.vlt == payload.new_vlt {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_vlt(payload.id, existing.vlt)]

    })())
}
//#endregion 🔖️Inverse
