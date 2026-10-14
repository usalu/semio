//! ↩️ Inverse for `AddSpaceListMember` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::AddSpaceListMember, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::AddSpaceListMember> + From<crate::RemoveSpaceListMember>,
{
    Ok((|| {
    match base.model.space_lists.iter().find(|item| item.id == payload.id) {
        Some(item) if !item.space_ids.contains(&payload.space_id) && payload.index as usize <= item.space_ids.len() && base.model.spaces.iter().any(|row| row.id == payload.space_id) => {
            vec![vocabulary::remove_space_list_member(payload.id, payload.space_id)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
