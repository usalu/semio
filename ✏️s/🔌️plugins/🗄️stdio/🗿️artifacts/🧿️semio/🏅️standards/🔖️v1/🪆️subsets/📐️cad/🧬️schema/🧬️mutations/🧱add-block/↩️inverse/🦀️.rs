//! ↩️ Inverse for `AddBlock`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::AddBlock, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::AddBlock { block } = payload;
    Ok(vec![SemioCadMutation::RemoveBlock(remove_block::RemoveBlock { name: block.name.clone() })])
}
//#endregion 🔖️Inverse
