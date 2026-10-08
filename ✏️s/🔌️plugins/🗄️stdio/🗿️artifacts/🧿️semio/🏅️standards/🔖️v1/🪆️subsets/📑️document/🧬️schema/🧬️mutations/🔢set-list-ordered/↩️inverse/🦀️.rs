//! ↩️ Inverse for `SetListOrdered`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetListOrdered, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::SetListOrdered { path, .. } = payload;
    Ok(match block_at(base, path) {
        Some(DocBlock::List { ordered, .. }) => vec![SemioDocumentMutation::SetListOrdered(set_list_ordered::SetListOrdered { path: path.clone(), ordered: *ordered })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
