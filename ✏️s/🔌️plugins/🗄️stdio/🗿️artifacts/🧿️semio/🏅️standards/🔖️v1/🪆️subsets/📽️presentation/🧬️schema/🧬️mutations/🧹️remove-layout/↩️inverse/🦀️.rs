//! ↩️ Inverse for `RemoveLayout`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveLayout, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::RemoveLayout { id } = payload;
    Ok(match layout_at(base, id) {
        Some(l) => vec![SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout: l.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
