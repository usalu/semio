//! ↩️ Inverse for `DeleteSpace` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteSpace, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteSpace> + From<crate::CreateSpace>,
{
    Ok((|| {
    let Some((index, existing)) = base.model.spaces.iter().enumerate().find(|(_, item)| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.space_lists.iter().any(|item| item.space_ids.contains(&payload.id)) || false {
        return Vec::new();
    }
    vec![vocabulary::create_space(existing.id, existing.name.clone(), existing.zone_id, existing.floor_area_m2, Some(index as u32))]

    })())
}
//#endregion 🔖️Inverse
