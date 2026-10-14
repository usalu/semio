//! ↩️ Inverse for `ChangeSizingObjectSizingType` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeSizingObjectSizingType, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeSizingObjectSizingType>,
{
    Ok((|| {
    match base.model.sizing_objects.iter().find(|item| item.id == payload.id) {
        Some(item) if item.sizing_type != payload.new_sizing_type => vec![vocabulary::change_sizing_object_sizing_type(payload.id, item.sizing_type)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
