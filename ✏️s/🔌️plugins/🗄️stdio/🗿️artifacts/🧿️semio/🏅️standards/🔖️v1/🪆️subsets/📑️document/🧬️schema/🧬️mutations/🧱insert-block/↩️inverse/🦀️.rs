//! ↩️ Inverse for `InsertBlock`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertBlock, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::InsertBlock { path, .. } = payload;
    Ok(vec![SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path: path.clone() })])
}
//#endregion 🔖️Inverse
