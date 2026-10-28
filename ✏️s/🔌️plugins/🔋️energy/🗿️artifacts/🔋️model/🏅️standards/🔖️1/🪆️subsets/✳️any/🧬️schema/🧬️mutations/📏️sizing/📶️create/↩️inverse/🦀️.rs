//! ↩️ Inverse for `CreateSizingObject` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateSizingObject, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateSizingObject> + From<crate::DeleteSizingObject>,
{
    Ok((|| {
    if (base.model.sizing_objects.iter().any(|item| item.id == payload.id)) || (!base.model.zones.iter().any(|zone| zone.id == payload.zone_id)) {
        return Vec::new();
    }
    if payload.index.is_some_and(|index| index as usize > base.model.sizing_objects.len()) {
        return Vec::new();
    }
    vec![vocabulary::delete_sizing_object(payload.id)]

    })())
}
//#endregion 🔖️Inverse
