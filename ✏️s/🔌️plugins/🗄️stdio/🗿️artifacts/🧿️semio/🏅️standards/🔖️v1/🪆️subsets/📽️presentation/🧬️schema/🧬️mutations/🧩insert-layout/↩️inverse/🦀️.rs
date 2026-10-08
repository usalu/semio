//! ↩️ Inverse for `InsertLayout`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertLayout, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::InsertLayout { layout, .. } = payload;
    Ok(vec![SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id: layout.id.clone() })])
}
//#endregion 🔖️Inverse
