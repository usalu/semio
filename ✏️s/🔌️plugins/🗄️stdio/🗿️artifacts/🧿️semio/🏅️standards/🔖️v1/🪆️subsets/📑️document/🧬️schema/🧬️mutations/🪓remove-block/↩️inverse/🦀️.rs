//! ↩️ Inverse for `RemoveBlock`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveBlock, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::RemoveBlock { path } = payload;
    Ok(match block_at(base, path) {
        Some(block) => vec![SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: path.clone(), block: block.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
