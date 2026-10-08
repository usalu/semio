//! ↩️ Inverse for `RemoveBlock`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveBlock, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::RemoveBlock { name } = payload;
    Ok(match find_block(base, name) {
        Some(b) => vec![SemioCadMutation::AddBlock(add_block::AddBlock { block: b.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
