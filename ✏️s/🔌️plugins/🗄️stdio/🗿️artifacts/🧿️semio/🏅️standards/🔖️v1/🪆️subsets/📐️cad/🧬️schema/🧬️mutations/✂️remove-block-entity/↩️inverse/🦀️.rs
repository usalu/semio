//! ↩️ Inverse for `RemoveBlockEntity`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveBlockEntity, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::RemoveBlockEntity { block_name, handle } = payload;
    Ok(match find_block_entity(base, block_name, handle) {
        Some(e) => vec![SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity { block_name: block_name.clone(), entity: e.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
