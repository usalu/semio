//! ↩️ Inverse for `AddBlockEntity`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::AddBlockEntity, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::AddBlockEntity { block_name, entity, .. } = payload;
    Ok(vec![SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name: block_name.clone(), handle: entity.handle.clone() })])
}
//#endregion 🔖️Inverse
