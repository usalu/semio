//! ↩️ Inverse for `SetBlockContent`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetBlockContent, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::SetBlockContent { path, .. } = payload;
    Ok(match block_at(base, path) {
        Some(block) => vec![SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path: path.clone(), block: block.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
