//! ↩️ Inverse for `RenameAirLoop` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::RenameAirLoop, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::RenameAirLoop>,
{
    Ok((|| {
    match base.model.air_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if item.name != payload.new_name && !((payload.new_name.trim().is_empty()) || (base.model.air_loops.iter().any(|item| item.id != payload.id && item.name == payload.new_name))) => {
            vec![vocabulary::rename_air_loop(payload.id, item.name.clone())]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
