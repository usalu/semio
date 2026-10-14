//! ↩️ Inverse for `ClearFenestrationGlazingConstruction` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ClearFenestrationGlazingConstruction, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ClearFenestrationGlazingConstruction> + From<crate::BindFenestrationGlazingConstruction>,
{
    Ok((|| {
    match base.model.fenestrations.iter().find(|item| item.id == payload.id).and_then(|item| item.glazing_construction_id) {
        Some(previous) => vec![vocabulary::bind_fenestration_glazing_construction(payload.id, previous)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
